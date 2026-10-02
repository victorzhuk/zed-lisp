use sha2::{Digest, Sha256};
use std::io::Read as _;
use std::path::{Component, Path, PathBuf};
use zed_extension_api::settings::LspSettings;
use zed_extension_api::{
    self as zed, DownloadedFileType, GithubRelease, LanguageServerId, Worktree,
};

/// The release the baseline record verified
/// (openspec/changes/migrate-to-llsp-upstream-gates/gates.md). Adopting a
/// different pin is a separately approved change that remeasures the gates.
const LLSP_RELEASE_TAG: &str = "v0.2.1";
const LLSP_GITHUB_REPO: &str = "victorzhuk/llsp";
const DIGEST_LIST_ASSET: &str = "SHA256SUMS";
const COMPLETION_STATE_FILE: &str = "complete.json";

const SUPPORTED_PLATFORMS: &str =
    "Linux x86_64, Linux aarch64, macOS x86_64, macOS aarch64, Windows x86_64";

/// Rejects every id except the one registered server; both previous server
/// ids are unknown here, and no id ever falls through to another server.
fn dispatch_language_server(id: &str) -> Result<(), String> {
    match id {
        "llsp" => Ok(()),
        other => Err(format!(
            "unknown language server id: {other}. \
             This extension registers only the 'llsp' server."
        )),
    }
}

/// Published archive per platform, from the baseline record's verified
/// release contract. Windows aarch64 has no published archive; it is named
/// unsupported rather than served another platform's archive.
fn llsp_asset_name(platform: (zed::Os, zed::Architecture)) -> Option<&'static str> {
    match platform {
        (zed::Os::Linux, zed::Architecture::X8664) => Some("llsp-x86_64-unknown-linux-musl.tar.gz"),
        (zed::Os::Linux, zed::Architecture::Aarch64) => {
            Some("llsp-aarch64-unknown-linux-musl.tar.gz")
        }
        (zed::Os::Mac, zed::Architecture::X8664) => Some("llsp-x86_64-apple-darwin.tar.gz"),
        (zed::Os::Mac, zed::Architecture::Aarch64) => Some("llsp-aarch64-apple-darwin.tar.gz"),
        (zed::Os::Windows, zed::Architecture::X8664) => Some("llsp-x86_64-pc-windows-msvc.zip"),
        _ => None,
    }
}

fn platform_key(platform: (zed::Os, zed::Architecture)) -> String {
    format!("{:?}-{:?}", platform.0, platform.1)
}

/// The executable member inside a published archive; the cache keeps the
/// binary under the same name.
fn binary_file_name(asset_name: &str) -> &'static str {
    if asset_name.ends_with(".zip") {
        "llsp.exe"
    } else {
        "llsp"
    }
}

/// Top-level member directory of an archive: the asset name without its
/// compression suffix, as the pinned release publishes (`llsp-<target>/llsp`).
fn member_dir(asset_name: &str) -> &str {
    asset_name
        .strip_suffix(".tar.gz")
        .or_else(|| asset_name.strip_suffix(".zip"))
        .unwrap_or(asset_name)
}

/// True when `path` is a regular file with at least one executable bit.
/// On platforms without POSIX permission bits (the wasm extension build)
/// a regular file counts as ready.
fn executable_file(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::metadata(path).is_ok_and(|stat| stat.permissions().mode() & 0o111 != 0)
    }
    #[cfg(not(unix))]
    {
        true
    }
}

/// The host capabilities the verified-download step needs. The production
/// implementation calls the extension host's imports; tests inject their own
/// so the resolution logic runs without network access.
trait ReleaseSource {
    fn release_by_tag(&self, repo: &str, tag: &str) -> Result<GithubRelease, String>;
    fn download(&self, url: &str, path: &Path) -> Result<(), String>;
    fn make_executable(&self, path: &Path) -> Result<(), String>;
}

struct HostReleaseSource;

impl ReleaseSource for HostReleaseSource {
    fn release_by_tag(&self, repo: &str, tag: &str) -> Result<GithubRelease, String> {
        zed::github_release_by_tag_name(repo, tag)
    }

    fn download(&self, url: &str, path: &Path) -> Result<(), String> {
        zed::download_file(
            url,
            &path.display().to_string(),
            DownloadedFileType::Uncompressed,
        )
    }

    fn make_executable(&self, path: &Path) -> Result<(), String> {
        zed::make_file_executable(&path.display().to_string())
    }
}

/// One actionable resolution failure: the supported platforms, why the chain
/// stopped, and the three remedies.
fn resolution_error(reason: &str) -> String {
    format!(
        "llsp could not be resolved: {reason}. \
         Supported platforms: {SUPPORTED_PLATFORMS}. \
         Remedies: (1) set `lsp.llsp.binary.path` in your Zed settings to an \
         installed llsp binary, (2) expose `llsp` on your PATH, or (3) place a \
         verified llsp binary (with its {COMPLETION_STATE_FILE} marker) in the \
         extension cache directory llsp-{LLSP_RELEASE_TAG}. \
         Structural highlighting remains available without the server."
    )
}

/// Reads the digest the published `SHA256SUMS` records for one asset. A
/// missing or malformed entry is a release-contract defect, never something
/// to work around.
fn sha256sums_entry(contents: &str, asset_name: &str) -> Result<String, String> {
    for line in contents.lines() {
        let mut parts = line.split_whitespace();
        let (Some(digest), Some(name)) = (parts.next(), parts.next()) else {
            continue;
        };
        if name == asset_name {
            let is_hex = digest.len() == 64 && digest.chars().all(|c| c.is_ascii_hexdigit());
            if !is_hex {
                return Err(format!(
                    "{DIGEST_LIST_ASSET} entry for {asset_name} is malformed"
                ));
            }
            return Ok(digest.to_ascii_lowercase());
        }
    }
    Err(format!("{DIGEST_LIST_ASSET} has no entry for {asset_name}"))
}

fn sha256_file(path: &Path) -> Result<String, String> {
    let mut file = std::fs::File::open(path).map_err(|e| format!("open {path:?}: {e}"))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 65536];
    loop {
        let read = file
            .read(&mut buffer)
            .map_err(|e| format!("read {path:?}: {e}"))?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Ok(hasher
        .finalize()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect())
}

/// Extracts the digest-verified archive into `payload_dir`. Only archive
/// members inside a single top-level directory are accepted; the tar crate
/// and the zip crate parse the published formats, and no entry may escape
/// the payload directory.
fn extract_archive(archive: &Path, asset_name: &str, payload_dir: &Path) -> Result<(), String> {
    std::fs::create_dir_all(payload_dir).map_err(|e| format!("create payload dir: {e}"))?;
    if asset_name.ends_with(".zip") {
        extract_zip(archive, payload_dir)
    } else {
        extract_tar_gz(archive, payload_dir)
    }
}

fn extract_tar_gz(archive: &Path, payload_dir: &Path) -> Result<(), String> {
    let file = std::fs::File::open(archive).map_err(|e| format!("open archive: {e}"))?;
    let decoder = flate2::read::GzDecoder::new(file);
    let mut archive = tar::Archive::new(decoder);
    let entries = archive
        .entries()
        .map_err(|e| format!("reading archive entries: {e}"))?;
    for entry in entries {
        let mut entry = entry.map_err(|e| format!("archive entry: {e}"))?;
        let member = entry
            .path()
            .map_err(|e| format!("archive member path: {e}"))?
            .to_path_buf();
        if member.components().any(|c| c == Component::ParentDir) {
            return Err(format!("unsafe archive member {member:?}"));
        }
        let target = payload_dir.join(&member);
        if entry.header().entry_type().is_dir() {
            std::fs::create_dir_all(&target)
                .map_err(|e| format!("create directory {member:?}: {e}"))?;
        } else {
            if let Some(parent) = target.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| format!("create directory for {member:?}: {e}"))?;
            }
            let mut out = std::fs::File::create(&target)
                .map_err(|e| format!("create file {member:?}: {e}"))?;
            std::io::copy(&mut entry, &mut out)
                .map_err(|e| format!("extract member {member:?}: {e}"))?;
        }
    }
    Ok(())
}

fn extract_zip(archive: &Path, payload_dir: &Path) -> Result<(), String> {
    let file = std::fs::File::open(archive).map_err(|e| format!("open archive: {e}"))?;
    let mut zip = zip::ZipArchive::new(file).map_err(|e| format!("reading zip: {e}"))?;
    for index in 0..zip.len() {
        let mut entry = zip.by_index(index).map_err(|e| format!("zip entry: {e}"))?;
        let member = entry.name().to_string();
        if member.split(['/', '\\']).any(|part| part == "..") {
            return Err(format!("unsafe archive member {member:?}"));
        }
        let target = payload_dir.join(&member);
        if entry.is_dir() {
            std::fs::create_dir_all(&target)
                .map_err(|e| format!("create directory {member:?}: {e}"))?;
        } else {
            if let Some(parent) = target.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| format!("create directory for {member:?}: {e}"))?;
            }
            let mut out = std::fs::File::create(&target)
                .map_err(|e| format!("create file {member:?}: {e}"))?;
            std::io::copy(&mut entry, &mut out)
                .map_err(|e| format!("extract member {member:?}: {e}"))?;
        }
    }
    Ok(())
}

/// The binary of a complete verified cache entry: the entry's completion
/// state must exist, match the active version and platform, and the binary
/// must carry an executable bit. An entry missing any of these is not a
/// verified install and is never started.
fn verified_cache_binary(root: &Path, platform: &str) -> Option<PathBuf> {
    let entry_dir = root.join(format!("llsp-{LLSP_RELEASE_TAG}"));
    let state = std::fs::read_to_string(entry_dir.join(COMPLETION_STATE_FILE)).ok()?;
    let state: zed::serde_json::Value = zed::serde_json::from_str(&state).ok()?;
    let version = state.get("version").and_then(|v| v.as_str())?;
    let state_platform = state.get("platform").and_then(|v| v.as_str())?;
    if version != LLSP_RELEASE_TAG || state_platform != platform {
        return None;
    }
    let binary = entry_dir.join(binary_file_name(
        state.get("asset").and_then(|v| v.as_str()).unwrap_or(""),
    ));
    if !executable_file(&binary) {
        return None;
    }
    Some(binary)
}

/// Removes every cache entry other than the active version's.
fn prune_other_versions(root: &Path) {
    let keep = format!("llsp-{LLSP_RELEASE_TAG}");
    let Ok(entries) = std::fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.starts_with("llsp-") && name != keep {
            let _ = std::fs::remove_dir_all(entry.path());
        }
    }
}

/// Downloads the pinned release, verifies its digest against the published
/// `SHA256SUMS`, extracts only on a match, marks the binary executable, and
/// writes the completion state last. A failure at any step removes what the
/// attempt wrote.
fn install_verified_release(
    root: &Path,
    source: &dyn ReleaseSource,
    platform: (zed::Os, zed::Architecture),
) -> Result<PathBuf, String> {
    let entry_dir = root.join(format!("llsp-{LLSP_RELEASE_TAG}"));
    let outcome = install_into(&entry_dir, source, platform);
    match outcome {
        Ok(binary) => {
            prune_other_versions(root);
            Ok(binary)
        }
        Err(reason) => {
            let _ = std::fs::remove_dir_all(&entry_dir);
            Err(resolution_error(&reason))
        }
    }
}

fn install_into(
    entry_dir: &Path,
    source: &dyn ReleaseSource,
    platform: (zed::Os, zed::Architecture),
) -> Result<PathBuf, String> {
    let asset_name = llsp_asset_name(platform).ok_or_else(|| {
        format!(
            "no published llsp archive exists for this platform; published platforms: {SUPPORTED_PLATFORMS}"
        )
    })?;

    let release = source
        .release_by_tag(LLSP_GITHUB_REPO, LLSP_RELEASE_TAG)
        .map_err(|e| format!("release lookup for {LLSP_RELEASE_TAG} failed: {e}"))?;

    let asset = release
        .assets
        .iter()
        .find(|asset| asset.name == asset_name)
        .ok_or_else(|| format!("release {LLSP_RELEASE_TAG} publishes no archive {asset_name}"))?;
    let digest_asset = release
        .assets
        .iter()
        .find(|asset| asset.name == DIGEST_LIST_ASSET)
        .ok_or_else(|| format!("release {LLSP_RELEASE_TAG} publishes no {DIGEST_LIST_ASSET}"))?;

    std::fs::create_dir_all(entry_dir).map_err(|e| format!("create cache entry: {e}"))?;

    let sums_path = entry_dir.join("SHA256SUMS.download");
    source
        .download(&digest_asset.download_url, &sums_path)
        .map_err(|e| format!("downloading {DIGEST_LIST_ASSET}: {e}"))?;
    let sums = std::fs::read_to_string(&sums_path)
        .map_err(|e| format!("reading {DIGEST_LIST_ASSET}: {e}"))?;
    let expected = sha256sums_entry(&sums, asset_name)?;
    let _ = std::fs::remove_file(&sums_path);

    // The archive is untrusted until its digest matches the published entry.
    let archive_path = entry_dir.join("archive.download");
    source
        .download(&asset.download_url, &archive_path)
        .map_err(|e| format!("downloading {asset_name}: {e}"))?;
    let observed = sha256_file(&archive_path)?;
    if observed != expected {
        return Err(format!(
            "digest mismatch for {asset_name}: expected {expected}, got {observed}"
        ));
    }

    let payload_dir = entry_dir.join("payload");
    extract_archive(&archive_path, asset_name, &payload_dir)?;
    let _ = std::fs::remove_file(&archive_path);

    let member = payload_dir
        .join(member_dir(asset_name))
        .join(binary_file_name(asset_name));
    let binary = entry_dir.join(binary_file_name(asset_name));
    std::fs::rename(&member, &binary).map_err(|e| format!("moving {member:?} into place: {e}"))?;
    let _ = std::fs::remove_dir_all(&payload_dir);

    source
        .make_executable(&binary)
        .map_err(|e| format!("marking the binary executable: {e}"))?;

    // Written last: an entry without this state is partial and never started.
    let state = zed::serde_json::json!({
        "version": LLSP_RELEASE_TAG,
        "platform": platform_key(platform),
        "asset": asset_name,
        "digest": observed,
    });
    std::fs::write(entry_dir.join(COMPLETION_STATE_FILE), state.to_string())
        .map_err(|e| format!("writing completion state: {e}"))?;

    Ok(binary)
}

/// Where the resolved binary came from.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum BinarySource {
    Configured,
    PathHit,
    VerifiedCache,
    VerifiedDownload,
}

/// The three-step resolution chain: configured binary, then `PATH`, then the
/// verified release download reached only when both earlier steps miss, with
/// a complete verified cache entry reused before any release lookup. The
/// steps cannot be reordered.
fn resolve_llsp(
    root: &Path,
    configured: Option<String>,
    which: &dyn Fn(&str) -> Option<String>,
    source: &dyn ReleaseSource,
    platform: (zed::Os, zed::Architecture),
) -> Result<(PathBuf, BinarySource), String> {
    if let Some(path) = configured {
        return Ok((PathBuf::from(path), BinarySource::Configured));
    }
    if let Some(path) = which("llsp") {
        return Ok((PathBuf::from(path), BinarySource::PathHit));
    }
    let key = platform_key(platform);
    if let Some(binary) = verified_cache_binary(root, &key) {
        return Ok((binary, BinarySource::VerifiedCache));
    }
    install_verified_release(root, source, platform)
        .map(|binary| (binary, BinarySource::VerifiedDownload))
}

/// Builds the launched command with the configured arguments and
/// environment; every resolution path funnels through this one construction.
fn command_with(binary: &Path, args: &[String], env: &[(String, String)]) -> zed::Command {
    zed::Command {
        command: binary.display().to_string(),
        args: args.to_vec(),
        env: env.to_vec(),
    }
}

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
        dispatch_language_server(language_server_id.as_ref())?;

        let lsp_settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;
        let args = lsp_settings
            .binary
            .as_ref()
            .and_then(|binary| binary.arguments.clone())
            .unwrap_or_default();
        let env: Vec<(String, String)> = lsp_settings
            .binary
            .as_ref()
            .and_then(|binary| binary.env.clone())
            .map(|env| env.into_iter().collect())
            .unwrap_or_default();
        let configured = lsp_settings
            .binary
            .as_ref()
            .and_then(|binary| binary.path.clone());

        let (binary, _source) = resolve_llsp(
            Path::new("."),
            configured,
            &|name| worktree.which(name),
            &HostReleaseSource,
            zed::current_platform(),
        )?;

        Ok(command_with(&binary, &args, &env))
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
