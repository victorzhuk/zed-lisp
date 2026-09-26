use super::{dispatch_language_server, resolve_lispico_command, CommonLispExtension, ServerKind};
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

#[test]
fn cached_sextant_dir_picks_newest_version_with_binary() {
    let entries = [
        ("sextant-0.1.0".to_string(), true),
        ("sextant-0.10.0".to_string(), true),
        ("sextant-0.9.0".to_string(), true),
        ("sextant-0.11.0".to_string(), false),
        ("sextant".to_string(), true),
        ("grammar".to_string(), true),
    ];

    assert_eq!(
        CommonLispExtension::latest_cached_sextant_dir(entries),
        Some("0.10.0".to_string())
    );
}

#[test]
fn cached_sextant_dir_is_none_without_a_usable_download() {
    let only_empty_dir = [("sextant-1.2.3".to_string(), false)];
    let no_sextant_dirs = [("grammar".to_string(), true), ("".to_string(), false)];

    assert_eq!(
        CommonLispExtension::latest_cached_sextant_dir(only_empty_dir),
        None
    );
    assert_eq!(
        CommonLispExtension::latest_cached_sextant_dir(no_sextant_dirs),
        None
    );
}

#[cfg(unix)]
#[test]
fn executable_file_requires_an_executable_bit() {
    use std::os::unix::fs::PermissionsExt;

    let dir = std::env::temp_dir().join(format!("sextant-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("sextant");
    std::fs::write(&path, b"#!/bin/sh\n").unwrap();

    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).unwrap();
    assert!(!CommonLispExtension::executable_file(&path));

    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
    assert!(CommonLispExtension::executable_file(&path));

    std::fs::remove_dir_all(&dir).unwrap();
}

#[cfg(unix)]
#[test]
fn cached_sextant_dir_skips_non_executable_cache_binary() {
    use std::os::unix::fs::PermissionsExt;
    use std::sync::Mutex;

    static CWD_LOCK: Mutex<()> = Mutex::new(());
    let _guard = CWD_LOCK.lock().unwrap();

    let original = std::env::current_dir().unwrap();
    let dir = std::env::temp_dir().join(format!("sextant-cache-{}", std::process::id()));
    let stale = dir.join("sextant-1.0.0");
    let fresh = dir.join("sextant-0.9.0");
    std::fs::create_dir_all(&stale).unwrap();
    std::fs::create_dir_all(&fresh).unwrap();
    std::fs::write(stale.join("sextant"), b"#!/bin/sh\n").unwrap();
    std::fs::write(fresh.join("sextant"), b"#!/bin/sh\n").unwrap();
    std::fs::set_permissions(
        stale.join("sextant"),
        std::fs::Permissions::from_mode(0o644),
    )
    .unwrap();
    std::fs::set_permissions(
        fresh.join("sextant"),
        std::fs::Permissions::from_mode(0o755),
    )
    .unwrap();

    std::env::set_current_dir(&dir).unwrap();
    let picked = CommonLispExtension::cached_sextant_dir();
    std::env::set_current_dir(original).unwrap();
    std::fs::remove_dir_all(&dir).unwrap();

    assert_eq!(picked, Some("0.9.0".to_string()));
}
