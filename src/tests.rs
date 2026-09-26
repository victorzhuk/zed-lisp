use super::{dispatch_language_server, resolve_lispico_command, ServerKind};
use zed_extension_api as zed;

fn command(path: &str, args: &[&str]) -> zed::Command {
    zed::Command {
        command: path.to_string(),
        args: args.iter().map(|a| a.to_string()).collect(),
        env: Vec::new(),
    }
}

#[test]
fn dispatch_maps_only_registered_server_ids() {
    assert_eq!(dispatch_language_server("lispico"), Ok(ServerKind::Lispico));
    assert_eq!(dispatch_language_server("sextant"), Ok(ServerKind::Sextant));
}

#[test]
fn unknown_server_ids_do_not_fall_through_to_sextant() {
    for id in ["lispico-lsp", "sextant2", "lispico ", "", "Lispico"] {
        let dispatch = dispatch_language_server(id);
        assert!(
            dispatch.is_err(),
            "id {id:?} must not resolve to any server"
        );
        assert!(
            !dispatch.unwrap_err().contains("sextant not found"),
            "the unknown-id error must not suggest a sextant launch"
        );
    }
}

#[test]
fn configured_lispico_binary_wins_over_path() {
    let configured = command("/opt/lispico/bin/lispico-lsp", &["--project"]);
    let resolved = resolve_lispico_command(
        Some(configured.clone()),
        vec!["--project".to_string()],
        vec![],
        |_| Some("/usr/bin/lispico-lsp".to_string()),
    )
    .unwrap();

    assert_eq!(resolved.command, configured.command);
    assert_eq!(resolved.args, configured.args);
    assert_eq!(resolved.env, configured.env);
}

#[test]
fn path_lispico_binary_receives_configured_arguments_and_env() {
    let resolved = resolve_lispico_command(
        None,
        vec!["--project".to_string(), ".lispico.json".to_string()],
        vec![("LISPICO_LOG".to_string(), "debug".to_string())],
        |name| {
            assert_eq!(name, "lispico-lsp");
            Some("/usr/local/bin/lispico-lsp".to_string())
        },
    )
    .unwrap();

    assert_eq!(resolved.command, "/usr/local/bin/lispico-lsp");
    assert_eq!(resolved.args, vec!["--project", ".lispico.json"]);
    assert_eq!(
        resolved.env,
        vec![("LISPICO_LOG".to_string(), "debug".to_string())]
    );
}

#[test]
fn missing_lispico_binary_is_an_actionable_error_without_fallback() {
    let err = resolve_lispico_command(None, Vec::new(), Vec::new(), |_| None).unwrap_err();

    assert!(err.contains("lispico-lsp not found"));
    assert!(err.contains("\"binary\""));
    assert!(!err.to_lowercase().contains("download"));
    assert!(!err.to_lowercase().contains("roswell"));
    assert!(!err.to_lowercase().contains("sextant"));
}
