//! Bireysel oyuncu analizleri (100% hilesiz):
//! 1. Taktiksel Verimlilik Darboğaz Raporu ("Taktiği kim aşağı çekiyor?") — BepInEx `Players` verisinden
//! 2. Sakatlık Önleyici Akıllı Bireysel Antrenman Yoğunluğu (Yarı / Normal / Çift) — FM27 resmi UI olayıyla

use eframe::egui::{self, CornerRadius, Frame, Margin, RichText, Stroke, Ui, vec2};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::live::LiveState;
use crate::model::{level_color, level_name};
use crate::t;
use crate::theme::{self, icon};

/// FM27 Bireysel Antrenman Yoğunluğu seviyeleri (`EventFmxPlayerTrainingIntensityLevelChanged`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntensityLevel {
    Half = 0,
    Normal = 1,
    Double = 2,
}

impl IntensityLevel {
    pub fn code(self) -> i32 {
        self as i32
    }

    pub fn from_code(c: i64) -> Option<Self> {
        match c {
            0 => Some(Self::Half),
            1 => Some(Self::Normal),
            2 => Some(Self::Double),
            _ => None,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Half => t!("Halve intensiteit", "Half Intensity", "Yarı Yoğunluk"),
            Self::Normal => t!("Normale intensiteit", "Normal Intensity", "Normal Yoğunluk"),
            Self::Double => t!("Dubbele intensiteit", "Double Intensity", "Çift Yoğunluk"),
        }
    }

    pub fn short_label(self) -> &'static str {
        match self {
            Self::Half => t!("Half", "Half", "Yarı"),
            Self::Normal => t!("Normaal", "Normal", "Normal"),
            Self::Double => t!("Dubbel", "Double", "Çift"),
        }
    }

    pub fn color(self) -> egui::Color32 {
        match self {
            Self::Half => theme::INFO,
            Self::Normal => theme::SUCCESS,
            Self::Double => theme::WARNING,
        }
    }
}

#[derive(Clone, Debug)]
pub struct IntensityRecommendation {
    pub runtime_id: u32,
    pub name: String,
    pub age: Option<u8>,
    pub condition: f32,
    pub sharpness: f32,
    pub injured: bool,
    pub recommended: IntensityLevel,
    pub current: Option<IntensityLevel>,
    pub rest_match_warning: bool,
    pub reason: String,
}

#[derive(Clone, Debug)]
pub struct PlayerBottleneck {
    pub runtime_id: u32,
    pub name: String,
    pub overall: f32,
    pub understanding: Option<f32>,
    pub capability: Option<f32>,
    pub role_suitability: Option<f32>,
    pub buy_in: Option<f32>,
    pub weakest_label: &'static str,
    pub weakest_score: f32,
    pub advice: &'static str,
}

#[derive(Default)]
pub struct IndividualState {
    pub bottlenecks: Vec<PlayerBottleneck>,
    pub current_intensities: HashMap<u32, IntensityLevel>,
    pub session_ready: bool,
    pub auto_apply_intensity: bool,
    pub filter_bottlenecks_only: bool,
    pub last_cmd_result_utc: String,
    pub last_auto_apply: Option<std::time::Instant>,
    pub status: String,
    pub status_error: bool,
}

fn cmd_file() -> PathBuf {
    crate::data_dir().join("bepinex-intensity-cmd.json")
}

fn cmd_result_file() -> PathBuf {
    crate::data_dir().join("bepinex-intensity-result.json")
}

fn find_prefix<'a>(obj: &'a Value, prefix: &str) -> Option<&'a Value> {
    let map = obj.as_object()?;
    let with_colon = format!("{prefix}:");
    map.iter()
        .find(|(k, _)| k.eq_ignore_ascii_case(prefix) || k.len() > with_colon.len() && k[..with_colon.len()].eq_ignore_ascii_case(&with_colon))
        .map(|(_, v)| v)
}

fn parse_num(v: &Value) -> Option<f32> {
    v.as_f64().map(|n| n as f32).or_else(|| v.as_str()?.trim().parse::<f32>().ok())
}

fn score_of(node: &Value) -> Option<f32> {
    if let Some(n) = parse_num(node) {
        return Some(n.clamp(0.0, 100.0));
    }
    let inner = find_prefix(node, "OverallEffectiveness")
        .or_else(|| find_prefix(node, "Score"))
        .or_else(|| find_prefix(node, "Value"))
        .unwrap_or(node);
    let raw = find_prefix(inner, "MaxValue")
        .or_else(|| find_prefix(inner, "MinValue"))
        .or_else(|| find_prefix(inner, "Value"))?;
    parse_num(raw).map(|v| v.clamp(0.0, 100.0))
}

fn extract_runtime_id(node: &Value) -> Option<u32> {
    let map = node.as_object()?;
    if let Some(id) = map.get("runtime_id").and_then(|v| v.as_u64()) {
        if id > 0 {
            return Some(id as u32);
        }
    }
    for (_, v) in map {
        if v.is_object() {
            if let Some(id) = extract_runtime_id(v) {
                return Some(id);
            }
        }
    }
    None
}

fn extract_name(node: &Value) -> Option<String> {
    let map = node.as_object()?;
    for key in ["Name", "DisplayName", "ShortName", "FullName", "PlayerName"] {
        if let Some(s) = find_prefix(node, key).and_then(|v| v.as_str()) {
            let trimmed = s.trim();
            if !trimmed.is_empty() && !trimmed.starts_with('<') {
                return Some(trimmed.to_string());
            }
        }
    }
    for key in ["Person", "Player", "PlayerInfo"] {
        if let Some(sub) = find_prefix(node, key) {
            if let Some(n) = extract_name(sub) {
                return Some(n);
            }
        }
    }
    let _ = map;
    None
}

/// BepInEx JSON kökünden oyuncu darboğaz listesini ve mevcut yoğunluk haritasını çözümler.
pub fn parse_bepinex_individual(
    root: &Value,
    squad_names: &HashMap<u32, String>,
) -> (Vec<PlayerBottleneck>, HashMap<u32, IntensityLevel>, bool) {
    let session_ready = root.get("session_ready").and_then(|v| v.as_bool()).unwrap_or(false);
    let mut intensities = HashMap::new();
    if let Some(ci) = root.get("current_intensities").and_then(|v| v.as_object()) {
        for (k, v) in ci {
            if let (Ok(rid), Some(lvl)) = (k.parse::<u32>(), v.as_i64().and_then(IntensityLevel::from_code)) {
                intensities.insert(rid, lvl);
            }
        }
    }

    let mut out = Vec::new();
    let Some(data) = root.get("data").and_then(|v| v.as_object()) else {
        return (out, intensities, session_ready);
    };
    let Some(te) = data
        .iter()
        .find(|(k, _)| k.starts_with("TeamTactic:") && k.ends_with(".TacticalEffectiveness"))
        .or_else(|| data.iter().find(|(k, _)| k.ends_with(".TacticalEffectiveness")))
        .map(|(_, v)| v)
    else {
        return (out, intensities, session_ready);
    };

    let Some(players) = find_prefix(te, "Players").and_then(|v| v.as_array()) else {
        return (out, intensities, session_ready);
    };

    for item in players {
        let runtime_id = extract_runtime_id(item).unwrap_or(0);
        let name = squad_names
            .get(&runtime_id)
            .cloned()
            .or_else(|| extract_name(item))
            .unwrap_or_else(|| {
                if runtime_id != 0 {
                    format!("Oyuncu #{runtime_id}")
                } else {
                    "—".to_string()
                }
            });

        let understanding = find_prefix(item, "TacticalUnderstanding")
            .or_else(|| find_prefix(item, "PlayerTacticalUnderstanding"))
            .or_else(|| find_prefix(item, "Comprehension"))
            .and_then(score_of);

        let pa_node = find_prefix(item, "PlayerAbility");
        let capability = find_prefix(item, "CapabilityKeyFactor")
            .or_else(|| find_prefix(item, "Capability"))
            .or_else(|| pa_node.and_then(|pa| find_prefix(pa, "CapabilityKeyFactor")))
            .or_else(|| pa_node)
            .and_then(score_of);

        let role_suitability = find_prefix(item, "RoleSuitabilityKeyFactor")
            .or_else(|| find_prefix(item, "RoleSuitability"))
            .or_else(|| pa_node.and_then(|pa| find_prefix(pa, "RoleSuitabilityKeyFactor")))
            .and_then(score_of);

        let buy_in = find_prefix(item, "PlayerBuyIn")
            .or_else(|| find_prefix(item, "BuyIn"))
            .and_then(score_of);

        let overall = find_prefix(item, "OverallEffectiveness")
            .and_then(score_of)
            .or_else(|| {
                let parts: Vec<f32> = [understanding, capability, role_suitability, buy_in].into_iter().flatten().collect();
                (!parts.is_empty()).then(|| parts.iter().sum::<f32>() / parts.len() as f32)
            });

        let Some(overall) = overall else { continue };

        let candidates: [(&'static str, Option<f32>, &'static str); 4] = [
            (
                t!("Tactisch begrip", "Tactical Understanding", "Taktik Anlayış"),
                understanding,
                t!(
                    "Geef minuten in oefen-/rotatiewedstrijden; stijgt via tactische teamsessies.",
                    "Give minutes in rotation matches; rises through team tactical sessions.",
                    "Rotasyon maçlarında süre verin; takım taktik seanslarıyla hızla yükselir."
                ),
            ),
            (
                t!("Rolgeschiktheid", "Role Suitability", "Rol Uyumu"),
                role_suitability,
                t!(
                    "Past minder bij zijn huidige rol; bekijk het tabblad Tactiek voor een betere rol.",
                    "Less suited to current role; check Tactics tab for a better-fitting role.",
                    "Mevcut rolüne tam uymuyor; Taktikler sekmesindeki alternatif rol önerisine bakın."
                ),
            ),
            (
                t!("Spelerskwaliteit", "Player Capability", "Oyuncu Kalitesi"),
                capability,
                t!(
                    "Blijft achter bij het teamniveau; overweeg rotatie in topwedstrijden.",
                    "Below squad level; consider rotation in big matches.",
                    "Takım seviyesinin altında kalıyor; zorlu maçlarda rotasyon düşünün."
                ),
            ),
            (
                t!("Buy-In", "Buy-In", "Benimseme (Buy-In)"),
                buy_in,
                t!(
                    "Relaties of steun zijn laag; Team Bonding en speeltijd helpen.",
                    "Relationships or support are low; Team Bonding and playing time help.",
                    "İlişki/destek düşük; Takım Kaynaşması seansı ve forma süresi artırır."
                ),
            ),
        ];

        let (weakest_label, weakest_score, advice) = candidates
            .into_iter()
            .filter_map(|(lbl, sc, adv)| sc.map(|v| (lbl, v, adv)))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or((
                t!("Algemeen", "Overall", "Genel Uyum"),
                overall,
                t!(
                    "Onder het teamgemiddelde in deze tactiek.",
                    "Below squad average in this tactic.",
                    "Bu taktikte takım ortalamasının altında."
                ),
            ));

        out.push(PlayerBottleneck {
            runtime_id,
            name,
            overall,
            understanding,
            capability,
            role_suitability,
            buy_in,
            weakest_label,
            weakest_score,
            advice,
        });
    }

    out.sort_by(|a, b| a.overall.partial_cmp(&b.overall).unwrap_or(std::cmp::Ordering::Equal));
    (out, intensities, session_ready)
}

/// Sakatlık önleyici akıllı bireysel yoğunluk algoritması (100% meşru FM yoğunluk seviyeleri).
pub fn recommend_intensity(
    age: Option<u8>,
    condition_pct: f32,
    sharpness_pct: f32,
    injured: bool,
    week_matches: usize,
) -> (IntensityLevel, bool, String) {
    if injured {
        return (
            IntensityLevel::Half,
            true,
            t!(
                "Geblesseerd → halve intensiteit voor veilig herstel.",
                "Injured → half intensity for safe recovery.",
                "Sakat → güvenli dönüş için Yarı Yoğunluk."
            )
            .to_string(),
        );
    }

    if condition_pct < 70.0 {
        return (
            IntensityLevel::Half,
            true,
            t!(
                format!("Conditie {condition_pct:.0}% (<70%) → hoog blessurerisico, geef rust in wedstrijd."),
                format!("Condition {condition_pct:.0}% (<70%) → high injury risk, rest on matchday."),
                format!("Kondisyon %{condition_pct:.0} (<%70) → sakatlık riski yüksek, maçta dinlendirin.")
            ),
        );
    }

    if condition_pct < 80.0 {
        return (
            IntensityLevel::Half,
            false,
            t!(
                format!("Conditie {condition_pct:.0}% (<80%) → herstel krijgt voorrang."),
                format!("Condition {condition_pct:.0}% (<80%) → recovery priority."),
                format!("Kondisyon %{condition_pct:.0} (<%80) → toparlanma öncelikli.")
            ),
        );
    }

    let veteran = age.is_some_and(|a| a >= 32);
    if veteran {
        let a = age.unwrap_or(32);
        if week_matches >= 2 || condition_pct < 88.0 {
            return (
                IntensityLevel::Half,
                false,
                t!(
                    format!("Veteraan ({a}j) + druk schema/conditie {condition_pct:.0}% → spierbelasting beperken."),
                    format!("Veteran ({a}y) + congested week/condition {condition_pct:.0}% → limit muscle load."),
                    format!("Veteran ({a} yaş) + yoğun hafta/kondisyon %{condition_pct:.0} → kas yükü koruması.")
                ),
            );
        }
        return (
            IntensityLevel::Normal,
            false,
            t!(
                format!("Veteraan ({a}j) fris ({condition_pct:.0}%) → maximaal Normaal om blessures te vermijden."),
                format!("Veteran ({a}y) fresh ({condition_pct:.0}%) → capped at Normal to prevent injuries."),
                format!("Veteran ({a} yaş, %{condition_pct:.0}) → sakatlık önlemek için en fazla Normal.")
            ),
        );
    }

    if week_matches >= 2 {
        if condition_pct < 85.0 {
            return (
                IntensityLevel::Half,
                false,
                t!(
                    format!("{week_matches} wedstrijden deze week + conditie {condition_pct:.0}% → overbelasting voorkomen."),
                    format!("{week_matches} matches this week + condition {condition_pct:.0}% → prevent overload."),
                    format!("Bu hafta {week_matches} maç + kondisyon %{condition_pct:.0} → aşırı yüklenme koruması.")
                ),
            );
        }
        if condition_pct >= 93.0 && sharpness_pct < 72.0 && age.is_none_or(|a| a < 30) {
            return (
                IntensityLevel::Double,
                false,
                t!(
                    format!("Wisselspeler: conditie {condition_pct:.0}%, scherpte {sharpness_pct:.0}% → scherpte opbouwen."),
                    format!("Rotation player: condition {condition_pct:.0}%, sharpness {sharpness_pct:.0}% → build sharpness."),
                    format!("Rotasyon oyuncusu: kondisyon %{condition_pct:.0}, keskinlik %{sharpness_pct:.0} → maç ritmi kazanımı.")
                ),
            );
        }
        return (
            IntensityLevel::Normal,
            false,
            t!(
                format!("{week_matches} wedstrijden deze week → Dubbel vermeden voor blessurepreventie."),
                format!("{week_matches} matches this week → Double avoided for injury prevention."),
                format!("Bu hafta {week_matches} maç → sakatlık koruması için Çift Yoğunluk kapatıldı.")
            ),
        );
    }

    // 0 veya 1 maçlı hafta
    let young = age.is_some_and(|a| a <= 23);
    let under_31 = age.is_none_or(|a| a <= 30);
    if condition_pct >= 92.0 && under_31 && (young || sharpness_pct < 80.0 || week_matches == 0 || condition_pct >= 95.0) {
        let detail = if young {
            t!(
                format!("Jong talent ({}j) & fris ({condition_pct:.0}%) → maximale ontwikkeling.", age.unwrap_or(20)),
                format!("Young prospect ({}y) & fresh ({condition_pct:.0}%) → max development.", age.unwrap_or(20)),
                format!("Genç oyuncu ({} yaş) & diri (%{condition_pct:.0}) → maksimum gelişim.", age.unwrap_or(20))
            )
        } else if sharpness_pct < 80.0 {
            t!(
                format!("Fris ({condition_pct:.0}%), lage scherpte ({sharpness_pct:.0}%) → ritme verhogen."),
                format!("Fresh ({condition_pct:.0}%), low sharpness ({sharpness_pct:.0}%) → boost sharpness."),
                format!("Kondisyon yüksek (%{condition_pct:.0}), keskinlik düşük (%{sharpness_pct:.0}) → form yüklemesi.")
            )
        } else {
            t!(
                format!("Topconditie ({condition_pct:.0}%) & rustige week → veilig Dubbel trainen."),
                format!("Peak condition ({condition_pct:.0}%) & light week → safe for Double intensity."),
                format!("Zirve kondisyon (%{condition_pct:.0}) & rahat fikstür → güvenli Çift Yoğunluk.")
            )
        };
        return (IntensityLevel::Double, false, detail);
    }

    (
        IntensityLevel::Normal,
        false,
        t!(
            format!("Conditie {condition_pct:.0}% · gebalanceerde belasting."),
            format!("Condition {condition_pct:.0}% · balanced workload."),
            format!("Kondisyon %{condition_pct:.0} · dengeli bireysel yük.")
        ),
    )
}

pub fn build_squad_recommendations(
    live: &LiveState,
    current_map: &HashMap<u32, IntensityLevel>,
) -> (Vec<IntensityRecommendation>, usize) {
    let today = live.training.as_ref().and_then(|o| o.today);
    let week_matches = live
        .training
        .as_ref()
        .and_then(|o| {
            o.weeks
                .iter()
                .find(|w| today.is_none_or(|t| w.start + chrono::Duration::days(6) >= t))
                .map(|w| w.matches)
        })
        .unwrap_or(1);

    let mut rows = Vec::with_capacity(live.squad.len());
    for p in &live.squad {
        let runtime_id = p.get("runtime_id").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
        let name = p.get("name").and_then(|v| v.as_str()).unwrap_or("?").to_string();
        let age = p.get("age").and_then(|v| v.as_u64()).map(|a| a as u8);
        let cond_raw = p.get("condition").and_then(|v| v.as_u64()).unwrap_or(0);
        let sharp_raw = p.get("sharpness").and_then(|v| v.as_u64()).unwrap_or(0);
        let condition = (cond_raw as f32 / 100.0).clamp(0.0, 100.0);
        let sharpness = (sharp_raw as f32 / 100.0).clamp(0.0, 100.0);
        let injured = p.get("injuries").and_then(|v| v.as_array()).is_some_and(|a| !a.is_empty());

        if cond_raw == 0 && !injured {
            continue;
        }

        let (recommended, rest_match_warning, reason) =
            recommend_intensity(age, condition, sharpness, injured, week_matches);
        rows.push(IntensityRecommendation {
            runtime_id,
            name,
            age,
            condition,
            sharpness,
            injured,
            recommended,
            current: current_map.get(&runtime_id).copied(),
            rest_match_warning,
            reason,
        });
    }

    // Önce en yorgun / dikkat gerektiren oyuncular üstte görünsün
    rows.sort_by(|a, b| {
        b.rest_match_warning
            .cmp(&a.rest_match_warning)
            .then_with(|| a.condition.partial_cmp(&b.condition).unwrap_or(std::cmp::Ordering::Equal))
    });
    (rows, week_matches)
}

pub fn write_intensity_command(manager_runtime_id: u32, items: &[(u32, IntensityLevel)]) -> Result<usize, String> {
    let valid: Vec<Value> = items
        .iter()
        .filter(|(rid, _)| *rid != 0)
        .map(|(rid, lvl)| json!({ "runtime_id": rid, "level": lvl.code() }))
        .collect();
    if valid.is_empty() {
        return Err(t!(
            "Geen geldige spelers gevonden om in te stellen.",
            "No valid players found to apply.",
            "Uygulanacak geçerli oyuncu referansı bulunamadı."
        )
        .to_string());
    }
    let count = valid.len();
    let payload = json!({
        "manager_runtime_id": manager_runtime_id,
        "items": valid,
    });
    let _ = std::fs::create_dir_all(crate::data_dir());
    let path = cmd_file();
    let tmp = path.with_extension("json.tmp");
    let serialized = serde_json::to_string_pretty(&payload).map_err(|e| e.to_string())?;
    std::fs::write(&tmp, serialized).map_err(|e| e.to_string())?;
    std::fs::rename(&tmp, &path).map_err(|e| e.to_string())?;
    Ok(count)
}

impl IndividualState {
    pub fn poll_result(&mut self) {
        let Ok(text) = std::fs::read_to_string(cmd_result_file()) else { return };
        let Ok(v) = serde_json::from_str::<Value>(&text) else { return };
        let Some(utc) = v.get("updated_utc").and_then(|x| x.as_str()) else { return };
        if utc == self.last_cmd_result_utc {
            return;
        }
        if self.last_cmd_result_utc.is_empty() {
            // İlk açılışta eski sonuç dosyasını sessizce işaretle
            self.last_cmd_result_utc = utc.to_string();
            return;
        }
        self.last_cmd_result_utc = utc.to_string();
        let applied = v.get("applied").and_then(|x| x.as_u64()).unwrap_or(0);
        let errors: Vec<String> = v
            .get("errors")
            .and_then(|x| x.as_array())
            .map(|a| a.iter().filter_map(|e| e.as_str().map(str::to_string)).collect())
            .unwrap_or_default();
        if errors.is_empty() {
            self.status = t!(
                format!("Individuele intensiteit bijgewerkt voor {applied} spelers in FM27."),
                format!("Updated individual intensity for {applied} players in FM27."),
                format!("{applied} oyuncunun bireysel antrenman yoğunluğu FM27'de güncellendi.")
            );
            self.status_error = false;
        } else {
            self.status = format!(
                "{} ({applied} ok, {} err: {})",
                t!("Intensiteit deels toegepast", "Intensity partially applied", "Yoğunluk kısmen uygulandı"),
                errors.len(),
                errors.first().cloned().unwrap_or_default()
            );
            self.status_error = applied == 0;
        }
    }

    pub fn maybe_auto_apply(&mut self, live: &LiveState) {
        if !self.auto_apply_intensity || !live.connected || !self.session_ready {
            return;
        }
        if self.last_auto_apply.is_some_and(|t| t.elapsed().as_secs() < 15) {
            return;
        }
        let (rows, _) = build_squad_recommendations(live, &self.current_intensities);
        let diff: Vec<(u32, IntensityLevel)> = rows
            .iter()
            .filter(|r| r.runtime_id != 0 && r.current != Some(r.recommended))
            .map(|r| (r.runtime_id, r.recommended))
            .collect();
        self.last_auto_apply = Some(std::time::Instant::now());
        if diff.is_empty() {
            return;
        }
        let mgr_id = live
            .club
            .as_ref()
            .and_then(|c| c.get("manager_runtime_id"))
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u32;
        if write_intensity_command(mgr_id, &diff).is_ok() {
            for (rid, lvl) in diff {
                self.current_intensities.insert(rid, lvl);
            }
        }
    }
}

/// Bireysel Analiz & Yoğunluk sayfası.
pub fn render_page(
    ui: &mut Ui,
    ctx: &egui::Context,
    state: &mut IndividualState,
    live: &mut LiveState,
    on_read_bepinex: &mut bool,
    on_save_auto_toggle: &mut bool,
) {
    let (recs, week_matches) = build_squad_recommendations(live, &state.current_intensities);
    let mgr_id = live
        .club
        .as_ref()
        .and_then(|c| c.get("manager_runtime_id"))
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as u32;

    let can_apply = live.connected && !recs.is_empty();
    let mut apply_all = false;
    let mut refresh_squad = false;

    theme::page_header(
        ui,
        t!("Individueel & Knelpunten", "Individual & Bottlenecks", "Bireysel & Darboğaz"),
        t!(
            "Zie welke spelers de tactiek omlaag halen en stuur blessureveilige individuele intensiteit naar FM27.",
            "See which players drag down tactical effectiveness and apply injury-safe individual training intensity to FM27.",
            "Taktiği aşağı çeken oyuncuları görün ve sakatlık önleyici akıllı bireysel yoğunluğu tek tıkla FM27'ye uygulayın."
        ),
        |ui| {
            if theme::button(
                ui,
                icon::ARROW_COUNTER_CLOCKWISE,
                t!("Ververs selectie", "Refresh squad", "Kadroyu yenile"),
                !live.busy(),
            )
            .clicked()
            {
                refresh_squad = true;
            }
            if theme::button(
                ui,
                icon::BRAIN,
                t!("Lees uit BepInEx", "Read from BepInEx", "BepInEx'ten oku"),
                true,
            )
            .clicked()
            {
                *on_read_bepinex = true;
            }
            if theme::primary_button(
                ui,
                icon::LIGHTNING,
                t!(
                    "Zet aanbevolen intensiteit in FM",
                    "Apply recommended intensity",
                    "Önerilen yoğunlukları FM'ye uygula"
                ),
                can_apply,
            )
            .clicked()
            {
                apply_all = true;
            }
        },
    );

    if refresh_squad {
        live.refresh(ctx, t!("verversen", "refresh", "yenile"), None);
    }

    if apply_all {
        let batch: Vec<(u32, IntensityLevel)> = recs
            .iter()
            .filter(|r| r.runtime_id != 0)
            .map(|r| (r.runtime_id, r.recommended))
            .collect();
        match write_intensity_command(mgr_id, &batch) {
            Ok(n) => {
                for (rid, lvl) in &batch {
                    state.current_intensities.insert(*rid, *lvl);
                }
                state.status = t!(
                    format!("Opdracht voor {n} spelers verstuurd naar FM27 (BepInEx)…"),
                    format!("Sent intensity command for {n} players to FM27 (BepInEx)…"),
                    format!("{n} oyuncu için yoğunluk komutu FM27'ye (BepInEx) gönderildi…")
                );
                state.status_error = false;
            }
            Err(e) => {
                state.status = e;
                state.status_error = true;
            }
        }
    }

    if !state.status.is_empty() {
        let (c, ic) = if state.status_error {
            (theme::DANGER, icon::WARNING)
        } else {
            (theme::INFO, icon::CHECK_CIRCLE)
        };
        Frame::new()
            .fill(theme::tint(c, 0.12))
            .stroke(Stroke::new(1.0, theme::tint(c, 0.4)))
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::symmetric(12, 8))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.horizontal_wrapped(|ui| {
                    ui.label(theme::ic(ic).color(c));
                    ui.label(RichText::new(&state.status).color(theme::TEXT));
                });
            });
        ui.add_space(12.0);
    }

    // ---- BÖLÜM 1: Oyuncu Bazlı "Taktiği Kim Aşağı Çekiyor?" Raporu
    theme::card(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.label(theme::ic(icon::TARGET).color(theme::WARNING).size(18.0));
            ui.label(theme::h2(t!(
                "Wie haalt de tactiek omlaag?",
                "Who drags the tactic down?",
                "Taktiği Kim Aşağı Çekiyor? (Oyuncu Darboğaz Raporu)"
            )));
            if !state.bottlenecks.is_empty() {
                let avg = state.bottlenecks.iter().map(|b| b.overall).sum::<f32>() / state.bottlenecks.len() as f32;
                theme::pill(
                    ui,
                    &t!(
                        format!("{} spelers · gem. {avg:.0}", state.bottlenecks.len()),
                        format!("{} players · avg {avg:.0}", state.bottlenecks.len()),
                        format!("{} oyuncu · ort. {avg:.0}", state.bottlenecks.len())
                    ),
                    theme::rgb(level_color(avg)),
                );
                ui.add_space(8.0);
                ui.checkbox(
                    &mut state.filter_bottlenecks_only,
                    t!("Alleen knelpunten (<65)", "Only bottlenecks (<65)", "Sadece darboğazları göster (<65)"),
                );
            }
        });
        ui.add_space(6.0);

        if state.bottlenecks.is_empty() {
            ui.add(
                egui::Label::new(theme::dim(t!(
                    "Nog geen spelersdata uit BepInEx ontvangen. Open in FM27 eenmaal het scherm Tactical Effectiveness (met FM27Trainer.Bridge v0.6+ actief) zodat de individuele scores binnenkomen.",
                    "No per-player BepInEx data yet. Open Tactical Effectiveness once in FM27 (with FM27Trainer.Bridge v0.6+ active) to capture individual tactical scores.",
                    "Henüz oyuncu bazlı BepInEx verisi yok. FM27'de (Bridge v0.6+ yüklüyken) Taktiksel Verimlilik ekranını bir kez açtığınızda her oyuncunun taktik puanı burada en zayıftan en güçlüye sıralanır."
                )))
                .wrap(),
            );
        } else {
            let shown: Vec<&PlayerBottleneck> = state
                .bottlenecks
                .iter()
                .filter(|b| !state.filter_bottlenecks_only || b.overall < 65.0)
                .collect();

            if shown.is_empty() {
                ui.label(theme::dim(t!(
                    "Geen spelers onder 65 — de hele selectie zit op een goed tactisch niveau!",
                    "No players below 65 — the entire squad is at a solid tactical level!",
                    "65 puanın altında oyuncu yok — tüm kadro taktiğe iyi uyum sağlamış durumda!"
                )));
            } else {
                egui::Grid::new("bottleneck_grid")
                    .num_columns(7)
                    .spacing([14.0, 7.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label(theme::caps(t!("Speler", "Player", "Oyuncu")));
                        ui.label(theme::caps(t!("Totaal", "Overall", "Genel")));
                        ui.label(theme::caps(t!("Begrip", "Understanding", "Anlayış")));
                        ui.label(theme::caps(t!("Rol", "Role Fit", "Rol Uyumu")));
                        ui.label(theme::caps(t!("Kwaliteit", "Capability", "Kalite")));
                        ui.label(theme::caps(t!("Zwakste punt", "Main Drag", "Ana Darboğaz")));
                        ui.label(theme::caps(t!("Advies", "Advice", "Öneri")));
                        ui.end_row();

                        for b in shown.into_iter().take(25) {
                            ui.label(theme::label_strong(&b.name));
                            theme::pill(
                                ui,
                                &format!("{:.0} · {}", b.overall, level_name(b.overall)),
                                theme::rgb(level_color(b.overall)),
                            );
                            score_cell(ui, b.understanding);
                            score_cell(ui, b.role_suitability);
                            score_cell(ui, b.capability);
                            theme::pill(
                                ui,
                                &format!("{} ({:.0})", b.weakest_label, b.weakest_score),
                                theme::rgb(level_color(b.weakest_score)),
                            );
                            ui.add(egui::Label::new(theme::dim(b.advice)).wrap());
                            ui.end_row();
                        }
                    });
            }
        }
    });

    ui.add_space(14.0);

    // ---- BÖLÜM 2: Sakatlık Önleyici Akıllı Bireysel Antrenman Yoğunluğu
    theme::card(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.label(theme::ic(icon::HEART).color(theme::SUCCESS).size(18.0));
            ui.label(theme::h2(t!(
                "Slimme individuele trainingsintensiteit",
                "Smart Injury-Safe Individual Intensity",
                "Sakatlık Önleyici Akıllı Bireysel Yoğunluk"
            )));
            let half_n = recs.iter().filter(|r| r.recommended == IntensityLevel::Half).count();
            let norm_n = recs.iter().filter(|r| r.recommended == IntensityLevel::Normal).count();
            let dbl_n = recs.iter().filter(|r| r.recommended == IntensityLevel::Double).count();
            if !recs.is_empty() {
                theme::pill(
                    ui,
                    &t!(
                        format!("{week_matches} wedstr. deze week"),
                        format!("{week_matches} match(es) this week"),
                        format!("Bu hafta {week_matches} maç")
                    ),
                    if week_matches >= 2 { theme::DANGER } else { theme::INFO },
                );
                theme::pill(ui, &format!("{}: {half_n}", IntensityLevel::Half.short_label()), IntensityLevel::Half.color());
                theme::pill(ui, &format!("{}: {norm_n}", IntensityLevel::Normal.short_label()), IntensityLevel::Normal.color());
                theme::pill(ui, &format!("{}: {dbl_n}", IntensityLevel::Double.short_label()), IntensityLevel::Double.color());
            }
        });
        ui.add_space(6.0);

        ui.horizontal_wrapped(|ui| {
            if theme::toggle(ui, &mut state.auto_apply_intensity).changed() {
                *on_save_auto_toggle = true;
            }
            ui.label(theme::label_strong(t!(
                "Intensiteit automatisch bewaken bij conditiewijziging",
                "Automatically guard intensity as condition changes",
                "Kondisyon değiştikçe önerilen yoğunluğu otomatik uygula"
            )));
            ui.label(theme::faint(t!(
                "(100% legitiem via FM's eigen individuele trainingsmenu — geen geheugen-cheats)",
                "(100% legitimate via FM's native individual training event — zero memory cheats)",
                "(%100 hilesiz: belleğe dokunmaz, FM'nin kendi Bireysel Antrenman menüsü olayını kullanır)"
            )));
        });
        ui.add_space(8.0);

        if !live.connected || recs.is_empty() {
            ui.label(theme::dim(t!(
                "Verbind eerst met een geopende save in FM27 om conditie, scherpte en leeftijd van je selectie te lezen.",
                "Connect to an active save in FM27 first to read squad condition, sharpness and age.",
                "Kadro kondisyon, maç keskinliği ve yaş verilerini okumak için FM27'de kariyerinizi açın."
            )));
            return;
        }

        let mut single_cmd: Option<(u32, String, IntensityLevel)> = None;

        egui::Grid::new("smart_intensity_grid")
            .num_columns(7)
            .spacing([12.0, 6.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label(theme::caps(t!("Speler", "Player", "Oyuncu")));
                ui.label(theme::caps(t!("Lft", "Age", "Yaş")));
                ui.label(theme::caps(t!("Conditie", "Condition", "Kondisyon")));
                ui.label(theme::caps(t!("Scherpte", "Sharpness", "Keskinlik")));
                ui.label(theme::caps(t!("Aanbevolen", "Recommended", "Önerilen")));
                ui.label(theme::caps(t!("Snel zetten", "Quick set", "Tek Tık Seçim")));
                ui.label(theme::caps(t!("Waarom", "Reason", "Gerekçe")));
                ui.end_row();

                for r in &recs {
                    ui.horizontal(|ui| {
                        ui.label(theme::label_strong(&r.name));
                        if r.rest_match_warning {
                            theme::pill(
                                ui,
                                if r.injured {
                                    t!("BLESSURE", "INJURED", "SAKAT")
                                } else {
                                    t!("RUST", "REST", "DİNLENDİR")
                                },
                                theme::DANGER,
                            );
                        }
                    });
                    ui.label(theme::dim(r.age.map(|a| a.to_string()).unwrap_or_else(|| "—".into())));

                    let cond_col = if r.condition >= 85.0 {
                        theme::SUCCESS
                    } else if r.condition >= 72.0 {
                        theme::WARNING
                    } else {
                        theme::DANGER
                    };
                    theme::pill(ui, &format!("{:.0}%", r.condition), cond_col);

                    let shp_col = if r.sharpness >= 80.0 {
                        theme::SUCCESS
                    } else if r.sharpness >= 65.0 {
                        theme::INFO
                    } else {
                        theme::WARNING
                    };
                    theme::pill(ui, &format!("{:.0}%", r.sharpness), shp_col);

                    ui.horizontal(|ui| {
                        theme::pill(ui, r.recommended.label(), r.recommended.color());
                        if r.current == Some(r.recommended) {
                            ui.label(theme::ic(icon::CHECK_CIRCLE).color(theme::SUCCESS).size(14.0))
                                .on_hover_text(t!("Actief in FM", "Active in FM", "FM'de aktif"));
                        }
                    });

                    ui.horizontal(|ui| {
                        for lvl in [IntensityLevel::Half, IntensityLevel::Normal, IntensityLevel::Double] {
                            let active = r.current == Some(lvl);
                            let txt = if active {
                                format!("✓ {}", lvl.short_label())
                            } else {
                                lvl.short_label().to_string()
                            };
                            if theme::small_button(ui, &txt, r.runtime_id != 0).clicked() {
                                single_cmd = Some((r.runtime_id, r.name.clone(), lvl));
                            }
                        }
                    });

                    ui.add(egui::Label::new(theme::dim(&r.reason)).wrap());
                    ui.end_row();
                }
            });

        if let Some((rid, name, lvl)) = single_cmd {
            match write_intensity_command(mgr_id, &[(rid, lvl)]) {
                Ok(_) => {
                    state.current_intensities.insert(rid, lvl);
                    state.status = format!("{name} → {}", lvl.label());
                    state.status_error = false;
                }
                Err(e) => {
                    state.status = e;
                    state.status_error = true;
                }
            }
        }
    });
    let _ = vec2(0.0, 0.0);
}

fn score_cell(ui: &mut Ui, v: Option<f32>) {
    match v {
        Some(s) => {
            theme::pill(ui, &format!("{s:.0}"), theme::rgb(level_color(s)));
        }
        None => {
            ui.label(theme::faint("—"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intensity_protects_fatigued_injured_and_veterans() {
        // Sakat oyuncu her zaman Yarı Yoğunluk
        let (lvl, warn, _) = recommend_intensity(Some(22), 95.0, 90.0, true, 1);
        assert_eq!(lvl, IntensityLevel::Half);
        assert!(warn);

        // Kritik kondisyon (<70%) -> Yarı + maç uyarısı
        let (lvl, warn, _) = recommend_intensity(Some(25), 66.0, 88.0, false, 1);
        assert_eq!(lvl, IntensityLevel::Half);
        assert!(warn);

        // 34 yaş veteran + 2 maçlı hafta -> Yarı
        let (lvl, warn, _) = recommend_intensity(Some(34), 91.0, 85.0, false, 2);
        assert_eq!(lvl, IntensityLevel::Half);
        assert!(!warn);

        // 34 yaş veteran + tek maç + diri -> en fazla Normal (asla Çift değil)
        let (lvl, _, _) = recommend_intensity(Some(34), 95.0, 60.0, false, 0);
        assert_eq!(lvl, IntensityLevel::Normal);

        // 19 yaş genç + %94 kondisyon + tek maç -> Çift Yoğunluk
        let (lvl, _, _) = recommend_intensity(Some(19), 94.0, 82.0, false, 1);
        assert_eq!(lvl, IntensityLevel::Double);

        // 26 yaş as oyuncu + 2 maçlı hafta + %90 kondisyon -> Normal (Çift kapatılır)
        let (lvl, _, _) = recommend_intensity(Some(26), 90.0, 88.0, false, 2);
        assert_eq!(lvl, IntensityLevel::Normal);
    }

    #[test]
    fn parses_bepinex_players_and_sorts_weakest_first() {
        let sample = json!({
            "session_ready": true,
            "current_intensities": { "101": 2, "102": 0 },
            "data": {
                "TeamTactic:1.TacticalEffectiveness": {
                    "Players": [
                        {
                            "Person:1": { "ref": "Person", "runtime_id": 101 },
                            "OverallEffectiveness:1": { "MaxValue:1": "78" },
                            "TacticalUnderstanding:2": { "OverallEffectiveness:1": { "MaxValue:1": "80" } },
                            "PlayerAbility:3": {
                                "CapabilityKeyFactor:4": { "OverallEffectiveness:1": { "MaxValue:1": "76" } },
                                "RoleSuitabilityKeyFactor:5": { "OverallEffectiveness:1": { "MaxValue:1": "82" } }
                            }
                        },
                        {
                            "Person:1": { "ref": "Person", "runtime_id": 102 },
                            "OverallEffectiveness:1": { "MaxValue:1": "44" },
                            "TacticalUnderstanding:2": { "OverallEffectiveness:1": { "MaxValue:1": "35" } },
                            "PlayerAbility:3": {
                                "CapabilityKeyFactor:4": { "OverallEffectiveness:1": { "MaxValue:1": "68" } },
                                "RoleSuitabilityKeyFactor:5": { "OverallEffectiveness:1": { "MaxValue:1": "52" } }
                            }
                        }
                    ]
                }
            }
        });

        let mut names = HashMap::new();
        names.insert(101, "Yıldız Orta Saha".to_string());
        names.insert(102, "Yeni Transfer".to_string());

        let (players, intensities, ready) = parse_bepinex_individual(&sample, &names);
        assert!(ready);
        assert_eq!(intensities.get(&101), Some(&IntensityLevel::Double));
        assert_eq!(intensities.get(&102), Some(&IntensityLevel::Half));
        assert_eq!(players.len(), 2);
        assert_eq!(players[0].name, "Yeni Transfer");
        assert_eq!(players[0].overall, 44.0);
        assert_eq!(players[0].weakest_score, 35.0);
        assert_eq!(players[1].name, "Yıldız Orta Saha");
    }
}
