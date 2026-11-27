//! Bireysel oyuncu analizleri (100% hilesiz):
//! 1. Maç Öncesi Sakatlık Riski & Yorgunluk Radarı
//! 2. Taktiksel Verimlilik Darboğaz Raporu ("Taktiği kim aşağı çekiyor?") — BepInEx `Players` verisinden
//! 3. Sakatlık Önleyici Akıllı Bireysel Antrenman Yoğunluğu (Yarı / Normal / Çift) + 3 Hazır Profil
//! 4. Rol Zorlamadan Akıllı Ek Odak (Additional Focus) Önerisi & Uygulama (`EventFmxPlayerTrainingFocusChanged`)

use eframe::egui::{self, CornerRadius, Frame, Margin, RichText, Stroke, Ui};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use crate::live::LiveState;
use crate::model::level_color;
use crate::t;
use crate::tactics::{self, Player as TacticPlayer};
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PosRoleOption {
    pub role_id: i64,
    pub pos_id: i32,
    pub name: String,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoleScope {
    #[default]
    CurrentEleven,
    BestEleven,
}

impl RoleScope {
    pub fn label(self) -> &'static str {
        match self {
            Self::CurrentEleven => t!("Huidige elf", "Current eleven", "Mevcut ilk 11"),
            Self::BestEleven => t!("Beste elf", "Best eleven", "Önerilen en iyi 11"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RoleTrainingProposal {
    pub uid: u32,
    pub runtime_id: u32,
    pub name: String,
    pub place_label: &'static str,
    pub pos_idx: usize,
    pub pos_rating: u8,
    pub pos_word: &'static str,
    pub ip_bit: Option<u8>,
    pub ip_code: String,
    pub ip_name: String,
    pub oop_bit: Option<u8>,
    pub oop_code: String,
    pub oop_name: String,
    pub matched_pos_role: Option<PosRoleOption>,
    pub matched_oop_role: Option<(i64, String)>,
}

pub struct ParsedBepInEx {
    pub bottlenecks: Vec<PlayerBottleneck>,
    pub intensities: HashMap<u32, IntensityLevel>,
    pub focus_options: HashMap<i32, String>,
    pub pos_role_options: Vec<PosRoleOption>,
    pub oop_role_options: HashMap<i64, String>,
    pub session_ready: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PlayerSortCol {
    Name,
    Position,
    Condition,
    Risk,
    #[default]
    TacticalScore,
    RecommendedIntensity,
    RecommendedFocus,
}

#[derive(Clone, Debug)]
pub struct UnifiedPlayerRow {
    pub key: u32,
    pub uid: Option<u32>,
    pub runtime_id: u32,
    pub name: String,
    pub position: Option<&'static str>,
    pub age: Option<u8>,
    pub condition: Option<f32>,
    pub sharpness: Option<f32>,
    pub injured: bool,
    pub rest_match_warning: bool,
    pub tactical_overall: Option<f32>,
    pub tactical_detail: Option<String>,
    pub intensity_rec: Option<IntensityLevel>,
    pub intensity_cur: Option<IntensityLevel>,
    pub intensity_reason: Option<String>,
    pub focus_rec: Option<AdditionalFocus>,
    pub focus_weak_attrs: Option<String>,
    pub focus_reason: Option<String>,
}

impl UnifiedPlayerRow {
    pub fn risk_rank(&self) -> u8 {
        if self.injured {
            2
        } else if self.rest_match_warning {
            1
        } else {
            0
        }
    }

    pub fn has_applicable_suggestion(&self, focus_options: &HashMap<i32, String>) -> bool {
        if self.runtime_id == 0 {
            return false;
        }
        let has_int = self.intensity_rec.is_some();
        let has_foc = self
            .focus_rec
            .and_then(|f| f.match_option_id(focus_options))
            .is_some();
        has_int || has_foc
    }

    pub fn needs_action(
        &self,
        focus_options: &HashMap<i32, String>,
        applied_focuses: &HashMap<u32, AdditionalFocus>,
    ) -> bool {
        if self.runtime_id == 0 {
            return false;
        }
        let int_diff = matches!(self.intensity_rec, Some(lvl) if self.intensity_cur != Some(lvl));
        let foc_diff = self.focus_rec.is_some_and(|fc| {
            fc.match_option_id(focus_options).is_some()
                && self.uid.is_none_or(|u| applied_focuses.get(&u) != Some(&fc))
        });
        int_diff || foc_diff
    }
}

pub struct IndividualState {
    pub profile: IntensityProfile,
    pub bottlenecks: Vec<PlayerBottleneck>,
    pub current_intensities: HashMap<u32, IntensityLevel>,
    pub focus_options: HashMap<i32, String>,
    pub pos_role_options: Vec<PosRoleOption>,
    pub oop_role_options: HashMap<i64, String>,
    pub applied_focuses: HashMap<u32, AdditionalFocus>,
    pub role_training_mode: bool,
    pub role_scope: RoleScope,
    pub role_apply_ip: bool,
    pub role_apply_oop: bool,
    pub role_approved: HashMap<u32, bool>,
    pub applied_roles: HashMap<u32, (Option<u8>, Option<u8>)>,
    pub session_ready: bool,
    pub auto_apply_intensity: bool,
    pub sort_col: PlayerSortCol,
    pub sort_asc: bool,
    pub selected_players: HashSet<u32>,
    pub selection_seeded: bool,
    pub last_cmd_result_utc: String,
    pub last_auto_apply: Option<std::time::Instant>,
    pub status: String,
    pub status_error: bool,
    tactics_requested: bool,
}

impl Default for IndividualState {
    fn default() -> Self {
        Self {
            profile: IntensityProfile::default(),
            bottlenecks: Vec::new(),
            current_intensities: HashMap::new(),
            focus_options: HashMap::new(),
            pos_role_options: Vec::new(),
            oop_role_options: HashMap::new(),
            applied_focuses: HashMap::new(),
            role_training_mode: true,
            role_scope: RoleScope::default(),
            role_apply_ip: true,
            role_apply_oop: true,
            role_approved: HashMap::new(),
            applied_roles: HashMap::new(),
            session_ready: false,
            auto_apply_intensity: false,
            sort_col: PlayerSortCol::TacticalScore,
            sort_asc: true,
            selected_players: HashSet::new(),
            selection_seeded: false,
            last_cmd_result_utc: String::new(),
            last_auto_apply: None,
            status: String::new(),
            status_error: false,
            tactics_requested: false,
        }
    }
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

    let mut pos_role_options = Vec::new();
    if let Some(arr) = root.get("pos_role_options").and_then(|v| v.as_array()) {
        for item in arr {
            let role_id = item.get("role_id").and_then(|v| v.as_i64());
            let pos_id = item.get("pos_id").and_then(|v| v.as_i64());
            let name = item.get("name").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty());
            if let (Some(role_id), Some(pos_id), Some(name)) = (role_id, pos_id, name) {
                pos_role_options.push(PosRoleOption {
                    role_id,
                    pos_id: pos_id as i32,
                    name: name.to_string(),
                });
            }
        }
    }

    let mut oop_role_options = HashMap::new();
    if let Some(oo) = root.get("oop_role_options").and_then(|v| v.as_object()) {
        for (k, v) in oo {
            if let (Ok(id), Some(name)) = (k.parse::<i64>(), v.as_str()) {
                if !name.trim().is_empty() {
                    oop_role_options.insert(id, name.trim().to_string());
                }
            }
        }
    }

    let mut out = Vec::new();
    let Some(data) = root.get("data").and_then(|v| v.as_object()) else {
        return ParsedBepInEx {
            bottlenecks: out,
            intensities,
            focus_options,
            pos_role_options,
            oop_role_options,
            session_ready,
        };
    };
    let Some(te) = data
        .iter()
        .find(|(k, _)| k.starts_with("TeamTactic:") && k.ends_with(".TacticalEffectiveness"))
        .or_else(|| data.iter().find(|(k, _)| k.ends_with(".TacticalEffectiveness")))
        .map(|(_, v)| v)
    else {
        return ParsedBepInEx {
            bottlenecks: out,
            intensities,
            focus_options,
            pos_role_options,
            oop_role_options,
            session_ready,
        };
    };

    let Some(players) = find_prefix(te, "Players").and_then(|v| v.as_array()) else {
        return ParsedBepInEx {
            bottlenecks: out,
            intensities,
            focus_options,
            pos_role_options,
            oop_role_options,
            session_ready,
        };
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
    ParsedBepInEx {
        bottlenecks: out,
        intensities,
        focus_options,
        pos_role_options,
        oop_role_options,
        session_ready,
    }
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

fn best_position_index_for_place(player: &TacticPlayer, place: &tactics::Place) -> usize {
    let candidates = place.line.positions(place.side);
    candidates
        .iter()
        .copied()
        .max_by_key(|&idx| player.positions.get(idx).copied().unwrap_or(1))
        .unwrap_or_else(|| candidates.first().copied().unwrap_or(0))
}

fn match_pos_role_option(
    opts: &[PosRoleOption],
    ip_bit: Option<u8>,
    ip_name: &str,
    pos_idx: usize,
) -> Option<PosRoleOption> {
    let bit = ip_bit? as i64;
    let pos_matches = |pid: i32| pid == pos_idx as i32 || (pos_idx < 31 && pid == (1i32 << pos_idx));
    // 1. Kesin rol ID/mask + pozisyon eşleşmesi
    if let Some(m) = opts
        .iter()
        .find(|o| pos_matches(o.pos_id) && (o.role_id == bit || (bit < 62 && o.role_id == (1i64 << bit))))
    {
        return Some(m.clone());
    }
    // 2. İsim + pozisyon eşleşmesi
    if !ip_name.is_empty() {
        let low = ip_name.to_lowercase();
        if let Some(m) = opts
            .iter()
            .find(|o| pos_matches(o.pos_id) && o.name.to_lowercase().contains(&low))
        {
            return Some(m.clone());
        }
    }
    // 3. Yalnız rol ID eşleşmesi
    opts.iter()
        .find(|o| o.role_id == bit || (bit < 62 && o.role_id == (1i64 << bit)))
        .cloned()
}

fn match_oop_role_option(opts: &HashMap<i64, String>, oop_bit: Option<u8>, oop_name: &str) -> Option<(i64, String)> {
    let bit = oop_bit? as i64;
    if let Some(name) = opts.get(&bit) {
        return Some((bit, name.clone()));
    }
    if bit < 62 {
        let mask = 1i64 << bit;
        if let Some(name) = opts.get(&mask) {
            return Some((mask, name.clone()));
        }
    }
    if !oop_name.is_empty() {
        let low = oop_name.to_lowercase();
        if let Some((&id, name)) = opts.iter().find(|(_, n)| n.to_lowercase().contains(&low)) {
            return Some((id, name.clone()));
        }
    }
    None
}

pub fn compute_role_proposals(
    td: &tactics::Data,
    squad_rids: &HashMap<u32, u32>,
    pos_opts: &[PosRoleOption],
    oop_opts: &HashMap<i64, String>,
    scope: RoleScope,
) -> Vec<RoleTrainingProposal> {
    let Some(tactic) = td.tactics.get(td.current).or_else(|| td.tactics.first()) else {
        return Vec::new();
    };
    let places = tactics::places(tactic);
    let best_idx = if scope == RoleScope::BestEleven {
        tactics::best_eleven(&places, &td.players)
    } else {
        Vec::new()
    };

    let mut out = Vec::with_capacity(places.len());
    for (slot_i, place) in places.iter().enumerate() {
        let player_opt = match scope {
            RoleScope::CurrentEleven => place.uid.and_then(|uid| td.players.iter().find(|p| p.uid == uid)),
            RoleScope::BestEleven => best_idx.get(slot_i).and_then(|&j| td.players.get(j)),
        };
        let Some(p) = player_opt else { continue };
        let runtime_id = squad_rids.get(&p.uid).copied().unwrap_or(0);
        let pos_idx = best_position_index_for_place(p, place);
        let pos_rating = tactics::position_rating(p, place);
        let pos_word = tactics::position_word(pos_rating);
        let (ip_code, ip_name) = tactics::ip_label(place.ip);
        let (oop_code, oop_name) = tactics::oop_label(place.oop);
        let matched_pos_role = match_pos_role_option(pos_opts, place.ip, &ip_name, pos_idx);
        let matched_oop_role = match_oop_role_option(oop_opts, place.oop, &oop_name);

        out.push(RoleTrainingProposal {
            uid: p.uid,
            runtime_id,
            name: p.name.clone(),
            place_label: place.line.label(place.side),
            pos_idx,
            pos_rating,
            pos_word,
            ip_bit: place.ip,
            ip_code,
            ip_name,
            oop_bit: place.oop,
            oop_code,
            oop_name,
            matched_pos_role,
            matched_oop_role,
        });
    }
    out
}

pub fn build_role_proposals(
    live: &LiveState,
    pos_opts: &[PosRoleOption],
    oop_opts: &HashMap<i64, String>,
    scope: RoleScope,
) -> Vec<RoleTrainingProposal> {
    let Some(td) = &live.tactics else { return Vec::new() };
    let mut squad_rids = HashMap::new();
    for p in &live.squad {
        if let (Some(uid), Some(rid)) = (
            p.get("uid").and_then(|v| v.as_u64()),
            p.get("runtime_id").and_then(|v| v.as_u64()),
        ) {
            if rid > 0 {
                squad_rids.insert(uid as u32, rid as u32);
            }
        }
    }
    compute_role_proposals(td, &squad_rids, pos_opts, oop_opts, scope)
}

pub fn write_role_training_command(
    manager_runtime_id: u32,
    pos_items: &[(u32, i64, i32, String)],
    oop_items: &[(u32, i64, String)],
) -> Result<usize, String> {
    let valid_pos: Vec<Value> = pos_items
        .iter()
        .filter(|(rid, role_id, pos_id, _)| *rid != 0 && *role_id >= 0 && *pos_id >= 0)
        .map(|(rid, role_id, pos_id, name)| {
            json!({
                "runtime_id": rid,
                "role_id": role_id,
                "pos_id": pos_id,
                "name": name,
            })
        })
        .collect();
    let valid_oop: Vec<Value> = oop_items
        .iter()
        .filter(|(rid, role_id, _)| *rid != 0 && *role_id >= 0)
        .map(|(rid, role_id, name)| {
            json!({
                "runtime_id": rid,
                "role_id": role_id,
                "name": name,
            })
        })
        .collect();
    if valid_pos.is_empty() && valid_oop.is_empty() {
        return Err(t!(
            "Open eerst het Individuele Trainingsmenu in FM zodat positie- en rolopties gelezen worden.",
            "Open Individual Training once in FM so position/role option IDs are captured.",
            "Pozisyon/rol kimliklerinin alınması için FM27'de Bireysel Antrenman sekmesini bir kez açın."
        )
        .to_string());
    }
    let count = valid_pos.len() + valid_oop.len();
    write_cmd_payload(json!({
        "manager_runtime_id": manager_runtime_id,
        "pos_role_items": valid_pos,
        "oop_role_items": valid_oop,
    }))?;
    Ok(count)
}

pub fn build_unified_player_rows(
    live: &LiveState,
    state: &IndividualState,
) -> (Vec<UnifiedPlayerRow>, usize) {
    let (intensity_recs, week_matches) =
        build_squad_recommendations(live, &state.current_intensities, state.profile);
    let focus_recs = build_focus_recommendations(live);

    let mut rows: Vec<UnifiedPlayerRow> = Vec::with_capacity(
        intensity_recs.len().max(focus_recs.len()).max(state.bottlenecks.len()),
    );
    let mut by_rid: HashMap<u32, usize> = HashMap::new();
    let mut by_name: HashMap<String, usize> = HashMap::new();

    for r in intensity_recs {
        let norm = r.name.trim().to_lowercase();
        let idx = rows.len();
        if r.runtime_id != 0 {
            by_rid.insert(r.runtime_id, idx);
        }
        if !norm.is_empty() {
            by_name.entry(norm).or_insert(idx);
        }
        rows.push(UnifiedPlayerRow {
            key: r.runtime_id,
            uid: None,
            runtime_id: r.runtime_id,
            name: r.name,
            position: None,
            age: r.age,
            condition: Some(r.condition),
            sharpness: Some(r.sharpness),
            injured: r.injured,
            rest_match_warning: r.rest_match_warning,
            tactical_overall: None,
            tactical_detail: None,
            intensity_rec: Some(r.recommended),
            intensity_cur: r.current,
            intensity_reason: Some(r.reason),
            focus_rec: None,
            focus_weak_attrs: None,
            focus_reason: None,
        });
    }

    for f in focus_recs {
        let norm = f.name.trim().to_lowercase();
        let existing = (f.runtime_id != 0)
            .then(|| by_rid.get(&f.runtime_id).copied())
            .flatten()
            .or_else(|| by_name.get(&norm).copied());

        if let Some(idx) = existing {
            let row = &mut rows[idx];
            row.uid = Some(f.uid);
            if row.runtime_id == 0 && f.runtime_id != 0 {
                row.runtime_id = f.runtime_id;
                by_rid.insert(f.runtime_id, idx);
            }
            row.position = Some(f.natural_pos);
            row.focus_rec = Some(f.focus);
            row.focus_weak_attrs = Some(f.weak_attrs);
            row.focus_reason = Some(f.reason);
        } else {
            let idx = rows.len();
            if f.runtime_id != 0 {
                by_rid.insert(f.runtime_id, idx);
            }
            if !norm.is_empty() {
                by_name.entry(norm).or_insert(idx);
            }
            rows.push(UnifiedPlayerRow {
                key: f.runtime_id,
                uid: Some(f.uid),
                runtime_id: f.runtime_id,
                name: f.name,
                position: Some(f.natural_pos),
                age: None,
                condition: None,
                sharpness: None,
                injured: false,
                rest_match_warning: false,
                tactical_overall: None,
                tactical_detail: None,
                intensity_rec: None,
                intensity_cur: None,
                intensity_reason: None,
                focus_rec: Some(f.focus),
                focus_weak_attrs: Some(f.weak_attrs),
                focus_reason: Some(f.reason),
            });
        }
    }

    for b in &state.bottlenecks {
        let norm = b.name.trim().to_lowercase();
        let existing = (b.runtime_id != 0)
            .then(|| by_rid.get(&b.runtime_id).copied())
            .flatten()
            .or_else(|| by_name.get(&norm).copied());
        let detail = format!("{} ({:.0})", b.weakest_label, b.weakest_score);

        if let Some(idx) = existing {
            let row = &mut rows[idx];
            if row.runtime_id == 0 && b.runtime_id != 0 {
                row.runtime_id = b.runtime_id;
                by_rid.insert(b.runtime_id, idx);
            }
            row.tactical_overall = Some(b.overall);
            row.tactical_detail = Some(detail);
        } else {
            let idx = rows.len();
            if b.runtime_id != 0 {
                by_rid.insert(b.runtime_id, idx);
            }
            if !norm.is_empty() {
                by_name.entry(norm).or_insert(idx);
            }
            rows.push(UnifiedPlayerRow {
                key: b.runtime_id,
                uid: None,
                runtime_id: b.runtime_id,
                name: b.name.clone(),
                position: None,
                age: None,
                condition: None,
                sharpness: None,
                injured: false,
                rest_match_warning: false,
                tactical_overall: Some(b.overall),
                tactical_detail: Some(detail),
                intensity_rec: None,
                intensity_cur: state.current_intensities.get(&b.runtime_id).copied(),
                intensity_reason: None,
                focus_rec: None,
                focus_weak_attrs: None,
                focus_reason: None,
            });
        }
    }

    for (idx, row) in rows.iter_mut().enumerate() {
        row.key = if row.runtime_id != 0 {
            row.runtime_id
        } else if let Some(uid) = row.uid.filter(|u| *u != 0) {
            0x8000_0000 | uid
        } else {
            0x4000_0000 + idx as u32
        };
    }

    (rows, week_matches)
}

pub fn sort_unified_rows(rows: &mut [UnifiedPlayerRow], col: PlayerSortCol, asc: bool) {
    use std::cmp::Ordering;
    let cmp_opt_f32 = |a: Option<f32>, b: Option<f32>, asc: bool| -> Ordering {
        match (a, b) {
            (Some(x), Some(y)) => {
                let ord = x.partial_cmp(&y).unwrap_or(Ordering::Equal);
                if asc { ord } else { ord.reverse() }
            }
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => Ordering::Equal,
        }
    };
    let cmp_opt_str = |a: Option<&str>, b: Option<&str>, asc: bool| -> Ordering {
        match (a, b) {
            (Some(x), Some(y)) => {
                let ord = x.to_lowercase().cmp(&y.to_lowercase());
                if asc { ord } else { ord.reverse() }
            }
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => Ordering::Equal,
        }
    };

    rows.sort_by(|a, b| {
        let primary = match col {
            PlayerSortCol::Name => {
                let ord = a.name.to_lowercase().cmp(&b.name.to_lowercase());
                if asc { ord } else { ord.reverse() }
            }
            PlayerSortCol::Position => cmp_opt_str(a.position, b.position, asc),
            PlayerSortCol::Condition => cmp_opt_f32(a.condition, b.condition, asc),
            PlayerSortCol::Risk => {
                let ord = a.risk_rank().cmp(&b.risk_rank());
                if asc { ord } else { ord.reverse() }
            }
            PlayerSortCol::TacticalScore => cmp_opt_f32(a.tactical_overall, b.tactical_overall, asc),
            PlayerSortCol::RecommendedIntensity => cmp_opt_f32(
                a.intensity_rec.map(|l| l.code() as f32),
                b.intensity_rec.map(|l| l.code() as f32),
                asc,
            ),
            PlayerSortCol::RecommendedFocus => cmp_opt_str(
                a.focus_rec.map(|f| f.label()),
                b.focus_rec.map(|f| f.label()),
                asc,
            ),
        };
        primary.then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });
}

pub fn write_combined_recommendations(
    manager_runtime_id: u32,
    intensity_items: &[(u32, IntensityLevel)],
    focus_items: &[(u32, i32, String)],
) -> Result<usize, String> {
    let valid_int: Vec<Value> = intensity_items
        .iter()
        .filter(|(rid, _)| *rid != 0)
        .map(|(rid, lvl)| json!({ "runtime_id": rid, "level": lvl.code() }))
        .collect();
    let valid_foc: Vec<Value> = focus_items
        .iter()
        .filter(|(rid, opt_id, _)| *rid != 0 && *opt_id >= 0)
        .map(|(rid, opt_id, name)| json!({ "runtime_id": rid, "option_id": opt_id, "name": name }))
        .collect();

    match (valid_int.is_empty(), valid_foc.is_empty()) {
        (true, true) => Err(t!(
            "Geen geldige spelers gevonden om in te stellen.",
            "No valid players found to apply.",
            "Uygulanacak geçerli oyuncu referansı bulunamadı."
        )
        .to_string()),
        (false, true) => write_intensity_command(manager_runtime_id, intensity_items),
        (true, false) => write_focus_command(manager_runtime_id, focus_items),
        (false, false) => {
            let count = valid_int.len() + valid_foc.len();
            write_cmd_payload(json!({
                "manager_runtime_id": manager_runtime_id,
                "items": valid_int,
                "focus_items": valid_foc,
            }))?;
            Ok(count)
        }
    }
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

/// Oyuncular sayfası: birleşik sıralanabilir tablo + toplu öneri uygulama + taktik rolü kartı.
pub fn render_page(
    ui: &mut Ui,
    ctx: &egui::Context,
    state: &mut IndividualState,
    live: &mut LiveState,
    on_read_bepinex: &mut bool,
    on_save_prefs: &mut bool,
    on_open_setup: &mut bool,
) {
    if live.connected && live.tactics.is_none() && !live.tactics_busy() && !state.tactics_requested {
        state.tactics_requested = true;
        live.load_tactics(ctx);
    }
    if !live.connected {
        state.tactics_requested = false;
        state.selection_seeded = false;
    }

    let mgr_id = live
        .club
        .as_ref()
        .and_then(|c| c.get("manager_runtime_id"))
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as u32;

    let mut refresh_squad = false;

    theme::page_header(
        ui,
        t!("Spelers", "Players", "Oyuncular"),
        t!(
            "Blessurerisico, tactische knelpunten, trainingsintensiteit en extra focus in één overzicht.",
            "Injury risk, tactical bottlenecks, training intensity and Additional Focus in one sortable table.",
            "Sakatlık riski, taktik puanı, önerilen yoğunluk ve ek odak tek bir sıralanabilir tabloda."
        ),
        |ui| {
            if live.connected {
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
                    t!("Uit spelmodule lezen", "Read from game plugin", "Oyun eklentisinden oku"),
                    true,
                )
                .clicked()
                {
                    *on_read_bepinex = true;
                }
            }
        },
    );

    if refresh_squad {
        state.tactics_requested = false;
        state.selection_seeded = false;
        live.refresh(ctx, t!("verversen", "refresh", "yenile"), None);
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

    if !live.connected {
        theme::card(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(theme::ic(icon::USERS_THREE).size(24.0).color(theme::ACCENT));
                ui.vertical(|ui| {
                    ui.label(theme::h2(t!(
                        "Verbind met FM27 voor spelersgegevens",
                        "Connect to FM27 for player data",
                        "Oyuncu verileri için FM27'ye bağlan"
                    )));
                    ui.add(
                        egui::Label::new(theme::dim(t!(
                            "Conditie, knelpunten, intensiteit, tactische rollen en extra focus verschijnen zodra FM27 actief is.",
                            "Squad condition, bottlenecks, intensity, tactic roles and Additional Focus appear once FM27 is running.",
                            "Kadro kondisyonu, darboğaz raporu, bireysel yoğunluk, taktik rolü ve ek odak önerileri FM27 açıkken görünür."
                        )))
                        .wrap(),
                    );
                });
            });
            ui.add_space(10.0);
            if theme::primary_button(
                ui,
                icon::PLUGS_CONNECTED,
                t!("Hoe verbind ik?", "How do I connect?", "Nasıl bağlanırım?"),
                true,
            )
            .clicked()
            {
                *on_open_setup = true;
            }
        });
        return;
    }

    let (mut rows, week_matches) = build_unified_player_rows(live, state);
    if !state.selection_seeded && !rows.is_empty() {
        state.selected_players.clear();
        for r in &rows {
            if r.needs_action(&state.focus_options, &state.applied_focuses) {
                state.selected_players.insert(r.key);
            }
        }
        state.selection_seeded = true;
    }
    sort_unified_rows(&mut rows, state.sort_col, state.sort_asc);

    let mut apply_selected = false;
    let selected_count = rows
        .iter()
        .filter(|r| state.selected_players.contains(&r.key) && r.has_applicable_suggestion(&state.focus_options))
        .count();

    theme::card(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.label(theme::label_strong(t!("Intensiteitsprofiel:", "Intensity profile:", "Yoğunluk profili:")));
            let prof_opts: [(IntensityProfile, &str); 3] = [
                (IntensityProfile::Protective, IntensityProfile::Protective.label()),
                (IntensityProfile::Balanced, IntensityProfile::Balanced.label()),
                (IntensityProfile::Development, IntensityProfile::Development.label()),
            ];
            if theme::segmented(ui, &mut state.profile, &prof_opts) {
                *on_save_prefs = true;
                state.selection_seeded = false;
            }
            ui.label(theme::ic(icon::INFO).color(theme::TEXT_FAINT).size(14.0))
                .on_hover_text(state.profile.description());

            ui.add_space(8.0);
            if theme::toggle(ui, &mut state.auto_apply_intensity).changed() {
                *on_save_prefs = true;
            }
            ui.label(theme::label_strong(t!(
                "Automatisch toepassen",
                "Auto-apply",
                "Otomatik uygula"
            )))
            .on_hover_text(t!(
                "Intensiteit automatisch bewaken bij conditiewijziging",
                "Automatically guard intensity as condition changes",
                "Kondisyon değiştikçe önerilen yoğunluğu otomatik uygula"
            ));

            if week_matches >= 2 {
                theme::pill(
                    ui,
                    &t!(
                        format!("{week_matches} wedstr. deze week"),
                        format!("{week_matches} match(es) this week"),
                        format!("Bu hafta {week_matches} maç")
                    ),
                    theme::DANGER,
                );
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let apply_lbl = t!(
                    format!("Pas suggesties toe op selectie ({selected_count})"),
                    format!("Apply suggestions to selected ({selected_count})"),
                    format!("Seçilenlere önerileri uygula ({selected_count})")
                );
                if theme::primary_button(ui, icon::LIGHTNING, &apply_lbl, selected_count > 0)
                    .on_disabled_hover_text(t!(
                        "Selecteer minstens één speler met een beschikbare suggestie",
                        "Select at least one player with an applicable suggestion",
                        "Uygulanabilir önerisi olan en az bir oyuncu seçin"
                    ))
                    .clicked()
                {
                    apply_selected = true;
                }
            });
        });

        ui.add_space(10.0);

        if rows.is_empty() {
            ui.label(theme::dim(if live.busy() || live.tactics_busy() {
                t!("Spelersgegevens laden…", "Loading squad data…", "Oyuncu verileri yükleniyor…")
            } else {
                t!(
                    "Nog geen spelersdata uit FM27 beschikbaar.",
                    "No player data available from FM27 yet.",
                    "FM27'den henüz oyuncu verisi alınmadı."
                )
            }));
        } else {
            let selectable_keys: Vec<u32> = rows
                .iter()
                .filter(|r| r.has_applicable_suggestion(&state.focus_options))
                .map(|r| r.key)
                .collect();
            let mut all_checked = !selectable_keys.is_empty()
                && selectable_keys.iter().all(|k| state.selected_players.contains(k));

            egui::Grid::new("unified_players_grid")
                .num_columns(8)
                .spacing([12.0, 7.0])
                .striped(true)
                .show(ui, |ui| {
                    if ui
                        .add_enabled(
                            !selectable_keys.is_empty(),
                            egui::Checkbox::without_text(&mut all_checked),
                        )
                        .changed()
                    {
                        if all_checked {
                            for k in &selectable_keys {
                                state.selected_players.insert(*k);
                            }
                        } else {
                            state.selected_players.clear();
                        }
                    }

                    let cols: [(PlayerSortCol, &str); 7] = [
                        (PlayerSortCol::Name, t!("Speler", "Player", "Oyuncu")),
                        (PlayerSortCol::Position, t!("Pos", "Pos", "Mevki")),
                        (PlayerSortCol::Condition, t!("Conditie", "Condition", "Kondisyon")),
                        (PlayerSortCol::Risk, t!("Risico", "Risk", "Risk")),
                        (PlayerSortCol::TacticalScore, t!("Tactiekscore", "Tactical score", "Taktik puanı")),
                        (PlayerSortCol::RecommendedIntensity, t!("Aanbevolen intensiteit", "Rec. intensity", "Önerilen yoğunluk")),
                        (PlayerSortCol::RecommendedFocus, t!("Aanbevolen focus", "Rec. focus", "Önerilen ek odak")),
                    ];
                    for (col, label) in cols {
                        let active = state.sort_col == col;
                        let arrow = if active {
                            if state.sort_asc { " ▲" } else { " ▼" }
                        } else {
                            ""
                        };
                        let hdr_txt = theme::col_header(&format!("{label}{arrow}"))
                            .color(if active { theme::TEXT } else { theme::TEXT_FAINT });
                        let resp = ui.add(egui::Label::new(hdr_txt).sense(egui::Sense::click()));
                        if resp.clicked() {
                            if active {
                                state.sort_asc = !state.sort_asc;
                            } else {
                                state.sort_col = col;
                                state.sort_asc = col != PlayerSortCol::Risk;
                            }
                        }
                        resp.on_hover_cursor(egui::CursorIcon::PointingHand);
                    }
                    ui.end_row();

                    for r in &rows {
                        let can_pick = r.has_applicable_suggestion(&state.focus_options);
                        let mut checked = state.selected_players.contains(&r.key);
                        if ui
                            .add_enabled(can_pick, egui::Checkbox::without_text(&mut checked))
                            .changed()
                        {
                            if checked {
                                state.selected_players.insert(r.key);
                            } else {
                                state.selected_players.remove(&r.key);
                            }
                        }

                        let mut name_tip = String::new();
                        if let Some(a) = r.age {
                            name_tip.push_str(&format!("{a}"));
                        }
                        if let Some(s) = r.sharpness {
                            if !name_tip.is_empty() {
                                name_tip.push_str(" · ");
                            }
                            name_tip.push_str(&format!("{s:.0}%"));
                        }
                        let name_lbl = ui.label(theme::label_strong(&r.name));
                        if !name_tip.is_empty() {
                            name_lbl.on_hover_text(name_tip);
                        }

                        if let Some(pos) = r.position {
                            theme::pill(ui, pos, theme::ACCENT);
                        } else {
                            ui.label(theme::faint("—"));
                        }

                        if let Some(c) = r.condition {
                            let cond_col = if c >= 85.0 {
                                theme::SUCCESS
                            } else if c >= 72.0 {
                                theme::WARNING
                            } else {
                                theme::DANGER
                            };
                            theme::pill(ui, &format!("{c:.0}%"), cond_col);
                        } else {
                            ui.label(theme::faint("—"));
                        }

                        if r.injured {
                            theme::pill(ui, t!("BLESSURE", "INJURED", "SAKAT"), theme::DANGER);
                        } else if r.rest_match_warning {
                            theme::pill(ui, t!("RUST", "REST", "DİNLENDİR"), theme::WARNING);
                        } else {
                            ui.label(theme::faint("—"));
                        }

                        if let Some(ov) = r.tactical_overall {
                            let resp = ui
                                .horizontal(|ui| {
                                    theme::pill(ui, &format!("{ov:.0}"), theme::rgb(level_color(ov)));
                                })
                                .response;
                            if let Some(det) = &r.tactical_detail {
                                resp.on_hover_text(det);
                            }
                        } else {
                            ui.label(theme::faint("—"));
                        }

                        if let Some(lvl) = r.intensity_rec {
                            let resp = ui
                                .horizontal(|ui| {
                                    theme::pill(ui, lvl.short_label(), lvl.color());
                                    if r.intensity_cur == Some(lvl) {
                                        ui.label(theme::ic(icon::CHECK_CIRCLE).color(theme::SUCCESS).size(13.0))
                                            .on_hover_text(t!("Actief in FM", "Active in FM", "FM'de aktif"));
                                    }
                                })
                                .response;
                            if let Some(why) = &r.intensity_reason {
                                resp.on_hover_text(why);
                            }
                        } else {
                            ui.label(theme::faint("—"));
                        }

                        if let Some(fc) = r.focus_rec {
                            let resp = ui
                                .horizontal(|ui| {
                                    theme::pill(ui, fc.label(), theme::INFO);
                                    if r.uid.is_some_and(|u| state.applied_focuses.get(&u) == Some(&fc)) {
                                        ui.label(theme::ic(icon::CHECK_CIRCLE).color(theme::SUCCESS).size(13.0))
                                            .on_hover_text(t!("✓ Gezet", "✓ Set", "✓ Seçildi"));
                                    }
                                })
                                .response;
                            if let Some(tip) = r.focus_reason.as_deref().or(r.focus_weak_attrs.as_deref()) {
                                resp.on_hover_text(tip);
                            }
                        } else {
                            ui.label(theme::faint("—"));
                        }

                        ui.end_row();
                    }
                });
        }
    });

    if apply_selected {
        let mut int_batch: Vec<(u32, IntensityLevel)> = Vec::new();
        let mut foc_batch: Vec<(u32, i32, String)> = Vec::new();
        let mut foc_applied: Vec<(u32, AdditionalFocus)> = Vec::new();

        for r in &rows {
            if !state.selected_players.contains(&r.key) || r.runtime_id == 0 {
                continue;
            }
            if let Some(lvl) = r.intensity_rec {
                int_batch.push((r.runtime_id, lvl));
            }
            if let Some(fc) = r.focus_rec {
                if let Some((opt_id, opt_name)) = fc.match_option_id(&state.focus_options) {
                    foc_batch.push((r.runtime_id, opt_id, opt_name));
                    if let Some(uid) = r.uid {
                        foc_applied.push((uid, fc));
                    }
                }
            }
        }

        match write_combined_recommendations(mgr_id, &int_batch, &foc_batch) {
            Ok(n) => {
                for (rid, lvl) in &int_batch {
                    state.current_intensities.insert(*rid, *lvl);
                }
                for (uid, fc) in &foc_applied {
                    state.applied_focuses.insert(*uid, *fc);
                }
                state.status = t!(
                    format!("Aanbevelingen voor {n} acties verstuurd naar FM27…"),
                    format!("Sent recommendations for {n} actions to FM27…"),
                    format!("Seçili oyuncular için {n} öneri FM27'ye gönderildi…")
                );
                state.status_error = false;
            }
            Err(e) => {
                state.status = e;
                state.status_error = true;
            }
        }
    }

    ui.add_space(14.0);

    // ---- Taktik Rolü Bireysel Antrenmanı (Önizle -> Onayla -> FM27'ye Yaz)
    render_tactic_role_card(ui, state, live, mgr_id, on_save_prefs);
}

fn render_tactic_role_card(
    ui: &mut Ui,
    state: &mut IndividualState,
    live: &LiveState,
    mgr_id: u32,
    on_save_prefs: &mut bool,
) {
    theme::card_accent(ui, theme::ACCENT, |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.label(theme::ic(icon::STRATEGY).color(theme::ACCENT_HI).size(18.0));
            ui.label(theme::h2(t!(
                "Tactische rol als individuele training",
                "Tactic Role Individual Training",
                "Taktik Rolü Bireysel Antrenmanı"
            )));
            theme::pill(
                ui,
                t!("Alleen na goedkeuring", "Preview & Approve", "Önizle & Onayla"),
                theme::INFO,
            );
            let live_opts_count = state.pos_role_options.len() + state.oop_role_options.len();
            if live_opts_count > 0 {
                theme::pill(
                    ui,
                    &t!(
                        format!("{live_opts_count} rolopties gereed"),
                        format!("{live_opts_count} role options captured"),
                        format!("{live_opts_count} FM rol seçeneği hazır")
                    ),
                    theme::SUCCESS,
                );
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if theme::toggle(ui, &mut state.role_training_mode).changed() {
                    *on_save_prefs = true;
                }
            });
        });
        ui.add_space(4.0);
        ui.add(
            egui::Label::new(theme::dim(t!(
                "Toont eerst de rol en positie per speler uit je tactiek. Niets wordt naar FM geschreven totdat je de spelers aanvinkt en op Goedkeuren & Schrijven klikt.",
                "Previews each player's tactical position and In/Out of Possession role first. Nothing is written into FM27 until you review, tick players, and click Approve & Write.",
                "Oyunculara taktikteki mevki ve topla/topsuz rolleri bireysel antrenman olarak önceden gösterir. Onay kutularını seçip 'Onayla ve FM27'ye Yaz' demeden hiçbir şey yazılmaz."
            )))
            .wrap(),
        );

        if !state.role_training_mode {
            return;
        }

        ui.add_space(8.0);
        let proposals = build_role_proposals(live, &state.pos_role_options, &state.oop_role_options, state.role_scope);
        for p in &proposals {
            state.role_approved.entry(p.uid).or_insert(p.pos_rating >= 12);
        }

        if proposals.is_empty() {
            ui.label(theme::dim(t!(
                "Verbind met een actieve save in FM27 om de actieve tactiek en rollen op te halen.",
                "Connect to an active save in FM27 to preview your tactic's roles per player.",
                "Taktiğinizdeki rolleri oyuncu bazında önizlemek için FM27'de kariyerinizi açın."
            )));
            return;
        }

        ui.horizontal_wrapped(|ui| {
            let scopes = [
                (RoleScope::CurrentEleven, RoleScope::CurrentEleven.label()),
                (RoleScope::BestEleven, RoleScope::BestEleven.label()),
            ];
            theme::segmented(ui, &mut state.role_scope, &scopes);
            ui.add_space(10.0);
            ui.checkbox(
                &mut state.role_apply_ip,
                t!("Positie / rol met bal", "Position / IP Role", "Mevki / Topla Rol"),
            );
            ui.checkbox(
                &mut state.role_apply_oop,
                t!("Rol zonder bal", "OOP Role", "Topsuz Rol"),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if theme::small_button(ui, t!("Alles uit", "Clear", "Temizle"), true).clicked() {
                    for p in &proposals {
                        state.role_approved.insert(p.uid, false);
                    }
                }
                if theme::small_button(ui, t!("Alleen passend (12+)", "Fit 12+ only", "Yalnız 12+ Uyumlu"), true).clicked() {
                    for p in &proposals {
                        state.role_approved.insert(p.uid, p.pos_rating >= 12);
                    }
                }
                if theme::small_button(ui, t!("Alles aan", "Select all", "Tümünü Seç"), true).clicked() {
                    for p in &proposals {
                        state.role_approved.insert(p.uid, true);
                    }
                }
            });
        });

        let has_live_role_ids = proposals.iter().any(|p| p.matched_pos_role.is_some() || p.matched_oop_role.is_some());
        if !has_live_role_ids {
            ui.add_space(4.0);
            ui.label(theme::faint(t!(
                "Tip: open eenmaal Individuele Training in FM27 zodat de spelmodule rol-ID's kan lezen.",
                "Tip: open Individual Training once in FM27 so the game plugin can capture Position & Role IDs.",
                "İpucu: Canlı Mevki ve Rol kimliklerinin okunması için FM27'de Bireysel Antrenman ekranını bir kez açın."
            )));
        }

        ui.add_space(8.0);
        let mut selected_count = 0usize;
        let mut writable_pos: Vec<(u32, i64, i32, String)> = Vec::new();
        let mut writable_oop: Vec<(u32, i64, String)> = Vec::new();
        let mut pending_records: Vec<(u32, Option<u8>, Option<u8>)> = Vec::new();

        egui::Grid::new("tactic_role_preview_grid")
            .num_columns(7)
            .spacing([12.0, 6.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label(theme::col_header(t!("Akkoord", "Approve", "Onay")));
                ui.label(theme::col_header(t!("Speler", "Player", "Oyuncu")));
                ui.label(theme::col_header(t!("Positie", "Position", "Mevki")));
                ui.label(theme::col_header(t!("Rol met bal", "In Possession Role", "Topla Rol")));
                ui.label(theme::col_header(t!("Rol zonder bal", "Out of Possession Role", "Topsuz Rol")));
                ui.label(theme::col_header(t!("Positiewaarde", "Position Fit", "Mevki Uyumu")));
                ui.label(theme::col_header(t!("Status", "Status", "Durum")));
                ui.end_row();

                for p in &proposals {
                    let approved = state.role_approved.entry(p.uid).or_insert(p.pos_rating >= 12);
                    ui.checkbox(approved, "");
                    if *approved {
                        selected_count += 1;
                        if p.runtime_id != 0 {
                            if state.role_apply_ip {
                                if let Some(m) = &p.matched_pos_role {
                                    writable_pos.push((p.runtime_id, m.role_id, m.pos_id, m.name.clone()));
                                }
                            }
                            if state.role_apply_oop {
                                if let Some((id, nm)) = &p.matched_oop_role {
                                    writable_oop.push((p.runtime_id, *id, nm.clone()));
                                }
                            }
                            pending_records.push((p.uid, p.ip_bit, p.oop_bit));
                        }
                    }

                    ui.label(theme::label_strong(&p.name));
                    theme::pill(ui, p.place_label, theme::INFO);

                    if !p.ip_name.is_empty() {
                        ui.label(format!("{} · {}", p.ip_code, p.ip_name));
                    } else {
                        ui.label(theme::dim(&p.ip_code));
                    }

                    if !p.oop_name.is_empty() {
                        ui.label(format!("{} · {}", p.oop_code, p.oop_name));
                    } else {
                        ui.label(theme::dim(&p.oop_code));
                    }

                    let fit_col = if p.pos_rating >= 15 {
                        theme::SUCCESS
                    } else if p.pos_rating >= 12 {
                        theme::INFO
                    } else {
                        theme::WARNING
                    };
                    theme::pill(ui, &format!("{} ({})", p.pos_word, p.pos_rating), fit_col);

                    let written_same = state.applied_roles.get(&p.uid) == Some(&(p.ip_bit, p.oop_bit));
                    if written_same {
                        theme::pill(ui, t!("✓ Geschreven", "✓ Written", "✓ Yazıldı"), theme::SUCCESS);
                    } else if p.matched_pos_role.is_some() || p.matched_oop_role.is_some() {
                        theme::pill(ui, t!("Klaar", "Ready", "Hazır"), theme::ACCENT);
                    } else {
                        theme::pill(ui, t!("Wacht op FM-ID's", "Awaiting IDs", "FM verisi bekleniyor"), theme::TEXT_FAINT);
                    }
                    ui.end_row();
                }
            });

        ui.add_space(10.0);
        let can_write_roles = live.connected
            && selected_count > 0
            && (!writable_pos.is_empty() || !writable_oop.is_empty())
            && (state.role_apply_ip || state.role_apply_oop);

        ui.horizontal_wrapped(|ui| {
            ui.label(theme::dim(t!(
                format!("Geselecteerd: {selected_count} / {} spelers", proposals.len()),
                format!("Selected: {selected_count} / {} players", proposals.len()),
                format!("Onaylanan: {selected_count} / {} oyuncu", proposals.len())
            )));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if theme::button(
                    ui,
                    icon::CHECK_CIRCLE,
                    t!(
                        "Goedkeuren & in FM27 zetten",
                        "Approve & Write to FM27",
                        "Onayla ve FM27'ye Yaz"
                    ),
                    can_write_roles,
                )
                .clicked()
                {
                    match write_role_training_command(mgr_id, &writable_pos, &writable_oop) {
                        Ok(n) => {
                            for (uid, ipb, oopb) in pending_records {
                                state.applied_roles.insert(uid, (ipb, oopb));
                            }
                            state.status = t!(
                                format!("Goedgekeurde tactiekrollen ({n} acties) naar FM27 gestuurd…"),
                                format!("Sent approved tactic role training ({n} actions) to FM27…"),
                                format!("Onaylanan taktik rolleri ({n} işlem) FM27'ye gönderildi…")
                            );
                            state.status_error = false;
                        }
                        Err(e) => {
                            state.status = e;
                            state.status_error = true;
                        }
                    }
                }
            });
        });
    });
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

    #[test]
    fn parses_pos_and_oop_role_options_from_bepinex() {
        let sample = json!({
            "session_ready": true,
            "pos_role_options": [
                { "role_id": 43, "pos_id": 11, "name": "Playmaking Winger (AMR)" },
                { "role_id": 12, "pos_id": 0, "name": "Ball-Playing Goalkeeper" }
            ],
            "oop_role_options": {
                "1": "Sweeper Keeper",
                "38": "Winger"
            }
        });
        let parsed = parse_bepinex_individual(&sample, &HashMap::new());
        assert_eq!(parsed.pos_role_options.len(), 2);
        assert_eq!(parsed.pos_role_options[0].role_id, 43);
        assert_eq!(parsed.oop_role_options.get(&38).map(String::as_str), Some("Winger"));
    }

    #[test]
    fn computes_tactic_role_proposals_with_safe_approval_defaults() {
        let gk = mk_player(1, 0, &[]);
        let w = mk_player(2, 11, &[]);
        let td = tactics::Data {
            current: 0,
            tactics: vec![tactics::Tactic {
                index: 0,
                style: "4-3-3".into(),
                slots: vec![
                    tactics::Slot { uid: Some(1), ip: Some(12), oop: Some(1), oop_order: 0 },
                    tactics::Slot { uid: Some(2), ip: Some(43), oop: Some(38), oop_order: 8 },
                ],
            }],
            players: vec![gk, w],
            bench: Vec::new(),
        };
        let mut rids = HashMap::new();
        rids.insert(1, 501);
        rids.insert(2, 502);
        let pos_opts = vec![
            PosRoleOption { role_id: 12, pos_id: 0, name: "Ball-Playing Goalkeeper".into() },
            PosRoleOption { role_id: 43, pos_id: 11, name: "Playmaking Winger".into() },
        ];
        let mut oop_opts = HashMap::new();
        oop_opts.insert(1, "Sweeper Keeper".into());
        oop_opts.insert(38, "Winger".into());

        let props = compute_role_proposals(&td, &rids, &pos_opts, &oop_opts, RoleScope::CurrentEleven);
        assert_eq!(props.len(), 2);
        assert_eq!(props[0].runtime_id, 501);
        assert_eq!(props[0].matched_pos_role.as_ref().map(|x| x.role_id), Some(12));
        assert_eq!(props[1].matched_oop_role.as_ref().map(|x| x.0), Some(38));
    }

    #[test]
    fn unified_player_rows_merge_and_sort_cleanly() {
        let mut live = LiveState::default();
        live.squad = vec![
            json!({ "uid": 10, "runtime_id": 510, "name": "Alpha", "age": 22, "condition": 9400, "sharpness": 8500, "injuries": [] }),
            json!({ "uid": 11, "runtime_id": 511, "name": "Beta", "age": 31, "condition": 7800, "sharpness": 7500, "injuries": [] }),
        ];
        live.tactics = Some(tactics::Data {
            current: 0,
            tactics: Vec::new(),
            players: vec![
                mk_player(10, 12, &[("finishing", 9), ("composure", 10), ("long_shots", 8)]),
                mk_player(11, 3, &[("positioning", 10), ("marking", 10), ("tackling", 11)]),
            ],
            bench: Vec::new(),
        });
        let mut state = IndividualState::default();
        state.bottlenecks.push(PlayerBottleneck {
            runtime_id: 511,
            name: "Beta".into(),
            overall: 52.0,
            understanding: Some(49.0),
            capability: Some(58.0),
            role_suitability: Some(61.0),
            buy_in: Some(65.0),
            weakest_label: "Understanding",
            weakest_score: 49.0,
            advice: "Drill",
        });

        let (mut rows, _) = build_unified_player_rows(&live, &state);
        assert_eq!(rows.len(), 2);
        sort_unified_rows(&mut rows, PlayerSortCol::TacticalScore, true);
        assert_eq!(rows[0].name, "Beta");
        assert_eq!(rows[0].tactical_overall, Some(52.0));
        assert_eq!(rows[0].position, Some("STP"));
        assert_eq!(rows[1].name, "Alpha");
        assert_eq!(rows[1].position, Some("SNT"));

        sort_unified_rows(&mut rows, PlayerSortCol::Condition, false);
        assert_eq!(rows[0].name, "Alpha");
    }
}
