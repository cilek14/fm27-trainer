//! Otomatik sürüm denetimi ve platforma özgü kendi kendini güncelleme (GitHub Releases).

use crate::t;
use eframe::egui;
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::mpsc;

pub const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const RELEASES_API: &str = "https://api.github.com/repos/cilek14/fm27-trainer/releases/latest";
pub const RELEASES_PAGE: &str = "https://github.com/cilek14/fm27-trainer/releases/latest";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReleaseAsset {
    pub name: String,
    pub url: String,
    pub size: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReleaseInfo {
    pub tag: String,
    pub version: String,
    pub notes: String,
    pub html_url: String,
    pub asset: Option<ReleaseAsset>,
}

#[derive(Clone, Debug)]
pub enum UpdateStatus {
    Idle,
    Checking,
    UpToDate,
    Available(ReleaseInfo),
    Downloading { version: String },
    Applying { version: String },
    InstalledManual { message: String },
    Error(String),
}

pub enum InstallOutcome {
    Restarting,
    Manual { message: String },
}

pub struct UpdaterState {
    pub status: UpdateStatus,
    pub last_install_failed: bool,
    startup_checked: bool,
    rx_check: Option<mpsc::Receiver<Result<Option<ReleaseInfo>, String>>>,
    rx_install: Option<mpsc::Receiver<Result<InstallOutcome, String>>>,
}

impl Default for UpdaterState {
    fn default() -> Self {
        Self {
            status: UpdateStatus::Idle,
            last_install_failed: false,
            startup_checked: false,
            rx_check: None,
            rx_install: None,
        }
    }
}

pub(crate) fn append_update_log(updates_dir: &Path, msg: &str) {
    use std::io::Write;
    let _ = std::fs::create_dir_all(updates_dir);
    let log_path = updates_dir.join("update.log");
    let truncate = std::fs::metadata(&log_path)
        .map(|m| m.len() > 65_536)
        .unwrap_or(false);
    let file_res = if truncate {
        std::fs::File::create(&log_path)
    } else {
        std::fs::OpenOptions::new().create(true).append(true).open(&log_path)
    };
    if let Ok(mut f) = file_res {
        let ts = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        let _ = writeln!(f, "[{ts}] {msg}");
    }
}

pub(crate) fn should_skip_auto_install(updates_dir: &Path, target_version: &str) -> bool {
    let marker = updates_dir.join("last_auto_attempt.txt");
    std::fs::read_to_string(marker)
        .map(|s| s.trim() == target_version.trim())
        .unwrap_or(false)
}

pub(crate) fn record_auto_attempt(updates_dir: &Path, target_version: &str) {
    if std::fs::create_dir_all(updates_dir).is_ok() {
        let _ = std::fs::write(updates_dir.join("last_auto_attempt.txt"), target_version.trim());
    }
}

pub(crate) fn clear_auto_attempt(updates_dir: &Path) {
    let _ = std::fs::remove_file(updates_dir.join("last_auto_attempt.txt"));
}

#[cfg(windows)]
fn cleanup_stale_old_exe() {
    let Ok(raw_exe) = std::env::current_exe() else { return };
    let exe = strip_verbatim_prefix(raw_exe);
    let Some(parent) = exe.parent() else { return };
    let Some(name) = exe.file_name().and_then(|n| n.to_str()) else { return };
    let prefix = format!("{name}.old");
    let Ok(entries) = std::fs::read_dir(parent) else { return };
    let stale: Vec<PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n == prefix || n.starts_with(&format!("{prefix}.")))
        })
        .collect();
    if stale.is_empty() {
        return;
    }
    for _ in 0..20 {
        let mut all_gone = true;
        for p in &stale {
            if p.exists() && std::fs::remove_file(p).is_err() {
                all_gone = false;
            }
        }
        if all_gone {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(250));
    }
}

impl UpdaterState {
    pub fn busy(&self) -> bool {
        self.rx_check.is_some() || self.rx_install.is_some()
    }

    pub fn available_version(&self) -> Option<&str> {
        match &self.status {
            UpdateStatus::Available(info) => Some(info.version.as_str()),
            _ => None,
        }
    }

    pub fn start_check(&mut self, ctx: &egui::Context) {
        if self.busy() {
            return;
        }
        self.last_install_failed = false;
        self.status = UpdateStatus::Checking;
        let (tx, rx) = mpsc::channel();
        let ctx2 = ctx.clone();
        std::thread::spawn(move || {
            let res = fetch_latest_release();
            let _ = tx.send(res);
            ctx2.request_repaint();
        });
        self.rx_check = Some(rx);
    }

    pub fn start_install(&mut self, ctx: &egui::Context, info: ReleaseInfo) {
        if self.rx_install.is_some() {
            return;
        }
        self.last_install_failed = false;
        self.status = UpdateStatus::Downloading {
            version: info.version.clone(),
        };
        let (tx, rx) = mpsc::channel();
        let ctx2 = ctx.clone();
        std::thread::spawn(move || {
            let res = download_and_install(&info);
            let _ = tx.send(res);
            ctx2.request_repaint();
        });
        self.rx_install = Some(rx);
    }

    pub fn tick(&mut self, ctx: &egui::Context, auto_update: bool) {
        if !self.startup_checked {
            self.startup_checked = true;
            if std::env::var_os("FM27_SNAPSHOT").is_none() {
                #[cfg(windows)]
                std::thread::spawn(cleanup_stale_old_exe);
                self.start_check(ctx);
            }
        }

        if let Some(res) = self.rx_check.as_ref().and_then(|rx| rx.try_recv().ok()) {
            self.rx_check = None;
            match res {
                Ok(Some(info)) => {
                    let has_asset = info.asset.is_some();
                    self.status = UpdateStatus::Available(info.clone());
                    if auto_update && has_asset {
                        let updates_dir = crate::data_dir().join("updates");
                        if should_skip_auto_install(&updates_dir, &info.version) {
                            append_update_log(
                                &updates_dir,
                                &format!(
                                    "Skipping automatic update to v{} (previous attempt did not complete); manual install remains available.",
                                    info.version
                                ),
                            );
                        } else {
                            record_auto_attempt(&updates_dir, &info.version);
                            self.start_install(ctx, info);
                        }
                    }
                }
                Ok(None) => {
                    clear_auto_attempt(&crate::data_dir().join("updates"));
                    self.status = UpdateStatus::UpToDate;
                }
                Err(e) => {
                    self.status = UpdateStatus::Error(e);
                }
            }
        }

        if let Some(res) = self.rx_install.as_ref().and_then(|rx| rx.try_recv().ok()) {
            self.rx_install = None;
            match res {
                Ok(InstallOutcome::Restarting) => {
                    let version = match &self.status {
                        UpdateStatus::Downloading { version } | UpdateStatus::Applying { version } => {
                            version.clone()
                        }
                        _ => CURRENT_VERSION.to_string(),
                    };
                    self.status = UpdateStatus::Applying { version };
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
                Ok(InstallOutcome::Manual { message }) => {
                    self.status = UpdateStatus::InstalledManual { message };
                }
                Err(e) => {
                    self.last_install_failed = true;
                    append_update_log(&crate::data_dir().join("updates"), &format!("Install error: {e}"));
                    self.status = UpdateStatus::Error(e);
                }
            }
        }
    }
}

/// `"v1.2.0"` veya `"1.2.0-beta.1"` biçimindeki sürüm dizgesini ayrıştırır.
pub fn parse_semver(s: &str) -> Option<(u64, u64, u64, bool)> {
    let trimmed = s.trim().trim_start_matches(['v', 'V']);
    if trimmed.is_empty() {
        return None;
    }
    let (core, is_pre) = match trimmed.split_once('-') {
        Some((c, _)) => (c, true),
        None => (trimmed, false),
    };
    let mut parts = core.split('.');
    let major = parts.next()?.parse::<u64>().ok()?;
    let minor = parts.next()?.parse::<u64>().ok()?;
    let patch = match parts.next() {
        Some(p) => p.parse::<u64>().ok()?,
        None => 0,
    };
    Some((major, minor, patch, is_pre))
}

pub fn is_newer(remote: &str, current: &str) -> bool {
    let (Some(r), Some(c)) = (parse_semver(remote), parse_semver(current)) else {
        return false;
    };
    if (r.0, r.1, r.2) != (c.0, c.1, c.2) {
        return (r.0, r.1, r.2) > (c.0, c.1, c.2);
    }
    !r.3 && c.3
}

pub fn pick_asset_for_platform(assets: &[ReleaseAsset], is_windows: bool, is_macos: bool) -> Option<ReleaseAsset> {
    if is_windows {
        let mut best_zip: Option<(u8, &ReleaseAsset)> = None;
        let mut best_exe: Option<(u8, &ReleaseAsset)> = None;
        for a in assets {
            let lower = a.name.to_ascii_lowercase();
            if lower.contains("macos") || lower.contains("darwin") || lower.ends_with(".dmg") {
                continue;
            }
            if lower.ends_with(".zip") {
                let score = if lower.contains("windows") {
                    3
                } else if lower.contains("assistant") || lower.contains("trainer") {
                    2
                } else {
                    1
                };
                if best_zip.is_none_or(|(bs, _)| score > bs) {
                    best_zip = Some((score, a));
                }
            } else if lower.ends_with(".exe") {
                let score = if lower.contains("assistant") || lower.contains("trainer") { 2 } else { 1 };
                if best_exe.as_ref().is_none_or(|(bs, _)| score > *bs) {
                    best_exe = Some((score, a));
                }
            }
        }
        return best_zip.map(|(_, a)| a.clone()).or_else(|| best_exe.map(|(_, a)| a.clone()));
    }

    if is_macos {
        let mut best_dmg: Option<(u8, &ReleaseAsset)> = None;
        let mut best_zip: Option<(u8, &ReleaseAsset)> = None;
        for a in assets {
            let lower = a.name.to_ascii_lowercase();
            if lower.ends_with(".dmg") {
                let score = if lower.contains("assistant") || lower.contains("trainer") { 2 } else { 1 };
                if best_dmg.is_none_or(|(bs, _)| score > bs) {
                    best_dmg = Some((score, a));
                }
            } else if lower.ends_with(".zip") && (lower.contains("macos") || lower.contains("darwin")) {
                let score = if lower.contains("assistant") || lower.contains("trainer") { 2 } else { 1 };
                if best_zip.is_none_or(|(bs, _)| score > bs) {
                    best_zip = Some((score, a));
                }
            }
        }
        return best_dmg.map(|(_, a)| a.clone()).or_else(|| best_zip.map(|(_, a)| a.clone()));
    }

    None
}

pub fn parse_release_json(raw: &str, is_windows: bool, is_macos: bool) -> Result<ReleaseInfo, String> {
    let v: Value = serde_json::from_str(raw).map_err(|e| {
        t!(
            format!("JSON-fout: {e}"),
            format!("JSON error: {e}"),
            format!("JSON hatası: {e}")
        )
    })?;
    if v.get("draft").and_then(|d| d.as_bool()) == Some(true) {
        return Err(t!(
            "Conceptversie overgeslagen.",
            "Draft release skipped.",
            "Taslak sürüm atlandı."
        )
        .into());
    }
    let tag = v
        .get("tag_name")
        .and_then(|s| s.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| {
            t!(
                "Geen versietag gevonden.",
                "Release tag not found.",
                "Sürüm etiketi bulunamadı."
            )
            .to_string()
        })?
        .to_string();
    let version = tag.trim_start_matches(['v', 'V']).to_string();
    if parse_semver(&version).is_none() {
        return Err(t!(
            format!("Ongeldig versienummer: {tag}"),
            format!("Invalid version format: {tag}"),
            format!("Geçersiz sürüm biçimi: {tag}")
        ));
    }
    let notes = v
        .get("body")
        .and_then(|s| s.as_str())
        .unwrap_or("")
        .trim()
        .to_string();
    let html_url = v
        .get("html_url")
        .and_then(|s| s.as_str())
        .filter(|s| !s.is_empty())
        .unwrap_or(RELEASES_PAGE)
        .to_string();

    let mut assets = Vec::new();
    if let Some(arr) = v.get("assets").and_then(|a| a.as_array()) {
        for item in arr {
            let Some(name) = item.get("name").and_then(|s| s.as_str()) else { continue };
            let Some(url) = item.get("browser_download_url").and_then(|s| s.as_str()) else { continue };
            let size = item.get("size").and_then(|n| n.as_u64()).unwrap_or(0);
            assets.push(ReleaseAsset {
                name: name.to_string(),
                url: url.to_string(),
                size,
            });
        }
    }

    let asset = pick_asset_for_platform(&assets, is_windows, is_macos);
    Ok(ReleaseInfo {
        tag,
        version,
        notes,
        html_url,
        asset,
    })
}

fn new_curl_cmd() -> Command {
    let mut cmd = Command::new(if cfg!(windows) { "curl.exe" } else { "curl" });
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    cmd
}

fn fetch_latest_release() -> Result<Option<ReleaseInfo>, String> {
    let out = new_curl_cmd()
        .args([
            "-fsSL",
            "--max-time",
            "15",
            "-H",
            "Accept: application/vnd.github+json",
            "-A",
            "FM27-Training-Assistant",
            RELEASES_API,
        ])
        .stdin(Stdio::null())
        .output()
        .map_err(|e| {
            t!(
                format!("Kon curl niet starten: {e}"),
                format!("Failed to start curl: {e}"),
                format!("curl başlatılamadı: {e}")
            )
        })?;

    if !out.status.success() {
        let err = String::from_utf8_lossy(&out.stderr).trim().to_string();
        return Err(if err.is_empty() {
            t!(
                format!("Updatecheck mislukt (code {:?})", out.status.code()),
                format!("Update check failed (code {:?})", out.status.code()),
                format!("Güncelleme denetimi başarısız (kod {:?})", out.status.code())
            )
        } else {
            err
        });
    }

    let body = String::from_utf8_lossy(&out.stdout);
    let info = parse_release_json(&body, cfg!(windows), cfg!(target_os = "macos"))?;
    if is_newer(&info.version, CURRENT_VERSION) {
        Ok(Some(info))
    } else {
        Ok(None)
    }
}

fn download_and_install(info: &ReleaseInfo) -> Result<InstallOutcome, String> {
    let asset = info.asset.as_ref().ok_or_else(|| {
        t!(
            "Geen geschikt pakket gevonden voor dit platform.",
            "No suitable release package found for this platform.",
            "Bu platform için uygun sürüm dosyası bulunamadı."
        )
        .to_string()
    })?;

    let updates_dir = crate::data_dir().join("updates");
    std::fs::create_dir_all(&updates_dir).map_err(|e| {
        t!(
            format!("Kon updatemap niet maken: {e}"),
            format!("Could not create updates folder: {e}"),
            format!("Güncelleme klasörü oluşturulamadı: {e}")
        )
    })?;

    let safe_name: String = asset
        .name
        .chars()
        .map(|c| if matches!(c, '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|') { '_' } else { c })
        .collect();
    let dl_path = updates_dir.join(&safe_name);
    let tmp_path = updates_dir.join(format!("{safe_name}.part"));
    let _ = std::fs::remove_file(&tmp_path);

    let out = new_curl_cmd()
        .args([
            "-fsSL",
            "--max-time",
            "120",
            "-A",
            "FM27-Training-Assistant",
            "-o",
        ])
        .arg(&tmp_path)
        .arg(&asset.url)
        .stdin(Stdio::null())
        .output()
        .map_err(|e| {
            t!(
                format!("Kon download niet starten: {e}"),
                format!("Could not start download: {e}"),
                format!("İndirme başlatılamadı: {e}")
            )
        })?;

    if !out.status.success() {
        let _ = std::fs::remove_file(&tmp_path);
        let err = String::from_utf8_lossy(&out.stderr).trim().to_string();
        return Err(if err.is_empty() {
            t!(
                "Updatebestand downloaden mislukt.",
                "Failed to download update file.",
                "Güncelleme dosyası indirilemedi."
            )
            .to_string()
        } else {
            err
        });
    }

    let meta = std::fs::metadata(&tmp_path).map_err(|e| {
        t!(
            format!("Kon gedownload bestand niet lezen: {e}"),
            format!("Could not read downloaded file: {e}"),
            format!("İndirilen dosya okunamadı: {e}")
        )
    })?;
    if meta.len() < 200_000 {
        let _ = std::fs::remove_file(&tmp_path);
        return Err(t!(
            format!("Gedownload bestand te klein ({} bytes).", meta.len()),
            format!("Downloaded file smaller than expected ({} bytes).", meta.len()),
            format!("İndirilen dosya beklenenden küçük ({} bayt).", meta.len())
        ));
    }

    let mut hdr = [0u8; 4];
    {
        use std::io::Read;
        let mut f = std::fs::File::open(&tmp_path).map_err(|e| {
            t!(
                format!("Kon bestand niet controleren: {e}"),
                format!("Could not verify downloaded file: {e}"),
                format!("Dosya doğrulanamadı: {e}")
            )
        })?;
        let n = f.read(&mut hdr).map_err(|e| {
            t!(
                format!("Kon header niet lezen: {e}"),
                format!("Could not read file header: {e}"),
                format!("Başlık okunamadı: {e}")
            )
        })?;
        if n < 2 {
            let _ = std::fs::remove_file(&tmp_path);
            return Err(t!(
                "Beschadigd updatepakket.",
                "Corrupted update package.",
                "Bozuk güncelleme paketi."
            )
            .into());
        }
    }

    let lower = safe_name.to_ascii_lowercase();
    if lower.ends_with(".zip") && hdr != [b'P', b'K', 0x03, 0x04] {
        let _ = std::fs::remove_file(&tmp_path);
        return Err(t!(
            "Ongeldig ZIP-archief gedownload.",
            "Downloaded ZIP archive is invalid.",
            "İndirilen ZIP arşivi geçersiz."
        )
        .into());
    }
    if lower.ends_with(".exe") && &hdr[..2] != b"MZ" {
        let _ = std::fs::remove_file(&tmp_path);
        return Err(t!(
            "Ongeldig EXE-bestand gedownload.",
            "Downloaded EXE file is invalid.",
            "İndirilen EXE dosyası geçersiz."
        )
        .into());
    }

    let _ = std::fs::remove_file(&dl_path);
    std::fs::rename(&tmp_path, &dl_path).map_err(|e| {
        t!(
            format!("Kon bestand niet verplaatsen: {e}"),
            format!("Could not move downloaded file: {e}"),
            format!("Dosya taşınamadı: {e}")
        )
    })?;

    #[cfg(windows)]
    {
        install_windows(&dl_path, &updates_dir)
    }

    #[cfg(target_os = "macos")]
    {
        install_macos(&dl_path, &updates_dir)
    }

    #[cfg(not(any(windows, target_os = "macos")))]
    {
        let _ = (dl_path, updates_dir);
        Err(t!(
            "Automatisch installeren wordt op dit systeem niet ondersteund.",
            "Automatic installation is not supported on this platform.",
            "Bu platformda otomatik yükleme desteklenmiyor."
        )
        .into())
    }
}

#[cfg_attr(not(any(windows, test)), allow(dead_code))]
pub(crate) fn strip_verbatim_prefix(p: PathBuf) -> PathBuf {
    let s = p.to_string_lossy();
    if let Some(rest) = s.strip_prefix(r"\\?\")
        && !rest.to_ascii_uppercase().starts_with(r"UNC\")
    {
        return PathBuf::from(rest);
    }
    p
}

#[cfg_attr(not(any(windows, test)), allow(dead_code))]
pub(crate) fn old_exe_path(exe: &Path) -> PathBuf {
    let mut os = exe.as_os_str().to_os_string();
    os.push(".old");
    PathBuf::from(os)
}

#[cfg(windows)]
fn ps_escape(p: &Path) -> String {
    p.to_string_lossy().replace('\'', "''")
}

#[cfg(windows)]
fn find_extracted_files(dir: &Path) -> (Option<PathBuf>, Option<PathBuf>) {
    let mut best_exe: Option<(u8, PathBuf)> = None;
    let mut best_dll: Option<PathBuf> = None;
    let mut stack = vec![dir.to_path_buf()];
    while let Some(cur) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&cur) else { continue };
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                stack.push(p);
                continue;
            }
            let Some(name) = p.file_name().and_then(|n| n.to_str()) else { continue };
            let lower = name.to_ascii_lowercase();
            if lower == "fm27trainer.bridge.dll" {
                best_dll = Some(p);
            } else if lower.ends_with(".exe") && !lower.contains("unins") {
                let score = if lower.contains("assistant") || lower.contains("trainer") { 2 } else { 1 };
                if best_exe.as_ref().is_none_or(|(bs, _)| score > *bs) {
                    best_exe = Some((score, p));
                }
            }
        }
    }
    (best_exe.map(|(_, p)| p), best_dll)
}

#[cfg(windows)]
pub(crate) fn extract_zip_windows(dl_path: &Path, staging: &Path, updates_dir: &Path) -> Result<(), String> {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;

    let _ = std::fs::remove_dir_all(staging);
    std::fs::create_dir_all(staging).map_err(|e| {
        t!(
            format!("Tijdelijke map niet aangemaakt: {e}"),
            format!("Could not create temp folder: {e}"),
            format!("Geçici klasör oluşturulamadı: {e}")
        )
    })?;

    match Command::new("tar.exe")
        .arg("-xf")
        .arg(dl_path)
        .arg("-C")
        .arg(staging)
        .creation_flags(CREATE_NO_WINDOW)
        .stdin(Stdio::null())
        .output()
    {
        Ok(out) if out.status.success() => return Ok(()),
        Ok(out) => {
            let err = String::from_utf8_lossy(&out.stderr).trim().to_string();
            append_update_log(updates_dir, &format!("tar.exe extract failed ({:?}): {err}", out.status.code()));
        }
        Err(e) => {
            append_update_log(updates_dir, &format!("tar.exe unavailable ({e}); falling back to Expand-Archive"));
        }
    }

    let _ = std::fs::remove_dir_all(staging);
    std::fs::create_dir_all(staging).map_err(|e| {
        t!(
            format!("Tijdelijke map niet aangemaakt: {e}"),
            format!("Could not create temp folder: {e}"),
            format!("Geçici klasör oluşturulamadı: {e}")
        )
    })?;

    let exp = Command::new("powershell.exe")
        .args([
            "-NoProfile",
            "-WindowStyle",
            "Hidden",
            "-Command",
            "Expand-Archive -LiteralPath $env:FM27_ZIP -DestinationPath $env:FM27_DST -Force",
        ])
        .env("FM27_ZIP", dl_path)
        .env("FM27_DST", staging)
        .creation_flags(CREATE_NO_WINDOW)
        .stdin(Stdio::null())
        .output()
        .map_err(|e| {
            t!(
                format!("Kon PowerShell niet starten: {e}"),
                format!("Could not start PowerShell: {e}"),
                format!("PowerShell başlatılamadı: {e}")
            )
        })?;
    if !exp.status.success() {
        let err = String::from_utf8_lossy(&exp.stderr).trim().to_string();
        append_update_log(updates_dir, &format!("Expand-Archive failed ({:?}): {err}", exp.status.code()));
        return Err(t!(
            "ZIP-archief kon niet worden uitgepakt.",
            "Could not extract ZIP archive.",
            "ZIP arşivi açılamadı."
        )
        .into());
    }
    Ok(())
}

#[cfg(windows)]
pub(crate) fn verify_windows_exe_min(path: &Path, min_size: u64) -> Result<u64, String> {
    use std::io::Read;
    let meta = std::fs::metadata(path).map_err(|e| {
        t!(
            format!("Kon bestand niet controleren: {e}"),
            format!("Could not verify downloaded file: {e}"),
            format!("Dosya doğrulanamadı: {e}")
        )
    })?;
    if meta.len() < min_size {
        return Err(t!(
            format!("Gedownload bestand te klein ({} bytes).", meta.len()),
            format!("Downloaded file smaller than expected ({} bytes).", meta.len()),
            format!("İndirilen dosya beklenenden küçük ({} bayt).", meta.len())
        ));
    }
    let mut hdr = [0u8; 2];
    let mut f = std::fs::File::open(path).map_err(|e| {
        t!(
            format!("Kon bestand niet openen: {e}"),
            format!("Could not open update file: {e}"),
            format!("Güncelleme dosyası açılamadı: {e}")
        )
    })?;
    if f.read_exact(&mut hdr).is_err() || &hdr != b"MZ" {
        return Err(t!(
            "Ongeldig EXE-bestand gedownload.",
            "Downloaded EXE file is invalid.",
            "İndirilen EXE dosyası geçersiz."
        )
        .into());
    }
    Ok(meta.len())
}

#[cfg(windows)]
pub(crate) fn replace_running_exe_windows(current_exe: &Path, new_exe: &Path, min_size: u64) -> Result<PathBuf, String> {
    let expected_len = verify_windows_exe_min(new_exe, min_size)?;

    let mut old_exe = old_exe_path(current_exe);
    if old_exe.exists() && std::fs::remove_file(&old_exe).is_err() {
        let mut alt = current_exe.as_os_str().to_os_string();
        alt.push(format!(".old.{}", std::process::id()));
        old_exe = PathBuf::from(alt);
    }

    std::fs::rename(current_exe, &old_exe).map_err(|e| format!("rename failed: {e}"))?;

    let mut copy_err: Option<String> = None;
    for attempt in 0..10 {
        match std::fs::copy(new_exe, current_exe) {
            Ok(written) if written == expected_len && verify_windows_exe_min(current_exe, min_size).is_ok() => {
                return Ok(old_exe);
            }
            Ok(written) => {
                copy_err = Some(format!("incomplete copy ({written} != {expected_len} bytes)"));
            }
            Err(e) => {
                copy_err = Some(e.to_string());
            }
        }
        if attempt < 9 {
            std::thread::sleep(std::time::Duration::from_millis(300));
        }
    }

    let _ = std::fs::remove_file(current_exe);
    let _ = std::fs::rename(&old_exe, current_exe);
    let reason = copy_err.unwrap_or_else(|| "unknown copy error".to_string());
    Err(t!(
        format!("Kon update niet kopiëren: {reason}"),
        format!("Could not copy update executable: {reason}"),
        format!("Güncelleme dosyası kopyalanamadı: {reason}")
    ))
}

#[cfg(windows)]
pub(crate) fn build_windows_fallback_ps1(
    pid: u32,
    src_exe: &Path,
    current_exe: &Path,
    src_dll: Option<&Path>,
    staging: Option<&Path>,
    dl_path: &Path,
    log_path: &Path,
) -> Vec<u8> {
    let work_dir = current_exe.parent().unwrap_or_else(|| Path::new("."));
    let dll_str = src_dll.map(ps_escape).unwrap_or_default();
    let staging_str = staging.map(ps_escape).unwrap_or_default();
    let script = format!(
        "$ErrorActionPreference = 'Continue'\r\n\
         $logPath = '{log}'\r\n\
         function Write-Log($m) {{ try {{ Add-Content -LiteralPath $logPath -Value \"[$(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')] $m\" -Encoding UTF8 }} catch {{}} }}\r\n\
         $pidWait = {pid}\r\n\
         for ($i = 0; $i -lt 60; $i++) {{\r\n\
           if (-not (Get-Process -Id $pidWait -ErrorAction SilentlyContinue)) {{ break }}\r\n\
           Start-Sleep -Milliseconds 250\r\n\
         }}\r\n\
         Start-Sleep -Milliseconds 250\r\n\
         try {{ Unblock-File -LiteralPath '{src}' -ErrorAction SilentlyContinue }} catch {{}}\r\n\
         $copied = $false\r\n\
         for ($i = 0; $i -lt 40; $i++) {{\r\n\
           try {{\r\n\
             Copy-Item -LiteralPath '{src}' -Destination '{dst}' -Force -ErrorAction Stop\r\n\
             try {{ Unblock-File -LiteralPath '{dst}' -ErrorAction SilentlyContinue }} catch {{}}\r\n\
             $copied = $true\r\n\
             break\r\n\
           }} catch {{\r\n\
             if ($i -eq 39) {{ Write-Log \"Fallback Copy-Item failed: $($_.Exception.Message)\" }}\r\n\
             Start-Sleep -Milliseconds 300\r\n\
           }}\r\n\
         }}\r\n\
         $srcDll = '{dll}'\r\n\
         if ($srcDll -ne '' -and (Test-Path -LiteralPath $srcDll)) {{\r\n\
           $bridgeDir = Join-Path '{dir}' 'BepInEx\\plugins\\FM27Trainer.Bridge'\r\n\
           if (Test-Path -LiteralPath $bridgeDir) {{\r\n\
             try {{ Copy-Item -LiteralPath $srcDll -Destination (Join-Path $bridgeDir 'FM27Trainer.Bridge.dll') -Force -ErrorAction Stop }} catch {{ Write-Log \"DLL copy failed: $($_.Exception.Message)\" }}\r\n\
           }}\r\n\
         }}\r\n\
         $stagingDir = '{stg}'\r\n\
         if ($stagingDir -ne '' -and (Test-Path -LiteralPath $stagingDir)) {{\r\n\
           Remove-Item -LiteralPath $stagingDir -Recurse -Force -ErrorAction SilentlyContinue\r\n\
         }}\r\n\
         if ($copied) {{\r\n\
           Remove-Item -LiteralPath '{dl}' -Force -ErrorAction SilentlyContinue\r\n\
         }}\r\n\
         try {{\r\n\
           Start-Process -FilePath '{dst}' -WorkingDirectory '{dir}' -WindowStyle Normal -ErrorAction Stop\r\n\
         }} catch {{\r\n\
           Write-Log \"Fallback Start-Process failed: $($_.Exception.Message)\"\r\n\
         }}\r\n\
         Remove-Item -LiteralPath $PSCommandPath -Force -ErrorAction SilentlyContinue\r\n",
        log = ps_escape(log_path),
        pid = pid,
        src = ps_escape(src_exe),
        dst = ps_escape(current_exe),
        dir = ps_escape(work_dir),
        dll = dll_str,
        stg = staging_str,
        dl = ps_escape(dl_path),
    );
    let mut out = Vec::with_capacity(3 + script.len());
    out.extend_from_slice(&[0xEF, 0xBB, 0xBF]);
    out.extend_from_slice(script.as_bytes());
    out
}

#[cfg(windows)]
fn spawn_windows_fallback_script(
    pid: u32,
    src_exe: &Path,
    current_exe: &Path,
    src_dll: Option<&Path>,
    staging: Option<&Path>,
    dl_path: &Path,
    updates_dir: &Path,
) -> Result<(), String> {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    const CREATE_NEW_PROCESS_GROUP: u32 = 0x0000_0200;
    const CREATE_BREAKAWAY_FROM_JOB: u32 = 0x0100_0000;

    let ps1 = updates_dir.join("apply_update.ps1");
    let log_path = updates_dir.join("update.log");
    let bytes = build_windows_fallback_ps1(pid, src_exe, current_exe, src_dll, staging, dl_path, &log_path);
    std::fs::write(&ps1, bytes).map_err(|e| {
        t!(
            format!("Updatescript niet opgeslagen: {e}"),
            format!("Could not write updater script: {e}"),
            format!("Güncelleme betiği yazılamadı: {e}")
        )
    })?;

    let spawn_ps = |flags: u32| {
        Command::new("powershell.exe")
            .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-WindowStyle", "Hidden", "-File"])
            .arg(&ps1)
            .creation_flags(flags)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
    };

    if spawn_ps(CREATE_NO_WINDOW | CREATE_NEW_PROCESS_GROUP | CREATE_BREAKAWAY_FROM_JOB).is_err() {
        spawn_ps(CREATE_NO_WINDOW | CREATE_NEW_PROCESS_GROUP).map_err(|e| {
            t!(
                format!("Kon updater niet starten: {e}"),
                format!("Could not launch updater: {e}"),
                format!("Güncelleyici başlatılamadı: {e}")
            )
        })?;
    }
    Ok(())
}

#[cfg(windows)]
fn install_windows(dl_path: &Path, updates_dir: &Path) -> Result<InstallOutcome, String> {
    use std::os::windows::process::CommandExt;
    const CREATE_NEW_PROCESS_GROUP: u32 = 0x0000_0200;

    let raw_exe = std::env::current_exe().map_err(|e| {
        t!(
            format!("App-pad niet gevonden: {e}"),
            format!("Could not locate current executable: {e}"),
            format!("Uygulama yolu alınamadı: {e}")
        )
    })?;
    let current_exe = strip_verbatim_prefix(raw_exe);
    let parent = current_exe.parent().unwrap_or_else(|| Path::new("."));
    {
        let probe = parent.join(".fm27_update_probe");
        if std::fs::write(&probe, b"").is_ok() {
            let _ = std::fs::remove_file(&probe);
        } else {
            append_update_log(updates_dir, &format!("Folder not writable: {}", parent.display()));
            let _ = Command::new("explorer.exe")
                .arg(format!("/select,{}", dl_path.display()))
                .spawn();
            return Ok(InstallOutcome::Manual {
                message: t!(
                    format!("Geen schrijfrechten in deze map; updatepakket gedownload: {}", dl_path.display()),
                    format!("No write permission in this folder; update downloaded and opened: {}", dl_path.display()),
                    format!("Bu klasöre yazma izni yok; güncelleme paketi indirildi ve açıldı: {}", dl_path.display())
                ),
            });
        }
    }

    let staging = updates_dir.join("staging");
    let lower = dl_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    let is_zip = lower.ends_with(".zip");

    let (src_exe, src_dll) = if is_zip {
        extract_zip_windows(dl_path, &staging, updates_dir)?;
        let (exe_opt, dll_opt) = find_extracted_files(&staging);
        let exe = exe_opt.ok_or_else(|| {
            t!(
                "Geen .exe gevonden in het update-archief.",
                "No .exe found inside the update archive.",
                "Güncelleme arşivinde .exe bulunamadı."
            )
            .to_string()
        })?;
        (exe, dll_opt)
    } else {
        (dl_path.to_path_buf(), None)
    };

    append_update_log(
        updates_dir,
        &format!("Applying Windows update from {} -> {}", src_exe.display(), current_exe.display()),
    );

    match replace_running_exe_windows(&current_exe, &src_exe, 200_000) {
        Ok(old_exe) => {
            if let Some(ref dll) = src_dll
                && dll.exists()
            {
                let bridge_dir = parent.join("BepInEx").join("plugins").join("FM27Trainer.Bridge");
                if bridge_dir.exists() {
                    let _ = std::fs::copy(dll, bridge_dir.join("FM27Trainer.Bridge.dll"));
                }
            }
            if is_zip {
                let _ = std::fs::remove_dir_all(&staging);
            }
            let _ = std::fs::remove_file(dl_path);

            let mut child = match Command::new(&current_exe)
                .current_dir(parent)
                .creation_flags(CREATE_NEW_PROCESS_GROUP)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
            {
                Ok(c) => c,
                Err(e) => {
                    append_update_log(updates_dir, &format!("Direct relaunch spawn failed: {e}"));
                    let _ = std::fs::remove_file(&current_exe);
                    let _ = std::fs::rename(&old_exe, &current_exe);
                    return Err(t!(
                        format!("Kon nieuwe versie niet starten: {e}"),
                        format!("Could not launch new version: {e}"),
                        format!("Yeni sürüm başlatılamadı: {e}")
                    ));
                }
            };

            std::thread::sleep(std::time::Duration::from_millis(250));
            if let Ok(Some(status)) = child.try_wait()
                && !status.success()
            {
                append_update_log(updates_dir, &format!("New executable exited immediately: {status}"));
                let _ = std::fs::remove_file(&current_exe);
                let _ = std::fs::rename(&old_exe, &current_exe);
                return Err(t!(
                    format!("Nieuwe versie sloot direct af ({status})."),
                    format!("New version exited immediately ({status})."),
                    format!("Yeni sürüm hemen kapandı ({status}).")
                ));
            }

            append_update_log(
                updates_dir,
                &format!("New executable launched (pid={}); closing old instance.", child.id()),
            );
            Ok(InstallOutcome::Restarting)
        }
        Err(err) if err.starts_with("rename failed:") && current_exe.exists() => {
            append_update_log(updates_dir, &format!("In-place rename unavailable ({err}); using PowerShell fallback."));
            spawn_windows_fallback_script(
                std::process::id(),
                &src_exe,
                &current_exe,
                src_dll.as_deref(),
                is_zip.then_some(staging.as_path()),
                dl_path,
                updates_dir,
            )?;
            Ok(InstallOutcome::Restarting)
        }
        Err(err) => {
            append_update_log(updates_dir, &format!("In-place replacement failed: {err}"));
            Err(err)
        }
    }
}

#[cfg(target_os = "macos")]
fn sh_escape(p: &Path) -> String {
    format!("'{}'", p.to_string_lossy().replace('\'', "'\\''"))
}

#[cfg(target_os = "macos")]
fn install_macos(dl_path: &Path, updates_dir: &Path) -> Result<InstallOutcome, String> {
    use std::os::unix::process::CommandExt;

    let current_exe = std::env::current_exe().ok();
    let app_bundle = current_exe.as_ref().and_then(|exe| {
        exe.ancestors()
            .find(|p| p.extension().and_then(|e| e.to_str()) == Some("app"))
            .map(Path::to_path_buf)
    });
    let is_dmg = dl_path
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| e.eq_ignore_ascii_case("dmg"));

    if is_dmg {
        if let Some(target_app) = app_bundle {
            if target_app.to_string_lossy().contains("/AppTranslocation/") {
                append_update_log(
                    updates_dir,
                    &format!("App is running under AppTranslocation ({}); opening DMG manually.", target_app.display()),
                );
            } else if let Some(parent) = target_app.parent() {
                let probe = parent.join(".fm27_update_probe");
                if std::fs::write(&probe, b"").is_ok() {
                    let _ = std::fs::remove_file(&probe);
                    let mnt = updates_dir.join("dmg_mount");
                    let sh = updates_dir.join("apply_update.sh");
                    let log_path = updates_dir.join("update.log");
                    let pid = std::process::id();
                    let script = format!(
                        "#!/bin/sh\n\
                         PID={pid}\n\
                         LOG={log}\n\
                         log() {{ printf '[%s] %s\\n' \"$(date '+%Y-%m-%d %H:%M:%S')\" \"$1\" >> \"$LOG\"; }}\n\
                         i=0\n\
                         while kill -0 $PID 2>/dev/null && [ $i -lt 60 ]; do\n\
                           sleep 0.25\n\
                           i=$((i+1))\n\
                         done\n\
                         sleep 0.2\n\
                         MNT={mnt}\n\
                         DMG={dmg}\n\
                         TARGET={target}\n\
                         NEW_APP=\"$TARGET.new\"\n\
                         OLD_APP=\"$TARGET.old\"\n\
                         rm -rf \"$MNT\" \"$NEW_APP\"\n\
                         mkdir -p \"$MNT\"\n\
                         COPIED=0\n\
                         if hdiutil attach -nobrowse -quiet -mountpoint \"$MNT\" \"$DMG\"; then\n\
                           SRC_APP=$(find \"$MNT\" -maxdepth 1 -name '*.app' | head -n 1)\n\
                           if [ -n \"$SRC_APP\" ] && cp -R \"$SRC_APP\" \"$NEW_APP\"; then\n\
                             rm -rf \"$OLD_APP\"\n\
                             if [ ! -e \"$TARGET\" ] || mv \"$TARGET\" \"$OLD_APP\"; then\n\
                               if mv \"$NEW_APP\" \"$TARGET\"; then\n\
                                 rm -rf \"$OLD_APP\"\n\
                                 xattr -cr \"$TARGET\" 2>/dev/null || true\n\
                                 COPIED=1\n\
                               else\n\
                                 log \"Failed to move new app into place\"\n\
                                 mv \"$OLD_APP\" \"$TARGET\" 2>/dev/null || true\n\
                               fi\n\
                             else\n\
                               log \"Failed to back up existing app bundle\"\n\
                               rm -rf \"$NEW_APP\"\n\
                             fi\n\
                           else\n\
                             log \"Failed to copy app bundle from DMG\"\n\
                             rm -rf \"$NEW_APP\"\n\
                           fi\n\
                           hdiutil detach -quiet \"$MNT\" 2>/dev/null || true\n\
                         else\n\
                           log \"hdiutil attach failed\"\n\
                         fi\n\
                         rm -rf \"$MNT\"\n\
                         if [ $COPIED -eq 1 ]; then\n\
                           rm -f \"$DMG\"\n\
                         fi\n\
                         open -n \"$TARGET\" || open \"$TARGET\"\n\
                         rm -f \"$0\"\n",
                        pid = pid,
                        log = sh_escape(&log_path),
                        mnt = sh_escape(&mnt),
                        dmg = sh_escape(dl_path),
                        target = sh_escape(&target_app),
                    );
                    if std::fs::write(&sh, script).is_ok() {
                        if Command::new("/bin/sh")
                            .arg(&sh)
                            .stdin(Stdio::null())
                            .stdout(Stdio::null())
                            .stderr(Stdio::null())
                            .process_group(0)
                            .spawn()
                            .is_ok()
                        {
                            return Ok(InstallOutcome::Restarting);
                        }
                    }
                }
            }
        }
    }

    let _ = Command::new("open").arg(dl_path).spawn();
    Ok(InstallOutcome::Manual {
        message: t!(
            format!("Update gedownload en geopend: {}", dl_path.display()),
            format!("Update downloaded and opened: {}", dl_path.display()),
            format!("Güncelleme indirildi ve açıldı: {}", dl_path.display())
        ),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_test_dir(tag: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!(
            "fm27_updater_test_{tag}_{}_{}",
            std::process::id(),
            chrono::Local::now().timestamp_nanos_opt().unwrap_or(0)
        ));
        let _ = std::fs::remove_dir_all(&p);
        std::fs::create_dir_all(&p).unwrap();
        p
    }

    #[test]
    fn parses_and_compares_semver_versions() {
        assert_eq!(parse_semver("v1.2.0"), Some((1, 2, 0, false)));
        assert_eq!(parse_semver("1.3.2"), Some((1, 3, 2, false)));
        assert_eq!(parse_semver("v1.3.0-rc.1"), Some((1, 3, 0, true)));
        assert!(is_newer("1.2.1", "1.2.0"));
        assert!(is_newer("v1.3.0", "1.2.0"));
        assert!(is_newer("2.0.0", "1.9.9"));
        assert!(is_newer("1.2.0", "1.2.0-beta.1"));
        assert!(!is_newer("1.2.0", "1.2.0"));
        assert!(!is_newer("1.1.9", "1.2.0"));
    }

    #[test]
    fn picks_expected_windows_and_macos_assets() {
        let json = r#"{
            "tag_name": "v1.3.0",
            "html_url": "https://github.com/cilek14/fm27-trainer/releases/tag/v1.3.0",
            "body": "Otomatik güncelleme ve performans iyileştirmeleri.",
            "assets": [
                {
                    "name": "FM27-Training-Assistant-macOS.dmg",
                    "browser_download_url": "https://example.com/mac.dmg",
                    "size": 10985325
                },
                {
                    "name": "FM27-Training-Assistant-v1.3.0.zip",
                    "browser_download_url": "https://example.com/win.zip",
                    "size": 4338074
                }
            ]
        }"#;

        let win = parse_release_json(json, true, false).unwrap();
        assert_eq!(win.version, "1.3.0");
        assert_eq!(win.asset.as_ref().map(|a| a.name.as_str()), Some("FM27-Training-Assistant-v1.3.0.zip"));

        let mac = parse_release_json(json, false, true).unwrap();
        assert_eq!(mac.asset.as_ref().map(|a| a.name.as_str()), Some("FM27-Training-Assistant-macOS.dmg"));

        let legacy = vec![ReleaseAsset {
            name: "Fm.27.Training.Assistant.zip".into(),
            url: "https://example.com/legacy.zip".into(),
            size: 2993956,
        }];
        assert_eq!(
            pick_asset_for_platform(&legacy, true, false).map(|a| a.name),
            Some("Fm.27.Training.Assistant.zip".into())
        );
    }

    #[test]
    fn anti_loop_attempt_marker_skips_same_version_only() {
        let dir = temp_test_dir("antiloop");
        assert!(!should_skip_auto_install(&dir, "1.4.3"));
        record_auto_attempt(&dir, "1.4.3");
        assert!(should_skip_auto_install(&dir, "1.4.3"));
        assert!(!should_skip_auto_install(&dir, "1.4.4"));
        clear_auto_attempt(&dir);
        assert!(!should_skip_auto_install(&dir, "1.4.3"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn strip_verbatim_and_old_exe_path_work() {
        assert_eq!(
            strip_verbatim_prefix(PathBuf::from(r"\\?\C:\Games\FM27 Training Assistant.exe")),
            PathBuf::from(r"C:\Games\FM27 Training Assistant.exe")
        );
        assert_eq!(
            strip_verbatim_prefix(PathBuf::from(r"\\?\UNC\server\share\app.exe")),
            PathBuf::from(r"\\?\UNC\server\share\app.exe")
        );
        assert_eq!(
            old_exe_path(Path::new(r"C:\Games\FM27 Training Assistant.exe")),
            PathBuf::from(r"C:\Games\FM27 Training Assistant.exe.old")
        );
    }

    #[cfg(windows)]
    #[test]
    fn extract_zip_handles_spaces_unicode_and_brackets() {
        let dir = temp_test_dir("zip_unicode");
        let sub = dir.join("Şükrü Ümit [v1]");
        std::fs::create_dir_all(&sub).unwrap();
        let src_dir = sub.join("payload");
        let dll_dir = src_dir.join("BepInEx").join("plugins").join("FM27Trainer.Bridge");
        std::fs::create_dir_all(&dll_dir).unwrap();
        std::fs::write(src_dir.join("FM27 Training Assistant.exe"), b"MZ_FAKE_EXE").unwrap();
        std::fs::write(dll_dir.join("FM27Trainer.Bridge.dll"), b"MZ_FAKE_DLL").unwrap();

        let zip_path = sub.join("update [1.4.3].zip");
        let status = Command::new("tar.exe")
            .args(["-a", "-cf"])
            .arg(&zip_path)
            .arg("-C")
            .arg(&src_dir)
            .arg(".")
            .status()
            .unwrap();
        assert!(status.success());

        let staging = sub.join("staging [out]");
        extract_zip_windows(&zip_path, &staging, &sub).unwrap();
        let (exe, dll) = find_extracted_files(&staging);
        assert_eq!(
            exe.as_deref().and_then(|p| p.file_name()).and_then(|n| n.to_str()),
            Some("FM27 Training Assistant.exe")
        );
        assert_eq!(
            dll.as_deref().and_then(|p| p.file_name()).and_then(|n| n.to_str()),
            Some("FM27Trainer.Bridge.dll")
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[cfg(windows)]
    #[test]
    fn replace_running_exe_swaps_locked_binary_and_rolls_back_on_bad_source() {
        let dir = temp_test_dir("swap_running");
        let current = dir.join("FM27 Training Assistant.exe");
        let new_ok = dir.join("new_ok.exe");
        let bad_src = dir.join("bad_src.exe");
        let cmd_sys = Path::new(r"C:\Windows\System32\cmd.exe");

        std::fs::copy(cmd_sys, &current).unwrap();
        std::fs::copy(cmd_sys, &new_ok).unwrap();
        std::fs::write(&bad_src, vec![b'X'; 4096]).unwrap();

        // Corrupt source must fail before touching current
        let err = replace_running_exe_windows(&current, &bad_src, 1024).unwrap_err();
        assert!(!err.is_empty());
        assert!(current.exists());
        assert!(!old_exe_path(&current).exists());

        // Spawn running binary and swap while running
        let mut child = Command::new(&current)
            .args(["/c", "ping -n 3 127.0.0.1 >nul"])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        std::thread::sleep(std::time::Duration::from_millis(200));

        let old = replace_running_exe_windows(&current, &new_ok, 1024).unwrap();
        assert!(old.exists());
        assert!(current.exists());
        assert_eq!(
            std::fs::metadata(&current).unwrap().len(),
            std::fs::metadata(&new_ok).unwrap().len()
        );

        let _ = child.wait();
        let _ = std::fs::remove_file(&old);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[cfg(windows)]
    #[test]
    fn fallback_ps1_has_utf8_bom_and_normal_window() {
        let dir = temp_test_dir("ps1_bom");
        let bytes = build_windows_fallback_ps1(
            1234,
            &dir.join("src.exe"),
            &dir.join("Şükrü").join("FM27 Training Assistant.exe"),
            Some(&dir.join("FM27Trainer.Bridge.dll")),
            Some(&dir.join("staging")),
            &dir.join("dl.zip"),
            &dir.join("update.log"),
        );
        assert!(bytes.starts_with(&[0xEF, 0xBB, 0xBF]));
        let text = std::str::from_utf8(&bytes[3..]).unwrap();
        assert!(text.contains("-WorkingDirectory"));
        assert!(text.contains("-WindowStyle Normal"));
        assert!(text.contains("update.log"));
        assert!(text.contains("Şükrü"));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
