//! De app: zijbalk + pagina's. Alle teksten via `t!` (NL/EN), alle stijl via `theme`.

use anyhow::{Result, anyhow};
use chrono::{Duration, NaiveDate, Weekday};
use eframe::egui::{self, CornerRadius, Frame, Margin, RichText, ScrollArea, Stroke, Ui, vec2};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::path::PathBuf;
use std::sync::mpsc;

use crate::agent;
use crate::export;
use crate::history;
use crate::i18n::{self, Lang};
use crate::individual::{self, IndividualState};
use crate::live::LiveState;
use crate::live_training;
use crate::model::{Component, Effectiveness, Factor, category_of, level_color, level_name, session_by_code};
use crate::planner::{self, CalendarOptions, CalendarWeek, PlannerSettings, WeekKind, WeekPlan, unique_schedules};
use crate::t;
use crate::theme::{self, icon};
use crate::updater::{self, UpdateStatus, UpdaterState};
use crate::vision;

pub fn run() -> Result<()> {
    // testhulp: FM27_WINDOW=1320x1400 voor hogere schermafbeeldingen
    let size = std::env::var("FM27_WINDOW")
        .ok()
        .and_then(|v| {
            let (w, h) = v.split_once('x')?;
            Some([w.trim().parse::<f32>().ok()?, h.trim().parse::<f32>().ok()?])
        })
        .unwrap_or([1320.0, 880.0]);
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("FM27 Training Assistant")
            .with_inner_size(size)
            .with_min_inner_size([1040.0, 660.0])
            .with_icon(eframe::icon_data::from_png_bytes(include_bytes!("../assets/icon/icon_256.png")).unwrap_or_default()),
        vsync: true,
        persist_window: false,
        ..Default::default()
    };
    eframe::run_native(
        "FM27 Training Assistant",
        options,
        Box::new(|cc| {
            theme::install(&cc.egui_ctx);
            Ok(Box::new(App::new()))
        }),
    )
    .map_err(|e| anyhow!("{e}"))
}

// ------------------------------------------------------------------------ instellingen

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
enum UiScale {
    #[default]
    Auto,
    Pct90,
    Pct100,
    Pct115,
    Pct125,
    Pct150,
}

impl UiScale {
    const ALL: [Self; 6] = [
        Self::Auto,
        Self::Pct90,
        Self::Pct100,
        Self::Pct115,
        Self::Pct125,
        Self::Pct150,
    ];

    fn label(self) -> &'static str {
        match self {
            Self::Auto => t!("Automatisch", "Auto", "Otomatik"),
            Self::Pct90 => "90%",
            Self::Pct100 => "100%",
            Self::Pct115 => "115%",
            Self::Pct125 => "125%",
            Self::Pct150 => "150%",
        }
    }

    fn factor(self, unzoomed_logical_w: f32) -> f32 {
        match self {
            Self::Auto => {
                if unzoomed_logical_w > 2800.0 {
                    1.25
                } else if unzoomed_logical_w > 2200.0 {
                    1.15
                } else {
                    1.0
                }
            }
            Self::Pct90 => 0.90,
            Self::Pct100 => 1.0,
            Self::Pct115 => 1.15,
            Self::Pct125 => 1.25,
            Self::Pct150 => 1.50,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct Settings {
    lang: Lang,
    #[allow(dead_code)] // Eski settings.json dosyalarıyla geriye dönük uyumluluk için tutulur.
    rate: f32,
    capture_delay: bool,
    #[serde(default = "yes")]
    auto_training: bool,
    /// Aanpassen in FM toestaan op een build die nog niet live getest is.
    #[serde(default)]
    allow_untested: bool,
    /// Tactisch begrip op 100 % houden zolang de app open is (FM laat het wekelijks zakken).
    #[serde(default)]
    keep_understanding: bool,
    /// De Breakdown elke zoveel seconden zelf opnieuw uitlezen zolang hij in FM open staat
    /// (idee en eerste versie: arhxmmmmm, fork forked-fm27-trainer).
    #[serde(default)]
    auto_read: bool,
    /// Taktik Anlayış (8 alan) + Buy-In verilerini doğrudan FM belleğinden otomatik güncelle.
    #[serde(default)]
    auto_memory_sync: bool,
    /// Hilesiz mod: editör/hile ekranlarını (Kulüp, Kadro, Transfer) gizler ve arka planda %100 kilidini kapatır.
    #[serde(default = "yes")]
    no_cheat_mode: bool,
    /// Kondisyon değiştikçe önerilen sakatlık önleyici bireysel yoğunluğu otomatik uygula.
    #[serde(default)]
    auto_individual_intensity: bool,
    /// Bireysel yoğunluk profili (Korumacı / Dengeli / Gelişim Odaklı).
    #[serde(default)]
    intensity_profile: individual::IntensityProfile,
    #[serde(default = "ten")]
    auto_read_secs: u32,
    /// Bij meer dan één manager in de save: de UID van de club van de manager die de app bedient.
    #[serde(default)]
    manager_club: Option<u32>,
    /// Het team waarvan de app de training plant (plek in de teamlijst van de club; None = eigen team).
    #[serde(default)]
    training_team: Option<usize>,
    /// Otomatik güncelleme: yeni sürüm varsa arka planda indirip uygular.
    #[serde(default = "yes")]
    auto_update: bool,
    /// Taktik rollerini bireysel antrenman olarak önizleyip onaylayarak FM27'ye yazma modu.
    #[serde(default = "yes")]
    role_training_mode: bool,
    /// Arayüz ölçeği (Auto veya sabit yüzde).
    #[serde(default)]
    ui_scale: UiScale,
    /// Altyapı takımlarında Akademi Gelişim Modu.
    #[serde(default = "yes")]
    youth_dev_mode: bool,
    /// Kullanıcı sezon öncesi / kondisyon kampı yoğunluk tarzını en az bir kez seçti mi.
    #[serde(default)]
    preseason_intensity_chosen: bool,
    planner: PlannerSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Settings { lang: i18n::detect(), rate: 1.17, capture_delay: true, auto_training: true, allow_untested: false, keep_understanding: false, auto_read: false, auto_memory_sync: false, no_cheat_mode: true, auto_individual_intensity: false, intensity_profile: individual::IntensityProfile::default(), auto_read_secs: 10, manager_club: None, training_team: None, auto_update: true, role_training_mode: true, ui_scale: UiScale::Auto, youth_dev_mode: true, preseason_intensity_chosen: false, planner: PlannerSettings::default() }
    }
}

fn yes() -> bool {
    true
}

fn ten() -> u32 {
    10
}

/// Zelfde waarden (op een halve punt na) als bij de vorige uitlezing.
fn same_values(a: &Effectiveness, b: &Effectiveness) -> bool {
    Factor::ALL.iter().all(|f| (a.get(*f) - b.get(*f)).abs() < 0.5) && Component::ALL.iter().all(|c| (a.component(*c) - b.component(*c)).abs() < 0.5)
}

fn settings_file() -> PathBuf {
    crate::data_dir().join("settings.json")
}

fn calendar_file_for(team: Option<usize>) -> PathBuf {
    match team {
        None => crate::data_dir().join("calendar.json"),
        Some(i) => crate::data_dir().join(format!("calendar_team_{i}.json")),
    }
}

fn bepinex_dump_file() -> PathBuf {
    crate::data_dir().join("bepinex-tactical-dump.json")
}

fn peek_bepinex_utc() -> Option<String> {
    let text = std::fs::read_to_string(bepinex_dump_file()).ok()?;
    let root: serde_json::Value = serde_json::from_str(&text).ok()?;
    root.get("updated_utc")?.as_str().map(str::to_string)
}

fn team_short_label(t: &Value) -> String {
    let own = t.get("own").and_then(|b| b.as_bool()) == Some(true);
    let kind = t.get("kind").and_then(|v| v.as_u64()).unwrap_or(0);
    let age = t.get("age_limit").and_then(|v| v.as_u64()).unwrap_or(100);
    if own || kind == 0 {
        t!("Eerste elftal", "First Team", "As Takım").to_string()
    } else if (12..=25).contains(&age) {
        format!("U{age}")
    } else {
        t!("Beloften / B", "Reserves / B", "B / Rezerv").to_string()
    }
}

fn demo_teams_payload() -> Value {
    json!({
        "managers": [
            { "club_uid": 1001, "club": "FC Demo", "selected": true }
        ],
        "teams": [
            { "index": 0, "kind": 0, "age_limit": 100, "players": 24, "own": true, "selected": true },
            { "index": 1, "kind": 10, "age_limit": 21, "players": 19, "own": false, "selected": false },
            { "index": 2, "kind": 11, "age_limit": 18, "players": 22, "own": false, "selected": false }
        ]
    })
}

fn find_prefix<'a>(obj: &'a serde_json::Value, prefix: &str) -> Option<&'a serde_json::Value> {
    let map = obj.as_object()?;
    let with_colon = format!("{prefix}:");
    map.iter().find(|(k, _)| k.starts_with(&with_colon) || k.as_str() == prefix).map(|(_, v)| v)
}

fn read_bepinex_dump() -> Option<(Effectiveness, String)> {
    read_bepinex_dump_for(&std::collections::HashSet::new())
}

/// BepInEx eklentisinin yazdığı `bepinex-tactical-dump.json` dosyasından 4 bileşen + 17 faktörü okur.
fn read_bepinex_dump_for(squad_rids: &std::collections::HashSet<u32>) -> Option<(Effectiveness, String)> {
    let text = std::fs::read_to_string(bepinex_dump_file()).ok()?;
    let root: serde_json::Value = serde_json::from_str(&text).ok()?;
    let updated = root.get("updated_utc")?.as_str()?.to_string();
    let data = root.get("data")?.as_object()?;
    let te = individual::pick_tactical_effectiveness_entry(&root, data, squad_rids)?;

    let mut eff = Effectiveness::default();
    eff.overall = find_prefix(te, "OverallEffectiveness").and_then(individual::score_of_node);
    let mut found = 0usize;

    if let Some(tu) = find_prefix(te, "TacticalUnderstanding") {
        if let Some(v) = individual::score_of_node(tu) {
            eff.set_component(Component::TacticalUnderstanding, v);
        }
        let tu_map: [(&str, Factor); 10] = [
            ("InPossessionShape", Factor::InPossessionShape),
            ("PassingStyle", Factor::PassingStyle),
            ("BuildUpStrategy", Factor::BuildUp),
            ("ChanceCreation", Factor::ChanceCreation),
            ("OutOfPossessionShape", Factor::OutOfPossessionShape),
            ("LineOfEngagement", Factor::LineOfEngagement),
            ("PressingIntensity", Factor::PressingIntensity),
            ("PressingTrap", Factor::PressingTrap),
            ("TacticalAbilityKeyFactor", Factor::PlayerTacticalAbility),
            ("TacticalComplexity", Factor::TacticalComplexity),
        ];
        for (key, factor) in tu_map {
            if let Some(v) = find_prefix(tu, key).and_then(individual::score_of_node) {
                eff.set(factor, v);
                found += 1;
            }
        }
    }

    if let Some(pa) = find_prefix(te, "PlayerAbility") {
        if let Some(v) = individual::score_of_node(pa) {
            eff.set_component(Component::PlayerAbility, v);
        }
        for (key, factor) in [
            ("CapabilityKeyFactor", Factor::PlayerCapability),
            ("RoleSuitabilityKeyFactor", Factor::PlayerRoleSuitability),
        ] {
            if let Some(v) = find_prefix(pa, key).and_then(individual::score_of_node) {
                eff.set(factor, v);
                found += 1;
            }
        }
    }

    if let Some(bi) = find_prefix(te, "PlayerBuyIn") {
        if let Some(v) = individual::score_of_node(bi) {
            eff.set_component(Component::PlayerBuyIn, v);
        }
        for (key, factor) in [
            ("ManagerSupportKeyFactor", Factor::ManagerialSupport),
            ("TeammateAndStaffRelationshipsKeyFactor", Factor::TeammatesAndStaffRelationships),
            ("ManagerTendenciesKeyFactor", Factor::ManagerTendencies),
        ] {
            if let Some(v) = find_prefix(bi, key).and_then(individual::score_of_node) {
                eff.set(factor, v);
                found += 1;
            }
        }
    }

    if let Some(nm) = find_prefix(te, "NextMatchInformation") {
        if let Some(v) = individual::score_of_node(nm) {
            eff.set_component(Component::NextMatchInfo, v);
        }
        for (key, factor) in [
            ("QualityOfReportKeyFactor", Factor::QualityOfNextOppositionReport),
            ("OppositionPlayerKnowledgeKeyFactor", Factor::OppositionPlayerKnowledge),
        ] {
            if let Some(v) = find_prefix(nm, key).and_then(individual::score_of_node) {
                eff.set(factor, v);
                found += 1;
            }
        }
    }

    if found < 8 {
        return None;
    }
    let when = chrono::Local::now().format("%d-%m %H:%M");
    eff.source = format!("FM27 · {when}");
    Some((eff, updated))
}

// ------------------------------------------------------------------------ app

#[derive(PartialEq, Clone, Copy)]
enum Page {
    Home,
    Overview,
    Calendar,
    Individual,
    Tactics,
    Settings,
}

#[derive(PartialEq, Clone, Copy)]
enum PlanTab {
    Week,
    Season,
}

#[derive(PartialEq, Clone, Copy)]
enum FactorSort {
    ByScore,
    ByComponent,
}

#[derive(PartialEq, Eq, Clone, Copy, Default)]
enum SettingsSection {
    #[default]
    General,
    Planner,
    FmConnection,
    Updates,
    Folders,
}

type AnalysisResult = Result<(vision::Analysis, String, Option<NaiveDate>), String>;

struct App {
    page: Page,
    /// de pagina die het vorige beeld toonde (voor de intrede-animatie)
    shown_page: Option<Page>,
    settings: Settings,
    applied_scale_pref: Option<UiScale>,
    last_auto_zoom: f32,
    settings_section: SettingsSection,
    settings_scroll_to: Option<SettingsSection>,
    show_adv_thresholds: bool,
    eff: Effectiveness,
    status: String,
    status_error: bool,
    analysis_rx: Option<mpsc::Receiver<AnalysisResult>>,
    /// de lopende uitlezing is door automatisch uitlezen gestart (stil, haalt FM niet naar voren)
    auto_pending: bool,
    auto_fallback_file: bool,
    auto_note: String,
    last_auto: std::time::Instant,
    image_path: String,
    show_file: bool,
    edit_mode: bool,
    show_setup: bool,
    plan_tab: PlanTab,
    factor_sort: FactorSort,
    week_start: String,
    week_matches: [bool; 7],
    prev_sunday: bool,
    next_monday: bool,
    week_kind: WeekKind,
    week_plan: Option<WeekPlan>,
    cal_opts: CalendarOptions,
    year_start: String,
    calendar: Vec<CalendarWeek>,
    year: Vec<WeekPlan>,
    live: LiveState,
    /// Son bellek senkronizasyon sürümü (live.memory_rev ile karşılaştırılır)
    last_memory_rev: u64,
    /// Kullanıcı "FM belleğinden oku" butonuna bastıysa yenileme sonrası zorla senkronize et
    memory_sync_pending: bool,
    /// Son okunan BepInEx dump zaman damgası
    last_bepinex_utc: String,
    last_bepinex_check: std::time::Instant,
    individual: IndividualState,
    updater: UpdaterState,
    show_camp_modal: bool,
    camp_modal_draft: planner::PreSeasonIntensity,
    camp_prompted_this_session: bool,
    history: history::HistoryStore,
    /// de tactiek die de tactiekpagina toont (None: de tactiek die in FM in gebruik is)
    tactic_pick: Option<usize>,
    tactics_asked: bool,
    frames: u32,
}

impl App {
    fn new() -> Self {
        let mut settings: Settings = std::fs::read_to_string(settings_file()).ok().and_then(|s| serde_json::from_str(&s).ok()).unwrap_or_default();
        match std::env::var("FM27_LANG").as_deref() {
            Ok("nl") => settings.lang = Lang::Nl,
            Ok("en") => settings.lang = Lang::En,
            Ok("tr") => settings.lang = Lang::Tr,
            Ok("de") => settings.lang = Lang::De,
            Ok("fr") => settings.lang = Lang::Fr,
            Ok("it") => settings.lang = Lang::It,
            Ok("es") => settings.lang = Lang::Es,
            _ => {}
        }
        i18n::set(settings.lang);
        let active_team = settings.training_team;
        let cal_opts = CalendarOptions::default();
        let calendar = std::fs::read_to_string(calendar_file_for(active_team))
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_else(|| planner::default_calendar(&cal_opts));
        let monday = planner::monday_of(chrono::Local::now().date_naive());
        let mut week_matches = [false; 7];
        week_matches[6] = true;
        // FM27_PAGE=home|overview|training|planning|calendar|season|individual|players|tactics|settings opent direct die pagina (handig bij testen)
        let page_env = std::env::var("FM27_PAGE").unwrap_or_default();
        let season_page = page_env == "season";
        let page = match page_env.as_str() {
            "overview" | "analysis" => Page::Overview,
            "training" | "planning" | "calendar" | "season" => Page::Calendar,
            "individual" | "players" => Page::Individual,
            "tactics" => Page::Tactics,
            "settings" => Page::Settings,
            "home" | "" => Page::Home,
            _ => Page::Home,
        };
        let mut individual = IndividualState::default();
        individual.auto_apply_intensity = settings.auto_individual_intensity;
        individual.profile = settings.intensity_profile;
        individual.role_training_mode = settings.role_training_mode;
        let show_camp_modal = std::env::var_os("FM27_CAMP_MODAL").is_some();
        let camp_modal_draft = settings.planner.preseason_intensity;
        App {
            page,
            shown_page: None,
            eff: crate::load_effectiveness_for(active_team).unwrap_or_default(),
            history: history::HistoryStore::load(),
            applied_scale_pref: None,
            last_auto_zoom: 1.0,
            settings_section: SettingsSection::General,
            settings_scroll_to: None,
            show_adv_thresholds: false,
            status: String::new(),
            status_error: false,
            analysis_rx: None,
            auto_pending: false,
            auto_fallback_file: false,
            auto_note: String::new(),
            last_auto: std::time::Instant::now(),
            image_path: String::new(),
            show_file: false,
            edit_mode: false,
            show_setup: std::env::var_os("FM27_SETUP").is_some(),
            plan_tab: if season_page { PlanTab::Season } else { PlanTab::Week },
            factor_sort: FactorSort::ByScore,
            week_start: monday.format("%Y-%m-%d").to_string(),
            week_matches,
            prev_sunday: true,
            next_monday: false,
            week_kind: WeekKind::Season,
            week_plan: None,
            year_start: cal_opts.start.format("%Y-%m-%d").to_string(),
            cal_opts,
            calendar,
            year: Vec::new(),
            tactic_pick: None,
            tactics_asked: false,
            live: {
                let mut l = LiveState::default();
                l.sell.buy = std::env::var("FM27_PAGE").as_deref() == Ok("buy");
                l.youth_dev_mode = settings.youth_dev_mode;
                l.choice = (settings.manager_club, active_team);
                if std::env::var_os("FM27_SNAPSHOT").is_some() && std::env::var_os("FM27_DEMO_TEAMS").is_some() {
                    l.teams = Some(demo_teams_payload());
                }
                l
            },
            settings,
            last_memory_rev: 0,
            memory_sync_pending: false,
            last_bepinex_utc: String::new(),
            last_bepinex_check: std::time::Instant::now() - std::time::Duration::from_secs(10),
            individual,
            updater: UpdaterState::default(),
            show_camp_modal,
            camp_modal_draft,
            camp_prompted_this_session: false,
            frames: 0,
        }
    }

    /// Testhulp: FM27_SNAPSHOT=pad.png legt na het laden het eigen venster vast en sluit af.
    fn snapshot(&mut self, ctx: &egui::Context) {
        let Ok(path) = std::env::var("FM27_SNAPSHOT") else { return };
        self.frames += 1;
        if self.frames == 90 {
            ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
        }
        let shot = ctx.input(|i| {
            i.events.iter().find_map(|e| match e {
                egui::Event::Screenshot { image, .. } => Some(image.clone()),
                _ => None,
            })
        });
        if let Some(img) = shot {
            let [w, h] = img.size;
            let bytes: Vec<u8> = img.pixels.iter().flat_map(|c| c.to_array()).collect();
            if let Some(buf) = image::RgbaImage::from_raw(w as u32, h as u32, bytes) {
                let _ = buf.save(&path);
            }
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        ctx.request_repaint();
    }

    fn save_settings(&self) {
        let _ = std::fs::create_dir_all(crate::data_dir());
        if let Ok(s) = serde_json::to_string_pretty(&self.settings) {
            let _ = std::fs::write(settings_file(), s);
        }
    }

    fn active_team_slot(&self) -> Option<usize> {
        self.settings.training_team
    }

    fn active_team_short_label(&self) -> Option<String> {
        let slot = self.active_team_slot()?;
        if let Some(teams) = self.live.teams.as_ref().and_then(|d| d.get("teams")).and_then(|v| v.as_array())
            && let Some(t) = teams.iter().find(|x| x.get("index").and_then(|v| v.as_u64()) == Some(slot as u64))
        {
            return Some(team_short_label(t));
        }
        Some(format!("Team{}", slot + 1))
    }

    fn save_calendar(&self) {
        let _ = std::fs::create_dir_all(crate::data_dir());
        if let Ok(s) = serde_json::to_string_pretty(&self.calendar) {
            let _ = std::fs::write(calendar_file_for(self.active_team_slot()), s);
        }
    }

    fn set_status(&mut self, msg: impl Into<String>, error: bool) {
        self.status = msg.into();
        self.status_error = error;
    }

    fn has_data(&self) -> bool {
        !self.eff.source.is_empty()
    }

    fn last_read_hhmm(&self) -> Option<&str> {
        let s = self.eff.source.trim().trim_end_matches('*').trim();
        let tok = s.split_whitespace().last()?;
        let b = tok.as_bytes();
        if b.len() == 5 && b[2] == b':' && b[0].is_ascii_digit() && b[1].is_ascii_digit() && b[3].is_ascii_digit() && b[4].is_ascii_digit() {
            Some(tok)
        } else {
            None
        }
    }

    fn club_history_key(&self) -> String {
        let base = self
            .live
            .club
            .as_ref()
            .and_then(|c| c.get("uid"))
            .and_then(|v| v.as_u64())
            .map(|u| u.to_string())
            .unwrap_or_else(|| "default".to_string());
        match self.active_team_short_label() {
            Some(lbl) => format!("{base} · {lbl}"),
            None => base,
        }
    }

    fn switch_team_context(&mut self, ctx: &egui::Context, club: Option<u32>, team: Option<usize>) {
        if self.has_data() {
            let _ = crate::save_effectiveness_for(self.active_team_slot(), &self.eff);
        }
        self.save_calendar();

        let next_team = team;
        self.settings.manager_club = club;
        self.settings.training_team = next_team;
        self.save_settings();

        self.eff = crate::load_effectiveness_for(next_team).unwrap_or_default();
        self.calendar = std::fs::read_to_string(calendar_file_for(next_team))
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_else(|| planner::default_calendar(&self.cal_opts));
        self.last_bepinex_utc = peek_bepinex_utc().unwrap_or_default();
        self.week_plan = None;
        self.year.clear();
        self.tactic_pick = None;
        self.tactics_asked = false;
        self.individual.bottlenecks.clear();
        self.live.choose(ctx, (club, next_team));
    }

    // -------------------------------------------------------------------- analyse

    fn start_analysis(&mut self, ctx: &egui::Context, file: Option<PathBuf>) {
        let (tx, rx) = mpsc::channel();
        let delay = self.settings.capture_delay && file.is_none() && !self.auto_pending;
        let ctx2 = ctx.clone();
        std::thread::spawn(move || {
            if delay && cfg!(target_os = "macos") {
                let _ = std::process::Command::new("open").args(["-b", "com.SportsInteractive.FootballManager27"]).status();
                std::thread::sleep(std::time::Duration::from_millis(1500));
            }
            let base = match file {
                Some(p) => vision::analyze_file(&p).map(|(a, _)| (a, p.file_name().map(|f| f.to_string_lossy().into_owned()).unwrap_or_default())).map_err(|e| format!("{e:#}")),
                None => vision::analyze_screen().map(|(a, _)| (a, t!("FM27-scherm", "FM27 screen").to_string())).map_err(|e| format!("{e:#}")),
            };
            let res: AnalysisResult = base.map(|(a, src)| {
                let game_date = agent::request(json!({"cmd": "game_date"})).ok().and_then(|v| {
                    let y = v.get("year").and_then(|x| x.as_i64())?;
                    let d = v.get("doy").and_then(|x| x.as_u64())?;
                    NaiveDate::from_yo_opt(y as i32, d as u32)
                });
                (a, src, game_date)
            });
            let _ = tx.send(res);
            ctx2.request_repaint();
        });
        self.analysis_rx = Some(rx);
        if !self.auto_pending {
            self.set_status(t!("Bezig met uitlezen…", "Reading…"), false);
        }
    }

    fn poll_analysis(&mut self, ctx: &egui::Context) {
        let Some(rx) = &self.analysis_rx else { return };
        if let Ok(res) = rx.try_recv() {
            self.analysis_rx = None;
            let _ = ctx;
            let auto = std::mem::take(&mut self.auto_pending);
            match res {
                // automatisch: alleen een volledige, zekere uitlezing telt, en alleen als er iets veranderd is
                Ok((a, _, _)) if auto && (a.gauges.len() < 17 || a.components.len() < 4 || a.gauges.iter().any(|g| g.doubt)) => {
                    self.auto_note = t!("wacht op de Breakdown in FM", "waiting for the Breakdown in FM").to_string();
                }
                Ok((a, _, _)) if auto && same_values(&self.eff, &a.to_effectiveness("")) => {
                    self.auto_note = t!(format!("geen verandering ({})", chrono::Local::now().format("%H:%M:%S")), format!("no change ({})", chrono::Local::now().format("%H:%M:%S"))).to_string();
                }
                Err(_) if auto => {
                    self.auto_note = t!("wacht op de Breakdown in FM", "waiting for the Breakdown in FM").to_string();
                }
                Ok((a, src, game_date)) => {
                    self.auto_fallback_file = false;
                    if auto {
                        self.auto_note = t!(format!("bijgewerkt ({})", chrono::Local::now().format("%H:%M:%S")), format!("updated ({})", chrono::Local::now().format("%H:%M:%S"))).to_string();
                    }
                    let src_is_screen = src == t!("FM27-scherm", "FM27 screen");
                    let when = chrono::Local::now().format("%d-%m %H:%M");
                    self.eff = a.to_effectiveness(&format!("{src} · {when}"));
                    // spel-datum erbij, zodat de app weet hoe oud de meting in speltijd is
                    if let Some(gd) = game_date {
                        self.eff.game_date = Some(gd);
                    }
                    let _ = crate::save_effectiveness_for(self.active_team_slot(), &self.eff);
                    let hkey = self.club_history_key();
                    if self.history.record(&hkey, &self.eff) {
                        self.history.save();
                    }
                    let mut msg = t!(format!("{} van 17 meters gelezen.", a.gauges.len()), format!("Read {} of 17 gauges.", a.gauges.len())).to_string();
                    if !a.lang.starts_with("english") {
                        msg.push_str(&format!(" (FM: {})", a.lang));
                    }
                    // een meting die niet bij de kleur van de meter past: laten nakijken
                    let doubts: Vec<&str> = a.gauges.iter().filter(|g| g.doubt).map(|g| g.factor.display_label()).collect();
                    if !doubts.is_empty() {
                        msg.push(' ');
                        msg.push_str(&t!(
                            format!("Kijk na (kleur en vulling kloppen niet): {}. Pas aan met 'Waarden handmatig aanpassen'.", doubts.join(", ")),
                            format!("Please check (colour and fill disagree): {}. Fix them with 'Adjust values manually'.", doubts.join(", "))
                        ));
                    }
                    let incomplete = a.gauges.len() < 17 || a.components.len() < 4 || !doubts.is_empty();
                    if incomplete && src_is_screen {
                        // zonder de opname is een verkeerde uitlezing niet na te spelen
                        msg.push(' ');
                        msg.push_str(t!(
                            "Klopt het niet? Stuur last-breakdown.png en last-analysis.txt uit de app-map (Instellingen → Open app-map) mee met je melding.",
                            "Not right? Attach last-breakdown.png and last-analysis.txt from the app folder (Settings → Open app folder) to your report."
                        ));
                    }
                    self.set_status(msg, incomplete);
                    self.week_plan = None;
                    self.year.clear();
                }
                Err(e) => {
                    if std::mem::take(&mut self.auto_fallback_file) && !auto {
                        self.show_file = true;
                        self.set_status(
                            t!(
                                "Kies hieronder een schermafbeelding (PNG) om uit bestand te lezen.",
                                "Select a Breakdown screenshot (PNG) below to read from file.",
                                "Dosyadan okumak için aşağıdan bir ekran görüntüsü (PNG) seçin."
                            ),
                            false,
                        );
                    } else {
                        self.set_status(e, true);
                    }
                }
            }
        }
    }

    /// Veri kaynağını otomatik seçer: önce oyun eklentisi, yoksa ekran, o da yoksa dosya.
    fn refresh_data_auto(&mut self, ctx: &egui::Context) {
        if read_bepinex_dump().is_some() {
            self.sync_from_bepinex(true);
        } else if self.live.connected {
            self.last_auto = std::time::Instant::now();
            self.auto_fallback_file = true;
            self.start_analysis(ctx, None);
        } else if !self.image_path.trim().is_empty() {
            let p = PathBuf::from(self.image_path.trim().trim_matches('\''));
            self.start_analysis(ctx, Some(p));
        } else {
            self.last_auto = std::time::Instant::now();
            self.auto_fallback_file = true;
            self.start_analysis(ctx, None);
        }
    }

    // -------------------------------------------------------------------- shell

    fn apply_ui_scale(&mut self, ctx: &egui::Context) {
        let minimized = ctx.input(|i| i.viewport().minimized == Some(true));
        let unzoomed_w = ctx.input(|i| {
            i.viewport()
                .inner_rect
                .map(|r| r.width())
                .unwrap_or_else(|| i.screen_rect().width() * ctx.zoom_factor())
        });
        if minimized || unzoomed_w < 100.0 {
            return;
        }
        let pref = self.settings.ui_scale;
        if self.applied_scale_pref != Some(pref) {
            let z = pref.factor(unzoomed_w);
            ctx.set_zoom_factor(z);
            self.applied_scale_pref = Some(pref);
            if pref == UiScale::Auto {
                self.last_auto_zoom = z;
            }
        } else if pref == UiScale::Auto {
            let z = pref.factor(unzoomed_w);
            if (z - self.last_auto_zoom).abs() > 0.001 {
                ctx.set_zoom_factor(z);
                self.last_auto_zoom = z;
            }
        }
    }

    fn topbar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("topbar")
            .show_separator_line(true)
            .frame(Frame::new().fill(theme::SIDEBAR).inner_margin(Margin { left: 28, right: 28, top: 10, bottom: 10 }))
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                theme::centered_column(ui, theme::CONTENT_MAX, |ui| {
                    ui.allocate_ui(vec2(ui.available_width(), 40.0), |ui| {
                        ui.horizontal_centered(|ui| {
                            let compact = ui.available_width() < 1160.0;
                            theme::brand_logo(ui, if compact { 30.0 } else { 34.0 });
                            ui.add_space(4.0);
                            ui.allocate_ui_with_layout(
                                vec2(if compact { 125.0 } else { 185.0 }, if compact { 20.0 } else { 34.0 }),
                                egui::Layout::top_down(egui::Align::Min),
                                |ui| {
                                    ui.spacing_mut().item_spacing = vec2(4.0, 1.0);
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            RichText::new(if compact { "FM27" } else { "FM27 Training" })
                                                .font(egui::FontId::new(if compact { 13.0 } else { 13.5 }, theme::bold()))
                                                .color(theme::TEXT),
                                        );
                                        ui.label(
                                            RichText::new("Assistant")
                                                .font(egui::FontId::new(if compact { 12.5 } else { 13.0 }, theme::semibold()))
                                                .color(theme::ACCENT_HI),
                                        );
                                    });
                                    if !compact {
                                        ui.label(
                                            RichText::new(format!("v{}", env!("CARGO_PKG_VERSION")))
                                                .size(10.5)
                                                .color(theme::TEXT_FAINT),
                                        );
                                    }
                                },
                            );

                            ui.add_space(if compact { 6.0 } else { 12.0 });
                            let items = [
                                (Page::Home, icon::HOUSE, t!("Deze week", "This week", "Bu hafta")),
                                (Page::Overview, icon::GAUGE, t!("Teamanalyse", "Team analysis", "Takım analizi")),
                                (Page::Calendar, icon::CALENDAR_DOTS, t!("Kalender", "Calendar", "Takvim")),
                                (Page::Individual, icon::USERS_THREE, t!("Spelers", "Players", "Oyuncular")),
                                (Page::Tactics, icon::STRATEGY, t!("Tactiek", "Tactics", "Taktik")),
                                (Page::Settings, icon::GEAR_SIX, t!("Instellingen", "Settings", "Ayarlar")),
                            ];
                            for (p, ic, label) in items {
                                if theme::nav_tab(ui, ic, label, self.page == p).clicked() {
                                    self.page = p;
                                }
                                if !compact {
                                    ui.add_space(2.0);
                                }
                            }

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                let tip = t!(
                                    "Steun via Buy Me a Coffee (buymeacoffee.com/cilek14)",
                                    "Support on Buy Me a Coffee (buymeacoffee.com/cilek14)",
                                    "Buy Me a Coffee ile destek ol (buymeacoffee.com/cilek14)"
                                );
                                let sup_lbl = if compact { "" } else { t!("Steun", "Support", "Destek ol") };
                                if theme::support_button(ui, icon::COFFEE, sup_lbl, false)
                                    .on_hover_text(tip)
                                    .clicked()
                                {
                                    open_path(std::path::Path::new(DONATE_URL));
                                }

                                ui.add_space(4.0);
                                let (c, txt) = if !agent::LIVE_SUPPORTED {
                                    (theme::TEXT_FAINT, t!("Live: alleen macOS", "Live: macOS only").to_string())
                                } else if self.live.connected {
                                    (theme::SUCCESS, self.live.club_name().unwrap_or_else(|| t!("Verbonden met FM27", "Connected to FM27").to_string()))
                                } else {
                                    (theme::TEXT_FAINT, t!("Niet verbonden", "Not connected").to_string())
                                };
                                let hhmm = if compact { None } else { self.last_read_hhmm().map(str::to_string) };
                                let resp = theme::status_pill(ui, c, &txt, hhmm.as_deref());
                                if self.has_data() {
                                    resp.on_hover_text(t!(
                                        format!("Laatste meting: {}", self.eff.source),
                                        format!("Last reading: {}", self.eff.source),
                                        format!("Son okuma: {}", self.eff.source)
                                    ));
                                }

                                if let Some(ver) = self.updater.available_version().map(str::to_string) {
                                    ui.add_space(4.0);
                                    if theme::small_button(ui, &format!("↑ v{ver}"), true).clicked() {
                                        self.page = Page::Settings;
                                    }
                                }
                            });
                        });
                    });
                });
            });
    }

    fn status_line(&self, ui: &mut Ui, msg: &str, error: bool) {
        if msg.is_empty() {
            return;
        }
        let (c, ic) = if error { (theme::DANGER, icon::WARNING) } else { (theme::INFO, icon::CHECK_CIRCLE) };
        Frame::new().fill(theme::tint(c, 0.12)).stroke(Stroke::new(1.0_f32, theme::tint(c, 0.4))).corner_radius(CornerRadius::same(10)).inner_margin(Margin::symmetric(12, 8)).show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal_wrapped(|ui| {
                ui.label(theme::ic(ic).color(c));
                ui.label(RichText::new(msg).color(theme::TEXT));
            });
        });
        ui.add_space(12.0);
    }

    fn render_global_strip(&mut self, ui: &mut Ui) {
        if self.live.connected && self.has_data() {
            return;
        }
        let col = if !self.live.connected { theme::WARNING } else { theme::INFO };
        let msg = if !self.live.connected && !self.has_data() {
            t!(
                "FM27 niet verbonden en nog geen gegevens gelezen.",
                "FM27 not connected and no data read yet.",
                "FM27 bağlı değil ve henüz veri okunmadı."
            )
        } else if !self.live.connected {
            t!(
                "FM27 niet verbonden — open het spel voor livefuncties.",
                "FM27 not connected — open the game for live features.",
                "FM27 bağlı değil — canlı özellikler için oyunu açın."
            )
        } else {
            t!(
                "Tactical Effectiveness is nog niet uitgelezen.",
                "Tactical Effectiveness has not been read yet.",
                "Taktiksel Verimlilik henüz okunmadı."
            )
        };
        Frame::new()
            .fill(theme::tint(col, 0.10))
            .stroke(Stroke::new(1.0_f32, theme::tint(col, 0.38)))
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::symmetric(14, 7))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.horizontal(|ui| {
                    theme::status_dot(ui, col);
                    ui.add(egui::Label::new(RichText::new(msg).size(13.0).color(theme::TEXT)).truncate());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let btn_lbl = if self.show_setup {
                            t!("Sluiten", "Close", "Kapat")
                        } else {
                            t!("Hoe verbind ik?", "How do I connect?", "Nasıl bağlanırım?")
                        };
                        if theme::small_button(ui, btn_lbl, true).clicked() {
                            self.show_setup = !self.show_setup;
                        }
                    });
                });
            });
        ui.add_space(10.0);
    }

    fn render_setup_flow(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        let step1_done = self.live.connected;
        let step2_done = self.live.connected && self.live.club.is_some();
        let step3_done = self.has_data();
        let mut trigger_start = false;
        let mut trigger_reconnect = false;
        let mut trigger_refresh = false;
        let mut close = false;
        let windows = cfg!(windows);
        let live_supported = agent::LIVE_SUPPORTED;
        let busy = self.live.busy() || self.analysis_rx.is_some();

        theme::card_accent(ui, theme::ACCENT, |ui| {
            ui.horizontal(|ui| {
                ui.label(theme::ic(icon::PLUGS_CONNECTED).color(theme::ACCENT_HI).size(18.0));
                ui.label(theme::h2(t!("Verbinden met FM27", "Connect to FM27", "FM27'ye bağlanma adımları")));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if theme::small_button(ui, t!("Sluiten", "Close", "Kapat"), true).clicked() {
                        close = true;
                    }
                });
            });
            ui.add_space(8.0);
            if !live_supported {
                ui.label(theme::dim(t!(
                    "Live-verbinding is op dit systeem niet beschikbaar; uitlezen en plannen werken wel.",
                    "Live connection is unavailable on this system; reading and planning still work.",
                    "Canlı bağlantı bu sistemde kullanılamıyor; okuma ve planlama özellikleri yine çalışır."
                )));
            }
            let draw_step1 = |ui: &mut Ui, trigger_start: &mut bool, trigger_reconnect: &mut bool| {
                theme::step_card_status(
                    ui,
                    1,
                    step1_done,
                    icon::PLAY,
                    t!("Start FM27", "Start FM27", "FM27'yi başlat"),
                    |ui| {
                        ui.add(
                            egui::Label::new(theme::dim(if windows {
                                t!(
                                    "Start FM27 via Steam, Game Pass of Epic.",
                                    "Launch FM27 via Steam, Game Pass or Epic.",
                                    "FM27'yi Steam, Game Pass veya Epic üzerinden başlat."
                                )
                            } else {
                                t!(
                                    "Start FM27 met de module via Steam.",
                                    "Launch FM27 with the module via Steam.",
                                    "Steam açıkken FM27'yi modül ile başlat."
                                )
                            }))
                            .wrap(),
                        );
                        if !step1_done && live_supported {
                            ui.add_space(6.0);
                            ui.horizontal_wrapped(|ui| {
                                let start_lbl = if windows {
                                    t!("Start FM27", "Start FM27", "FM27'yi başlat")
                                } else {
                                    t!("Start FM met module", "Start FM with module", "FM27'yi modülle başlat")
                                };
                                if theme::small_button(ui, start_lbl, !busy).clicked() {
                                    *trigger_start = true;
                                }
                                if theme::small_button(ui, t!("Opnieuw verbinden", "Reconnect", "Yeniden bağlan"), !busy).clicked() {
                                    *trigger_reconnect = true;
                                }
                            });
                        }
                    },
                );
            };
            let draw_step2 = |ui: &mut Ui| {
                theme::step_card_status(
                    ui,
                    2,
                    step2_done,
                    icon::FOLDER_OPEN,
                    t!("Laad je save", "Load your save", "Kariyerini yükle"),
                    |ui| {
                        ui.add(
                            egui::Label::new(theme::dim(t!(
                                "Laad je carrière; de app verbindt vanzelf.",
                                "Load your save; the app connects automatically.",
                                "Kariyerini yükle; uygulama oyuna otomatik bağlanır."
                            )))
                            .wrap(),
                        );
                    },
                );
            };
            let draw_step3 = |ui: &mut Ui, trigger_refresh: &mut bool| {
                theme::step_card_status(
                    ui,
                    3,
                    step3_done,
                    icon::GAUGE,
                    t!("Open Tactical Effectiveness", "Open Tactical Effectiveness", "Taktiksel Verimlilik ekranını aç"),
                    |ui| {
                        ui.add(
                            egui::Label::new(theme::dim(t!(
                                "Open het scherm in FM27 en klik op Verileri güncelle.",
                                "Open Tactical Effectiveness in FM27 and refresh data.",
                                "FM27'de Taktiksel Verimlilik ekranını aç ve Verileri güncelle'ye tıkla."
                            )))
                            .wrap(),
                        );
                        ui.add_space(6.0);
                        ui.horizontal(|ui| {
                            if theme::small_button(ui, t!("Gegevens vernieuwen", "Refresh data", "Verileri güncelle"), !busy).clicked() {
                                *trigger_refresh = true;
                            }
                        });
                    },
                );
            };
            if ui.available_width() >= 900.0 {
                ui.columns(3, |cols| {
                    draw_step1(&mut cols[0], &mut trigger_start, &mut trigger_reconnect);
                    draw_step2(&mut cols[1]);
                    draw_step3(&mut cols[2], &mut trigger_refresh);
                });
            } else {
                draw_step1(ui, &mut trigger_start, &mut trigger_reconnect);
                ui.add_space(8.0);
                draw_step2(ui);
                ui.add_space(8.0);
                draw_step3(ui, &mut trigger_refresh);
            }
            ui.add_space(6.0);
            ui.label(theme::faint(t!(
                "Alleen training: de app past uitsluitend het trainingsschema aan, zonder cheats.",
                "Training-only: only training schedules and options are written, zero memory cheats.",
                "Yalnızca antrenman: editör veya hile içermez, yalnızca antrenman ayarlarını yazar."
            )));
        });
        ui.add_space(12.0);

        if trigger_start {
            self.live.start_fm(ctx);
        }
        if trigger_reconnect {
            self.live.refresh(ctx, t!("verbinden", "connect"), None);
        }
        if trigger_refresh {
            self.refresh_data_auto(ctx);
        }
        if close {
            self.show_setup = false;
        }
    }

    /// Tekil sayfa boş durumu (bağlantı yoksa tekrarlanan kart yerine global şerit rehberlik eder).
    fn empty_connect_card(&self, ui: &mut Ui, title: &str, desc: &str) {
        theme::card(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(theme::ic(icon::PLUGS_CONNECTED).size(24.0).color(theme::ACCENT));
                ui.vertical(|ui| {
                    ui.label(theme::h2(title));
                    ui.add(egui::Label::new(theme::dim(desc)).wrap());
                });
            });
        });
    }

    /// BepInEx köprüsünden (bepinex-tactical-dump.json) 4 bileşen + 17 faktörü kesin değerlerle yükler.
    fn sync_from_bepinex(&mut self, explicit: bool) {
        let mut names = std::collections::HashMap::new();
        let mut uid_to_rid = std::collections::HashMap::new();
        for p in &self.live.squad {
            let rid = p.get("runtime_id").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
            if rid == 0 {
                continue;
            }
            if let Some(uid) = p.get("uid").and_then(|v| v.as_u64()) {
                uid_to_rid.insert(uid as u32, rid);
            }
            if let Some(n) = p.get("name").and_then(|v| v.as_str()).filter(|s| !s.trim().is_empty()) {
                names.insert(rid, n.to_string());
            }
        }
        if let Some(td) = &self.live.tactics {
            for tp in &td.players {
                if let Some(&rid) = uid_to_rid.get(&tp.uid) {
                    if !tp.name.trim().is_empty() {
                        names.entry(rid).or_insert_with(|| tp.name.clone());
                    }
                }
            }
        }
        let squad_rids: std::collections::HashSet<u32> = names.keys().copied().collect();

        let Some((mut next, updated)) = read_bepinex_dump_for(&squad_rids) else {
            if explicit {
                self.set_status(
                    t!(
                        "Geen gegevens van spelmodule gevonden. Open in FM27 eenmaal Tactical Effectiveness.",
                        "No game plugin data found. Open Tactical Effectiveness once in FM27.",
                        "Oyun eklentisi verisi bulunamadı. FM27'de Taktiksel Verimlilik ekranını bir kez açın."
                    ),
                    true,
                );
            }
            return;
        };
        if !explicit && updated == self.last_bepinex_utc {
            // Kadro isimleri sonradan yüklendiyse eksik oyuncu isimlerini tamamla
            if !self.individual.bottlenecks.is_empty() && !names.is_empty() {
                for b in &mut self.individual.bottlenecks {
                    if b.name.starts_with("Oyuncu #") || b.name.starts_with("Player #") || b.name == "—" {
                        if let Some(n) = names.get(&b.runtime_id) {
                            b.name = n.clone();
                        }
                    }
                }
            }
            return;
        }
        self.last_bepinex_utc = updated;
        if let Ok(text) = std::fs::read_to_string(bepinex_dump_file()) {
            if let Ok(root) = serde_json::from_str::<serde_json::Value>(&text) {
                let parsed = individual::parse_bepinex_individual(&root, &names);
                self.individual.bottlenecks = parsed.bottlenecks;
                for (rid, lvl) in parsed.intensities {
                    self.individual.current_intensities.insert(rid, lvl);
                }
                for (rid, fid) in parsed.current_focuses {
                    self.individual.current_focuses.insert(rid, fid);
                }
                for (rid, pr) in parsed.current_pos_roles {
                    self.individual.current_pos_roles.insert(rid, pr);
                }
                for (rid, oop) in parsed.current_oop_roles {
                    self.individual.current_oop_roles.insert(rid, oop);
                }
                for (id, name) in parsed.focus_options {
                    self.individual.focus_options.insert(id, name);
                }
                for opt in parsed.pos_role_options {
                    if let Some(existing) = self
                        .individual
                        .pos_role_options
                        .iter_mut()
                        .find(|x| x.pos_id == opt.pos_id && x.role_id == opt.role_id)
                    {
                        existing.name = opt.name;
                    } else {
                        self.individual.pos_role_options.push(opt);
                    }
                }
                for (id, name) in parsed.oop_role_options {
                    self.individual.oop_role_options.insert(id, name);
                }
                self.individual.session_ready = parsed.session_ready;
            }
        }
        let changed = !same_values(&self.eff, &next);
        if let Some(d) = self.live.club.as_ref().and_then(|c| c.get("today")) {
            let y = d.get("year").and_then(|x| x.as_i64());
            let doy = d.get("doy").and_then(|x| x.as_u64());
            if let (Some(y), Some(doy)) = (y, doy) {
                next.game_date = NaiveDate::from_yo_opt(y as i32, doy as u32);
            }
        }
        self.eff = next;
        let _ = crate::save_effectiveness_for(self.active_team_slot(), &self.eff);
        let hkey = self.club_history_key();
        if self.history.record(&hkey, &self.eff) {
            self.history.save();
        }
        if changed {
            self.week_plan = None;
            self.year.clear();
        }
        if explicit || changed {
            self.set_status(
                t!(
                    "Alle 17 factoren exact uit de spelmodule gelezen.",
                    "Read all 17 exact factors from game plugin.",
                    "17 faktör ve 4 bileşen oyun eklentisi üzerinden kesin değerlerle okundu."
                ),
                false,
            );
        }
    }

    // -------------------------------------------------------------------- overzicht

    fn page_overview(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        let has_data = self.has_data();
        let subtitle = if has_data {
            t!(format!("Laatst uitgelezen: {}", self.eff.source), format!("Last read: {}", self.eff.source))
        } else {
            t!("Open in FM27 de Breakdown van Tactical Effectiveness en lees hem uit.", "Open the Tactical Effectiveness Breakdown in FM27 and read it.").to_string()
        };
        let busy = self.analysis_rx.is_some();
        let live_busy = self.live.busy();
        let mut refresh_auto = false;
        let mut read = false;
        let mut read_bep = false;
        let mut file = false;
        let menu_id = ui.make_persistent_id("overview_read_menu");
        theme::page_header(
            ui,
            t!("Teamanalyse", "Team analysis", "Takım analizi"),
            &subtitle,
            |ui| {
                let menu_resp = theme::button(ui, icon::CARET_DOWN, "", !busy).on_hover_text(t!(
                    "Leesopties",
                    "Read options",
                    "Okuma seçenekleri"
                ));
                if menu_resp.clicked() {
                    ui.memory_mut(|m| m.toggle_popup(menu_id));
                }
                egui::popup::popup_below_widget(
                    ui,
                    menu_id,
                    &menu_resp,
                    egui::PopupCloseBehavior::CloseOnClickOutside,
                    |ui| {
                        ui.set_min_width(240.0);
                        if theme::button(
                            ui,
                            icon::BRAIN,
                            t!("Uit spelmodule lezen", "Read from game plugin", "Oyun eklentisinden oku"),
                            !busy,
                        )
                        .clicked()
                        {
                            read_bep = true;
                            ui.memory_mut(|m| m.close_popup());
                        }
                        ui.add_space(2.0);
                        if theme::button(
                            ui,
                            icon::CAMERA,
                            t!("Van scherm lezen", "Read from screen", "Ekrandan oku"),
                            !busy,
                        )
                        .clicked()
                        {
                            read = true;
                            ui.memory_mut(|m| m.close_popup());
                        }
                        ui.add_space(2.0);
                        if theme::button(
                            ui,
                            icon::EXPORT,
                            t!("Uit bestand lezen", "Read from file", "Dosyadan oku"),
                            !busy,
                        )
                        .clicked()
                        {
                            file = true;
                            ui.memory_mut(|m| m.close_popup());
                        }
                        ui.separator();
                        ui.horizontal(|ui| {
                            if theme::toggle(ui, &mut self.settings.auto_read).changed() {
                                self.last_auto = std::time::Instant::now() - std::time::Duration::from_secs(600);
                                self.auto_note.clear();
                                self.save_settings();
                            }
                            ui.label(theme::label_strong(t!(
                                "Automatisch van scherm lezen",
                                "Auto-read from screen",
                                "Ekrandan otomatik oku"
                            )));
                        });
                        if self.settings.auto_read {
                            ui.horizontal(|ui| {
                                ui.label(theme::dim(t!("elke", "every")));
                                if ui.add(egui::DragValue::new(&mut self.settings.auto_read_secs).range(5..=120).suffix(" s")).changed() {
                                    self.save_settings();
                                }
                            });
                        }
                    },
                );
                if (has_data || self.edit_mode)
                    && theme::primary_button(
                        ui,
                        icon::ARROW_COUNTER_CLOCKWISE,
                        t!("Gegevens vernieuwen", "Refresh data", "Verileri güncelle"),
                        !busy,
                    )
                    .clicked()
                {
                    refresh_auto = true;
                }
                if busy || live_busy {
                    ui.spinner();
                }
            },
        );
        if !has_data && !self.edit_mode {
            if let Some(msg) = (!self.status.is_empty()).then(|| (self.status.clone(), self.status_error)) {
                self.status_line(ui, &msg.0, msg.1);
            }
            if !refresh_auto {
                theme::card(ui, |ui| {
                    ui.label(theme::h2(t!(
                        "Nog geen teamanalyse beschikbaar",
                        "No team analysis data yet",
                        "Henüz takım analizi verisi yok"
                    )));
                    ui.add_space(4.0);
                    ui.add(
                        egui::Label::new(theme::dim(t!(
                            "Open in FM27 de Breakdown van Tactical Effectiveness om de 4 onderdelen en 17 factoren te lezen.",
                            "Open the Tactical Effectiveness Breakdown in FM27 to read all 4 components and 17 factors.",
                            "4 ana bileşeni ve 17 faktörü görmek için FM27'de Taktiksel Verimlilik ekranını açıp verileri güncelleyin."
                        )))
                        .wrap(),
                    );
                    ui.add_space(10.0);
                    if theme::primary_button(
                        ui,
                        icon::ARROW_COUNTER_CLOCKWISE,
                        t!("Gegevens vernieuwen", "Refresh data", "Verileri güncelle"),
                        !busy,
                    )
                    .clicked()
                    {
                        refresh_auto = true;
                    }
                });
            }
        }
        if refresh_auto {
            self.refresh_data_auto(ctx);
        }
        if read_bep {
            self.sync_from_bepinex(true);
        }
        if read {
            self.last_auto = std::time::Instant::now();
            self.auto_fallback_file = false;
            self.start_analysis(ctx, None);
        }
        if file {
            self.show_file = !self.show_file;
        }
        if self.settings.auto_read && !self.auto_note.is_empty() {
            theme::pill(ui, &self.auto_note, theme::INFO);
            ui.add_space(8.0);
        }
        if self.show_file {
            theme::card(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(t!("Screenshot (PNG):", "Screenshot (PNG):"));
                    ui.add(egui::TextEdit::singleline(&mut self.image_path).hint_text("/…/breakdown.png").desired_width(420.0));
                    if theme::button(ui, "", t!("Lees", "Read"), !busy && !self.image_path.trim().is_empty()).clicked() {
                        let p = PathBuf::from(self.image_path.trim().trim_matches('\''));
                        self.start_analysis(ctx, Some(p));
                    }
                });
            });
            ui.add_space(10.0);
        }
        if !has_data && !self.edit_mode {
            return;
        }
        let (msg, err) = (self.status.clone(), self.status_error);
        self.status_line(ui, &msg, err);

        let hkey = self.club_history_key();
        let diff = self.history.previous_diff(&hkey, &self.eff);
        let wide = ui.available_width() >= 860.0;
        let mut changed = false;

        let render_left = |ui: &mut Ui, eff: &Effectiveness, diff: Option<history::HistoryDiff>, points: &[history::HistoryPoint]| {
            let icons = [icon::TARGET, icon::SOCCER_BALL, icon::USERS_THREE, icon::EYE];
            for row in 0..2 {
                ui.columns(2, |cols| {
                    for (col_idx, col_ui) in cols.iter_mut().enumerate() {
                        let i = row * 2 + col_idx;
                        let c = Component::ALL[i];
                        let v = eff.component(c);
                        let col = theme::rgb(level_color(v));
                        theme::component_tile(
                            col_ui,
                            icons[i],
                            c.display_label(),
                            v,
                            level_name(v),
                            col,
                            diff.map(|d| d.components[i]),
                        );
                    }
                });
                if row == 0 {
                    ui.add_space(10.0);
                }
            }
            ui.add_space(12.0);
            history::render_trend_card(ui, points);
        };

        let mut render_right = |ui: &mut Ui, eff: &mut Effectiveness, factor_sort: &mut FactorSort, edit_mode: &mut bool| {
            theme::card(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(theme::ic(icon::SLIDERS).color(theme::ACCENT_HI).size(16.0));
                    ui.label(theme::h2(t!("17 factoren", "17 factors", "17 faktör")));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let sort_opts = [
                            (FactorSort::ByScore, t!("Op score", "By score", "Skora göre")),
                            (FactorSort::ByComponent, t!("Per onderdeel", "By component", "Bileşene göre")),
                        ];
                        theme::segmented(ui, "factor_sort", factor_sort, &sort_opts);
                    });
                });
                ui.add_space(10.0);

                let label_font = egui::FontId::new(13.0, egui::FontFamily::Proportional);
                let max_label_w = ui.fonts(|f| {
                    Factor::ALL
                        .iter()
                        .map(|fac| f.layout_no_wrap(fac.display_label().to_string(), label_font.clone(), theme::TEXT_DIM).size().x)
                        .fold(0.0_f32, f32::max)
                });
                let name_w = (max_label_w + 6.0).clamp(135.0, 235.0);

                let render_factor = |ui: &mut Ui, f: Factor, eff: &mut Effectiveness, diff: Option<history::HistoryDiff>, edit: bool, changed: &mut bool| {
                    let v = eff.get(f);
                    let fd = diff.and_then(|d| Factor::ALL.iter().position(|x| *x == f).map(|idx| d.factors[idx]));
                    theme::factor_bar_row(ui, name_w, f.display_label(), v, theme::rgb(level_color(v)), fd);
                    if edit {
                        let mut nv = v;
                        if ui.add(egui::Slider::new(&mut nv, 0.0..=100.0).show_value(false)).changed() && (nv - v).abs() > 0.01 {
                            eff.set(f, nv);
                            *changed = true;
                        }
                    }
                };

                match *factor_sort {
                    FactorSort::ByScore => {
                        let mut items: Vec<(Factor, f32)> = Factor::ALL.iter().map(|&f| (f, eff.get(f))).collect();
                        items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
                        for (f, _) in items {
                            render_factor(ui, f, eff, diff, *edit_mode, &mut changed);
                        }
                    }
                    FactorSort::ByComponent => {
                        for (ci, c) in Component::ALL.iter().enumerate() {
                            if ci > 0 {
                                ui.add_space(6.0);
                            }
                            ui.label(theme::caps(c.display_label()));
                            ui.add_space(4.0);
                            for f in Factor::ALL.iter().filter(|f| f.component() == *c) {
                                render_factor(ui, *f, eff, diff, *edit_mode, &mut changed);
                            }
                        }
                    }
                }

                ui.add_space(8.0);
                ui.checkbox(
                    edit_mode,
                    t!(
                        "Waarden handmatig aanpassen",
                        "Adjust values manually",
                        "Değerleri elle düzenle"
                    ),
                );
            });
        };

        let pts = self.history.points(&hkey).to_vec();
        if wide {
            ui.columns(2, |cols| {
                render_left(&mut cols[0], &self.eff, diff, &pts);
                render_right(&mut cols[1], &mut self.eff, &mut self.factor_sort, &mut self.edit_mode);
            });
        } else {
            render_left(ui, &self.eff, diff, &pts);
            ui.add_space(12.0);
            render_right(ui, &mut self.eff, &mut self.factor_sort, &mut self.edit_mode);
        }

        if changed {
            self.eff.components.clear();
            if !self.eff.source.ends_with('*') {
                self.eff.source = format!("{} *", if self.eff.source.is_empty() { t!("handmatig", "manual") } else { &self.eff.source });
            }
            let _ = crate::save_effectiveness_for(self.active_team_slot(), &self.eff);
            self.week_plan = None;
            self.year.clear();
        }
    }

    // -------------------------------------------------------------------- calendar (unified Training + Planning)

    fn page_calendar(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        let connected = agent::LIVE_SUPPORTED && self.live.connected;
        let can = connected && !self.live.busy();
        let mut tab = self.plan_tab;
        let (mut view_live, mut write_live, mut build_week, mut build_season) = (false, false, false, false);

        theme::page_header(
            ui,
            t!("Kalender", "Calendar", "Takvim"),
            t!(
                "Week- en seizoensplanning, gekoppeld aan FM's trainingskalender.",
                "Weekly and season training schedule, synced with FM's calendar.",
                "FM antrenman takvimiyle senkronize hafta ve sezon planı."
            ),
            |ui| {
                match tab {
                    PlanTab::Week if connected => {
                        if theme::primary_button(ui, icon::LIGHTNING, t!("Zet training nu in FM", "Apply training now", "Planı FM'ye uygula"), can)
                            .on_disabled_hover_text(t!("Bezig…", "Reading…", "Okunuyor…"))
                            .clicked()
                        {
                            write_live = true;
                        }
                    }
                    PlanTab::Week => {
                        if theme::primary_button(ui, icon::SPARKLE, t!("Maak weekplan", "Build week plan", "Haftalık plan oluştur"), true).clicked() {
                            build_week = true;
                        }
                    }
                    PlanTab::Season => {
                        if theme::primary_button(ui, icon::SPARKLE, t!("Maak seizoensplan", "Build season plan", "Sezon planı oluştur"), true).clicked() {
                            build_season = true;
                        }
                    }
                }
                ui.add_space(6.0);
                theme::segmented(
                    ui,
                    "plan_tab",
                    &mut tab,
                    &[(PlanTab::Week, t!("Week", "Week", "Hafta")), (PlanTab::Season, t!("Seizoen", "Season", "Sezon"))],
                );
                ui.add_space(6.0);
                ui.label(theme::ic(icon::INFO).color(theme::TEXT_FAINT).size(16.0))
                    .on_hover_text(legend_tooltip_text());
                if self.live.busy() {
                    ui.spinner();
                }
            },
        );
        self.plan_tab = tab;

        if write_live {
            self.live.training(ctx, &self.eff, &self.settings.planner, true, false);
        }

        if let Some(latest) = self.live.auto_events.first() {
            let log_id = ui.make_persistent_id("calendar_activity_popup");
            ui.add_space(4.0);
            let resp = Frame::new()
                .fill(theme::tint(theme::SUCCESS, 0.12))
                .stroke(Stroke::new(1.0_f32, theme::tint(theme::SUCCESS, 0.35)))
                .corner_radius(CornerRadius::same(16))
                .inner_margin(Margin::symmetric(10, 4))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(theme::ic(icon::SHIELD_CHECK).color(theme::SUCCESS).size(14.0));
                        ui.label(RichText::new(&latest.text).size(12.0).color(theme::TEXT));
                        ui.label(theme::ic(icon::CARET_DOWN).color(theme::TEXT_FAINT).size(11.0));
                    });
                })
                .response
                .interact(egui::Sense::click())
                .on_hover_cursor(egui::CursorIcon::PointingHand);
            if resp.clicked() {
                ui.memory_mut(|m| m.toggle_popup(log_id));
            }
            egui::popup::popup_below_widget(
                ui,
                log_id,
                &resp,
                egui::PopupCloseBehavior::CloseOnClickOutside,
                |ui| {
                    ui.set_min_width(380.0);
                    ui.label(theme::caps(t!("Activiteitenlog (laatste 20)", "Activity log (last 20)", "Etkinlik kaydı (son 20)")));
                    ui.add_space(4.0);
                    for ev in &self.live.auto_events {
                        ui.horizontal(|ui| {
                            ui.label(theme::ic(icon::SHIELD_CHECK).color(theme::SUCCESS).size(13.0));
                            ui.add(egui::Label::new(RichText::new(&ev.text).size(12.0).color(theme::TEXT)).wrap());
                        });
                        ui.add_space(2.0);
                    }
                },
            );
        }

        let (tm, te) = (self.live.train_status.clone(), self.live.train_error);
        self.status_line(ui, &tm, te);
        let (sm, se) = (self.status.clone(), self.status_error);
        self.status_line(ui, &sm, se);

        match self.plan_tab {
            PlanTab::Week => self.calendar_week(ui, ctx, connected, can, &mut view_live, build_week),
            PlanTab::Season => self.calendar_season(ui, build_season),
        }

        if view_live {
            self.live.training(ctx, &self.eff, &self.settings.planner, false, false);
        }
    }

    fn calendar_week(
        &mut self,
        ui: &mut Ui,
        ctx: &egui::Context,
        connected: bool,
        can: bool,
        view_live: &mut bool,
        build_week: bool,
    ) {
        let settings_id = ui.make_persistent_id("week_settings_popup");
        let mut rebuild_week = build_week || self.week_plan.is_none();

        theme::card(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                let s_resp = theme::button(
                    ui,
                    icon::SLIDERS,
                    t!("Weekinstellingen", "Week settings", "Hafta ayarları"),
                    true,
                );
                if s_resp.clicked() {
                    ui.memory_mut(|m| m.toggle_popup(settings_id));
                }
                egui::popup::popup_below_widget(
                    ui,
                    settings_id,
                    &s_resp,
                    egui::PopupCloseBehavior::CloseOnClickOutside,
                    |ui| {
                        ui.set_min_width(340.0);
                        ui.horizontal(|ui| {
                            ui.label(t!("Week vanaf", "Week from"));
                            if ui.add(egui::TextEdit::singleline(&mut self.week_start).desired_width(105.0)).changed() {
                                rebuild_week = true;
                            }
                        });
                        ui.add_space(4.0);
                        ui.label(theme::caps(t!("Wedstrijd op", "Match on")));
                        ui.horizontal_wrapped(|ui| {
                            for (i, m_flag) in self.week_matches.iter_mut().enumerate() {
                                if ui.toggle_value(m_flag, i18n::weekday_short(i)).changed() {
                                    rebuild_week = true;
                                }
                            }
                        });
                        ui.add_space(4.0);
                        if ui.checkbox(&mut self.prev_sunday, t!("wedstrijd zondag ervoor", "match previous Sunday")).changed() {
                            rebuild_week = true;
                        }
                        if ui.checkbox(&mut self.next_monday, t!("wedstrijd maandag erna", "match next Monday")).changed() {
                            rebuild_week = true;
                        }
                        ui.add_space(4.0);
                        let before_kind = self.week_kind;
                        kind_combo(ui, "weekkind", &mut self.week_kind);
                        if self.week_kind != before_kind {
                            rebuild_week = true;
                            if !before_kind.is_preseason()
                                && self.week_kind.is_preseason()
                                && !self.settings.preseason_intensity_chosen
                            {
                                self.camp_modal_draft = self.settings.planner.preseason_intensity;
                                self.show_camp_modal = true;
                            }
                        }
                    },
                );

                if connected {
                    ui.add_space(8.0);
                    if theme::toggle(ui, &mut self.settings.auto_training).changed() {
                        self.save_settings();
                    }
                    ui.label(theme::label_strong(t!(
                        "Automatisch bijhouden",
                        "Auto-keep on plan",
                        "Otomatik takip"
                    )));
                    let last = self.live.last_check.map(|t| t.format("%H:%M:%S").to_string()).unwrap_or_else(|| "—".into());
                    let auto_tip = t!(
                        format!("Controleert elke 8s en zet FM-afwijkingen terug. Laatst: {last} · Hersteld: {}", self.live.fixed_total),
                        format!("Checks every 8s and restores FM changes. Last check: {last} · Restored: {}", self.live.fixed_total),
                        format!("Her 8 sn'de kontrol edip FM asistanının değiştirdiği günleri geri alır. Son kontrol: {last} · Geri alınan: {}", self.live.fixed_total)
                    );
                    ui.label(theme::ic(icon::INFO).color(theme::TEXT_FAINT).size(14.0))
                        .on_hover_text(auto_tip);

                    ui.add_space(6.0);
                    if theme::button(ui, icon::EYE, t!("Bekijk plan", "Preview"), can).clicked() {
                        *view_live = true;
                    }
                } else if let Some(p) = self.week_plan.clone() {
                    ui.add_space(6.0);
                    if theme::button(ui, icon::EXPORT, t!("Exporteer & open", "Export & open"), true).clicked() {
                        let r = export::write_all(&export::output_dir(), "weekplan", t!("FM27 weekplan", "FM27 week plan"), &self.eff, std::slice::from_ref(&p));
                        self.after_export(r);
                    }
                }
            });

            ui.add_space(8.0);
            let mut mode_changed = false;
            let prev_camp = self.settings.planner.camp_mode;
            let live_has_preseason = self
                .live
                .training
                .as_ref()
                .is_some_and(|t| t.weeks.iter().any(|w| !w.rest && w.kind.is_preseason()));
            ui.horizontal_wrapped(|ui| {
                ui.label(theme::label_strong(t!("Planinstellingen:", "Plan settings:", "Plan ayarları:")));
                ui.label(theme::dim(t!("Tactiekstijl", "Tactic style", "Taktik tarzı")));
                egui::ComboBox::from_id_salt("cal_tactic_mode")
                    .selected_text(self.settings.planner.tactic_mode.label())
                    .show_ui(ui, |ui| {
                        for m in planner::TacticMode::ALL {
                            if ui.selectable_value(&mut self.settings.planner.tactic_mode, m, m.label()).changed() {
                                mode_changed = true;
                            }
                        }
                    });
                ui.add_space(8.0);
                ui.label(theme::dim(t!("Speciaal kamp", "Special camp", "Özel kamp modu")));
                egui::ComboBox::from_id_salt("cal_camp_mode")
                    .selected_text(self.settings.planner.camp_mode.label())
                    .show_ui(ui, |ui| {
                        for c in planner::CampMode::ALL {
                            if ui.selectable_value(&mut self.settings.planner.camp_mode, c, c.label()).changed() {
                                mode_changed = true;
                            }
                        }
                    });
                ui.label(theme::ic(icon::INFO).color(theme::TEXT_FAINT).size(14.0))
                    .on_hover_text(self.settings.planner.camp_mode.description());
                if self.settings.planner.camp_mode != planner::CampMode::Auto {
                    theme::pill(ui, self.settings.planner.camp_mode.label(), theme::WARNING);
                }
                if self.settings.planner.camp_mode == planner::CampMode::PreSeasonFitness
                    || self.week_kind.is_preseason()
                    || live_has_preseason
                {
                    let camp_btn_lbl = format!(
                        "{}: {}",
                        t!("Kampstijl", "Camp style", "Kamp tarzı"),
                        self.settings.planner.preseason_intensity.short_label()
                    );
                    if theme::small_button(ui, &camp_btn_lbl, true).clicked() {
                        self.camp_modal_draft = self.settings.planner.preseason_intensity;
                        self.show_camp_modal = true;
                    }
                }
            });
            if self.settings.planner.camp_mode == planner::CampMode::PreSeasonFitness
                && prev_camp != planner::CampMode::PreSeasonFitness
            {
                self.camp_modal_draft = self.settings.planner.preseason_intensity;
                self.show_camp_modal = true;
            }
            if mode_changed {
                self.save_settings();
                rebuild_week = true;
                self.year.clear();
                if connected && !self.live.busy() {
                    self.live.training(ctx, &self.eff, &self.settings.planner, false, false);
                }
            }
        });

        if rebuild_week {
            let start = NaiveDate::parse_from_str(self.week_start.trim(), "%Y-%m-%d")
                .map(planner::monday_of)
                .unwrap_or_else(|_| planner::monday_of(chrono::Local::now().date_naive()));
            let week = CalendarWeek { start, kind: self.week_kind, matches: self.week_matches, note: String::new() };
            self.week_plan = Some(planner::plan_week(&week, self.prev_sunday, self.next_monday, &self.eff, &self.settings.planner, 1));
        }

        ui.add_space(12.0);
        let has_data = self.has_data();
        if connected {
            if let Some(o) = self.live.training.clone() {
                for w in &o.weeks {
                    week_card(ui, w, o.today, &self.live.fm_had, &self.live.reverted_days, self.settings.planner.no_tactical, has_data);
                    ui.add_space(10.0);
                }
            } else if !self.live.busy() {
                self.live.training(ctx, &self.eff, &self.settings.planner, false, false);
            }
        } else if let Some(p) = &self.week_plan {
            theme::card(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    let heading = readable_plan_title(p.focus, &p.targets, self.settings.planner.no_tactical, p.kind);
                    ui.label(theme::h2(heading)).on_hover_text(&p.schedule_name);
                    if !has_data {
                        theme::pill(
                            ui,
                            t!(
                                "Voorbeeldplan · wordt vernieuwd zodra echte data is gelezen",
                                "Sample plan · updates once real data is read",
                                "Örnek plan · gerçek veriler okunduğunda yenilenir"
                            ),
                            theme::WARNING,
                        );
                    }
                    if self.settings.planner.no_tactical && p.focus != "Full Week of Rest" {
                        let pf = match p.focus {
                            "In Possession" => t!("Balbezit", "In Possession", "Topa Sahipken"),
                            "Out Of Possession" => t!("Balverlies", "Out Of Possession", "Topsuz Alanda"),
                            _ => t!("Gebalanceerd", "Balanced", "Dengeli"),
                        };
                        theme::pill(ui, pf, theme::ACCENT);
                    }
                });
                ui.add_space(8.0);
                plan_grid(ui, p);
                if !p.notes.is_empty() {
                    ui.add_space(8.0);
                    ui.label(theme::col_header(t!("Buiten de training", "Beyond training")));
                    for n in &p.notes {
                        ui.add(egui::Label::new(theme::dim(format!("• {n}"))).wrap());
                    }
                }
            });
        }
    }

    /// Tüm sekmelerde çalışan takım seçici şerit: As Takım ücretsiz, U18/U19/U21/B takımları Pro.
    fn render_team_bar(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        let demo = std::env::var_os("FM27_TEAMS_JSON");
        if let (Some(p), None) = (&demo, &self.live.teams) {
            self.live.teams = std::fs::read_to_string(p).ok().and_then(|s| serde_json::from_str(&s).ok());
        }
        if !self.live.connected && demo.is_none() && std::env::var_os("FM27_DEMO_TEAMS").is_none() {
            return;
        }
        let Some(data) = self.live.teams.clone() else { return };
        let list = |k: &str| data.get(k).and_then(|v| v.as_array()).cloned().unwrap_or_default();
        let (managers, teams) = (list("managers"), list("teams"));
        if managers.len() < 2 && teams.len() < 2 {
            return;
        }
        let sel = |v: &Value| v.get("selected").and_then(|b| b.as_bool()) == Some(true);
        let mut pick: Option<(Option<u32>, Option<usize>)> = None;

        theme::card(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = vec2(8.0, 6.0);
                if managers.len() > 1 {
                    ui.label(theme::ic(icon::USERS_THREE).color(theme::ACCENT));
                    let cur = managers.iter().find(|m| sel(m)).and_then(|m| m["club"].as_str()).unwrap_or("?").to_string();
                    egui::ComboBox::from_id_salt("manager_pick").selected_text(cur).show_ui(ui, |ui| {
                        for m in &managers {
                            let uid = m["club_uid"].as_u64().unwrap_or(0) as u32;
                            if ui.selectable_label(sel(m), m["club"].as_str().unwrap_or("?")).clicked() && !sel(m) {
                                pick = Some((Some(uid), None));
                            }
                        }
                    });
                    ui.add_space(6.0);
                }

                for x in &teams {
                    let own = x.get("own").and_then(|b| b.as_bool()) == Some(true);
                    let idx = x["index"].as_u64().unwrap_or(0) as usize;
                    let players = x.get("players").and_then(|v| v.as_u64()).unwrap_or(0);
                    let short = team_short_label(x);
                    let is_sel = if own {
                        self.active_team_slot().is_none()
                    } else {
                        self.active_team_slot() == Some(idx)
                    };

                    let (fill, stroke, txt_col, ic_str, ic_col) = if is_sel {
                        (
                            theme::ACCENT_SOFT,
                            Stroke::new(1.2_f32, theme::ACCENT),
                            egui::Color32::WHITE,
                            if own { icon::SHIELD } else { icon::SPARKLE },
                            theme::ACCENT_HI,
                        )
                    } else {
                        (
                            theme::SURFACE2,
                            Stroke::new(1.0_f32, theme::BORDER),
                            theme::TEXT,
                            if own { icon::SHIELD } else { icon::USERS_THREE },
                            theme::TEXT_DIM,
                        )
                    };

                    let lbl = format!("{short} · {players}");
                    let mut job = egui::text::LayoutJob::default();
                    let fmt = |font_id: egui::FontId, color: egui::Color32| egui::text::TextFormat {
                        font_id,
                        color,
                        valign: egui::Align::Center,
                        ..Default::default()
                    };
                    job.append(ic_str, 0.0, fmt(egui::FontId::new(13.5, theme::icons()), ic_col));
                    job.append(" ", 0.0, fmt(egui::FontId::new(12.5, theme::medium()), txt_col));
                    job.append(&lbl, 0.0, fmt(egui::FontId::new(12.5, if is_sel { theme::semibold() } else { theme::medium() }), txt_col));
                    let galley = ui.fonts(|f| f.layout_job(job));
                    let size = vec2(galley.size().x + 22.0, 30.0);
                    let (rect, resp) = ui.allocate_exact_size(size, egui::Sense::click());
                    let hov = resp.hovered() && !is_sel;
                    let draw_fill = if hov { theme::mix(fill, theme::HOVER, 0.6) } else { fill };
                    let p = ui.painter();
                    p.rect_filled(rect, CornerRadius::same(15), draw_fill);
                    p.rect_stroke(rect, CornerRadius::same(15), stroke, egui::StrokeKind::Inside);
                    p.galley(rect.center() - galley.size() / 2.0, galley, txt_col);
                    let resp = resp.on_hover_cursor(egui::CursorIcon::PointingHand);
                    if resp.clicked() && !is_sel {
                        pick = Some((self.settings.manager_club, if own { None } else { Some(idx) }));
                    }
                }

                if self.live.is_youth_team_active() {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if theme::toggle(ui, &mut self.settings.youth_dev_mode).changed() {
                            self.live.youth_dev_mode = self.settings.youth_dev_mode;
                            self.save_settings();
                            self.week_plan = None;
                            self.year.clear();
                        }
                        ui.label(
                            RichText::new(t!("Academie-ontwikkelingsmodus", "Academy Dev Mode", "Akademi Gelişim Modu"))
                                .size(12.5)
                                .color(theme::TEXT_DIM),
                        )
                        .on_hover_text(t!(
                            "Geeft voorrang aan spelerontwikkeling en veilige dubbele intensiteit voor jeugdspelers.",
                            "Prioritizes Player Development slots in schedule and Double Intensity for fit youngsters.",
                            "Takvimde Oyuncu Gelişimi seanslarına ve uygun genç oyuncular için Çift Yoğunluğa öncelik verir."
                        ));
                        ui.label(theme::ic(icon::SPARKLE).color(theme::ACCENT_HI).size(14.0));
                    });
                }
            });

            if self.live.is_youth_team_active() && !self.has_data() {
                ui.add_space(4.0);
                ui.add(
                    egui::Label::new(theme::dim(t!(
                        "Let op: in FM27 moet je de training van dit jeugdteam zelf overnemen (Staf → Verantwoordelijkheden) voordat de kalender bewerkt kan worden.",
                        "Note: in FM27, take training control of this youth team (Staff → Responsibilities) to read/apply its calendar.",
                        "Not: FM27'de bu altyapı takımının antrenman sorumluluğunu üstlendiğinizde (Ekip → Sorumluluklar) takvim okunabilir ve yazılabilir."
                    )))
                    .wrap(),
                );
            }
        });
        ui.add_space(10.0);

        if let Some((club, team)) = pick {
            self.switch_team_context(ctx, club, team);
        }
    }

    fn calendar_season(&mut self, ui: &mut Ui, build_season: bool) {
        if build_season {
            self.year = planner::plan_year(&self.calendar, &self.eff, &self.settings.planner);
            self.save_calendar();
        }
        theme::card(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label("Start");
                ui.add(egui::TextEdit::singleline(&mut self.year_start).desired_width(96.0));
                ui.label(t!("Competitie op", "League on"));
                weekday_combo(ui, "league", &mut self.cal_opts.league_day);
                ui.label(t!("Europees", "Europe"));
                let mut e = self.cal_opts.europe;
                egui::ComboBox::from_id_salt("europe")
                    .selected_text(match e {
                        None => t!("geen", "none"),
                        Some(Weekday::Tue) => t!("di (CL)", "Tue (CL)"),
                        Some(Weekday::Wed) => t!("wo (CL)", "Wed (CL)"),
                        _ => t!("do (EL/ECL)", "Thu (EL/ECL)"),
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut e, None, t!("geen", "none"));
                        ui.selectable_value(&mut e, Some(Weekday::Tue), t!("di (CL)", "Tue (CL)"));
                        ui.selectable_value(&mut e, Some(Weekday::Wed), t!("wo (CL)", "Wed (CL)"));
                        ui.selectable_value(&mut e, Some(Weekday::Thu), t!("do (EL/ECL)", "Thu (EL/ECL)"));
                    });
                self.cal_opts.europe = e;
                ui.checkbox(&mut self.cal_opts.cup, t!("beker", "cup"));
                ui.add(egui::DragValue::new(&mut self.cal_opts.weeks).range(4..=60).suffix(t!(" weken", " weeks")));
            });
            ui.add_space(6.0);
            ui.horizontal_wrapped(|ui| {
                if theme::button(ui, icon::CALENDAR_BLANK, t!("Standaardkalender", "Default calendar"), true).clicked() {
                    if let Ok(d) = NaiveDate::parse_from_str(self.year_start.trim(), "%Y-%m-%d") {
                        self.cal_opts.start = d;
                    }
                    self.calendar = planner::default_calendar(&self.cal_opts);
                    self.year.clear();
                    self.save_calendar();
                }
                if !self.year.is_empty() {
                    if theme::button(ui, icon::EXPORT, t!("Exporteer & open", "Export & open"), true).clicked() {
                        let r = export::write_all(&export::output_dir(), "seasonplan", t!("FM27 seizoensplan", "FM27 season plan"), &self.eff, &self.year);
                        self.after_export(r);
                    }
                    let (n, u) = (self.year.len(), unique_schedules(&self.year).len());
                    ui.label(theme::faint(t!(format!("{n} weken · {u} verschillende weekschema's"), format!("{n} weeks · {u} distinct weekly schedules"))));
                }
            });
        });
        ui.add_space(12.0);
        let mut edited = false;
        theme::card(ui, |ui| {
            egui::Grid::new("cal").num_columns(5).spacing([14.0, 8.0]).striped(true).show(ui, |ui| {
                for h in [t!("Week van", "Week of"), t!("Periode", "Period"), t!("Wedstrijden", "Matches"), "Focus", t!("Tactisch", "Tactical")] {
                    ui.label(theme::col_header(h));
                }
                ui.end_row();
                for (wi, week) in self.calendar.iter_mut().enumerate() {
                    ui.label(i18n::date(week.start));
                    let before = week.kind;
                    kind_combo(ui, &format!("k{wi}"), &mut week.kind);
                    edited |= week.kind != before;
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 3.0;
                        for (d, m_flag) in week.matches.iter_mut().enumerate() {
                            let short_wd: String = i18n::weekday_short(d).chars().take(1).collect();
                            if ui.toggle_value(m_flag, short_wd).changed() {
                                edited = true;
                            }
                        }
                        if !week.note.is_empty() {
                            ui.label(theme::faint(&week.note));
                        }
                    });
                    if let Some(p) = self.year.get(wi) {
                        theme::pill(ui, p.focus, theme::ACCENT);
                        let t: Vec<String> = p.targets.iter().map(|(f, n)| format!("{}×{}", planner::short(*f), n)).collect();
                        ui.label(theme::faint(t.join("  ")));
                    } else {
                        ui.label("");
                        ui.label("");
                    }
                    ui.end_row();
                }
            });
        });
        if edited {
            self.year.clear();
            self.save_calendar();
        }
    }

    fn after_export(&mut self, res: Result<PathBuf>) {
        match res {
            Ok(p) => {
                open_path(&p);
                self.set_status(t!(format!("Opgeslagen: {}", p.display()), format!("Saved: {}", p.display())), false);
            }
            Err(e) => self.set_status(format!("{e:#}"), true),
        }
    }

    // -------------------------------------------------------------------- club

    fn page_tactics(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        use crate::tactics as tx;
        // testhulp: FM27_TACTICS_JSON=<bestand> toont de pagina met bewaarde gegevens, zonder FM
        let from_file = std::env::var_os("FM27_TACTICS_JSON");
        if let (Some(f), None) = (&from_file, &self.live.tactics) {
            self.live.tactics = std::fs::read_to_string(f).ok().and_then(|t| serde_json::from_str(&t).ok());
        }
        let connected = self.live.connected || (from_file.is_some() && self.live.tactics.is_some());
        let busy = self.live.tactics_busy();
        // bij het openen één keer vanzelf ophalen
        if connected && !self.tactics_asked && from_file.is_none() {
            self.tactics_asked = true;
            self.live.load_tactics(ctx);
        }
        if !connected {
            self.tactics_asked = false;
        }
        let mut reload = false;
        theme::page_header(
            ui,
            t!("Tactiek", "Tactics", "Taktik"),
            t!("Hoe goed je spelers bij de rollen van je tactiek passen", "How well your players fit the roles of your tactic"),
            |ui| {
                if connected && theme::primary_button(ui, icon::ARROWS_CLOCKWISE, if busy { t!("Bezig…", "Reading…") } else { t!("Opnieuw uitlezen", "Read again") }, !busy).clicked() {
                    reload = true;
                }
            },
        );
        if reload {
            self.live.load_tactics(ctx);
        }
        if !connected {
            self.empty_connect_card(
                ui,
                t!("Verbind met FM27 voor tactiekanalyse", "Connect to FM27 for tactic analysis", "Taktik analizi için FM27'ye bağlan"),
                t!(
                    "Rolgeschiktheid en het beste elftal worden berekend zodra je carrière in FM27 is geopend.",
                    "Role fit and your best eleven are calculated once your FM27 save is loaded.",
                    "Rol uygunluğu ve en iyi 11 hesabı FM27'de kariyerin açıldığında burada görünür."
                ),
            );
            return;
        }
        let err = self.live.tactics_error.clone();
        self.status_line(ui, &err, true);
        let Some(data) = self.live.tactics.clone() else {
            if err.is_empty() {
                theme::card(ui, |ui| {
                    ui.label(theme::faint(t!("Tactiek uitlezen…", "Reading your tactic…")));
                });
            }
            return;
        };
        if data.tactics.is_empty() {
            theme::card(ui, |ui| {
                ui.label(theme::label_strong(t!(
                    "Nog geen tactiekopstelling geladen voor dit team.",
                    "No tactical line-up loaded for this team yet.",
                    "Bu takım için henüz taktik dizilişi okunamadı."
                )));
                ui.add_space(4.0);
                ui.add(
                    egui::Label::new(theme::dim(t!(
                        "Open in FM het tactiekscherm van het team om de beste elf en rolgeschiktheid te zien. Individuele intensiteit en extra focus op Spelers werken wel direct.",
                        "Open the team's tactics screen in FM to see Best XI and role fit. Individual intensity and focus recommendations on Players already work.",
                        "En iyi 11 ve rol uygunluğunu görmek için FM'de takımın taktik ekranını açın. Oyuncular sekmesindeki yoğunluk ve ek odak önerileri şimdiden çalışır."
                    )))
                    .wrap(),
                );
            });
            return;
        }
        let pick = self.tactic_pick.filter(|i| *i < data.tactics.len()).unwrap_or(data.current.min(data.tactics.len() - 1));
        let tactic = &data.tactics[pick];
        let in_use = pick == data.current.min(data.tactics.len() - 1);
        let mut places = tx::places(tactic);
        // FM bewaart de basiself in de volgorde van de tactiek in gebruik: bij een andere tactiek
        // zegt die volgorde niets over wie waar zou staan
        if !in_use {
            for p in &mut places {
                p.uid = None;
            }
        }
        let by_uid: std::collections::HashMap<u32, &tx::Player> = data.players.iter().map(|p| (p.uid, p)).collect();
        let in_eleven: std::collections::HashSet<u32> = places.iter().filter_map(|p| p.uid).collect();
        // veldvolgorde: keeper eerst, dan per linie van rechts naar links
        let mut order: Vec<usize> = (0..places.len()).collect();
        order.sort_by_key(|&i| places[i].oop_order);
        let fits: Vec<Option<f32>> = places.iter().map(|pl| pl.uid.and_then(|u| by_uid.get(&u)).map(|p| tx::fit(p, pl))).collect();
        let known: Vec<f32> = fits.iter().flatten().copied().collect();
        let avg = if known.is_empty() { 0.0 } else { known.iter().sum::<f32>() / known.len() as f32 };
        let weakest = order.iter().filter_map(|&i| fits[i].map(|f| (i, f))).min_by(|a, b| a.1.total_cmp(&b.1));
        let best = tx::best_eleven(&places, &data.players);
        let best_avg = if best.len() == places.len() { places.iter().zip(&best).map(|(pl, &j)| tx::fit(&data.players[j], pl)).sum::<f32>() / places.len() as f32 } else { avg };

        // welke tactiek
        ui.horizontal(|ui| {
            let labels: Vec<String> = data
                .tactics
                .iter()
                .map(|t| {
                    let mut l = format!("{} {}", t!("Tactiek", "Tactic"), t.index + 1);
                    if t.index == data.tactics[data.current.min(data.tactics.len() - 1)].index {
                        l.push_str(t!(" · in gebruik", " · in use"));
                    }
                    l
                })
                .collect();
            let options: Vec<(usize, &str)> = labels.iter().enumerate().map(|(i, l)| (i, l.as_str())).collect();
            let mut v = pick;
            if theme::segmented(ui, "tactic_pick", &mut v, &options) {
                self.tactic_pick = Some(v);
                theme::restart_enter(ctx);
            }
            if !tactic.style.is_empty() {
                theme::pill(ui, &format!("{} {}", t!("sjabloon", "template"), tactic.style), theme::TEXT_DIM);
            }
        });
        ui.add_space(12.0);
        if !in_use {
            ui.add(
                egui::Label::new(theme::faint(t!(
                    "Dit is niet de tactiek die nu in gebruik is: FM bewaart je basiself alleen voor die tactiek. Hieronder staat daarom alleen de beste elf voor deze tactiek.",
                    "This is not the tactic in use: FM keeps your starting eleven for that tactic only. So below you only see the best eleven for this tactic."
                )))
                .wrap(),
            );
            ui.add_space(8.0);
        }

        let gain = best_avg - avg;
        let (wname, wval) = weakest
            .map(|(i, f)| (format!("{} · {}", places[i].line.label(places[i].side), places[i].uid.and_then(|u| by_uid.get(&u)).map(|p| p.name.as_str()).unwrap_or("?")), f))
            .unwrap_or((t!("geen elftal gekozen", "no eleven picked").to_string(), 0.0));
        let draw_tiles = |ui0: &mut Ui, ui1: &mut Ui, ui2: &mut Ui| {
            theme::stat_tile(ui0, icon::STRATEGY, t!("Elftal nu", "Current eleven"), &format!("{avg:.0}"), t!("gemiddelde geschiktheid", "average fit"), theme::rgb(level_color(avg)), Some(avg));
            theme::stat_tile(
                ui1,
                icon::USERS_THREE,
                t!("Beste elf", "Best eleven"),
                &format!("{best_avg:.0}"),
                &if gain >= 0.5 { t!(format!("+{gain:.1} te winnen"), format!("+{gain:.1} to gain")) } else { t!("je elftal is al het beste".to_string(), "your eleven is already the best".to_string()) },
                theme::rgb(level_color(best_avg)),
                Some(best_avg),
            );
            theme::stat_tile(ui2, icon::WARNING, t!("Zwakste plek", "Weakest place"), &format!("{wval:.0}"), &wname, theme::rgb(level_color(wval)), Some(wval));
        };
        if ui.available_width() >= 820.0 {
            ui.columns(3, |c| {
                let (c0, rest) = c.split_at_mut(1);
                let (c1, c2) = rest.split_at_mut(1);
                draw_tiles(&mut c0[0], &mut c1[0], &mut c2[0]);
            });
        } else {
            theme::stat_tile(ui, icon::STRATEGY, t!("Elftal nu", "Current eleven"), &format!("{avg:.0}"), t!("gemiddelde geschiktheid", "average fit"), theme::rgb(level_color(avg)), Some(avg));
            ui.add_space(8.0);
            theme::stat_tile(
                ui,
                icon::USERS_THREE,
                t!("Beste elf", "Best eleven"),
                &format!("{best_avg:.0}"),
                &if gain >= 0.5 { t!(format!("+{gain:.1} te winnen"), format!("+{gain:.1} to gain")) } else { t!("je elftal is al het beste".to_string(), "your eleven is already the best".to_string()) },
                theme::rgb(level_color(best_avg)),
                Some(best_avg),
            );
            ui.add_space(8.0);
            theme::stat_tile(ui, icon::WARNING, t!("Zwakste plek", "Weakest place"), &format!("{wval:.0}"), &wname, theme::rgb(level_color(wval)), Some(wval));
        }
        ui.add_space(14.0);

        // de elf plekken
        theme::card(ui, |ui| {
            ui.label(theme::h2(t!("Je elftal in deze tactiek", "Your eleven in this tactic")));
            ui.add_space(6.0);
            egui::Grid::new("tactic_slots").num_columns(6).spacing([18.0, 10.0]).striped(true).show(ui, |ui| {
                for h in [t!("Plek", "Place"), t!("Rol met / zonder bal", "Role in / out of possession"), t!("Speler", "Player"), t!("Geschikt", "Fit"), t!("Positie", "Position"), t!("Beter in de selectie", "Better in the squad")] {
                    ui.label(theme::col_header(h));
                }
                ui.end_row();
                for &i in &order {
                    let pl = &places[i];
                    ui.label(theme::label_strong(pl.line.label(pl.side)));
                    let (ic, iname) = tx::ip_label(pl.ip);
                    let (oc, oname) = tx::oop_label(pl.oop);
                    ui.horizontal(|ui| {
                        theme::pill(ui, &ic, theme::ACCENT_HI).on_hover_text(iname);
                        theme::pill(ui, &oc, theme::INFO).on_hover_text(oname);
                    });
                    let cur = pl.uid.and_then(|u| by_uid.get(&u));
                    match cur {
                        Some(p) => {
                            let f = tx::fit(p, pl);
                            let r = tx::position_rating(p, pl);
                            ui.horizontal(|ui| {
                                ui.label(theme::label_strong(&p.name));
                                if p.injured {
                                    theme::pill(ui, t!("geblesseerd", "injured"), theme::DANGER);
                                }
                                if p.banned {
                                    theme::pill(ui, t!("geschorst", "suspended"), theme::WARNING);
                                }
                            });
                            ui.horizontal(|ui| {
                                ui.scope(|ui| {
                                    ui.set_width(80.0);
                                    theme::progress(ui, f, 6.0, theme::rgb(level_color(f)));
                                });
                                ui.label(RichText::new(format!("{f:.0}")).color(theme::TEXT));
                            });
                            theme::pill(ui, tx::position_word(r), theme::rgb(level_color(r as f32 * 5.0)));
                            // de beste speler buiten de elf voor deze plek
                            let alt = data
                                .players
                                .iter()
                                .filter(|q| !in_eleven.contains(&q.uid) && !q.injured && !q.banned)
                                .map(|q| (q, tx::fit(q, pl)))
                                .max_by(|a, b| a.1.total_cmp(&b.1));
                            match alt {
                                Some((q, qf)) if qf >= f + 2.0 => {
                                    ui.label(RichText::new(format!("{} (+{:.0})", q.name, qf - f)).color(theme::SUCCESS));
                                }
                                _ => {
                                    ui.label(theme::faint("—"));
                                }
                            }
                        }
                        None => {
                            ui.label(theme::faint(t!("leeg", "empty")));
                            ui.label(theme::faint("—"));
                            ui.label(theme::faint("—"));
                            ui.label(theme::faint("—"));
                        }
                    }
                    ui.end_row();
                }
            });
        });

        // de beste elf: alleen wat anders is dan nu
        theme::card(ui, |ui| {
            ui.label(theme::h2(t!("Beste elf voor deze tactiek", "Best eleven for this tactic")));
            ui.add(
                egui::Label::new(theme::faint(t!(
                    "De verdeling van je fitte spelers over de elf plekken met de hoogste totale geschiktheid. Geblesseerde en geschorste spelers doen niet mee.",
                    "The way to spread your available players over the eleven places with the highest total fit. Injured and suspended players are left out."
                )))
                .wrap(),
            );
            ui.add_space(6.0);
            let mut changes = 0;
            if best.len() == places.len() {
                for &i in &order {
                    let pl = &places[i];
                    let b = &data.players[best[i]];
                    if pl.uid == Some(b.uid) {
                        continue;
                    }
                    changes += 1;
                    let now = pl.uid.and_then(|u| by_uid.get(&u));
                    ui.horizontal(|ui| {
                        ui.scope(|ui| {
                            ui.set_width(90.0);
                            ui.label(theme::label_strong(pl.line.label(pl.side)));
                        });
                        ui.label(theme::dim(now.map(|p| p.name.as_str()).unwrap_or(t!("leeg", "empty"))));
                        ui.label(theme::ic(icon::ARROW_RIGHT).color(theme::TEXT_FAINT));
                        ui.label(theme::label_strong(&b.name));
                        let (bf, nf) = (tx::fit(b, pl), now.map(|p| tx::fit(p, pl)));
                        theme::pill(ui, &match nf { Some(nf) => format!("{nf:.0} → {bf:.0}"), None => format!("{bf:.0}") }, theme::rgb(level_color(bf)));
                        if in_eleven.contains(&b.uid) {
                            ui.label(theme::faint(t!("(schuift door)", "(moves across)")));
                        }
                    });
                }
            }
            if changes == 0 {
                ui.label(theme::dim(t!("Je huidige elf is volgens deze berekening al de beste.", "By this calculation your current eleven is already the best.")));
            }
        });

        // een rol die beter bij de speler past
        let role_tips: Vec<(String, &'static str, &'static str, &'static str, f32, f32)> = order
            .iter()
            .filter_map(|&i| {
                let pl = &places[i];
                let p = pl.uid.and_then(|u| by_uid.get(&u))?;
                let (r, now, then) = tx::better_role(p, pl)?;
                Some((p.name.clone(), tx::ip_role(pl.ip?)?.code, r.code, r.name, now, then))
            })
            .collect();
        theme::card(ui, |ui| {
            ui.label(theme::h2(t!("Rollen die beter passen", "Roles that fit better")));
            ui.add(
                egui::Label::new(theme::faint(t!(
                    "Een andere rol met bal in dezelfde linie waar de attributen van de speler duidelijk beter bij passen.",
                    "Another in-possession role in the same line that the player's attributes suit clearly better."
                )))
                .wrap(),
            );
            ui.add_space(6.0);
            if role_tips.is_empty() {
                ui.label(theme::dim(t!("Geen: elke speler staat in een rol die bij hem past.", "None: every player is in a role that suits him.")));
            }
            for (name, cur, code, full, now, then) in &role_tips {
                ui.horizontal(|ui| {
                    ui.label(theme::label_strong(name));
                    theme::pill(ui, &format!("{cur} {now:.0}"), theme::TEXT_DIM);
                    ui.label(theme::ic(icon::ARROW_RIGHT).color(theme::TEXT_FAINT));
                    theme::pill(ui, &format!("{code} {then:.0}"), theme::SUCCESS).on_hover_text(*full);
                    ui.label(theme::faint(*full));
                });
            }
        });

        ui.add(
            egui::Label::new(theme::faint(t!(
                "De rollen, posities en attributen komen uit FM. Welke attributen bij een rol horen is onze eigen inschatting, en links of rechts is afgeleid uit de volgorde van de plekken: zie de score als hulpmiddel, niet als cijfer van FM. Er wordt niets in FM veranderd.",
                "Roles, positions and attributes come from FM. Which attributes belong to a role is our own estimate, and left or right is derived from the order of the places: treat the score as a guide, not as a number from FM. Nothing is changed in FM."
            )))
            .wrap(),
        );
    }

    fn render_update_banner(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        enum BannerKind {
            Available(updater::ReleaseInfo),
            Working(String),
            Error(String),
        }
        let kind = match &self.updater.status {
            UpdateStatus::Available(info) => BannerKind::Available(info.clone()),
            UpdateStatus::Downloading { version } => BannerKind::Working(t!(
                format!("Update v{version} wordt gedownload…"),
                format!("Downloading update v{version}…"),
                format!("v{version} güncellemesi indiriliyor…")
            )),
            UpdateStatus::Applying { version } => BannerKind::Working(t!(
                format!("Update v{version} wordt toegepast, app herstart…"),
                format!("Applying update v{version}, restarting…"),
                format!("v{version} yükleniyor, uygulama yeniden başlatılıyor…")
            )),
            UpdateStatus::Error(err) if self.updater.last_install_failed => BannerKind::Error(err.clone()),
            _ => return,
        };

        let col = match &kind {
            BannerKind::Available(_) => theme::SUCCESS,
            BannerKind::Working(_) => theme::INFO,
            BannerKind::Error(_) => theme::WARNING,
        };
        Frame::new()
            .fill(theme::tint(col, 0.12))
            .stroke(Stroke::new(1.0_f32, theme::tint(col, 0.42)))
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::symmetric(12, 8))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.horizontal_wrapped(|ui| match kind {
                    BannerKind::Available(info) => {
                        ui.label(theme::ic(icon::SPARKLE).color(theme::SUCCESS));
                        ui.label(
                            RichText::new(t!(
                                format!("Nieuwe versie beschikbaar: v{}", info.version),
                                format!("New version available: v{}", info.version),
                                format!("Yeni sürüm mevcut: v{}", info.version)
                            ))
                            .strong()
                            .color(theme::TEXT),
                        );
                        if info.asset.is_some() {
                            if theme::small_button(
                                ui,
                                t!("Nu installeren", "Install & restart", "Şimdi yükle ve yeniden başlat"),
                                !self.updater.busy(),
                            )
                            .clicked()
                            {
                                self.updater.start_install(ctx, info);
                            }
                        } else if theme::small_button(ui, "GitHub Releases", true).clicked() {
                            open_path(std::path::Path::new(&info.html_url));
                        }
                    }
                    BannerKind::Working(msg) => {
                        ui.label(theme::ic(icon::ARROWS_CLOCKWISE).color(theme::INFO));
                        ui.label(RichText::new(msg).color(theme::TEXT));
                    }
                    BannerKind::Error(err) => {
                        ui.label(theme::ic(icon::WARNING).color(theme::WARNING));
                        ui.label(RichText::new(err).color(theme::TEXT));
                        if theme::small_button(ui, "GitHub Releases", true).clicked() {
                            open_path(std::path::Path::new(updater::RELEASES_PAGE));
                        }
                    }
                });
            });
        ui.add_space(12.0);
    }

    fn page_settings(&mut self, ui: &mut Ui) {
        theme::page_header(ui, t!("Instellingen", "Settings"), "", |_| {});
        let ctx = ui.ctx().clone();
        let mut dirty = false;
        let mut trigger_check = false;
        let mut trigger_install: Option<updater::ReleaseInfo> = None;

        let sections: [(SettingsSection, &str, &str); 5] = [
            (SettingsSection::General, icon::GEAR_SIX, t!("Algemeen", "General", "Genel")),
            (SettingsSection::Planner, icon::CALENDAR_DOTS, t!("Planner", "Planner", "Planlayıcı")),
            (SettingsSection::FmConnection, icon::PLUGS_CONNECTED, t!("FM-verbinding", "FM connection", "FM bağlantısı")),
            (SettingsSection::Updates, icon::ARROWS_CLOCKWISE, t!("Updates", "Updates", "Güncellemeler")),
            (SettingsSection::Folders, icon::FOLDER_OPEN, t!("Mappen", "Folders", "Klasörler")),
        ];

        let avail = ui.available_width();
        let side_nav = avail >= 760.0;
        let mut selected = self.settings_section;
        let mut clicked_sec: Option<SettingsSection> = None;

        let draw_right_body = |ui: &mut Ui,
                               app: &mut App,
                               dirty: &mut bool,
                               trigger_check: &mut bool,
                               trigger_install: &mut Option<updater::ReleaseInfo>| {
            let section_heading = |ui: &mut Ui, sec: SettingsSection, title: &str, scroll_to: &mut Option<SettingsSection>, first: bool| {
                if !first {
                    ui.add_space(22.0);
                }
                let resp = ui.label(RichText::new(title).font(egui::FontId::new(16.0, theme::bold())).color(theme::TEXT));
                if *scroll_to == Some(sec) {
                    resp.scroll_to_me(Some(egui::Align::TOP));
                    *scroll_to = None;
                }
                ui.add_space(6.0);
                let (sep, _) = ui.allocate_exact_size(vec2(ui.available_width().max(20.0), 1.0), egui::Sense::hover());
                ui.painter().hline(sep.x_range(), sep.center().y, Stroke::new(1.0_f32, theme::BORDER));
                ui.add_space(2.0);
            };

            // ---- Genel
            section_heading(ui, SettingsSection::General, t!("Algemeen", "General", "Genel"), &mut app.settings_scroll_to, true);
            let lang_name = |l: Lang| match l {
                Lang::Tr => "Türkçe",
                Lang::En => "English",
                Lang::De => "Deutsch",
                Lang::Nl => "Nederlands",
                Lang::Fr => "Français",
                Lang::It => "Italiano",
                Lang::Es => "Español",
            };
            theme::setting_row(ui, t!("Taal", "Language"), None, |ui| {
                let mut lang = app.settings.lang;
                egui::ComboBox::from_id_salt("settings_lang")
                    .width(170.0)
                    .selected_text(lang_name(lang))
                    .show_ui(ui, |ui| {
                        for l in [Lang::Tr, Lang::En, Lang::De, Lang::Nl, Lang::Fr, Lang::It, Lang::Es] {
                            ui.selectable_value(&mut lang, l, lang_name(l));
                        }
                    });
                if lang != app.settings.lang {
                    app.settings.lang = lang;
                    i18n::set(lang);
                    *dirty = true;
                }
            });

            theme::setting_row_tip(
                ui,
                t!("Interfaceschaal", "UI scale", "Arayüz ölçeği"),
                Some(t!(
                    "Schaalt automatisch op brede schermen · Ctrl + / − / 0",
                    "Auto-scales on wide screens · Ctrl + / − / 0",
                    "Geniş ekranlarda otomatik büyür · Ctrl + / − / 0"
                )),
                None,
                cfg!(target_os = "macos"),
                |ui| {
                    let mut sc = app.settings.ui_scale;
                    egui::ComboBox::from_id_salt("settings_ui_scale")
                        .width(150.0)
                        .selected_text(sc.label())
                        .show_ui(ui, |ui| {
                            for opt in UiScale::ALL {
                                ui.selectable_value(&mut sc, opt, opt.label());
                            }
                        });
                    if sc != app.settings.ui_scale {
                        app.settings.ui_scale = sc;
                        app.applied_scale_pref = None;
                        *dirty = true;
                    }
                },
            );

            if cfg!(target_os = "macos") {
                theme::setting_row_tip(
                    ui,
                    t!("FM eerst naar voren halen bij uitlezen", "Bring FM to front before reading", "Okumadan önce FM'yi öne getir"),
                    None,
                    None,
                    false,
                    |ui| {
                        *dirty |= theme::toggle(ui, &mut app.settings.capture_delay).changed();
                    },
                );
            }

            // ---- Planlayıcı
            section_heading(ui, SettingsSection::Planner, t!("Planner", "Planner", "Planlayıcı"), &mut app.settings_scroll_to, false);
            use planner::Load;
            let rest_help = match app.settings.planner.load {
                Load::Light => t!(
                    "Elke trainingsdag en de dag voor een wedstrijd eindigt met Rest.",
                    "Every training day and the day before a match ends with Rest."
                ),
                Load::Balanced => t!("Het gewone plan.", "The regular plan."),
                Load::Intense => t!(
                    "De dag na een wedstrijd krijgt na Recovery ook een tactische sessie.",
                    "The day after a match gets a tactical session after Recovery."
                ),
            };
            theme::setting_row(ui, t!("Rust", "Rest", "Dinlenme"), Some(rest_help), |ui| {
                *dirty |= theme::segmented(
                    ui,
                    "planner_load",
                    &mut app.settings.planner.load,
                    &[
                        (Load::Light, t!("Meer rust", "More rest")),
                        (Load::Balanced, t!("Gebalanceerd", "Balanced")),
                        (Load::Intense, t!("Minder rust", "Less rest")),
                    ],
                );
            });

            theme::setting_row(
                ui,
                t!(
                    "Voorbereiding & conditiekamp",
                    "Pre-season & fitness camp",
                    "Sezon öncesi ve kondisyon kampı"
                ),
                Some(app.settings.planner.preseason_intensity.description()),
                |ui| {
                    use planner::PreSeasonIntensity;
                    if theme::segmented(
                        ui,
                        "preseason_intensity",
                        &mut app.settings.planner.preseason_intensity,
                        &[
                            (PreSeasonIntensity::Heavy, PreSeasonIntensity::Heavy.short_label()),
                            (PreSeasonIntensity::Balanced, PreSeasonIntensity::Balanced.short_label()),
                            (PreSeasonIntensity::TacticalFocus, PreSeasonIntensity::TacticalFocus.short_label()),
                        ],
                    ) {
                        *dirty = true;
                        app.settings.preseason_intensity_chosen = true;
                    }
                },
            );

            theme::setting_row(
                ui,
                t!(
                    "Automatisch meer rust in drukke weken",
                    "Auto extra rest in busy weeks",
                    "Yoğun haftalarda otomatik fazla dinlenme"
                ),
                Some(t!(
                    "Wordt actief bij 2+ wedstrijden in één week",
                    "Active in weeks with 2+ matches",
                    "Haftada 2+ maç olduğunda devreye girer"
                )),
                |ui| {
                    *dirty |= theme::toggle(ui, &mut app.settings.planner.auto_light_congested).changed();
                },
            );

            theme::setting_row_tip(
                ui,
                t!("Geen tactische sessies", "No tactical sessions", "Taktik seans koyma"),
                Some(t!(
                    "Tactische vakken gaan naar spelersontwikkeling",
                    "Tactical slots switch to player development",
                    "Taktik yuvaları oyuncu gelişimine gider"
                )),
                Some(t!(
                    "Die plekken gaan naar spelersontwikkeling; wedstrijdvoorbereiding blijft. Bedoeld voor wie het tactisch begrip op 100 % laat houden (Selectie).",
                    "Those slots go to player development; match preparation stays. Meant for use with \"Keep at 100 %\" for tactical understanding (Squad)."
                )),
                true,
                |ui| {
                    *dirty |= theme::toggle(ui, &mut app.settings.planner.no_tactical).changed();
                },
            );

            theme::setting_row_tip(
                ui,
                t!(
                    "Team Bonding naar zondag verplaatsen",
                    "Move Team Bonding to Sunday",
                    "Takım kaynaşmasını Pazar'a taşı"
                ),
                Some(t!(
                    "Verplaatst één Team Bonding-sessie naar de rustdag op zondag",
                    "Moves one Team Bonding session to Sunday's rest day",
                    "Bir Takım Kaynaşması seansını Pazar dinlenme gününe alır"
                )),
                None,
                true,
                |ui| {
                    *dirty |= theme::toggle(ui, &mut app.settings.planner.sunday_team_bonding).changed();
                },
            );

            // Gelişmiş eşikler (varsayılan kapalı)
            ui.add_space(8.0);
            let adv_summary = t!(
                format!(
                    "Binding {:.0} · Conditie {:.0} · Ontwikkeling {:.0} · Standaard {} · Herberekenen {}",
                    app.settings.planner.bonding_below,
                    app.settings.planner.auto_light_below,
                    app.settings.planner.auto_dev_above,
                    app.settings.planner.set_pieces_every,
                    app.settings.planner.replan_every_weeks,
                ),
                format!(
                    "Bonding {:.0} · Condition {:.0} · Dev {:.0} · Set pieces {} · Re-target {}",
                    app.settings.planner.bonding_below,
                    app.settings.planner.auto_light_below,
                    app.settings.planner.auto_dev_above,
                    app.settings.planner.set_pieces_every,
                    app.settings.planner.replan_every_weeks,
                ),
                format!(
                    "Kaynaşma {:.0} · Kondisyon {:.0} · Gelişim {:.0} · Duran top {} · Yenileme {}",
                    app.settings.planner.bonding_below,
                    app.settings.planner.auto_light_below,
                    app.settings.planner.auto_dev_above,
                    app.settings.planner.set_pieces_every,
                    app.settings.planner.replan_every_weeks,
                )
            );
            let adv_hdr = ui
                .horizontal_wrapped(|ui| {
                    let caret = if app.show_adv_thresholds { icon::CARET_DOWN } else { icon::CARET_RIGHT };
                    ui.label(theme::ic(caret).color(theme::TEXT_DIM).size(13.0));
                    ui.label(RichText::new(t!("Geavanceerde drempels", "Advanced thresholds", "Gelişmiş eşikler")).font(egui::FontId::new(13.5, theme::semibold())).color(theme::TEXT));
                    if !app.show_adv_thresholds {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(theme::faint(&adv_summary));
                        });
                    }
                })
                .response
                .interact(egui::Sense::click())
                .on_hover_cursor(egui::CursorIcon::PointingHand);
            if adv_hdr.clicked() {
                app.show_adv_thresholds = !app.show_adv_thresholds;
            }

            if app.show_adv_thresholds {
                ui.add_space(4.0);
                theme::setting_row(
                    ui,
                    t!("Team Bonding-drempel", "Team Bonding threshold", "Takım kaynaşması eşiği"),
                    Some(t!("Wordt gepland onder dit relatiepercentage", "Scheduled below this relationship %", "İlişkiler bu değerin altındayken eklenir")),
                    |ui| {
                        *dirty |= ui.add(egui::DragValue::new(&mut app.settings.planner.bonding_below).range(0.0..=100.0).speed(1.0).fixed_decimals(0).suffix("%")).changed();
                    },
                );
                let cond_tip = app.live.avg_condition().map(|avg| {
                    t!(
                        format!("Huidige selectieconditie: {avg:.0}% (0 = uit)."),
                        format!("Current squad condition: {avg:.0}% (0 = off)."),
                        format!("Mevcut kadro kondisyonu: %{avg:.0} (0 = kapalı).")
                    )
                });
                theme::setting_row_tip(
                    ui,
                    t!("Drempel automatische rust", "Auto rest threshold", "Otomatik dinlenme eşiği"),
                    Some(t!("Bij lagere selectieconditie · 0 = uit", "When squad condition drops below · 0 = off", "Kadro kondisyonu altındaysa · 0 kapalı")),
                    cond_tip.as_deref(),
                    true,
                    |ui| {
                        *dirty |= ui.add(egui::DragValue::new(&mut app.settings.planner.auto_light_below).range(0.0..=95.0).speed(1.0).fixed_decimals(0).suffix("%")).changed();
                    },
                );
                theme::setting_row_tip(
                    ui,
                    t!("Drempel ontwikkelmodus", "Auto development threshold", "Otomatik gelişim eşiği"),
                    Some(t!("Boven dit tactisch begrip · 0 = uit", "Above this tactical understanding · 0 = off", "Tüm 8 taktik alan üstündeyken · 0 kapalı")),
                    Some(t!(
                        "Als alle 8 tactische gebieden boven deze drempel zitten, gaan tactische vakken automatisch naar spelersontwikkeling (0 = uit).",
                        "When all 8 tactical areas are above this threshold, tactical slots automatically switch to player development (0 = off).",
                        "8 taktik alanın tamamı bu eşiğin üstündeyken taktik yuvaları otomatik oyuncu gelişimine döner; biri düşerse tekrar taktiğe geçer (0 = kapalı)."
                    )),
                    true,
                    |ui| {
                        *dirty |= ui.add(egui::DragValue::new(&mut app.settings.planner.auto_dev_above).range(0.0..=100.0).speed(1.0).fixed_decimals(0).suffix("%")).changed();
                    },
                );
                theme::setting_row(
                    ui,
                    t!("Frequentie standaardsituaties", "Set piece frequency", "Duran top sıklığı"),
                    Some(t!("Om de hoeveel weken · 0 = uit", "Every N weeks · 0 = off", "Kaç haftada bir · 0 kapalı")),
                    |ui| {
                        *dirty |= ui.add(egui::DragValue::new(&mut app.settings.planner.set_pieces_every).range(0..=8)).changed();
                    },
                );
                theme::setting_row_tip(
                    ui,
                    t!("Frequentie doelen herzien", "Goal re-target frequency", "Hedef yenileme sıklığı"),
                    Some(t!("Om de hoeveel weken het doel wijzigt", "Re-evaluates priority factors every N weeks", "Kaç haftada bir yeniden hesaplanır")),
                    None,
                    false,
                    |ui| {
                        *dirty |= ui.add(egui::DragValue::new(&mut app.settings.planner.replan_every_weeks).range(1..=8)).changed();
                    },
                );
                ui.add_space(6.0);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if theme::button(
                        ui,
                        icon::ARROW_COUNTER_CLOCKWISE,
                        t!("Standaardwaarden herstellen", "Reset to defaults", "Varsayılanlara dön"),
                        true,
                    )
                    .clicked()
                    {
                        let def = PlannerSettings::default();
                        app.settings.planner.bonding_below = def.bonding_below;
                        app.settings.planner.auto_light_below = def.auto_light_below;
                        app.settings.planner.auto_dev_above = def.auto_dev_above;
                        app.settings.planner.set_pieces_every = def.set_pieces_every;
                        app.settings.planner.replan_every_weeks = def.replan_every_weeks;
                        *dirty = true;
                    }
                });
            }

            // ---- FM bağlantısı
            section_heading(ui, SettingsSection::FmConnection, t!("FM-verbinding", "FM connection", "FM bağlantısı"), &mut app.settings_scroll_to, false);
            ui.add_space(4.0);
            Frame::new()
                .fill(theme::tint(theme::WARNING, 0.08))
                .stroke(Stroke::new(1.0_f32, theme::tint(theme::WARNING, 0.55)))
                .corner_radius(CornerRadius::same(10))
                .inner_margin(Margin::symmetric(14, 10))
                .show(ui, |ui| {
                    ui.set_min_width(ui.available_width());
                    if agent::LIVE_SUPPORTED {
                        theme::setting_row_tip(
                            ui,
                            t!(
                                "Schrijven toestaan op niet-geteste FM-versie",
                                "Allow writes on untested FM build",
                                "Test edilmemiş FM sürümünde yazmaya izin ver"
                            ),
                            Some(t!(
                                "Maak eerst een backup van je save. Geldt alleen voor deze sessie.",
                                "Back up your save first. Applies to this session only.",
                                "Önce kayıt yedeği alın. Yalnızca bu oturum için geçerli."
                            )),
                            Some(t!(
                                "FM27 Training Assistant past alleen iets aan op FM-versies die het kent (Steam op Windows en Mac, Game Pass / Xbox-app op Windows). Op andere versies (na een FM-update, of bijvoorbeeld Epic) staat de app op alleen lezen. Kloppen club, budget en selectie in de app met wat FM laat zien? Dan kun je aanpassen hier toestaan. Elke wijziging wordt eerst gecontroleerd, kan ongedaan worden gemaakt en blijft pas bewaard als je in FM opslaat. Maak eerst een backup van je save.",
                                "FM27 Training Assistant only changes things on FM builds it knows (Steam on Windows and Mac, Game Pass / Xbox app on Windows). On other builds (after an FM update, or Epic for example) the app is read-only. Do club, budget and squad in the app match what FM shows? Then you can allow changes here. Every change is checked first, can be undone and only sticks once you save in FM. Back up your save first."
                            )),
                            false,
                            |ui| {
                                let mut on = app.settings.allow_untested;
                                if theme::toggle(ui, &mut on).changed() {
                                    app.settings.allow_untested = on;
                                    app.live.allow_untested = on;
                                    *dirty = true;
                                    if app.live.connected {
                                        app.live.refresh(ui.ctx(), "", None);
                                    }
                                }
                            },
                        );
                        if app.live.connected {
                            let (c, txt) = if !app.live.untested_build {
                                (theme::SUCCESS, t!("Deze FM-versie is getest.", "This FM build is tested."))
                            } else if app.settings.allow_untested {
                                (theme::WARNING, t!("Niet-geteste FM-versie: aanpassen toegestaan.", "Untested FM build: changes allowed."))
                            } else {
                                (theme::WARNING, t!("Niet-geteste FM-versie: alleen lezen.", "Untested FM build: read-only."))
                            };
                            ui.horizontal(|ui| {
                                theme::status_dot(ui, c);
                                ui.label(RichText::new(txt).size(12.5).color(theme::TEXT));
                            });
                        }
                    } else {
                        ui.label(theme::dim(t!(
                            "Live-verbinding is niet beschikbaar op dit besturingssysteem.",
                            "Live connection is not available on this operating system.",
                            "Canlı FM bağlantısı bu işletim sisteminde kullanılamıyor."
                        )));
                    }
                });

            // ---- Güncellemeler
            section_heading(ui, SettingsSection::Updates, t!("Updates", "Updates", "Güncellemeler"), &mut app.settings_scroll_to, false);
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                theme::pill(ui, &format!("v{}", updater::CURRENT_VERSION), theme::ACCENT);
                let os_label = if cfg!(windows) {
                    "Windows"
                } else if cfg!(target_os = "macos") {
                    "macOS"
                } else {
                    std::env::consts::OS
                };
                theme::pill(ui, os_label, theme::INFO);
                match &app.updater.status {
                    UpdateStatus::Idle => {}
                    UpdateStatus::Checking => {
                        theme::pill(ui, t!("Controleren…", "Checking…", "Denetleniyor…"), theme::INFO);
                    }
                    UpdateStatus::UpToDate => {
                        theme::pill(ui, t!("Up-to-date", "Up to date", "Güncel"), theme::SUCCESS);
                    }
                    UpdateStatus::Available(info) => {
                        theme::pill(
                            ui,
                            &t!(
                                format!("Nieuw: v{}", info.version),
                                format!("Available: v{}", info.version),
                                format!("Yeni: v{}", info.version)
                            ),
                            theme::SUCCESS,
                        );
                    }
                    UpdateStatus::Downloading { version } => {
                        theme::pill(
                            ui,
                            &t!(
                                format!("v{version} downloaden…"),
                                format!("Downloading v{version}…"),
                                format!("v{version} indiriliyor…")
                            ),
                            theme::INFO,
                        );
                    }
                    UpdateStatus::Applying { version } => {
                        theme::pill(
                            ui,
                            &t!(
                                format!("v{version} installeren…"),
                                format!("Installing v{version}…"),
                                format!("v{version} yükleniyor…")
                            ),
                            theme::WARNING,
                        );
                    }
                    UpdateStatus::InstalledManual { .. } => {
                        theme::pill(ui, t!("Gedownload", "Downloaded", "İndirildi"), theme::SUCCESS);
                    }
                    UpdateStatus::Error(_) => {
                        theme::pill(ui, t!("Fout", "Error", "Hata"), theme::DANGER);
                    }
                }
            });

            theme::setting_row_tip(
                ui,
                t!(
                    "Automatische updates",
                    "Automatic updates",
                    "Otomatik güncelleme"
                ),
                Some(t!(
                    "Controleert bij opstarten via GitHub Releases",
                    "Checks GitHub Releases on launch",
                    "Açılışta yeni sürümleri denetler"
                )),
                Some(t!(
                    "Controleert bij opstarten via GitHub Releases en herstart automatisch",
                    "Checks GitHub Releases on launch, downloads the latest package and restarts automatically",
                    "Açılışta GitHub Releases'tan denetler; yeni sürüm varsa indirip kendini otomatik yeniler"
                )),
                false,
                |ui| {
                    *dirty |= theme::toggle(ui, &mut app.settings.auto_update).changed();
                },
            );

            match &app.updater.status {
                UpdateStatus::Error(err) => {
                    ui.add(egui::Label::new(RichText::new(err).size(12.5).color(theme::DANGER)).wrap());
                }
                UpdateStatus::InstalledManual { message } => {
                    ui.add(egui::Label::new(RichText::new(message).size(12.5).color(theme::SUCCESS)).wrap());
                }
                UpdateStatus::Available(info) if !info.notes.is_empty() => {
                    let preview: String = info.notes.lines().take(3).collect::<Vec<_>>().join(" · ");
                    let clipped: String = if preview.chars().count() > 220 {
                        format!("{}…", preview.chars().take(220).collect::<String>())
                    } else {
                        preview
                    };
                    ui.add(egui::Label::new(theme::dim(clipped)).wrap());
                }
                _ => {}
            }
            ui.add_space(6.0);
            ui.horizontal_wrapped(|ui| {
                if theme::button(
                    ui,
                    icon::ARROWS_CLOCKWISE,
                    t!("Controleren op updates", "Check for updates", "Güncellemeleri denetle"),
                    !app.updater.busy(),
                )
                .clicked()
                {
                    *trigger_check = true;
                }
                if let UpdateStatus::Available(info) = &app.updater.status
                    && info.asset.is_some()
                {
                    let lbl = t!(
                        format!("v{} installeren en herstarten", info.version),
                        format!("Install v{} & restart", info.version),
                        format!("v{} yükle ve yeniden başlat", info.version)
                    );
                    if theme::primary_button(ui, icon::SPARKLE, &lbl, !app.updater.busy()).clicked() {
                        *trigger_install = Some(info.clone());
                    }
                }
                if theme::button(ui, icon::EXPORT, "GitHub Releases", true).clicked() {
                    open_path(std::path::Path::new(updater::RELEASES_PAGE));
                }
            });

            // ---- Klasörler
            section_heading(ui, SettingsSection::Folders, t!("Mappen", "Folders", "Klasörler"), &mut app.settings_scroll_to, false);
            let export_path_str = export::output_dir().display().to_string();
            theme::setting_row_tip(ui, t!("Exportmap", "Export folder", "Dışa aktarma klasörü"), None, None, false, |ui| {
                if theme::button(ui, icon::EXPORT, t!("Open map", "Open folder", "Klasörü aç"), true).clicked() {
                    let _ = std::fs::create_dir_all(export::output_dir());
                    open_path(&export::output_dir());
                }
            });
            ui.add(egui::Label::new(RichText::new(&export_path_str).monospace().size(11.5).color(theme::TEXT_DIM)).selectable(true).wrap());
            ui.add_space(6.0);
            let (f_sep, _) = ui.allocate_exact_size(vec2(ui.available_width().max(20.0), 1.0), egui::Sense::hover());
            ui.painter().hline(f_sep.x_range(), f_sep.center().y, Stroke::new(1.0_f32, theme::mix(theme::SURFACE, theme::BORDER, 0.75)));
            ui.add_space(2.0);

            let data_path_str = crate::data_dir().display().to_string();
            theme::setting_row_tip(
                ui,
                t!("Instellingen- en app-map", "Settings & app folder", "Ayarlar ve uygulama klasörü"),
                Some(t!("Bevat settings.json en laatste diagnosebestanden", "Contains settings.json and recent diagnostic files", "settings.json ve son tanılama dosyalarını içerir")),
                None,
                false,
                |ui| {
                    if theme::button(ui, icon::EXPORT, t!("Open map", "Open folder", "Klasörü aç"), true).clicked() {
                        let _ = std::fs::create_dir_all(crate::data_dir());
                        open_path(&crate::data_dir());
                    }
                },
            );
            ui.add(egui::Label::new(RichText::new(&data_path_str).monospace().size(11.5).color(theme::TEXT_DIM)).selectable(true).wrap());

            if cfg!(target_os = "macos") {
                theme::setting_row(ui, t!("Schermopname-toestemming", "Screen Recording permission"), None, |ui| {
                    if theme::button(ui, icon::CAMERA, t!("Open instellingen", "Open settings", "Ayarları aç"), true).clicked() {
                        open_path(std::path::Path::new("x-apple.systempreferences:com.apple.preference.security?Privacy_ScreenCapture"));
                    }
                });
            }

            // ---- Hakkında + Destek (Klasörler altında)
            ui.add_space(14.0);
            let (sep, _) = ui.allocate_exact_size(vec2(ui.available_width().max(20.0), 1.0), egui::Sense::hover());
            ui.painter().hline(sep.x_range(), sep.center().y, Stroke::new(1.0_f32, theme::BORDER));
            ui.add_space(8.0);
            ui.label(RichText::new(t!("Over", "About", "Hakkında")).font(egui::FontId::new(14.0, theme::bold())).color(theme::TEXT));
            ui.add_space(4.0);
            ui.add(
                egui::Label::new(theme::dim(t!(
                    "Alleen training: uitsluitend de trainingskalender wordt geschreven; alle editor- en cheatfuncties zijn verwijderd.",
                    "Training-only build: only the training calendar is written; all editor/cheat tools are removed.",
                    "Saf hilesiz antrenman sürümü: yalnızca antrenman takvimi yazılır, tüm editör/hile araçları kaldırılmıştır."
                )))
                .wrap(),
            );
            ui.add(
                egui::Label::new(theme::faint(t!(
                    "Licentie: GPL-3.0. Sessienamen, limieten en presets komen uit FM27's eigen game-data. Alleen voor single-player. Lettertype: Inter (SIL OFL). Iconen: Phosphor (MIT).",
                    "License: GPL-3.0. Session names, limits and presets come from FM27's own game data. Single-player only. Font: Inter (SIL OFL). Icons: Phosphor (MIT).",
                    "Lisans: GPL-3.0. Seans adları, limitler ve şablonlar FM27'nin kendi oyun verilerinden gelir. Yalnızca tek oyunculu içindir. Yazı tipi: Inter (SIL OFL). Simgeler: Phosphor (MIT)."
                )))
                .wrap(),
            );
            ui.add_space(6.0);
            ui.horizontal_wrapped(|ui| {
                if theme::support_button(
                    ui,
                    icon::COFFEE,
                    t!("Koop een koffie · Buy Me a Coffee", "Buy Me a Coffee · Support", "Kahve ısmarla · Buy Me a Coffee"),
                    false,
                )
                .clicked()
                {
                    open_path(std::path::Path::new(DONATE_URL));
                }
                ui.label(theme::faint("buymeacoffee.com/cilek14"));
            });
        };

        if side_nav {
            let nav_w = if avail >= 1060.0 { 180.0 } else { 156.0 };
            let gap = 20.0;
            let content_w = (avail - nav_w - gap).clamp(280.0, theme::SETTINGS_MAX);
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.allocate_ui_with_layout(
                    vec2(nav_w, 0.0),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        ui.set_min_width(nav_w);
                        ui.set_max_width(nav_w);
                        for (sec, ic, lbl) in sections {
                            if theme::nav_tab(ui, ic, lbl, selected == sec).clicked() {
                                clicked_sec = Some(sec);
                            }
                            ui.add_space(2.0);
                        }
                    },
                );
                ui.add_space(gap);
                ui.allocate_ui_with_layout(
                    vec2(content_w, 0.0),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        ui.spacing_mut().item_spacing.x = 8.0;
                        ui.set_min_width(content_w);
                        ui.set_max_width(content_w);
                        draw_right_body(ui, self, &mut dirty, &mut trigger_check, &mut trigger_install);
                    },
                );
            });
        } else {
            ui.horizontal_wrapped(|ui| {
                for (sec, ic, lbl) in sections {
                    if theme::nav_tab(ui, ic, lbl, selected == sec).clicked() {
                        clicked_sec = Some(sec);
                    }
                }
            });
            ui.add_space(10.0);
            draw_right_body(ui, self, &mut dirty, &mut trigger_check, &mut trigger_install);
        }

        if let Some(sec) = clicked_sec {
            selected = sec;
            self.settings_section = selected;
            self.settings_scroll_to = Some(sec);
        }
        if trigger_check {
            self.updater.start_check(&ctx);
        }
        if let Some(info) = trigger_install {
            self.updater.start_install(&ctx, info);
        }
        if dirty {
            self.save_settings();
            self.week_plan = None;
            self.year.clear();
        }
    }

    fn render_camp_modal(&mut self, ctx: &egui::Context) {
        if !self.show_camp_modal {
            return;
        }
        let screen = ctx.screen_rect();
        egui::Area::new(egui::Id::new("camp_modal_backdrop"))
            .fixed_pos(screen.min)
            .order(egui::Order::Middle)
            .show(ctx, |ui| {
                let (rect, resp) = ui.allocate_exact_size(screen.size(), egui::Sense::click());
                ui.painter().rect_filled(rect, CornerRadius::ZERO, egui::Color32::from_rgba_unmultiplied(6, 10, 13, 185));
                if resp.clicked() {
                    self.show_camp_modal = false;
                }
            });

        let mut close_now = false;
        let mut apply_now = false;
        egui::Window::new("camp_modal_win")
            .title_bar(false)
            .resizable(false)
            .collapsible(false)
            .order(egui::Order::Foreground)
            .anchor(egui::Align2::CENTER_CENTER, vec2(0.0, 0.0))
            .default_width(480.0)
            .max_width(480.0)
            .frame(
                Frame::new()
                    .fill(theme::SURFACE)
                    .stroke(Stroke::new(1.2_f32, theme::tint(theme::ACCENT, 0.72)))
                    .corner_radius(CornerRadius::same(14))
                    .inner_margin(Margin::same(22)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(theme::ic(icon::LIGHTNING).color(theme::ACCENT_HI).size(22.0));
                    ui.label(
                        RichText::new(t!(
                            "Hoe wil je het conditiekamp indelen?",
                            "How should the fitness camp run?",
                            "Kondisyon kampı nasıl yapılsın?"
                        ))
                        .font(egui::FontId::new(18.0, theme::bold()))
                        .color(theme::TEXT),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if theme::small_button(ui, icon::X, true).clicked() {
                            close_now = true;
                        }
                    });
                });

                ui.add_space(6.0);
                ui.add(
                    egui::Label::new(theme::dim(t!(
                        "Kies de intensiteit voor voorbereidingsweken en het conditiekamp. Je kunt dit later altijd wijzigen.",
                        "Choose the intensity for pre-season weeks and the fitness camp. You can change this anytime.",
                        "Sezon öncesi haftalar ve kondisyon kampı için yoğunluk tarzını seçin. Bunu daha sonra istediğiniz zaman değiştirebilirsiniz."
                    )))
                    .wrap(),
                );
                ui.add_space(12.0);

                for (idx, opt) in planner::PreSeasonIntensity::ALL.into_iter().enumerate() {
                    let (col, ic_name) = match opt {
                        planner::PreSeasonIntensity::Heavy => (theme::WARNING, icon::LIGHTNING),
                        planner::PreSeasonIntensity::Balanced => (theme::ACCENT, icon::SCALES),
                        planner::PreSeasonIntensity::TacticalFocus => (theme::INFO, icon::COMPASS),
                    };
                    let selected = self.camp_modal_draft == opt;
                    let bg = if selected { theme::tint(col, 0.14) } else { theme::SURFACE2 };
                    let border = if selected { col } else { theme::BORDER };
                    let stroke_w = if selected { 1.5_f32 } else { 1.0_f32 };

                    let frame_resp = Frame::new()
                        .fill(bg)
                        .stroke(Stroke::new(stroke_w, border))
                        .corner_radius(CornerRadius::same(10))
                        .inner_margin(Margin::symmetric(14, 10))
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            ui.horizontal(|ui| {
                                ui.label(theme::ic(ic_name).color(col).size(17.0));
                                ui.label(
                                    RichText::new(opt.label())
                                        .font(egui::FontId::new(14.5, theme::bold()))
                                        .color(theme::TEXT),
                                );
                                if selected {
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        ui.label(theme::ic(icon::CHECK_CIRCLE).color(col).size(16.0));
                                    });
                                }
                            });
                            ui.add_space(4.0);
                            ui.add(egui::Label::new(theme::dim(opt.description())).wrap());
                        });

                    let click_resp = ui
                        .interact(frame_resp.response.rect, egui::Id::new(("camp_opt", idx)), egui::Sense::click())
                        .on_hover_cursor(egui::CursorIcon::PointingHand);
                    if click_resp.clicked() {
                        self.camp_modal_draft = opt;
                    }
                    ui.add_space(8.0);
                }

                ui.add_space(6.0);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if theme::primary_button(
                        ui,
                        icon::CHECK,
                        t!("Opslaan en toepassen", "Save & apply", "Kaydet ve uygula"),
                        true,
                    )
                    .clicked()
                    {
                        apply_now = true;
                    }
                    if theme::small_button(ui, t!("Annuleren", "Cancel", "İptal"), true).clicked() {
                        close_now = true;
                    }
                });
            });

        if apply_now {
            self.settings.planner.preseason_intensity = self.camp_modal_draft;
            self.settings.preseason_intensity_chosen = true;
            self.save_settings();
            self.week_plan = None;
            self.year.clear();
            self.show_camp_modal = false;
            if self.live.connected && !self.live.busy() {
                self.live.training(ctx, &self.eff, &self.settings.planner, false, false);
            }
        } else if close_now || ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.show_camp_modal = false;
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.snapshot(ctx);
        self.poll_analysis(ctx);
        self.updater.tick(ctx, self.settings.auto_update);
        self.live.youth_dev_mode = self.settings.youth_dev_mode;
        // automatisch uitlezen: niet tijdens handmatig aanpassen en nooit tijdens een testopname
        if self.settings.auto_read
            && !self.edit_mode
            && self.analysis_rx.is_none()
            && std::env::var_os("FM27_SNAPSHOT").is_none()
            && self.last_auto.elapsed().as_secs() >= self.settings.auto_read_secs.max(5) as u64
        {
            self.last_auto = std::time::Instant::now();
            self.auto_pending = true;
            self.start_analysis(ctx, None);
        }
        self.live.allow_untested = self.settings.allow_untested;
        self.live.choice = (self.settings.manager_club, self.active_team_slot());
        self.live.keep_understanding = false;
        self.live.tick(ctx, &self.eff, &self.settings.planner, self.settings.auto_training);
        if !self.settings.preseason_intensity_chosen
            && !self.camp_prompted_this_session
            && !self.show_camp_modal
            && std::env::var_os("FM27_SNAPSHOT").is_none()
            && self
                .live
                .training
                .as_ref()
                .is_some_and(|t| t.weeks.iter().any(|w| !w.rest && w.kind.is_preseason()))
        {
            self.camp_prompted_this_session = true;
            self.camp_modal_draft = self.settings.planner.preseason_intensity;
            self.show_camp_modal = true;
        }
        // BepInEx köprüsünden gelen kesin Taktiksel Verimlilik verisini otomatik izle (1.5 sn aralıkla)
        if !self.edit_mode
            && std::env::var_os("FM27_SNAPSHOT").is_none()
            && self.last_bepinex_check.elapsed().as_millis() >= 1500
        {
            self.last_bepinex_check = std::time::Instant::now();
            self.sync_from_bepinex(false);
            self.individual.poll_result();
            self.individual.maybe_auto_apply(&self.live);
        }
        self.last_memory_rev = self.live.memory_rev;
        let _ = (self.settings.auto_memory_sync, self.memory_sync_pending);
        let minimized = ctx.input(|i| i.viewport().minimized == Some(true));
        if minimized && std::env::var_os("FM27_SNAPSHOT").is_none() {
            ctx.request_repaint_after(std::time::Duration::from_secs(5));
            return;
        }
        self.apply_ui_scale(ctx);
        self.topbar(ctx);
        egui::CentralPanel::default()
            .frame(Frame::new().fill(theme::BG).inner_margin(Margin { left: 28, right: 28, top: 20, bottom: 18 }))
            .show(ctx, |ui| {
                ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                    let max_w = if self.page == Page::Settings {
                        theme::SETTINGS_MAX + 200.0
                    } else {
                        theme::CONTENT_MAX
                    };
                    theme::centered_column(ui, max_w, |ui| {
                        // pagina komt zacht binnen: kort vervagen en een paar pixels omhoog schuiven
                        let minimized = ctx.input(|i| i.viewport().minimized == Some(true));
                        if self.shown_page != Some(self.page) {
                            self.shown_page = Some(self.page);
                            theme::restart_enter(ctx);
                        }
                        let t = if minimized { 1.0 } else { theme::enter(ctx, 0.22) };
                        ui.set_opacity(0.25 + 0.75 * t);
                        ui.add_space(8.0 * (1.0 - t));
                        self.render_global_strip(ui);
                        if self.show_setup {
                            self.render_setup_flow(ui, ctx);
                        }
                        self.render_update_banner(ui, ctx);
                        if !matches!(self.page, Page::Settings) {
                            self.render_team_bar(ui, ctx);
                        }
                        match self.page {
                            Page::Home => {
                                let hkey = self.club_history_key();
                                let diff = self.history.previous_diff(&hkey, &self.eff);
                                let hhmm = self.last_read_hhmm().map(str::to_string);
                                let has_data = self.has_data();
                                let (msg, err) = (self.status.clone(), self.status_error);
                                let act = crate::home::render(
                                    ui,
                                    &self.eff,
                                    has_data,
                                    hhmm.as_deref(),
                                    diff,
                                    &self.settings.planner,
                                    &self.live,
                                    &self.individual,
                                    &msg,
                                    err,
                                );
                                if act.refresh_data {
                                    self.refresh_data_auto(ctx);
                                }
                                if act.start_fm {
                                    self.live.start_fm(ctx);
                                }
                                if act.reconnect {
                                    self.live.refresh(ctx, t!("verbinden", "connect"), None);
                                }
                                if act.apply_plan {
                                    self.live.training(ctx, &self.eff, &self.settings.planner, true, false);
                                }
                                if let Some(batch) = act.apply_attention {
                                    let mgr_id = self
                                        .live
                                        .club
                                        .as_ref()
                                        .and_then(|c| c.get("manager_runtime_id"))
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(0) as u32;
                                    match individual::write_intensity_command(mgr_id, &batch) {
                                        Ok(n) => {
                                            for (rid, lvl) in &batch {
                                                self.individual.current_intensities.insert(*rid, *lvl);
                                            }
                                            self.set_status(
                                                t!(
                                                    format!("Opdracht voor {n} spelers verstuurd naar FM27…"),
                                                    format!("Sent intensity command for {n} players to FM27…"),
                                                    format!("{n} oyuncu için yoğunluk komutu FM27'ye gönderildi…")
                                                ),
                                                false,
                                            );
                                        }
                                        Err(e) => self.set_status(e, true),
                                    }
                                }
                            }
                            Page::Overview => self.page_overview(ui, ctx),
                            Page::Calendar => self.page_calendar(ui, ctx),
                            Page::Individual => {
                                let mut read_bep = false;
                                let mut save_auto = false;
                                individual::render_page(
                                    ui,
                                    ctx,
                                    &mut self.individual,
                                    &mut self.live,
                                    &mut read_bep,
                                    &mut save_auto,
                                );
                                if read_bep {
                                    self.sync_from_bepinex(true);
                                }
                                if save_auto {
                                    self.settings.auto_individual_intensity = self.individual.auto_apply_intensity;
                                    self.settings.intensity_profile = self.individual.profile;
                                    self.settings.role_training_mode = self.individual.role_training_mode;
                                    self.save_settings();
                                }
                            }
                            Page::Tactics => self.page_tactics(ui, ctx),
                            Page::Settings => self.page_settings(ui),
                        }
                        ui.add_space(20.0);
                    });
                });
            });
        self.render_camp_modal(ctx);
        let minimized = ctx.input(|i| i.viewport().minimized == Some(true));
        if minimized {
            ctx.request_repaint_after(std::time::Duration::from_secs(5));
        } else if self.analysis_rx.is_some() || self.live.busy() || self.updater.busy() {
            ctx.request_repaint_after(std::time::Duration::from_millis(100));
        } else if self.settings.auto_read {
            ctx.request_repaint_after(std::time::Duration::from_secs(1));
        } else {
            ctx.request_repaint_after(std::time::Duration::from_secs(5));
        }
    }
}

// ------------------------------------------------------------------------ helpers

/// Destek sayfası (Yan menü ve Ayarlar).
const DONATE_URL: &str = "https://buymeacoffee.com/cilek14";

fn open_path(p: &std::path::Path) {
    #[cfg(target_os = "macos")]
    let _ = std::process::Command::new("open").arg(p).spawn();
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        let s = p.to_string_lossy();
        if s.starts_with("http://") || s.starts_with("https://") {
            let _ = std::process::Command::new("cmd")
                .args(["/C", "start", "", &s])
                .creation_flags(CREATE_NO_WINDOW)
                .spawn();
        } else {
            let _ = std::process::Command::new("explorer.exe").arg(p).spawn();
        }
    }
}

fn money(v: i64) -> String {
    let a = v.unsigned_abs() as f64;
    let s = if a >= 1e6 { format!("{:.1} M", a / 1e6) } else if a >= 1e3 { format!("{:.0}k", a / 1e3) } else { format!("{a:.0}") };
    if v < 0 { format!("-{s}") } else { s }
}

/// De acht onderdelen van FM's Tactical Understanding, in de volgorde van de module.
const UNDERSTANDING_AREAS: [&str; 8] = ["Passing Style", "Build Up", "Chance Creation", "In Possession Shape", "Line of Engagement", "Pressing Intensity", "Pressing Trap", "Out Of Possession Shape"];

fn morale_label(m: u64) -> &'static str {
    match m {
        0..=4 => "Abysmal",
        5..=7 => "Very Poor",
        8..=10 => "Poor",
        11..=13 => "Okay",
        14..=16 => "Good",
        17..=18 => "Very Good",
        _ => "Superb",
    }
}

/// Korte, leesbare sessienaam voor in chips.
pub(crate) fn chip_name(name: &str) -> String {
    name.replace("Out Of Possession", "OOP")
        .replace("In Possession", "IP")
        .replace("Defensive", "Def.")
        .replace("Attacking", "Att.")
        .replace("Possession Ball Work", "Poss. Ball Work")
        .replace("Conditioning", "Cond.")
        .replace("Individual Development", "Individual")
        .replace("Line of Engagement", "Line of Eng.")
        .replace("Pressing Intensity", "Press. Intensity")
        .replace("Topa Sahipken Diziliş", "Topla Diziliş")
        .replace("Topsuz Alanda Diziliş", "Topsuz Diziliş")
        .replace("Taktik Anlayış (Topla)", "Taktik (Topla)")
        .replace("Taktik Anlayış (Topsuz)", "Taktik (Topsuz)")
        .replace("Hücum Topla Çalışma", "Hücum Top")
        .replace("Savunma Topla Çalışma", "Savunma Top")
        .replace("Hafif Kondisyon", "Hafif Kond.")
        .replace("Ağır Kondisyon", "Ağır Kond.")
}

fn chip(ui: &mut Ui, name: &str, highlight: bool, faded: bool) {
    let col = if name == "Match" { theme::DANGER } else { theme::rgb(category_of(name).color()) };
    let col = if faded { theme::tint(col, 0.45) } else { col };
    let disp = crate::model::session_display_name(name);
    let hover = if disp != name { format!("{disp} ({name})") } else { name.to_string() };
    theme::session_chip_bar(ui, &chip_name(disp), col, highlight, name == "Match").on_hover_text(hover);
    ui.add_space(3.0);
}

fn readable_plan_title(focus: &str, targets: &[(Factor, u32)], no_tactical: bool, kind: WeekKind) -> String {
    let base = if matches!(kind, WeekKind::OffSeason) || focus == "Full Week of Rest" {
        t!("Rustweek", "Rest week", "Dinlenme haftası")
    } else if no_tactical {
        t!("Ontwikkelweek", "Development week", "Gelişim haftası")
    } else {
        match focus {
            "In Possession" => t!("Balbezit-week", "In-possession week", "Topa sahipken haftası"),
            "Out Of Possession" => t!("Balverlies-week", "Out-of-possession week", "Topsuz oyun haftası"),
            _ => t!("Gebalanceerde week", "Balanced week", "Dengeli hafta"),
        }
    };
    let mut ranked: Vec<(Factor, u32)> = targets.iter().filter(|(_, n)| *n > 0).copied().collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1));
    let top: Vec<&str> = ranked.into_iter().take(2).map(|(f, _)| f.display_label()).collect();
    if top.is_empty() || matches!(kind, WeekKind::OffSeason) || focus == "Full Week of Rest" {
        base.to_string()
    } else {
        format!("{base} · {}", top.join(" + "))
    }
}

fn week_preview_targets(planned: &[[u16; 3]]) -> Vec<(Factor, u32)> {
    let mut counts: Vec<(Factor, u32)> = Vec::new();
    for f in Factor::TACTICAL_AREAS {
        let lbl = f.label();
        let n = planned
            .iter()
            .flatten()
            .filter(|c| session_by_code(**c) == Some(lbl))
            .count() as u32;
        if n > 0 {
            counts.push((f, n));
        }
    }
    counts
}

fn legend_tooltip_text() -> String {
    use crate::model::Category::*;
    let cats = [TacticalUnderstanding, MatchPreparation, FootballingDevelopment, PhysicalRecovery, ExtraCurricular, Fixed]
        .iter()
        .map(|c| format!("• {}", c.display_label()))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "{cats}\n\n{}",
        t!(
            "Omlijnd = bijgewerkt door de app · schild = automatisch hersteld · doorgestreept = plan van FM",
            "Outlined = set by assistant · shield = auto-reverted from FM assistant · struck = what FM planned",
            "Çerçeveli = uygulama tarafından ayarlandı · kalkan = otomatik geri alındı · üstü çizili = FM'nin planı"
        )
    )
}

fn codes_text(c: &[u16; 3]) -> String {
    c.iter()
        .map(|x| {
            let name = session_by_code(*x).unwrap_or("?");
            chip_name(crate::model::session_display_name(name))
        })
        .collect::<Vec<_>>()
        .join(" · ")
}

fn week_card(
    ui: &mut Ui,
    w: &live_training::WeekPreview,
    today: Option<NaiveDate>,
    fm_had: &std::collections::HashMap<NaiveDate, [u16; 3]>,
    reverted_days: &std::collections::HashSet<NaiveDate>,
    no_tactical: bool,
    has_data: bool,
) {
    theme::card(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            let targets = week_preview_targets(&w.planned);
            let title = readable_plan_title(&w.focus, &targets, no_tactical, w.kind);
            let hdr = format!("{} · {title}", t!(format!("Week van {}", i18n::date_short(w.start)), format!("Week of {}", i18n::date_short(w.start))));
            let lbl = ui.label(theme::h2(hdr));
            if !w.name.is_empty() {
                lbl.on_hover_text(&w.name);
            }
            if !has_data {
                theme::pill(
                    ui,
                    t!(
                        "Voorbeeldplan · wordt vernieuwd zodra echte data is gelezen",
                        "Sample plan · updates once real data is read",
                        "Örnek plan · gerçek veriler okunduğunda yenilenir"
                    ),
                    theme::WARNING,
                );
            }
            if w.rest {
                theme::pill(ui, t!("Rustweek: blijft van FM", "Rest week: kept from FM"), theme::WARNING);
                return;
            }
            theme::pill(ui, &t!(format!("{} wedstrijd(en)", w.matches), format!("{} match(es)", w.matches)), theme::DANGER);
            if matches!(w.kind, WeekKind::InternationalBreak | WeekKind::WinterBreak | WeekKind::PreSeasonEarly | WeekKind::PreSeasonMid | WeekKind::PreSeasonLate) {
                theme::pill(ui, w.kind.label(), theme::INFO);
            }
            if no_tactical && w.focus != "Full Week of Rest" {
                let focus_label = match w.focus.as_str() {
                    "In Possession" => t!("Balbezit", "In Possession", "Topa Sahipken"),
                    "Out Of Possession" => t!("Balverlies", "Out Of Possession", "Topsuz Alanda"),
                    _ => t!("Gebalanceerd", "Balanced", "Dengeli"),
                };
                theme::pill(ui, focus_label, theme::ACCENT);
            }
            let pending = (0..7).filter(|d| w.current[*d] != w.planned[*d]).count();
            if w.written > 0 {
                theme::pill(ui, &t!(format!("{} dagen teruggezet", w.written), format!("{} days restored", w.written)), theme::SUCCESS);
            } else if pending > 0 {
                theme::pill(ui, &t!(format!("{pending} dagen wijken af van je plan"), format!("{pending} days differ from your plan")), theme::WARNING);
            } else {
                theme::pill(ui, t!("volgens plan", "on plan"), theme::SUCCESS);
            }
        });
        if w.rest {
            return;
        }
        ui.add_space(8.0);
        ui.columns(7, |cols| {
            for (d, col_ui) in cols.iter_mut().enumerate() {
                let date = w.start + Duration::days(d as i64);
                let past = today.is_some_and(|t| date <= t);
                col_ui.horizontal(|ui| {
                    ui.label(theme::col_header(i18n::weekday_short(d)));
                    if reverted_days.contains(&date) {
                        ui.label(theme::ic(icon::SHIELD_CHECK).color(theme::SUCCESS).size(13.0))
                            .on_hover_text(t!(
                                "Teruggezet nadat FM-assistent deze dag wijzigde",
                                "Reverted after FM assistant changed this day",
                                "FM asistanı bu günü değiştirdi, geri alındı"
                            ));
                    }
                });
                col_ui.label(theme::faint(i18n::date_short(date)));
                col_ui.add_space(3.0);
                // wat FM wilde: nu afwijkend (wordt aangepast) of eerder teruggezet
                let differs = w.current[d] != w.planned[d];
                let was = if differs { Some(w.current[d]) } else { fm_had.get(&date).copied().filter(|h| *h != w.planned[d]) };
                for code in w.planned[d] {
                    let name = session_by_code(code).unwrap_or("?");
                    chip(col_ui, name, was.is_some() && !past, past || !w.editable[d]);
                }
                if let Some(h) = was.filter(|_| !past && w.editable[d]) {
                    col_ui
                        .add(egui::Label::new(RichText::new(format!("FM: {}", codes_text(&h))).size(10.5).color(theme::TEXT_FAINT).strikethrough()).truncate())
                        .on_hover_text(t!(
                            format!("FM had ingepland: {}", codes_text(&h)),
                            format!("FM had planned: {}", codes_text(&h))
                        ));
                }
            }
        });
        // Haftalık etki özeti: 59 FM27 seansının gerçek etki vektörü [Kondisyon, Yorgunluk, Mutluluk, Sakatlık Riski, Keskinlik]
        let mut eff_sum = [0i32; 5];
        for d in 0..7 {
            for code in w.planned[d] {
                if code == 500 || code == 402 {
                    continue;
                }
                if let Some(s) = session_by_code(code).and_then(crate::model::session) {
                    for k in 0..5 {
                        eff_sum[k] += s.effects[k] as i32;
                    }
                }
            }
        }
        let fmt_signed = |v: i32| if v > 0 { format!("+{v}") } else { format!("{v}") };
        ui.add_space(8.0);
        ui.horizontal_wrapped(|ui| {
            let cond_col = if eff_sum[0] >= -4 { theme::SUCCESS } else if eff_sum[0] >= -10 { theme::WARNING } else { theme::DANGER };
            let fat_col = if eff_sum[1] >= 0 { theme::SUCCESS } else if eff_sum[1] >= -4 { theme::WARNING } else { theme::DANGER };
            let hap_col = if eff_sum[2] >= 2 { theme::SUCCESS } else if eff_sum[2] >= -2 { theme::INFO } else { theme::WARNING };
            let inj_col = if eff_sum[3] >= -8 { theme::SUCCESS } else if eff_sum[3] >= -14 { theme::WARNING } else { theme::DANGER };
            let shp_col = if eff_sum[4] >= 4 { theme::SUCCESS } else if eff_sum[4] >= 0 { theme::INFO } else { theme::WARNING };
            theme::pill(ui, &format!("{} {}", t!("Conditie", "Condition", "Kondisyon"), fmt_signed(eff_sum[0])), cond_col);
            theme::pill(ui, &format!("{} {}", t!("Vermoeidheid", "Fatigue", "Yorgunluk"), fmt_signed(eff_sum[1])), fat_col);
            theme::pill(ui, &format!("{} {}", t!("Geluk", "Happiness", "Mutluluk"), fmt_signed(eff_sum[2])), hap_col);
            theme::pill(ui, &format!("{} {}", t!("Blessurerisico", "Injury risk", "Sakatlık riski"), fmt_signed(eff_sum[3])), inj_col);
            theme::pill(ui, &format!("{} {}", t!("Scherpte", "Sharpness", "Keskinlik"), fmt_signed(eff_sum[4])), shp_col);
        });
    });
}

fn plan_grid(ui: &mut Ui, p: &WeekPlan) {
    ui.columns(7, |cols| {
        for (i, d) in p.days.iter().enumerate() {
            let ui = &mut cols[i];
            ui.label(theme::col_header(i18n::weekday_short(i)));
            let sub = match d.role {
                planner::DayRole::Training => i18n::date_short(d.date),
                _ => format!("{} · {}", i18n::date_short(d.date), d.role.label()),
            };
            ui.allocate_ui_with_layout(
                vec2(ui.available_width(), 32.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    ui.set_min_height(32.0);
                    ui.add(egui::Label::new(theme::faint(sub)).wrap());
                },
            );
            ui.add_space(3.0);
            for s in &d.slots {
                chip(ui, s, false, false);
            }
        }
    });
}

fn kind_combo(ui: &mut Ui, id: &str, kind: &mut WeekKind) {
    egui::ComboBox::from_id_salt(id).selected_text(kind.label()).width(170.0).show_ui(ui, |ui| {
        for k in WeekKind::ALL {
            ui.selectable_value(kind, k, k.label());
        }
    });
}

fn weekday_combo(ui: &mut Ui, id: &str, day: &mut Weekday) {
    let names = [(Weekday::Fri, t!("vrijdag", "Friday")), (Weekday::Sat, t!("zaterdag", "Saturday")), (Weekday::Sun, t!("zondag", "Sunday"))];
    let cur = names.iter().find(|(d, _)| d == day).map(|(_, n)| *n).unwrap_or("—");
    egui::ComboBox::from_id_salt(id).selected_text(cur).show_ui(ui, |ui| {
        for (d, n) in names {
            ui.selectable_value(day, d, n);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readable_plan_title_formats_focus_and_top_two_targets() {
        let targets = [
            (Factor::PassingStyle, 3),
            (Factor::InPossessionShape, 4),
            (Factor::PressingTrap, 1),
        ];
        let s = readable_plan_title("Balanced", &targets, false, WeekKind::Season);
        assert!(s.contains('·'), "expected middle dot separator in {s}");
        assert!(s.contains('+'), "expected + between top two factors in {s}");
        assert!(!s.contains("TE "), "template code should stay hidden from {s}");

        let rest = readable_plan_title("Full Week of Rest", &targets, false, WeekKind::OffSeason);
        assert!(!rest.contains('+'), "rest week title should stay concise: {rest}");
    }

    #[test]
    fn ui_scale_auto_breakpoints_and_serde_default() {
        assert!((UiScale::Auto.factor(1320.0) - 1.0).abs() < 0.001);
        assert!((UiScale::Auto.factor(2200.0) - 1.0).abs() < 0.001);
        assert!((UiScale::Auto.factor(2560.0) - 1.15).abs() < 0.001);
        assert!((UiScale::Auto.factor(3000.0) - 1.25).abs() < 0.001);

        let mut value = serde_json::to_value(Settings::default()).expect("serialize default Settings");
        if let Some(obj) = value.as_object_mut() {
            obj.remove("ui_scale");
        }
        let old_json = serde_json::to_string(&value).expect("stringify legacy Settings JSON");
        let decoded: Settings = serde_json::from_str(&old_json).expect("backward-compatible Settings decode");
        assert_eq!(decoded.ui_scale, UiScale::Auto);
    }
}
