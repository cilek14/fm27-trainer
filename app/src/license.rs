//! Çevrimiçi Pro lisans doğrulama ve cihaz kilidi (Lemon Squeezy / Gumroad API).

use crate::t;
use eframe::egui;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::mpsc;

/// Pro lisans satın alma sayfası (Lemon Squeezy veya Gumroad ürün bağlantınızı buraya yazın).
pub const CHECKOUT_URL: &str = "https://buymeacoffee.com/cilek14";
/// Lemon Squeezy Store ID (ör. `Some(12345)` — mağaza açıldığında doldurun; `None` iken mağaza filtresi uygulanmaz).
pub const LEMON_STORE_ID: Option<u64> = None;
/// Lemon Squeezy Product ID (ör. `Some(67890)` — ürün oluşturulduğunda doldurun).
pub const LEMON_PRODUCT_ID: Option<u64> = None;
/// Gumroad Product ID / Permalink (Gumroad kullanacaksanız ürün ID'sini buraya yazın; boşsa Gumroad atlanır).
pub const GUMROAD_PRODUCT_ID: &str = "";

/// Çevrimdışı çalışma müsamaha süresi (14 gün).
pub const OFFLINE_GRACE_SECS: i64 = 14 * 86_400;
/// Arka planda sessiz yeniden doğrulama aralığı (24 saat).
const REVERIFY_INTERVAL_SECS: i64 = 86_400;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct StoredLicense {
    pub provider: String,
    pub license_key: String,
    pub key_masked: String,
    #[serde(default)]
    pub instance_id: Option<String>,
    #[serde(default)]
    pub customer_email: Option<String>,
    pub activated_at_unix: i64,
    pub last_verified_unix: i64,
    pub machine_tag: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum VerifyOutcome {
    Valid {
        provider: String,
        instance_id: Option<String>,
        customer_email: Option<String>,
    },
    Revoked(String),
    NetworkError(String),
}

enum LicenseJobResult {
    Activated(Result<StoredLicense, String>),
    Reverified(VerifyOutcome),
    Deactivated,
}

pub struct LicenseState {
    pub stored: Option<StoredLicense>,
    pub key_input: String,
    pub busy: bool,
    pub status_msg: String,
    pub status_err: bool,
    startup_checked: bool,
    rx: Option<mpsc::Receiver<LicenseJobResult>>,
}

impl Default for LicenseState {
    fn default() -> Self {
        Self {
            stored: load_stored_license(),
            key_input: String::new(),
            busy: false,
            status_msg: String::new(),
            status_err: false,
            startup_checked: false,
            rx: None,
        }
    }
}

fn license_file() -> PathBuf {
    crate::data_dir().join("license.json")
}

fn now_unix() -> i64 {
    chrono::Utc::now().timestamp()
}

pub(crate) fn compute_machine_tag(host: &str, user: &str, os: &str) -> String {
    let input = format!("{host}|{user}|{os}|FM27-PRO-V1");
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for b in input.as_bytes() {
        hash ^= u64::from(*b);
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    format!("{hash:016x}")
}

pub fn current_machine_tag() -> String {
    let host = std::env::var("COMPUTERNAME")
        .or_else(|_| std::env::var("HOSTNAME"))
        .unwrap_or_else(|_| "unknown-host".into());
    let user = std::env::var("USERNAME")
        .or_else(|_| std::env::var("USER"))
        .unwrap_or_else(|_| "unknown-user".into());
    compute_machine_tag(&host, &user, std::env::consts::OS)
}

fn current_instance_name() -> String {
    let host = std::env::var("COMPUTERNAME")
        .or_else(|_| std::env::var("HOSTNAME"))
        .unwrap_or_else(|_| "FM27-PC".into());
    format!("FM27-{host}-{}", std::env::consts::OS)
}

pub fn mask_license_key(key: &str) -> String {
    let trimmed = key.trim();
    let chars: Vec<char> = trimmed.chars().collect();
    if chars.len() <= 8 {
        if chars.len() <= 4 {
            return "••••".to_string();
        }
        let tail: String = chars[chars.len() - 4..].iter().collect();
        return format!("••••-{tail}");
    }
    let head: String = chars[..4].iter().collect();
    let tail: String = chars[chars.len() - 4..].iter().collect();
    format!("{head}-••••-{tail}")
}

pub(crate) fn is_stored_pro_at(stored: Option<&StoredLicense>, now: i64, expected_machine: &str) -> bool {
    let Some(s) = stored else { return false };
    !s.license_key.trim().is_empty()
        && s.machine_tag == expected_machine
        && now.saturating_sub(s.last_verified_unix) <= OFFLINE_GRACE_SECS
}

fn load_stored_license() -> Option<StoredLicense> {
    let s = std::fs::read_to_string(license_file()).ok()?;
    serde_json::from_str(&s).ok()
}

fn save_stored_license(lic: &StoredLicense) {
    let _ = std::fs::create_dir_all(crate::data_dir());
    if let Ok(json) = serde_json::to_string_pretty(lic) {
        let _ = std::fs::write(license_file(), json);
    }
}

fn remove_stored_license() {
    let _ = std::fs::remove_file(license_file());
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

fn post_form(url: &str, params: &[(&str, &str)]) -> Result<String, String> {
    let mut cmd = new_curl_cmd();
    cmd.args(["-sS", "--max-time", "15", "-H", "Accept: application/json", "-X", "POST"]);
    for (k, v) in params {
        cmd.arg("--data-urlencode").arg(format!("{k}={v}"));
    }
    cmd.arg(url).stdin(Stdio::null());

    let out = cmd.output().map_err(|e| {
        t!(
            format!("Netwerkopdracht mislukt: {e}"),
            format!("Network request failed: {e}"),
            format!("Ağ isteği başarısız oldu: {e}")
        )
    })?;

    let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if !out.status.success() && stdout.is_empty() {
        let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
        return Err(if stderr.is_empty() {
            t!(
                "Geen verbinding met de licentieserver.",
                "Could not reach license server.",
                "Lisans sunucusuna ulaşılamadı."
            )
            .to_string()
        } else {
            stderr
        });
    }
    Ok(stdout)
}

pub(crate) fn parse_lemon_activate_response(
    raw: &str,
    expected_store_id: Option<u64>,
    expected_product_id: Option<u64>,
) -> VerifyOutcome {
    let Ok(v) = serde_json::from_str::<Value>(raw) else {
        return VerifyOutcome::NetworkError("Invalid JSON response".into());
    };

    let meta = v.get("meta");
    if let Some(req_store) = expected_store_id {
        let got_store = meta.and_then(|m| m.get("store_id")).and_then(|x| x.as_u64());
        if got_store != Some(req_store) {
            return VerifyOutcome::Revoked(
                t!(
                    "Licentiesleutel hoort bij een ander product.",
                    "License key belongs to a different store.",
                    "Lisans anahtarı başka bir mağazaya ait."
                )
                .to_string(),
            );
        }
    }
    if let Some(req_prod) = expected_product_id {
        let got_prod = meta.and_then(|m| m.get("product_id")).and_then(|x| x.as_u64());
        if got_prod != Some(req_prod) {
            return VerifyOutcome::Revoked(
                t!(
                    "Licentiesleutel hoort bij een ander product.",
                    "License key belongs to a different product.",
                    "Lisans anahtarı başka bir ürüne ait."
                )
                .to_string(),
            );
        }
    }

    let instance_id = v
        .get("instance")
        .and_then(|i| i.get("id"))
        .and_then(|s| s.as_str())
        .map(str::to_string);
    let key_status = v
        .get("license_key")
        .and_then(|k| k.get("status"))
        .and_then(|s| s.as_str())
        .unwrap_or("");
    let activated = v.get("activated").and_then(|b| b.as_bool()) == Some(true)
        || (key_status == "active" && instance_id.is_some());

    if activated {
        let email = meta
            .and_then(|m| m.get("customer_email"))
            .and_then(|s| s.as_str())
            .map(str::to_string);
        return VerifyOutcome::Valid {
            provider: "lemonsqueezy".to_string(),
            instance_id,
            customer_email: email,
        };
    }

    let err = v
        .get("error")
        .and_then(|s| s.as_str())
        .filter(|s| !s.is_empty())
        .unwrap_or("Invalid or activation-limit reached license key.");
    VerifyOutcome::Revoked(err.to_string())
}

pub(crate) fn parse_lemon_validate_response(
    raw: &str,
    expected_store_id: Option<u64>,
    expected_product_id: Option<u64>,
) -> VerifyOutcome {
    let Ok(v) = serde_json::from_str::<Value>(raw) else {
        return VerifyOutcome::NetworkError("Invalid JSON response".into());
    };
    let meta = v.get("meta");
    if let Some(req_store) = expected_store_id {
        let got_store = meta.and_then(|m| m.get("store_id")).and_then(|x| x.as_u64());
        if got_store.is_some() && got_store != Some(req_store) {
            return VerifyOutcome::Revoked("Store mismatch.".into());
        }
    }
    if let Some(req_prod) = expected_product_id {
        let got_prod = meta.and_then(|m| m.get("product_id")).and_then(|x| x.as_u64());
        if got_prod.is_some() && got_prod != Some(req_prod) {
            return VerifyOutcome::Revoked("Product mismatch.".into());
        }
    }
    let key_status = v
        .get("license_key")
        .and_then(|k| k.get("status"))
        .and_then(|s| s.as_str())
        .unwrap_or("");
    let valid = v.get("valid").and_then(|b| b.as_bool()) == Some(true)
        && !matches!(key_status, "expired" | "disabled");

    if valid {
        let email = meta
            .and_then(|m| m.get("customer_email"))
            .and_then(|s| s.as_str())
            .map(str::to_string);
        let instance_id = v
            .get("instance")
            .and_then(|i| i.get("id"))
            .and_then(|s| s.as_str())
            .map(str::to_string);
        VerifyOutcome::Valid {
            provider: "lemonsqueezy".to_string(),
            instance_id,
            customer_email: email,
        }
    } else if v.get("valid").is_some() || !key_status.is_empty() {
        let err = v
            .get("error")
            .and_then(|s| s.as_str())
            .filter(|s| !s.is_empty())
            .unwrap_or("License key expired or revoked.");
        VerifyOutcome::Revoked(err.to_string())
    } else {
        VerifyOutcome::NetworkError("Unexpected license validation payload".into())
    }
}

pub(crate) fn parse_gumroad_response(raw: &str) -> VerifyOutcome {
    let Ok(v) = serde_json::from_str::<Value>(raw) else {
        return VerifyOutcome::NetworkError("Invalid JSON response".into());
    };
    if v.get("success").and_then(|b| b.as_bool()) != Some(true) {
        let msg = v
            .get("message")
            .and_then(|s| s.as_str())
            .unwrap_or("License key not valid.");
        return VerifyOutcome::Revoked(msg.to_string());
    }
    let purchase = v.get("purchase");
    let refunded = purchase
        .and_then(|p| p.get("refunded"))
        .and_then(|b| b.as_bool())
        == Some(true);
    let chargebacked = purchase
        .and_then(|p| p.get("chargebacked"))
        .and_then(|b| b.as_bool())
        == Some(true);
    let cancelled = purchase
        .and_then(|p| p.get("subscription_cancelled_at"))
        .and_then(|x| x.as_str())
        .is_some_and(|s| !s.is_empty());
    let failed = purchase
        .and_then(|p| p.get("subscription_failed_at"))
        .and_then(|x| x.as_str())
        .is_some_and(|s| !s.is_empty());
    if refunded || chargebacked || cancelled || failed {
        return VerifyOutcome::Revoked(
            t!(
                "Deze licentie is terugbetaald of geannuleerd.",
                "This license was refunded or cancelled.",
                "Bu lisans iade edilmiş veya iptal edilmiş."
            )
            .to_string(),
        );
    }
    let email = purchase
        .and_then(|p| p.get("email"))
        .and_then(|s| s.as_str())
        .map(str::to_string);
    VerifyOutcome::Valid {
        provider: "gumroad".to_string(),
        instance_id: None,
        customer_email: email,
    }
}

fn activate_online(key: &str) -> Result<StoredLicense, String> {
    let clean = key.trim();
    if clean.len() < 8 {
        return Err(t!(
            "Voer een geldige licentiesleutel in.",
            "Enter a valid license key.",
            "Geçerli bir lisans anahtarı girin."
        )
        .to_string());
    }

    let inst_name = current_instance_name();
    let lemon_outcome = match post_form(
        "https://api.lemonsqueezy.com/v1/licenses/activate",
        &[("license_key", clean), ("instance_name", &inst_name)],
    ) {
        Ok(raw) => parse_lemon_activate_response(&raw, LEMON_STORE_ID, LEMON_PRODUCT_ID),
        Err(e) => VerifyOutcome::NetworkError(e),
    };

    let outcome = match lemon_outcome {
        VerifyOutcome::Valid { .. } => lemon_outcome,
        other if !GUMROAD_PRODUCT_ID.trim().is_empty() => {
            match post_form(
                "https://api.gumroad.com/v2/licenses/verify",
                &[
                    ("product_id", GUMROAD_PRODUCT_ID.trim()),
                    ("license_key", clean),
                    ("increment_uses_count", "true"),
                ],
            ) {
                Ok(raw) => parse_gumroad_response(&raw),
                Err(_) => other,
            }
        }
        other => other,
    };

    match outcome {
        VerifyOutcome::Valid {
            provider,
            instance_id,
            customer_email,
        } => {
            let now = now_unix();
            let lic = StoredLicense {
                provider,
                license_key: clean.to_string(),
                key_masked: mask_license_key(clean),
                instance_id,
                customer_email,
                activated_at_unix: now,
                last_verified_unix: now,
                machine_tag: current_machine_tag(),
            };
            save_stored_license(&lic);
            Ok(lic)
        }
        VerifyOutcome::Revoked(msg) | VerifyOutcome::NetworkError(msg) => Err(msg),
    }
}

fn revalidate_online(stored: &StoredLicense) -> VerifyOutcome {
    if stored.provider == "gumroad" {
        if GUMROAD_PRODUCT_ID.trim().is_empty() {
            return VerifyOutcome::NetworkError("Gumroad product ID unset".into());
        }
        return match post_form(
            "https://api.gumroad.com/v2/licenses/verify",
            &[
                ("product_id", GUMROAD_PRODUCT_ID.trim()),
                ("license_key", stored.license_key.as_str()),
                ("increment_uses_count", "false"),
            ],
        ) {
            Ok(raw) => parse_gumroad_response(&raw),
            Err(e) => VerifyOutcome::NetworkError(e),
        };
    }

    let mut params = vec![("license_key", stored.license_key.as_str())];
    if let Some(ref iid) = stored.instance_id {
        params.push(("instance_id", iid.as_str()));
    }
    match post_form("https://api.lemonsqueezy.com/v1/licenses/validate", &params) {
        Ok(raw) => parse_lemon_validate_response(&raw, LEMON_STORE_ID, LEMON_PRODUCT_ID),
        Err(e) => VerifyOutcome::NetworkError(e),
    }
}

fn deactivate_online(stored: &StoredLicense) {
    if stored.provider == "lemonsqueezy"
        && let Some(ref iid) = stored.instance_id
    {
        let _ = post_form(
            "https://api.lemonsqueezy.com/v1/licenses/deactivate",
            &[
                ("license_key", stored.license_key.as_str()),
                ("instance_id", iid.as_str()),
            ],
        );
    }
    remove_stored_license();
}

impl LicenseState {
    pub fn is_pro(&self) -> bool {
        is_stored_pro_at(self.stored.as_ref(), now_unix(), &current_machine_tag())
    }

    pub fn activate(&mut self, ctx: &egui::Context) {
        if self.busy {
            return;
        }
        let key = self.key_input.trim().to_string();
        if key.is_empty() {
            self.status_msg = t!(
                "Voer een geldige licentiesleutel in.",
                "Enter a valid license key.",
                "Geçerli bir lisans anahtarı girin."
            )
            .to_string();
            self.status_err = true;
            return;
        }
        self.busy = true;
        self.status_msg = t!("Licentie controleren…", "Checking license…", "Lisans doğrulanıyor…").to_string();
        self.status_err = false;
        let (tx, rx) = mpsc::channel();
        let ctx2 = ctx.clone();
        std::thread::spawn(move || {
            let res = activate_online(&key);
            let _ = tx.send(LicenseJobResult::Activated(res));
            ctx2.request_repaint();
        });
        self.rx = Some(rx);
    }

    pub fn deactivate(&mut self, ctx: &egui::Context) {
        if self.busy {
            return;
        }
        let Some(cur) = self.stored.clone() else { return };
        self.busy = true;
        let (tx, rx) = mpsc::channel();
        let ctx2 = ctx.clone();
        std::thread::spawn(move || {
            deactivate_online(&cur);
            let _ = tx.send(LicenseJobResult::Deactivated);
            ctx2.request_repaint();
        });
        self.rx = Some(rx);
    }

    pub fn tick(&mut self, ctx: &egui::Context) {
        if !self.startup_checked {
            self.startup_checked = true;
            if std::env::var_os("FM27_SNAPSHOT").is_none()
                && let Some(cur) = self.stored.clone()
                && !self.busy
                && (cur.machine_tag != current_machine_tag()
                    || now_unix().saturating_sub(cur.last_verified_unix) >= REVERIFY_INTERVAL_SECS)
            {
                let (tx, rx) = mpsc::channel();
                let ctx2 = ctx.clone();
                std::thread::spawn(move || {
                    let outcome = revalidate_online(&cur);
                    let _ = tx.send(LicenseJobResult::Reverified(outcome));
                    ctx2.request_repaint();
                });
                self.rx = Some(rx);
            }
        }

        if let Some(res) = self.rx.as_ref().and_then(|rx| rx.try_recv().ok()) {
            self.rx = None;
            self.busy = false;
            match res {
                LicenseJobResult::Activated(Ok(lic)) => {
                    self.stored = Some(lic);
                    self.key_input.clear();
                    self.status_msg = t!(
                        "Pro-licentie geactiveerd! Jeugdelftallen zijn ontgrendeld.",
                        "Pro license activated! Youth teams are unlocked.",
                        "Pro lisans etkinleştirildi! Altyapı takımları açıldı."
                    )
                    .to_string();
                    self.status_err = false;
                }
                LicenseJobResult::Activated(Err(e)) => {
                    self.status_msg = e;
                    self.status_err = true;
                }
                LicenseJobResult::Reverified(VerifyOutcome::Valid {
                    provider,
                    instance_id,
                    customer_email,
                }) => {
                    if let Some(ref mut s) = self.stored {
                        s.provider = provider;
                        if instance_id.is_some() {
                            s.instance_id = instance_id;
                        }
                        if customer_email.is_some() {
                            s.customer_email = customer_email;
                        }
                        s.last_verified_unix = now_unix();
                        s.machine_tag = current_machine_tag();
                        save_stored_license(s);
                    }
                }
                LicenseJobResult::Reverified(VerifyOutcome::Revoked(reason)) => {
                    self.stored = None;
                    remove_stored_license();
                    self.status_msg = reason;
                    self.status_err = true;
                }
                LicenseJobResult::Reverified(VerifyOutcome::NetworkError(_)) => {}
                LicenseJobResult::Deactivated => {
                    self.stored = None;
                    self.status_msg = t!(
                        "Licentie gedeactiveerd op dit apparaat.",
                        "License deactivated on this device.",
                        "Lisans bu cihazdan kaldırıldı."
                    )
                    .to_string();
                    self.status_err = false;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_lemonsqueezy_activation_and_store_filter() {
        let ok_json = r#"{
            "activated": true,
            "license_key": { "id": 1, "status": "active", "key": "38B1460A-510D-4067" },
            "instance": { "id": "inst_abc123", "name": "PC" },
            "meta": { "store_id": 12345, "product_id": 67890, "customer_email": "coach@club.org" }
        }"#;
        assert_eq!(
            parse_lemon_activate_response(ok_json, Some(12345), Some(67890)),
            VerifyOutcome::Valid {
                provider: "lemonsqueezy".into(),
                instance_id: Some("inst_abc123".into()),
                customer_email: Some("coach@club.org".into()),
            }
        );
        assert!(matches!(
            parse_lemon_activate_response(ok_json, Some(99999), None),
            VerifyOutcome::Revoked(_)
        ));
        assert!(matches!(
            parse_lemon_activate_response(ok_json, Some(12345), Some(11111)),
            VerifyOutcome::Revoked(_)
        ));
    }

    #[test]
    fn parses_gumroad_verification_and_rejects_refunded() {
        let valid_json = r#"{
            "success": true,
            "uses": 1,
            "purchase": { "email": "manager@fm.io", "refunded": false, "chargebacked": false }
        }"#;
        assert_eq!(
            parse_gumroad_response(valid_json),
            VerifyOutcome::Valid {
                provider: "gumroad".into(),
                instance_id: None,
                customer_email: Some("manager@fm.io".into()),
            }
        );

        let refunded_json = r#"{
            "success": true,
            "purchase": { "email": "manager@fm.io", "refunded": true, "chargebacked": false }
        }"#;
        assert!(matches!(parse_gumroad_response(refunded_json), VerifyOutcome::Revoked(_)));
    }

    #[test]
    fn machine_tag_and_offline_grace_enforced() {
        let tag = compute_machine_tag("DESKTOP-1", "yasin", "windows");
        let lic = StoredLicense {
            provider: "lemonsqueezy".into(),
            license_key: "TEST-1234-5678-90AB".into(),
            key_masked: "TEST-••••-90AB".into(),
            instance_id: Some("inst_1".into()),
            customer_email: None,
            activated_at_unix: 1_000_000,
            last_verified_unix: 1_000_000,
            machine_tag: tag.clone(),
        };
        assert!(is_stored_pro_at(Some(&lic), 1_000_000 + 86_400, &tag));
        assert!(!is_stored_pro_at(Some(&lic), 1_000_000 + OFFLINE_GRACE_SECS + 1, &tag));
        assert!(!is_stored_pro_at(Some(&lic), 1_000_000 + 60, "different-machine"));
        assert!(!is_stored_pro_at(None, 1_000_000, &tag));
    }

    #[test]
    fn masks_license_key_cleanly() {
        assert_eq!(mask_license_key("38B1460A-510D-4067-9F2A"), "38B1-••••-9F2A");
        assert_eq!(mask_license_key("ABCD12"), "••••-CD12");
        assert_eq!(mask_license_key("XYZ"), "••••");
    }
}
