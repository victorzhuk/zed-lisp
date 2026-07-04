use zed_extension_api::lsp::{Completion, CompletionKind};
use zed_extension_api::settings::LspSettings;
use zed_extension_api::{
    self as zed, set_language_server_installation_status, CodeLabel, CodeLabelSpan,
    LanguageServerId, LanguageServerInstallationStatus, Worktree,
};

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
        _language_server_id: &LanguageServerId,
        completion: Completion,
    ) -> Option<CodeLabel> {
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

zed::register_extension!(CommonLispExtension);
