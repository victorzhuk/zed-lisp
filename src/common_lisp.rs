use zed_extension_api::lsp::{Completion, CompletionKind};
use zed_extension_api::settings::LspSettings;
use zed_extension_api::{
    self as zed, set_language_server_installation_status, CodeLabel, CodeLabelSpan,
    LanguageServerId, LanguageServerInstallationStatus, Worktree,
};

struct CommonLispExtension;

impl zed::Extension for CommonLispExtension {
    fn new() -> Self {
        Self
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

        if let Some(cl_lsp_path) = worktree.which("cl-lsp") {
            return Ok(zed::Command {
                command: cl_lsp_path,
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
                .args(["install", "cxxxr/cl-lsp"])
                .output();

            match output {
                Ok(output) if output.status == Some(0) => {
                    if let Some(cl_lsp_path) = worktree.which("cl-lsp") {
                        set_language_server_installation_status(
                            language_server_id,
                            &LanguageServerInstallationStatus::None,
                        );
                        return Ok(zed::Command {
                            command: cl_lsp_path,
                            args,
                            env,
                        });
                    }
                    return Err(
                        "cl-lsp installed via Roswell but not found on PATH. Add ~/.roswell/bin to PATH.".into()
                    );
                }
                Ok(output) => {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    let msg = if stderr.trim().is_empty() {
                        "ros install exited with non-zero status".to_string()
                    } else {
                        stderr.to_string()
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
                            "install cl-lsp via roswell: {}",
                            err
                        )),
                    );
                }
            }
        }

        Err(
            "cl-lsp not found. Please either:\n\
             1. Install cl-lsp manually and ensure it's on your PATH, or\n\
             2. Install Roswell (ros) and let the extension install cl-lsp automatically, or\n\
             3. Configure the path in your Zed settings: {\"lsp\": {\"cl-lsp\": {\"binary\": {\"path\": \"/path/to/cl-lsp\"}}}}}".into()
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

zed::register_extension!(CommonLispExtension);
