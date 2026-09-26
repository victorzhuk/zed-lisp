use zed_extension_api::lsp::{Completion, CompletionKind};
use zed_extension_api::settings::LspSettings;
use zed_extension_api::{
    self as zed, set_language_server_installation_status, CodeLabel, CodeLabelSpan,
    LanguageServerId, LanguageServerInstallationStatus, Worktree,
};

const LISPICO_SERVER_BINARY: &str = "lispico-lsp";

/// Chooses the server command for a language server id. The Lispico and
/// sextant servers are fully independent: an unknown id never falls through
/// to sextant, and the Lispico path never reaches sextant's download or
/// Roswell fallback.
fn dispatch_language_server(id: &str) -> Result<ServerKind, String> {
    match id {
        "lispico" => Ok(ServerKind::Lispico),
        "sextant" => Ok(ServerKind::Sextant),
        other => Err(format!(
            "unknown language server id: {other}. \
             This extension only provides the 'sextant' and 'lispico' servers."
        )),
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ServerKind {
    Lispico,
    Sextant,
}

/// Resolves the Lispico server command: configured binary path first, then
/// `lispico-lsp` on PATH. There is no download, build, Roswell, or sextant
/// fallback: a missing server leaves the structural (Tree-sitter) features in
/// place and reports one actionable error. Configured arguments and
/// environment apply to both resolution paths.
fn resolve_lispico_command(
    configured: Option<zed::Command>,
    args: Vec<String>,
    env: Vec<(String, String)>,
    which: impl Fn(&str) -> Option<String>,
) -> Result<zed::Command, String> {
    if let Some(command) = configured {
        return Ok(command);
    }

    if let Some(path) = which(LISPICO_SERVER_BINARY) {
        return Ok(zed::Command {
            command: path,
            args,
            env,
        });
    }

    Err(format!(
        "{LISPICO_SERVER_BINARY} not found. Install go-lispico and put {LISPICO_SERVER_BINARY} \
         on your PATH, or set the binary path in Zed settings:\n\
         {{\"lsp\": {{\"lispico\": {{\"binary\": {{\"path\": \"/path/to/{LISPICO_SERVER_BINARY}\"}}}}}}}}\n\
         Structural highlighting remains available without the server."
    ))
}

struct CommonLispExtension {
    cached_binary_path: Option<String>,
}

impl zed::Extension for CommonLispExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<zed::Command> {
        match dispatch_language_server(language_server_id.as_ref())? {
            ServerKind::Lispico => self.lispico_command(language_server_id, worktree),
            ServerKind::Sextant => self.sextant_command(language_server_id, worktree),
        }
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<Option<zed::serde_json::Value>> {
        let lsp_settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;
        Ok(lsp_settings.initialization_options)
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<Option<zed::serde_json::Value>> {
        let lsp_settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;
        Ok(lsp_settings.settings)
    }

    fn label_for_completion(
        &self,
        language_server_id: &LanguageServerId,
        completion: Completion,
    ) -> Option<CodeLabel> {
        // The Lispico server formats its own completion labels; only sextant's
        // label/detail shape is rendered here.
        if language_server_id.as_ref() != "sextant" {
            return None;
        }

        let kind = completion.kind?;

        match kind {
            CompletionKind::Function | CompletionKind::Method => {
                let label = completion.label;
                let detail = completion.detail.as_ref()?;
                let code = format!("{} {}", label, detail);

                Some(CodeLabel {
                    code,
                    spans: vec![
                        CodeLabelSpan::literal(label.clone(), Some("function".to_string())),
                        CodeLabelSpan::literal(format!(" {}", detail), None),
                    ],
                    filter_range: (0..label.len()).into(),
                })
            }
            _ => None,
        }
    }
}

impl CommonLispExtension {
    /// Launches the native Lispico language server with configured
    /// arguments, environment, and pass-through settings.
    fn lispico_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<zed::Command> {
        let lsp_settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;

        let args = lsp_settings
            .binary
            .as_ref()
            .and_then(|b| b.arguments.clone())
            .unwrap_or_default();
        let env: Vec<(String, String)> = lsp_settings
            .binary
            .as_ref()
            .and_then(|b| b.env.clone())
            .map(|h| h.into_iter().collect())
            .unwrap_or_default();
        let configured = lsp_settings
            .binary
            .and_then(|b| b.path)
            .map(|path| zed::Command {
                command: path,
                args: args.clone(),
                env: env.clone(),
            });

        resolve_lispico_command(configured, args, env, |name| worktree.which(name))
    }

    fn sextant_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<zed::Command> {
        let lsp_settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;

        let args = lsp_settings
            .binary
            .as_ref()
            .and_then(|b| b.arguments.clone())
            .unwrap_or_default();
        let env: Vec<(String, String)> = lsp_settings
            .binary
            .as_ref()
            .and_then(|b| b.env.clone())
            .map(|h| h.into_iter().collect())
            .unwrap_or_default();

        if let Some(path) = lsp_settings.binary.and_then(|b| b.path) {
            return Ok(zed::Command {
                command: path,
                args,
                env,
            });
        }

        if let Some(sextant_path) = worktree.which("sextant") {
            return Ok(zed::Command {
                command: sextant_path,
                args,
                env,
            });
        }

        if let Some(sextant_path) = self.download_sextant(language_server_id)? {
            return Ok(zed::Command {
                command: sextant_path,
                args,
                env,
            });
        }

        if let Some(ros_path) = worktree.which("ros") {
            set_language_server_installation_status(
                language_server_id,
                &LanguageServerInstallationStatus::Downloading,
            );

            let output = zed::process::Command::new(ros_path)
                .args(["install", "victorzhuk/sextant"])
                .output();

            match output {
                Ok(output) if output.status == Some(0) => {
                    if let Some(sextant_path) = worktree.which("sextant") {
                        set_language_server_installation_status(
                            language_server_id,
                            &LanguageServerInstallationStatus::None,
                        );
                        return Ok(zed::Command {
                            command: sextant_path,
                            args,
                            env,
                        });
                    }
                    return Err("sextant built via Roswell but not found on PATH. \
                                Add ~/.roswell/bin to PATH."
                        .into());
                }
                Ok(output) => {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    let msg = if stderr.trim().is_empty() {
                        "ros install victorzhuk/sextant exited with a non-zero status".to_string()
                    } else {
                        stderr.into_owned()
                    };
                    set_language_server_installation_status(
                        language_server_id,
                        &LanguageServerInstallationStatus::Failed(msg),
                    );
                }
                Err(err) => {
                    set_language_server_installation_status(
                        language_server_id,
                        &LanguageServerInstallationStatus::Failed(format!(
                            "build sextant via Roswell: {}",
                            err
                        )),
                    );
                }
            }
        }

        Err(
            "sextant not found on PATH and Roswell (ros) is unavailable to build it. \
             Install Roswell, then run:\n\
             ros install victorzhuk/sextant\n\
             and add ~/.roswell/bin to PATH, or set the binary path in Zed settings:\n\
             {\"lsp\": {\"sextant\": {\"binary\": {\"path\": \"/path/to/sextant\"}}}}"
                .into(),
        )
    }

    fn download_sextant(
        &mut self,
        language_server_id: &LanguageServerId,
    ) -> zed::Result<Option<String>> {
        if let Some(path) = &self.cached_binary_path {
            if std::fs::metadata(path).is_ok_and(|stat| stat.is_file()) {
                return Ok(Some(path.clone()));
            }
        }

        let asset_name = match zed::current_platform() {
            (zed::Os::Linux, zed::Architecture::X8664) => "sextant-linux-x64",
            (zed::Os::Linux, zed::Architecture::Aarch64) => "sextant-linux-arm64",
            (zed::Os::Mac, zed::Architecture::Aarch64) => "sextant-macos-arm64",
            _ => return Ok(None),
        };

        let release = match zed::latest_github_release(
            "victorzhuk/sextant",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        ) {
            Ok(release) => release,
            Err(_) => return Ok(None),
        };

        let Some(asset) = release.assets.iter().find(|asset| asset.name == asset_name) else {
            return Ok(None);
        };

        let version_dir = format!("sextant-{}", release.version);
        let binary_path = format!("{version_dir}/sextant");

        if !std::fs::metadata(&binary_path).is_ok_and(|stat| stat.is_file()) {
            set_language_server_installation_status(
                language_server_id,
                &LanguageServerInstallationStatus::Downloading,
            );
            zed::download_file(
                &asset.download_url,
                &binary_path,
                zed::DownloadedFileType::Uncompressed,
            )
            .map_err(|err| format!("download sextant {}: {err}", release.version))?;
            zed::make_file_executable(&binary_path)?;

            if let Ok(entries) = std::fs::read_dir(".") {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().into_owned();
                    if name.starts_with("sextant-") && name != version_dir {
                        std::fs::remove_dir_all(entry.path()).ok();
                    }
                }
            }

            set_language_server_installation_status(
                language_server_id,
                &LanguageServerInstallationStatus::None,
            );
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(Some(binary_path))
    }
}

/// Raw grammar entry points for the Rust test harness. The parser objects are
/// compiled from the pinned grammar submodules by `build.rs` and linked
/// through this crate's rlib target; the wasm extension build never
/// references them.
#[cfg(not(target_arch = "wasm32"))]
pub mod testing {
    extern "C" {
        pub fn tree_sitter_commonlisp() -> *const ();
        pub fn tree_sitter_clojure() -> *const ();
    }
}

#[cfg(test)]
mod tests;

zed::register_extension!(CommonLispExtension);
