//! Bireysel oyuncu analizleri (100% hilesiz):
//! 1. Maç Öncesi Sakatlık Riski & Yorgunluk Radarı
//! 2. Taktiksel Verimlilik Darboğaz Raporu ("Taktiği kim aşağı çekiyor?") — BepInEx `Players` verisinden
//! 3. Sakatlık Önleyici Akıllı Bireysel Antrenman Yoğunluğu (Yarı / Normal / Çift) + 3 Hazır Profil
//! 4. Rol Zorlamadan Akıllı Ek Odak (Additional Focus) Önerisi & Uygulama (`EventFmxPlayerTrainingFocusChanged`)

use eframe::egui::{self, CornerRadius, Frame, Margin, RichText, Stroke, Ui, vec2};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::live::LiveState;
use crate::model::{level_color, level_name};
use crate::t;
use crate::tactics::Player as TacticPlayer;
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

/// Sakatlık / gelişim dengesini belirleyen hazır yoğunluk profilleri.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntensityProfile {
    Protective,
    #[default]
    Balanced,
    Development,
}

impl IntensityProfile {
    pub const ALL: [Self; 3] = [Self::Protective, Self::Balanced, Self::Development];

    pub fn label(self) -> &'static str {
        match self {
            Self::Protective => t!("Beschermend", "Protective", "Korumacı"),
            Self::Balanced => t!("Gebalanceerd", "Balanced", "Dengeli"),
            Self::Development => t!("Ontwikkeling", "Development", "Gelişim Odaklı"),
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Protective => t!(
                "Strengere blessurepreventie: eerder Halve intensiteit, 30+ veteranen beschermd.",
                "Stricter injury guard: Half intensity earlier, 30+ veterans protected.",
                "Sıkı sakatlık koruması: kondisyon %84 altında Yarı, 30+ yaş veteran koruması ve sadece 0 maçlı haftada Çift."
            ),
            Self::Balanced => t!(
                "Aanbevolen balans tussen ontwikkeling en frisheid.",
                "Recommended balance between development and freshness.",
                "Önerilen denge: kondisyon %80 altı Yarı, 32+ yaş en fazla Normal, diri gençlere uygun haftada Çift."
            ),
            Self::Development => t!(
                "Meer Dubbele intensiteit voor fitte en jonge spelers bij lagere rustdrempel.",
                "More Double intensity for fit & young players with lower rest threshold.",
                "Gelişim odaklı: kondisyon >=%88 genç ve ritimsiz oyunculara daha sık Çift Yoğunluk."
            ),
        }
    }

    pub fn veteran_age(self) -> u8 {
        match self {
            Self::Protective => 30,
            Self::Balanced => 32,
            Self::Development => 34,
        }
    }
}

/// Rol zorlamadan oyuncunun kendi mevkisindeki eksik özelliklere göre Ek Odak (Additional Focus) önerileri.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AdditionalFocus {
    Quickness,
    Endurance,
    Strength,
    AgilityAndBalance,
    Aerial,
    BallControl,
    Passing,
    Shooting,
    FinalThird,
    DefensivePositioning,
    GkShotStopping,
    GkHandling,
    GkDistribution,
}

impl AdditionalFocus {
    pub fn label(self) -> &'static str {
        match self {
            Self::Quickness => t!("Snelheid", "Quickness", "Çabukluk"),
            Self::Endurance => t!("Uithoudingsvermogen", "Endurance", "Dayanıklılık"),
            Self::Strength => t!("Kracht", "Strength", "Güç"),
            Self::AgilityAndBalance => t!("Wendbaarheid & Balans", "Agility and Balance", "Çeviklik ve Denge"),
            Self::Aerial => t!("Luchtduels", "Aerial", "Hava Topları"),
            Self::BallControl => t!("Balcontrole", "Ball Control", "Top Kontrolü"),
            Self::Passing => t!("Passen", "Passing", "Pas"),
            Self::Shooting => t!("Afwerken", "Shooting", "Şut"),
            Self::FinalThird => t!("Laatste fase", "Final Third", "Üçüncü Bölge"),
            Self::DefensivePositioning => t!("Verdedigende positionering", "Defensive Positioning", "Savunma Pozisyonu"),
            Self::GkShotStopping => t!("Schoten tegenhouden", "Goalkeeper Shot Stopping", "Kaleci Şut Kurtarma"),
            Self::GkHandling => t!("Klemvastheid", "Goalkeeper Handling", "Kaleci Elle Kontrol"),
            Self::GkDistribution => t!("Spelhervatting keeper", "Goalkeeper Distribution", "Kaleci Oyuna Sokma"),
        }
    }

    pub fn attr_keys(self) -> &'static [&'static str] {
        match self {
            Self::Quickness => &["acceleration", "pace"],
            Self::Endurance => &["stamina", "work_rate"],
            Self::Strength => &["strength", "balance"],
            Self::AgilityAndBalance => &["agility", "balance"],
            Self::Aerial => &["jumping_reach", "heading"],
            Self::BallControl => &["first_touch", "technique", "dribbling"],
            Self::Passing => &["passing", "vision", "decisions"],
            Self::Shooting => &["finishing", "composure", "long_shots"],
            Self::FinalThird => &["off_the_ball", "decisions", "technique"],
            Self::DefensivePositioning => &["positioning", "marking", "tackling", "concentration"],
            Self::GkShotStopping => &["reflexes", "one_on_ones"],
            Self::GkHandling => &["handling", "aerial_reach", "command_of_area"],
            Self::GkDistribution => &["kicking", "throwing", "passing"],
        }
    }

    pub fn is_physical(self) -> bool {
        matches!(
            self,
            Self::Quickness | Self::Endurance | Self::Strength | Self::AgilityAndBalance
        )
    }

    /// FM27'nin canlı `focus_options` sözlüğünden eşleşen `(option_id, name)` çiftini bulur.
    pub fn match_option_id(self, options: &HashMap<i32, String>) -> Option<(i32, String)> {
        let keywords: &[&str] = match self {
            Self::Quickness => &["quickness", "çabukluk", "snelheid", "sprint"],
            Self::Endurance => &["endurance", "dayanıklılık", "uithouding"],
            Self::Strength => &["strength", "güç", "kuvvet", "kracht"],
            Self::AgilityAndBalance => &["agility", "çeviklik", "denge", "wendbaarheid"],
            Self::Aerial => &["aerial", "hava top", "kafa", "lucht"],
            Self::BallControl => &["ball control", "top kontrol", "balcontrole"],
            Self::Passing => &["passing", "pas", "passen"],
            Self::Shooting => &["shooting", "şut", "bitiricilik", "afwerken"],
            Self::FinalThird => &["final third", "üçüncü bölge", "hücum alan", "laatste fase"],
            Self::DefensivePositioning => &["defensive position", "savunma pozisyon", "verdedig"],
            Self::GkShotStopping => &["shot stopping", "şut kurtar"],
            Self::GkHandling => &["handling", "elle kontrol", "klemvast"],
            Self::GkDistribution => &["distribution", "oyuna sok", "dagitim", "dağıtım"],
        };
        let mut best: Option<(i32, String)> = None;
        for (&id, raw_name) in options {
            let low = raw_name.to_lowercase();
            if keywords.iter().any(|kw| low.contains(kw)) {
                if self == Self::Passing && (low.contains("shot") || low.contains("gk")) {
                    continue;
                }
                if best.as_ref().is_none_or(|(bid, _)| id < *bid) {
                    best = Some((id, raw_name.clone()));
                }
            }
        }
        best
    }
}

fn attr_tr(key: &str) -> &'static str {
    match key {
        "acceleration" => "Hızlanma",
        "pace" => "Hız",
        "stamina" => "Dayanıklılık",
        "work_rate" => "Çalışkanlık",
        "strength" => "Güç",
        "balance" => "Denge",
        "agility" => "Çeviklik",
        "jumping_reach" => "Zıplama",
        "heading" => "Kafa Vuruşu",
        "first_touch" => "İlk Kontrol",
        "technique" => "Teknik",
        "dribbling" => "Dribling",
        "passing" => "Pas",
        "vision" => "Vizyon",
        "decisions" => "Karar Alma",
        "finishing" => "Bitiricilik",
        "composure" => "Soğukkanlılık",
        "long_shots" => "Uzaktan Şut",
        "off_the_ball" => "Topsuz Alan",
        "positioning" => "Pozisyon Alma",
        "marking" => "Markaj",
        "tackling" => "Top Kapma",
        "concentration" => "Konsantrasyon",
        "reflexes" => "Refleks",
        "one_on_ones" => "Birebir",
        "handling" => "Elle Kontrol",
        "aerial_reach" => "Hava Hakimiyeti",
        "command_of_area" => "Ceza Sahası",
        "kicking" => "Degaj",
        "throwing" => "Elle Başlatma",
        _ => "Özellik",
    }
}

#[derive(Clone, Debug)]
pub struct FocusRecommendation {
    pub uid: u32,
    pub runtime_id: u32,
    pub name: String,
    pub natural_pos: &'static str,
    pub focus: AdditionalFocus,
    pub weak_attrs: String,
    pub avg_score: f32,
    pub reason: String,
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

pub struct ParsedBepInEx {
    pub bottlenecks: Vec<PlayerBottleneck>,
    pub intensities: HashMap<u32, IntensityLevel>,
    pub focus_options: HashMap<i32, String>,
    pub session_ready: bool,
}

#[derive(Default)]
pub struct IndividualState {
    pub profile: IntensityProfile,
    pub bottlenecks: Vec<PlayerBottleneck>,
    pub current_intensities: HashMap<u32, IntensityLevel>,
    pub focus_options: HashMap<i32, String>,
    pub applied_focuses: HashMap<u32, AdditionalFocus>,
    pub session_ready: bool,
    pub auto_apply_intensity: bool,
    pub filter_bottlenecks_only: bool,
    pub last_cmd_result_utc: String,
    pub last_auto_apply: Option<std::time::Instant>,
    pub status: String,
    pub status_error: bool,
    tactics_requested: bool,
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
        .find(|(k, _)| {
            k.eq_ignore_ascii_case(prefix)
                || (k.len() > with_colon.len() && k[..with_colon.len()].eq_ignore_ascii_case(&with_colon))
        })
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
    let _ = node.as_object()?;
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
    None
}

/// BepInEx JSON kökünden oyuncu darboğaz listesini, yoğunlukları ve ek odak seçeneklerini çözümler.
pub fn parse_bepinex_individual(root: &Value, squad_names: &HashMap<u32, String>) -> ParsedBepInEx {
    let session_ready = root.get("session_ready").and_then(|v| v.as_bool()).unwrap_or(false);
    let mut intensities = HashMap::new();
    if let Some(ci) = root.get("current_intensities").and_then(|v| v.as_object()) {
        for (k, v) in ci {
            if let (Ok(rid), Some(lvl)) = (k.parse::<u32>(), v.as_i64().and_then(IntensityLevel::from_code)) {
                intensities.insert(rid, lvl);
            }
        }
    }

    let mut focus_options = HashMap::new();
    if let Some(fo) = root.get("focus_options").and_then(|v| v.as_object()) {
        for (k, v) in fo {
            if let (Ok(id), Some(name)) = (k.parse::<i32>(), v.as_str()) {
                if !name.trim().is_empty() {
                    focus_options.insert(id, name.trim().to_string());
                }
            }
        }
    }

    let mut out = Vec::new();
    let Some(data) = root.get("data").and_then(|v| v.as_object()) else {
        return ParsedBepInEx { bottlenecks: out, intensities, focus_options, session_ready };
    };
    let Some(te) = data
        .iter()
        .find(|(k, _)| k.starts_with("TeamTactic:") && k.ends_with(".TacticalEffectiveness"))
        .or_else(|| data.iter().find(|(k, _)| k.ends_with(".TacticalEffectiveness")))
        .map(|(_, v)| v)
    else {
        return ParsedBepInEx { bottlenecks: out, intensities, focus_options, session_ready };
    };

    let Some(players) = find_prefix(te, "Players").and_then(|v| v.as_array()) else {
        return ParsedBepInEx { bottlenecks: out, intensities, focus_options, session_ready };
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

        let eff_node = find_prefix(item, "TacticalEffectiveness").unwrap_or(item);

        let understanding = find_prefix(eff_node, "TacticalUnderstanding")
            .or_else(|| find_prefix(eff_node, "PlayerTacticalUnderstanding"))
            .or_else(|| find_prefix(eff_node, "Comprehension"))
            .or_else(|| find_prefix(item, "TacticalUnderstanding"))
            .and_then(score_of);

        let pa_node = find_prefix(eff_node, "PlayerAbility").or_else(|| find_prefix(item, "PlayerAbility"));
        let capability = find_prefix(eff_node, "CapabilityKeyFactor")
            .or_else(|| find_prefix(eff_node, "CapabilityScore"))
            .or_else(|| find_prefix(eff_node, "Capability"))
            .or_else(|| pa_node.and_then(|pa| find_prefix(pa, "CapabilityKeyFactor").or_else(|| find_prefix(pa, "CapabilityScore"))))
            .or_else(|| pa_node)
            .and_then(score_of);

        let role_suitability = find_prefix(eff_node, "RoleSuitabilityKeyFactor")
            .or_else(|| find_prefix(eff_node, "RoleSuitabilityScore"))
            .or_else(|| find_prefix(eff_node, "RoleSuitability"))
            .or_else(|| pa_node.and_then(|pa| find_prefix(pa, "RoleSuitabilityKeyFactor").or_else(|| find_prefix(pa, "RoleSuitabilityScore"))))
            .and_then(score_of);

        let buy_in = find_prefix(eff_node, "PlayerBuyIn")
            .or_else(|| find_prefix(eff_node, "BuyIn"))
            .or_else(|| find_prefix(item, "PlayerBuyIn"))
            .and_then(score_of);

        let overall = find_prefix(eff_node, "OverallEffectiveness")
            .or_else(|| find_prefix(item, "OverallEffectiveness"))
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
    ParsedBepInEx { bottlenecks: out, intensities, focus_options, session_ready }
}

/// Sakatlık önleyici akıllı bireysel yoğunluk algoritması (%100 meşru FM yoğunluk seviyeleri + profil desteği).
pub fn recommend_intensity(
    age: Option<u8>,
    condition_pct: f32,
    sharpness_pct: f32,
    injured: bool,
    week_matches: usize,
    profile: IntensityProfile,
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

    let (crit_cond, half_cond, vet_cond, double_cond, double_young_age) = match profile {
        IntensityProfile::Protective => (74.0, 84.0, 91.0, 95.0, 21),
        IntensityProfile::Balanced => (70.0, 80.0, 88.0, 92.0, 23),
        IntensityProfile::Development => (66.0, 75.0, 84.0, 88.0, 25),
    };
    let vet_age = profile.veteran_age();

    if condition_pct < crit_cond {
        return (
            IntensityLevel::Half,
            true,
            t!(
                format!("Conditie {condition_pct:.0}% (<{crit_cond:.0}%) → hoog blessurerisico, geef rust in wedstrijd."),
                format!("Condition {condition_pct:.0}% (<{crit_cond:.0}%) → high injury risk, rest on matchday."),
                format!("Kondisyon %{condition_pct:.0} (<%{crit_cond:.0}) → sakatlık riski yüksek, maçta dinlendirin.")
            ),
        );
    }

    if condition_pct < half_cond {
        return (
            IntensityLevel::Half,
            false,
            t!(
                format!("Conditie {condition_pct:.0}% (<{half_cond:.0}%) → herstel krijgt voorrang."),
                format!("Condition {condition_pct:.0}% (<{half_cond:.0}%) → recovery priority."),
                format!("Kondisyon %{condition_pct:.0} (<%{half_cond:.0}) → toparlanma öncelikli.")
            ),
        );
    }

    if age.is_some_and(|a| a >= vet_age) {
        let a = age.unwrap_or(vet_age);
        if week_matches >= 2 || condition_pct < vet_cond {
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
        let congested_half = match profile {
            IntensityProfile::Protective => 89.0,
            IntensityProfile::Balanced => 85.0,
            IntensityProfile::Development => 80.0,
        };
        if condition_pct < congested_half {
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
        if profile != IntensityProfile::Protective
            && condition_pct >= 93.0
            && sharpness_pct < 72.0
            && age.is_none_or(|a| a < 30)
        {
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
    let allow_double_1_match = profile != IntensityProfile::Protective || week_matches == 0;
    let young = age.is_some_and(|a| a <= double_young_age);
    let under_vet = age.is_none_or(|a| a < vet_age);
    if allow_double_1_match
        && condition_pct >= double_cond
        && under_vet
        && (young || sharpness_pct < 80.0 || week_matches == 0 || profile == IntensityProfile::Development)
    {
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

/// Oyuncunun kendi doğal mevkisindeki kilit eksik özelliklere göre Ek Odak (Additional Focus) önerir.
/// Hiçbir oyuncuyu taktikteki uygun olmayan bir role zorlamaz.
pub fn recommend_additional_focus(
    player: &TacticPlayer,
    runtime_id: u32,
    age: Option<u8>,
    injured: bool,
) -> Option<FocusRecommendation> {
    if player.positions.len() < 15 || player.attributes.is_empty() {
        return None;
    }

    // Doğal mevkiyi bul: 0=GK, 1/3=DC, 2/4/13/14=FB/WB, 5/6/7/8=MC/DM, 9/10/11=AM/W, 12=ST
    let (best_idx, _) = player
        .positions
        .iter()
        .enumerate()
        .max_by_key(|&(_, v)| *v)?;

    let (natural_pos, candidates): (&'static str, &'static [AdditionalFocus]) = match best_idx {
        0 => (
            "KL",
            &[
                AdditionalFocus::GkShotStopping,
                AdditionalFocus::GkHandling,
                AdditionalFocus::GkDistribution,
            ],
        ),
        1 | 3 => (
            "STP",
            &[
                AdditionalFocus::DefensivePositioning,
                AdditionalFocus::Aerial,
                AdditionalFocus::Strength,
                AdditionalFocus::Quickness,
                AdditionalFocus::Passing,
            ],
        ),
        2 | 4 | 13 | 14 => (
            "BEK",
            &[
                AdditionalFocus::Endurance,
                AdditionalFocus::Quickness,
                AdditionalFocus::DefensivePositioning,
                AdditionalFocus::Passing,
                AdditionalFocus::AgilityAndBalance,
            ],
        ),
        5..=8 => (
            "OS",
            &[
                AdditionalFocus::Passing,
                AdditionalFocus::BallControl,
                AdditionalFocus::Endurance,
                AdditionalFocus::DefensivePositioning,
                AdditionalFocus::FinalThird,
                AdditionalFocus::Quickness,
            ],
        ),
        9..=11 => (
            "KNT/OOS",
            &[
                AdditionalFocus::FinalThird,
                AdditionalFocus::Quickness,
                AdditionalFocus::BallControl,
                AdditionalFocus::Passing,
                AdditionalFocus::Shooting,
                AdditionalFocus::AgilityAndBalance,
            ],
        ),
        _ => (
            "SNT",
            &[
                AdditionalFocus::Shooting,
                AdditionalFocus::FinalThird,
                AdditionalFocus::Quickness,
                AdditionalFocus::BallControl,
                AdditionalFocus::Aerial,
                AdditionalFocus::Strength,
            ],
        ),
    };

    let avoid_physical = injured || age.is_some_and(|a| a >= 32);

    let mut scored: Vec<(AdditionalFocus, f32, String)> = Vec::new();
    for &focus in candidates {
        if avoid_physical && focus.is_physical() {
            continue;
        }
        let keys = focus.attr_keys();
        let mut pairs: Vec<(&str, u8)> = keys
            .iter()
            .filter_map(|k| player.attributes.get(*k).map(|v| (*k, *v)))
            .collect();
        if pairs.is_empty() {
            continue;
        }
        let avg = pairs.iter().map(|(_, v)| *v as f32).sum::<f32>() / pairs.len() as f32;
        pairs.sort_by_key(|(_, v)| *v);
        let weak_attrs = pairs
            .iter()
            .take(2)
            .map(|(k, v)| format!("{} {v}", attr_tr(k)))
            .collect::<Vec<_>>()
            .join(", ");
        scored.push((focus, avg, weak_attrs));
    }

    if scored.is_empty() {
        // Fallback eğer tüm adaylar fiziksel olsaydı
        for &focus in candidates {
            let keys = focus.attr_keys();
            let mut pairs: Vec<(&str, u8)> = keys
                .iter()
                .filter_map(|k| player.attributes.get(*k).map(|v| (*k, *v)))
                .collect();
            if pairs.is_empty() {
                continue;
            }
            let avg = pairs.iter().map(|(_, v)| *v as f32).sum::<f32>() / pairs.len() as f32;
            pairs.sort_by_key(|(_, v)| *v);
            let weak_attrs = pairs
                .iter()
                .take(2)
                .map(|(k, v)| format!("{} {v}", attr_tr(k)))
                .collect::<Vec<_>>()
                .join(", ");
            scored.push((focus, avg, weak_attrs));
        }
    }

    scored.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
    let (focus, avg_score, weak_attrs) = scored.into_iter().next()?;

    let reason = if injured {
        t!(
            format!("Geblesseerd → lichte technische/tactische focus ({weak_attrs})."),
            format!("Injured → low-strain technical/tactical focus ({weak_attrs})."),
            format!("Sakatlık dönüşü → fiziksel yük bindirmeyen teknik/pozisyonel odak ({weak_attrs}).")
        )
    } else if age.is_some_and(|a| a >= 32) {
        t!(
            format!("32+ → spierbesparende focus op {weak_attrs}."),
            format!("32+ → low-strain focus on {weak_attrs}."),
            format!("Veteran → kas yormadan {weak_attrs} desteği.")
        )
    } else {
        t!(
            format!("Laagste kernpunten op {natural_pos}: {weak_attrs} (gem. {avg_score:.1})."),
            format!("Lowest core attributes for {natural_pos}: {weak_attrs} (avg {avg_score:.1})."),
            format!("Doğal {natural_pos} mevkisindeki en zayıf kilit grup: {weak_attrs} (ort. {avg_score:.1}).")
        )
    };

    Some(FocusRecommendation {
        uid: player.uid,
        runtime_id,
        name: player.name.clone(),
        natural_pos,
        focus,
        weak_attrs,
        avg_score,
        reason,
    })
}

pub fn build_squad_recommendations(
    live: &LiveState,
    current_map: &HashMap<u32, IntensityLevel>,
    profile: IntensityProfile,
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
            recommend_intensity(age, condition, sharpness, injured, week_matches, profile);
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

    rows.sort_by(|a, b| {
        b.rest_match_warning
            .cmp(&a.rest_match_warning)
            .then_with(|| a.condition.partial_cmp(&b.condition).unwrap_or(std::cmp::Ordering::Equal))
    });
    (rows, week_matches)
}

pub fn build_focus_recommendations(live: &LiveState) -> Vec<FocusRecommendation> {
    let Some(td) = &live.tactics else { return Vec::new() };
    let mut squad_meta: HashMap<u32, (u32, Option<u8>, bool)> = HashMap::new();
    for p in &live.squad {
        if let Some(uid) = p.get("uid").and_then(|v| v.as_u64()) {
            let rid = p.get("runtime_id").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
            let age = p.get("age").and_then(|v| v.as_u64()).map(|a| a as u8);
            let injured = p.get("injuries").and_then(|v| v.as_array()).is_some_and(|a| !a.is_empty());
            squad_meta.insert(uid as u32, (rid, age, injured));
        }
    }

    let mut out = Vec::new();
    for p in &td.players {
        let (rid, age, injured) = squad_meta.get(&p.uid).copied().unwrap_or((0, None, p.injured));
        if let Some(rec) = recommend_additional_focus(p, rid, age, injured || p.injured) {
            out.push(rec);
        }
    }
    out.sort_by(|a, b| a.avg_score.partial_cmp(&b.avg_score).unwrap_or(std::cmp::Ordering::Equal));
    out
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
    write_cmd_payload(json!({
        "manager_runtime_id": manager_runtime_id,
        "items": valid,
    }))?;
    Ok(count)
}

pub fn write_focus_command(manager_runtime_id: u32, items: &[(u32, i32, String)]) -> Result<usize, String> {
    let valid: Vec<Value> = items
        .iter()
        .filter(|(rid, opt_id, _)| *rid != 0 && *opt_id >= 0)
        .map(|(rid, opt_id, name)| json!({ "runtime_id": rid, "option_id": opt_id, "name": name }))
        .collect();
    if valid.is_empty() {
        return Err(t!(
            "Open eerst het Individuele Trainingsmenu in FM zodat focusopties gelezen worden.",
            "Open Individual Training once in FM so focus option IDs are captured.",
            "Ek Odak seçenek kimliklerinin alınması için FM27'de Bireysel Antrenman ekranını bir kez açın."
        )
        .to_string());
    }
    let count = valid.len();
    write_cmd_payload(json!({
        "manager_runtime_id": manager_runtime_id,
        "focus_items": valid,
    }))?;
    Ok(count)
}

fn write_cmd_payload(payload: Value) -> Result<(), String> {
    let _ = std::fs::create_dir_all(crate::data_dir());
    let path = cmd_file();
    let tmp = path.with_extension("json.tmp");
    let serialized = serde_json::to_string_pretty(&payload).map_err(|e| e.to_string())?;
    std::fs::write(&tmp, serialized).map_err(|e| e.to_string())?;
    std::fs::rename(&tmp, &path).map_err(|e| e.to_string())?;
    Ok(())
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
                format!("Individuele training bijgewerkt voor {applied} spelers in FM27."),
                format!("Updated individual training for {applied} players in FM27."),
                format!("{applied} işlem FM27'de başarıyla uygulandı.")
            );
            self.status_error = false;
        } else {
            self.status = format!(
                "{} ({applied} ok, {} err: {})",
                t!("Deels toegepast", "Partially applied", "Kısmen uygulandı"),
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
        let (rows, _) = build_squad_recommendations(live, &self.current_intensities, self.profile);
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

/// Bireysel Analiz, Radar, Ek Odak & Yoğunluk sayfası.
pub fn render_page(
    ui: &mut Ui,
    ctx: &egui::Context,
    state: &mut IndividualState,
    live: &mut LiveState,
    on_read_bepinex: &mut bool,
    on_save_prefs: &mut bool,
) {
    if live.connected && live.tactics.is_none() && !live.tactics_busy() && !state.tactics_requested {
        state.tactics_requested = true;
        live.load_tactics(ctx);
    }
    if !live.connected {
        state.tactics_requested = false;
    }

    let (recs, week_matches) = build_squad_recommendations(live, &state.current_intensities, state.profile);
    let focus_recs = build_focus_recommendations(live);
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
            "Pre-match blessureradar, knelpunt-rapport, slimme intensiteit en rolonafhankelijke extra focus.",
            "Pre-match injury radar, bottleneck report, smart intensity and role-free Additional Focus advisor.",
            "Maç öncesi risk radarı, taktiği aşağı çeken oyuncular, profil destekli yoğunluk ve rol zorlamadan Ek Odak."
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
        state.tactics_requested = false;
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
            .stroke(Stroke::new(1.0_f32, theme::tint(c, 0.4)))
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

    // ---- MAÇ ÖNCESİ YORGUNLUK & SAKATLIK RİSKİ RADARI
    if live.connected && !recs.is_empty() {
        let crit_cnt = recs.iter().filter(|r| r.rest_match_warning).count();
        let vet_age = state.profile.veteran_age();
        let vet_cnt = recs.iter().filter(|r| r.age.is_some_and(|a| a >= vet_age)).count();
        let healthy: Vec<&IntensityRecommendation> = recs.iter().filter(|r| !r.injured).collect();
        let avg_cond = if healthy.is_empty() {
            0.0
        } else {
            healthy.iter().map(|r| r.condition).sum::<f32>() / healthy.len() as f32
        };
        let avg_shp = if healthy.is_empty() {
            0.0
        } else {
            healthy.iter().map(|r| r.sharpness).sum::<f32>() / healthy.len() as f32
        };

        ui.columns(4, |cols| {
            let fix_col = if week_matches >= 2 { theme::DANGER } else { theme::INFO };
            theme::stat_tile(
                &mut cols[0],
                icon::CALENDAR_DOTS,
                t!("Weekprogramma", "Weekly Fixture", "Haftalık Fikstür"),
                &format!("{week_matches}"),
                if week_matches >= 2 {
                    t!("Drukke week", "Congested week", "Sıkışık hafta")
                } else if week_matches == 0 {
                    t!("Vrije week", "Quiet week", "Maçsız hafta")
                } else {
                    t!("Normale week", "Regular week", "Tek maçlı hafta")
                },
                fix_col,
                None,
            );

            let risk_col = if crit_cnt == 0 {
                theme::SUCCESS
            } else if crit_cnt <= 2 {
                theme::WARNING
            } else {
                theme::DANGER
            };
            theme::stat_tile(
                &mut cols[1],
                icon::WARNING,
                t!("Blessure & Rust", "Injury & Critical Risk", "Kritik Risk & Sakat"),
                &format!("{crit_cnt}"),
                if crit_cnt == 0 {
                    t!("Geen kritieke spelers", "Squad looks fresh", "Kritik oyuncu yok")
                } else {
                    t!("Spelers sparen", "Rotate / rest needed", "Dinlendirme önerilir")
                },
                risk_col,
                None,
            );

            theme::stat_tile(
                &mut cols[2],
                icon::USERS_THREE,
                t!("Veteranbescherming", "Veteran Guard", "Veteran Koruması"),
                &format!("{vet_cnt}"),
                &t!(
                    format!("{vet_age}+ jaar beschermd"),
                    format!("{vet_age}+ yrs guarded"),
                    format!("{vet_age}+ yaş korumada")
                ),
                theme::ACCENT_HI,
                None,
            );

            let ready_col = if avg_cond >= 84.0 {
                theme::SUCCESS
            } else if avg_cond >= 76.0 {
                theme::WARNING
            } else {
                theme::DANGER
            };
            theme::stat_tile(
                &mut cols[3],
                icon::HEART,
                t!("Selectiegereedheid", "Squad Readiness", "Kadro Hazırlığı"),
                &format!("{avg_cond:.0}%"),
                &t!(
                    format!("Scherpte {avg_shp:.0}%"),
                    format!("Sharpness {avg_shp:.0}%"),
                    format!("Keskinlik %{avg_shp:.0}")
                ),
                ready_col,
                Some(avg_cond),
            );
        });
        ui.add_space(14.0);
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
                    "Henüz oyuncu bazlı BepInEx verisi yok. FM27'de eklenti yüklüyken Taktiksel Verimlilik ekranını bir kez açtığınızda her oyuncunun taktik puanı burada en zayıftan en güçlüye sıralanır."
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

    // ---- BÖLÜM 2: Sakatlık Önleyici Akıllı Bireysel Antrenman Yoğunluğu + Hazır Profiller
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
            ui.label(theme::label_strong(t!("Intensiteitsprofiel:", "Intensity profile:", "Yoğunluk profili:")));
            let prof_opts: [(IntensityProfile, &str); 3] = [
                (IntensityProfile::Protective, IntensityProfile::Protective.label()),
                (IntensityProfile::Balanced, IntensityProfile::Balanced.label()),
                (IntensityProfile::Development, IntensityProfile::Development.label()),
            ];
            if theme::segmented(ui, &mut state.profile, &prof_opts) {
                *on_save_prefs = true;
            }
            ui.label(theme::faint(state.profile.description()));
        });
        ui.add_space(4.0);

        ui.horizontal_wrapped(|ui| {
            if theme::toggle(ui, &mut state.auto_apply_intensity).changed() {
                *on_save_prefs = true;
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
        } else {
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
        }
    });

    ui.add_space(14.0);

    // ---- BÖLÜM 3: Rol Zorlamadan Akıllı Ek Odak (Additional Focus) Önerisi
    theme::card(ui, |ui| {
        let mut apply_all_focus = false;
        let can_apply_focus = live.connected && !focus_recs.is_empty() && !state.focus_options.is_empty();

        ui.horizontal_wrapped(|ui| {
            ui.label(theme::ic(icon::SPARKLE).color(theme::ACCENT_HI).size(18.0));
            ui.label(theme::h2(t!(
                "Slimme Extra Focus (zonder rol te forceren)",
                "Smart Additional Focus (role-free)",
                "Rol Zorlamadan Akıllı Ek Odak (Additional Focus)"
            )));
            if !state.focus_options.is_empty() {
                theme::pill(
                    ui,
                    &t!(
                        format!("{} opties uit FM gelezen", state.focus_options.len()),
                        format!("{} options captured", state.focus_options.len()),
                        format!("{} FM seçeneği hazır", state.focus_options.len())
                    ),
                    theme::SUCCESS,
                );
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if theme::small_button(
                    ui,
                    t!(
                        "Pas alle extra focus toe",
                        "Apply all Additional Focus",
                        "Önerilen Ek Odakları FM'ye Uygula"
                    ),
                    can_apply_focus,
                )
                .clicked()
                {
                    apply_all_focus = true;
                }
            });
        });
        ui.add_space(4.0);
        ui.add(
            egui::Label::new(theme::dim(t!(
                "Gebruikt uitsluitend de natuurlijke positie van elke speler en zoekt de zwakste kernattributen op — geen enkele speler wordt in een ongeschikte tactische rol geduwd.",
                "Looks strictly at each player's own natural position and weakest core attributes — never forces backups into unsuitable tactic roles.",
                "Her oyuncunun sadece kendi doğal mevkisine göre en eksik kilit özellik grubunu bulur — yedekler uygun olmayan taktik rollerine asla zorlanmaz."
            )))
            .wrap(),
        );

        if state.focus_options.is_empty() && !focus_recs.is_empty() {
            ui.add_space(4.0);
            ui.label(theme::faint(t!(
                "Tip: open in FM27 eenmaal het scherm Individuele Training om de directe 'Toepassen'-knoppen te activeren.",
                "Tip: open Individual Training once in FM27 to capture live focus IDs and enable one-click apply.",
                "İpucu: Tek tıkla uygulama butonlarının aktifleşmesi için FM27'de Bireysel Antrenman sekmesini bir kez açın (öneriler aşağıda listelenir)."
            )));
        }
        ui.add_space(8.0);

        if focus_recs.is_empty() {
            ui.label(theme::dim(if live.tactics_busy() {
                t!("Spelersattributen ophalen…", "Loading squad attributes…", "Oyuncu özellikleri yükleniyor…")
            } else {
                t!(
                    "Verbind met FM27 om de attributen per speler uit te lezen.",
                    "Connect to FM27 to read each player's attributes.",
                    "Oyuncuların özellik profillerini görmek için FM27'ye bağlanın."
                )
            }));
            return;
        }

        if apply_all_focus {
            let batch: Vec<(u32, i32, String)> = focus_recs
                .iter()
                .filter_map(|r| {
                    if r.runtime_id == 0 {
                        return None;
                    }
                    let (opt_id, opt_name) = r.focus.match_option_id(&state.focus_options)?;
                    Some((r.runtime_id, opt_id, opt_name))
                })
                .collect();
            match write_focus_command(mgr_id, &batch) {
                Ok(n) => {
                    for r in &focus_recs {
                        if r.runtime_id != 0 && r.focus.match_option_id(&state.focus_options).is_some() {
                            state.applied_focuses.insert(r.uid, r.focus);
                        }
                    }
                    state.status = t!(
                        format!("Extra focus voor {n} spelers verstuurd naar FM27…"),
                        format!("Sent Additional Focus command for {n} players to FM27…"),
                        format!("{n} oyuncu için Ek Odak komutu FM27'ye gönderildi…")
                    );
                    state.status_error = false;
                }
                Err(e) => {
                    state.status = e;
                    state.status_error = true;
                }
            }
        }

        let mut single_focus: Option<(u32, u32, String, AdditionalFocus, i32, String)> = None;

        egui::Grid::new("smart_focus_grid")
            .num_columns(6)
            .spacing([12.0, 6.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label(theme::caps(t!("Speler", "Player", "Oyuncu")));
                ui.label(theme::caps(t!("Pos", "Pos", "Mevki")));
                ui.label(theme::caps(t!("Aanbevolen Focus", "Suggested Focus", "Önerilen Ek Odak")));
                ui.label(theme::caps(t!("Zwakste attributen", "Weakest Attributes", "Eksik Özellikler")));
                ui.label(theme::caps(t!("Actie", "Apply", "Uygula")));
                ui.label(theme::caps(t!("Uitleg", "Reason", "Açıklama")));
                ui.end_row();

                for r in focus_recs.iter().take(28) {
                    ui.label(theme::label_strong(&r.name));
                    theme::pill(ui, r.natural_pos, theme::ACCENT);
                    theme::pill(ui, r.focus.label(), theme::INFO);
                    ui.label(theme::dim(&r.weak_attrs));

                    let matched = if r.runtime_id != 0 {
                        r.focus.match_option_id(&state.focus_options)
                    } else {
                        None
                    };
                    let applied = state.applied_focuses.get(&r.uid) == Some(&r.focus);
                    let btn_txt = if applied {
                        t!("✓ Gezet", "✓ Set", "✓ Seçildi")
                    } else {
                        t!("Toepassen", "Apply", "Uygula")
                    };
                    if theme::small_button(ui, btn_txt, matched.is_some()).clicked() {
                        if let Some((opt_id, opt_name)) = matched {
                            single_focus = Some((r.uid, r.runtime_id, r.name.clone(), r.focus, opt_id, opt_name));
                        }
                    }

                    ui.add(egui::Label::new(theme::dim(&r.reason)).wrap());
                    ui.end_row();
                }
            });

        if let Some((uid, rid, name, focus, opt_id, opt_name)) = single_focus {
            match write_focus_command(mgr_id, &[(rid, opt_id, opt_name)]) {
                Ok(_) => {
                    state.applied_focuses.insert(uid, focus);
                    state.status = format!("{name} → {}", focus.label());
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

    fn mk_player(uid: u32, pos_idx: usize, attrs: &[(&str, u8)]) -> TacticPlayer {
        let mut positions = vec![1; 15];
        positions[pos_idx] = 20;
        let attributes: HashMap<String, u8> = attrs.iter().map(|(k, v)| (k.to_string(), *v)).collect();
        TacticPlayer {
            uid,
            name: format!("Player {uid}"),
            positions,
            attributes,
            left_foot: 10,
            right_foot: 15,
            injured: false,
            banned: false,
        }
    }

    #[test]
    fn intensity_profiles_differ_safely() {
        // Sakat oyuncu tüm profillerde Yarı + uyarı
        for p in IntensityProfile::ALL {
            let (lvl, warn, _) = recommend_intensity(Some(22), 95.0, 90.0, true, 1, p);
            assert_eq!(lvl, IntensityLevel::Half);
            assert!(warn);
        }

        // 30 yaş + %89 kondisyon: Korumacıda veteran & %91 altı -> Yarı; Dengelide -> Normal
        let (lvl_prot, _, _) = recommend_intensity(Some(30), 89.0, 85.0, false, 1, IntensityProfile::Protective);
        let (lvl_bal, _, _) = recommend_intensity(Some(30), 89.0, 85.0, false, 1, IntensityProfile::Balanced);
        assert_eq!(lvl_prot, IntensityLevel::Half);
        assert_eq!(lvl_bal, IntensityLevel::Normal);

        // 24 yaş genç + %90 kondisyon + 1 maç: Gelişim Odaklıda Çift; Dengelide Normal
        let (lvl_dev, _, _) = recommend_intensity(Some(24), 90.0, 85.0, false, 1, IntensityProfile::Development);
        let (lvl_bal2, _, _) = recommend_intensity(Some(24), 90.0, 85.0, false, 1, IntensityProfile::Balanced);
        assert_eq!(lvl_dev, IntensityLevel::Double);
        assert_eq!(lvl_bal2, IntensityLevel::Normal);
    }

    #[test]
    fn additional_focus_picks_position_weakness_and_protects_injured() {
        // Kaleci: degaj & pas düşük -> Kaleci Oyuna Sokma
        let gk = mk_player(
            1,
            0,
            &[
                ("reflexes", 17), ("one_on_ones", 16),
                ("handling", 15), ("aerial_reach", 15), ("command_of_area", 14),
                ("kicking", 8), ("throwing", 9), ("passing", 7),
            ],
        );
        let rec_gk = recommend_additional_focus(&gk, 1, Some(24), false).unwrap();
        assert_eq!(rec_gk.natural_pos, "KL");
        assert_eq!(rec_gk.focus, AdditionalFocus::GkDistribution);

        // Stoper fiziksel olarak zayıf ama sakat -> fiziksel odak yerine Savunma Pozisyonu / Hava / Pas seçer
        let dc_injured = mk_player(
            2,
            3,
            &[
                ("acceleration", 6), ("pace", 6), ("strength", 7), ("balance", 7),
                ("positioning", 11), ("marking", 11), ("tackling", 12), ("concentration", 10),
                ("jumping_reach", 15), ("heading", 15), ("passing", 13), ("vision", 13), ("decisions", 13),
            ],
        );
        let rec_dc = recommend_additional_focus(&dc_injured, 2, Some(27), true).unwrap();
        assert_eq!(rec_dc.focus, AdditionalFocus::DefensivePositioning);
        assert!(!rec_dc.focus.is_physical());

        // Sağlıklı genç forvet: bitiricilik düşük -> Şut
        let st = mk_player(
            3,
            12,
            &[
                ("finishing", 9), ("composure", 9), ("long_shots", 8),
                ("acceleration", 16), ("pace", 16),
                ("off_the_ball", 15), ("decisions", 14), ("technique", 14),
                ("first_touch", 14), ("dribbling", 14),
                ("jumping_reach", 13), ("heading", 13),
                ("strength", 13), ("balance", 13),
            ],
        );
        let rec_st = recommend_additional_focus(&st, 3, Some(20), false).unwrap();
        assert_eq!(rec_st.natural_pos, "SNT");
        assert_eq!(rec_st.focus, AdditionalFocus::Shooting);
    }

    #[test]
    fn focus_option_id_matches_turkish_and_english_fm_names() {
        let mut opts = HashMap::new();
        opts.insert(1, "Çabukluk".to_string());
        opts.insert(2, "Endurance".to_string());
        opts.insert(3, "Savunma Pozisyonu".to_string());
        opts.insert(4, "Ball Control".to_string());

        assert_eq!(AdditionalFocus::Quickness.match_option_id(&opts).map(|x| x.0), Some(1));
        assert_eq!(AdditionalFocus::Endurance.match_option_id(&opts).map(|x| x.0), Some(2));
        assert_eq!(AdditionalFocus::DefensivePositioning.match_option_id(&opts).map(|x| x.0), Some(3));
        assert_eq!(AdditionalFocus::BallControl.match_option_id(&opts).map(|x| x.0), Some(4));
    }

    #[test]
    fn parses_bepinex_players_and_focus_options() {
        let sample = json!({
            "session_ready": true,
            "current_intensities": { "101": 2, "102": 0 },
            "focus_options": { "3": "Çabukluk", "7": "Pas" },
            "data": {
                "TeamTactic:1.TacticalEffectiveness": {
                    "Players": [
                        {
                            "Person:1": { "ref": "Person", "runtime_id": 101 },
                            "OverallEffectiveness:1": { "MaxValue:1": "78" },
                            "TacticalUnderstanding:2": { "OverallEffectiveness:1": { "MaxValue:1": "80" } }
                        },
                        {
                            "Person:1": { "ref": "Person", "runtime_id": 102 },
                            "OverallEffectiveness:1": { "MaxValue:1": "44" },
                            "TacticalUnderstanding:2": { "OverallEffectiveness:1": { "MaxValue:1": "35" } }
                        }
                    ]
                }
            }
        });

        let mut names = HashMap::new();
        names.insert(101, "Yıldız Orta Saha".to_string());
        names.insert(102, "Yeni Transfer".to_string());

        let parsed = parse_bepinex_individual(&sample, &names);
        assert!(parsed.session_ready);
        assert_eq!(parsed.focus_options.get(&3).map(String::as_str), Some("Çabukluk"));
        assert_eq!(parsed.intensities.get(&101), Some(&IntensityLevel::Double));
        assert_eq!(parsed.bottlenecks.len(), 2);
        assert_eq!(parsed.bottlenecks[0].name, "Yeni Transfer");
    }

    #[test]
    fn parses_nested_player_tactical_effectiveness_records() {
        let sample = json!({
            "session_ready": true,
            "data": {
                "TeamTactic:1.TacticalEffectiveness": {
                    "Players": [
                        {
                            "Player:1": { "ref": "Person", "runtime_id": 201 },
                            "TacticalEffectiveness:123": {
                                "OverallEffectiveness:1": { "MaxValue:1": "54" },
                                "TacticalUnderstanding:2": { "OverallEffectiveness:1": { "MaxValue:1": "48" } },
                                "PlayerAbility:3": {
                                    "CapabilityScore:1": { "MaxValue:1": "62" },
                                    "RoleSuitabilityScore:1": { "MaxValue:1": "70" }
                                },
                                "PlayerBuyIn:4": { "OverallEffectiveness:1": { "MaxValue:1": "66" } }
                            }
                        }
                    ]
                }
            }
        });

        let mut names = HashMap::new();
        names.insert(201, "Kadro Bekçisi".to_string());

        let parsed = parse_bepinex_individual(&sample, &names);
        assert_eq!(parsed.bottlenecks.len(), 1);
        assert_eq!(parsed.bottlenecks[0].name, "Kadro Bekçisi");
        assert!((parsed.bottlenecks[0].overall - 54.0).abs() < 0.01);
        assert_eq!(parsed.bottlenecks[0].weakest_score, 48.0);
    }
}
