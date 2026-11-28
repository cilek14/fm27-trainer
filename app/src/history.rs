//! Taktiksel Verimlilik gelişim geçmişi ve trend grafiği (kulüp bazlı `history.json`).

use chrono::NaiveDate;
use eframe::egui::{self, Color32, FontId, Pos2, Rect, Sense, Stroke, Ui, vec2};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::model::{Component, Effectiveness, Factor};
use crate::t;
use crate::theme::{self, icon};

const MAX_POINTS: usize = 26;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HistoryPoint {
    pub label: String,
    pub game_date: Option<NaiveDate>,
    pub recorded_utc: String,
    pub components: [f32; 4],
    pub factors: [f32; 17],
}

impl HistoryPoint {
    pub fn from_eff(eff: &Effectiveness) -> Self {
        let label = eff
            .game_date
            .map(crate::i18n::date_short)
            .unwrap_or_else(|| chrono::Local::now().format("%d.%m").to_string());
        let mut components = [50.0; 4];
        for (i, c) in Component::ALL.iter().enumerate() {
            components[i] = eff.component(*c);
        }
        let mut factors = [50.0; 17];
        for (i, f) in Factor::ALL.iter().enumerate() {
            factors[i] = eff.get(*f);
        }
        HistoryPoint {
            label,
            game_date: eff.game_date,
            recorded_utc: chrono::Utc::now().to_rfc3339(),
            components,
            factors,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.components.iter().all(|&c| c > 0.0) && self.factors.iter().all(|&f| f > 0.0)
    }

    fn differs(&self, other: &Self) -> bool {
        self.components.iter().zip(&other.components).any(|(a, b)| (a - b).abs() >= 0.5)
            || self.factors.iter().zip(&other.factors).any(|(a, b)| (a - b).abs() >= 0.5)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct HistoryDiff {
    pub components: [f32; 4],
    pub factors: [f32; 17],
}

#[derive(Default, Serialize, Deserialize)]
pub struct HistoryStore {
    #[serde(default)]
    pub clubs: HashMap<String, Vec<HistoryPoint>>,
}

fn history_file() -> PathBuf {
    crate::data_dir().join("history.json")
}

impl HistoryStore {
    pub fn load() -> Self {
        let mut store: Self = std::fs::read_to_string(history_file())
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default();
        store.sanitize();
        store
    }

    pub fn sanitize(&mut self) {
        for list in self.clubs.values_mut() {
            list.retain(|pt| pt.components.iter().all(|&c| c > 0.0));
        }
    }

    pub fn save(&self) {
        let _ = std::fs::create_dir_all(crate::data_dir());
        let p = history_file();
        let tmp = p.with_extension("json.tmp");
        if let Ok(json) = serde_json::to_string_pretty(self) {
            if std::fs::write(&tmp, json).is_ok() {
                let _ = std::fs::rename(&tmp, &p);
            }
        }
    }

    pub fn points(&self, club_key: &str) -> &[HistoryPoint] {
        self.clubs.get(club_key).map(Vec::as_slice).unwrap_or(&[])
    }

    pub fn record(&mut self, club_key: &str, eff: &Effectiveness) -> bool {
        if eff.source.trim().is_empty() {
            return false;
        }
        let pt = HistoryPoint::from_eff(eff);
        if !pt.is_valid() {
            return false;
        }
        let list = self.clubs.entry(club_key.to_string()).or_default();
        list.retain(|p| p.components.iter().all(|&c| c > 0.0));

        let changed = match list.last_mut() {
            None => {
                list.push(pt);
                true
            }
            Some(last) if last.game_date.is_some() && last.game_date == pt.game_date => {
                if last.differs(&pt) {
                    *last = pt;
                    true
                } else {
                    false
                }
            }
            Some(last) => {
                let date_moved = pt.game_date.is_some() && last.game_date != pt.game_date;
                if date_moved || last.differs(&pt) {
                    list.push(pt);
                    true
                } else {
                    false
                }
            }
        };

        if list.len() > MAX_POINTS {
            let drain = list.len() - MAX_POINTS;
            list.drain(..drain);
        }
        changed
    }

    pub fn previous_diff(&self, club_key: &str, eff: &Effectiveness) -> Option<HistoryDiff> {
        let cur = HistoryPoint::from_eff(eff);
        if !cur.components.iter().all(|&c| c > 0.0) {
            return None;
        }
        let valid: Vec<&HistoryPoint> = self
            .points(club_key)
            .iter()
            .filter(|pt| pt.components.iter().all(|&c| c > 0.0))
            .collect();
        let prev = match valid.len() {
            0 => return None,
            1 => {
                if !valid[0].differs(&cur) {
                    return None;
                }
                valid[0]
            }
            n => {
                if !valid[n - 1].differs(&cur) {
                    valid[n - 2]
                } else {
                    valid[n - 1]
                }
            }
        };

        let mut components = [0.0; 4];
        for i in 0..4 {
            components[i] = cur.components[i] - prev.components[i];
        }
        let mut factors = [0.0; 17];
        for i in 0..17 {
            factors[i] = cur.factors[i] - prev.factors[i];
        }
        Some(HistoryDiff { components, factors })
    }
}

const COMP_COLORS: [Color32; 4] = [
    theme::ACCENT_HI,
    theme::SUCCESS,
    theme::INFO,
    theme::WARNING,
];

fn build_x_labels(points: &[HistoryPoint]) -> Vec<String> {
    let parsed: Vec<(String, Option<String>)> = points
        .iter()
        .map(|pt| {
            let utc_local = chrono::DateTime::parse_from_rfc3339(&pt.recorded_utc)
                .ok()
                .map(|dt| dt.with_timezone(&chrono::Local));
            let day_key = pt
                .game_date
                .map(crate::i18n::date_short)
                .or_else(|| utc_local.as_ref().map(|dt| dt.format("%d.%m").to_string()))
                .unwrap_or_else(|| pt.label.clone());
            let hhmm = utc_local.map(|dt| dt.format("%H:%M").to_string());
            (day_key, hhmm)
        })
        .collect();

    let mut counts: HashMap<&str, usize> = HashMap::new();
    for (day, _) in &parsed {
        *counts.entry(day.as_str()).or_insert(0) += 1;
    }

    parsed
        .iter()
        .map(|(day, hhmm)| {
            if counts.get(day.as_str()).copied().unwrap_or(0) > 1
                && let Some(t) = hhmm
            {
                format!("{day} {t}")
            } else {
                day.clone()
            }
        })
        .collect()
}

pub fn render_trend_card(ui: &mut Ui, points: &[HistoryPoint], eff: &Effectiveness, diff: Option<HistoryDiff>) {
    let mut plot_points: Vec<HistoryPoint> = points
        .iter()
        .filter(|pt| pt.components.iter().all(|&c| c > 0.0))
        .cloned()
        .collect();
    let cur = HistoryPoint::from_eff(eff);
    if !eff.source.trim().is_empty()
        && cur.components.iter().all(|&c| c > 0.0)
        && plot_points.last().is_none_or(|last| last.differs(&cur))
    {
        plot_points.push(cur);
    }

    theme::card(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.label(theme::ic(icon::GAUGE).color(theme::ACCENT_HI).size(18.0));
            ui.label(theme::h2(t!(
                "Ontwikkeling over tijd",
                "Progress Over Time",
                "Zaman İçinde Gelişim Trendi"
            )));
            if plot_points.len() >= 2 {
                theme::pill(
                    ui,
                    &t!(
                        format!("{} metingen", plot_points.len()),
                        format!("{} readings", plot_points.len()),
                        format!("{} ölçüm", plot_points.len())
                    ),
                    theme::INFO,
                );
            }
        });
        ui.add_space(6.0);

        if plot_points.len() < 2 {
            ui.add(
                egui::Label::new(theme::dim(t!(
                    "Zodra je minstens twee keer een waarde uitleest op verschillende datums, verschijnt hier een lijn-grafiek met de voortgang.",
                    "Once at least two readings from different dates are recorded, a progress line chart will appear here.",
                    "Farklı günlerde en az 2 kez Taktiksel Verimlilik okunduğunda 4 ana bileşenin gelişim grafiği burada oluşur."
                )))
                .wrap(),
            );
            return;
        }

        // Üst renkli efsane — kutucuklarla aynı güncel skor + fark kaynağı
        ui.horizontal_wrapped(|ui| {
            for (i, c) in Component::ALL.iter().enumerate() {
                let val = eff.component(*c);
                let d = diff.map(|d| d.components[i]).unwrap_or(0.0);
                let sign = if d >= 0.5 {
                    format!(" (+{:.0})", d)
                } else if d <= -0.5 {
                    format!(" ({:.0})", d)
                } else {
                    String::new()
                };
                theme::status_dot(ui, COMP_COLORS[i]);
                ui.label(
                    egui::RichText::new(format!("{}: {:.0}{sign}", c.display_label(), val))
                        .size(12.5)
                        .color(theme::TEXT_DIM),
                );
                ui.add_space(10.0);
            }
        });
        ui.add_space(8.0);

        let w = ui.available_width().max(260.0);
        let h = 165.0;
        let (rect, _) = ui.allocate_exact_size(vec2(w, h), Sense::hover());

        let plot = Rect::from_min_max(
            Pos2::new(rect.left() + 30.0, rect.top() + 8.0),
            Pos2::new(rect.right() - 10.0, rect.bottom() - 22.0),
        );

        let y_of = |val: f32| -> f32 {
            let t = (val / 100.0).clamp(0.0, 1.0);
            plot.bottom() - t * plot.height()
        };

        let axis_font = FontId::new(10.5, egui::FontFamily::Proportional);
        let x_labels = build_x_labels(&plot_points);
        let max_lbl_w = ui.fonts(|f| {
            x_labels
                .iter()
                .map(|lbl| f.layout_no_wrap(lbl.clone(), axis_font.clone(), theme::TEXT_FAINT).size().x)
                .fold(32.0_f32, f32::max)
        });
        let p = ui.painter();

        for level in [0, 25, 50, 75, 100] {
            let y = y_of(level as f32);
            p.line_segment(
                [Pos2::new(plot.left(), y), Pos2::new(plot.right(), y)],
                Stroke::new(1.0_f32, theme::BORDER),
            );
            p.text(
                Pos2::new(plot.left() - 6.0, y),
                egui::Align2::RIGHT_CENTER,
                format!("{level}"),
                axis_font.clone(),
                theme::TEXT_FAINT,
            );
        }

        let n = plot_points.len();
        let x_of = |idx: usize| -> f32 {
            if n <= 1 {
                plot.center().x
            } else {
                plot.left() + (idx as f32 / (n - 1) as f32) * plot.width()
            }
        };

        // Alt eksende genişliğe göre seyreltilmiş ve ardışık tekrarsız tarih/saat etiketleri
        let min_slot_w = max_lbl_w + 18.0;
        let max_labels = ((plot.width() / min_slot_w).floor() as usize).clamp(2, 6);
        let step = if n <= max_labels { 1 } else { (n - 1).div_ceil(max_labels - 1) };
        let mut last_lbl: Option<&str> = None;
        let mut last_x: f32 = f32::NEG_INFINITY;
        for (i, lbl) in x_labels.iter().enumerate() {
            let is_end = i == n - 1;
            if i != 0 && !is_end && i % step != 0 {
                continue;
            }
            if last_lbl == Some(lbl.as_str()) {
                continue;
            }
            let x = x_of(i);
            if !is_end && x - last_x < min_slot_w {
                continue;
            }
            if !is_end && x_of(n - 1) - x < min_slot_w * 0.85 {
                continue;
            }
            if is_end && x - last_x < max_lbl_w * 0.9 {
                continue;
            }
            let align = if i == 0 {
                egui::Align2::LEFT_TOP
            } else if is_end {
                egui::Align2::RIGHT_TOP
            } else {
                egui::Align2::CENTER_TOP
            };
            p.text(
                Pos2::new(x, plot.bottom() + 5.0),
                align,
                lbl,
                axis_font.clone(),
                theme::TEXT_FAINT,
            );
            last_lbl = Some(lbl.as_str());
            last_x = x;
        }

        // 4 ana bileşen çizgisi
        for (ci, col) in COMP_COLORS.iter().enumerate() {
            let pts: Vec<Pos2> = plot_points
                .iter()
                .enumerate()
                .map(|(i, pt)| Pos2::new(x_of(i), y_of(pt.components[ci])))
                .collect();
            p.add(egui::Shape::line(pts.clone(), Stroke::new(2.0_f32, *col)));
            for pt in pts {
                p.circle_filled(pt, 3.0, *col);
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_eff(date: Option<NaiveDate>, tu: f32, pa: f32) -> Effectiveness {
        let mut eff = Effectiveness::default();
        eff.source = "FM27 BepInEx".into();
        eff.game_date = date;
        eff.set_component(Component::TacticalUnderstanding, tu);
        eff.set_component(Component::PlayerAbility, pa);
        eff.set(Factor::PassingStyle, tu);
        eff
    }

    #[test]
    fn updates_same_day_and_appends_new_day_and_caps_length() {
        let mut store = HistoryStore::default();
        let d1 = NaiveDate::from_ymd_opt(2026, 8, 1);
        let d2 = NaiveDate::from_ymd_opt(2026, 8, 8);

        assert!(store.record("100", &mk_eff(d1, 50.0, 60.0)));
        assert_eq!(store.points("100").len(), 1);

        // Same game date updates in place rather than duplicating
        assert!(store.record("100", &mk_eff(d1, 55.0, 60.0)));
        assert_eq!(store.points("100").len(), 1);
        assert_eq!(store.points("100")[0].components[0], 55.0);

        // New game date appends
        assert!(store.record("100", &mk_eff(d2, 62.0, 64.0)));
        assert_eq!(store.points("100").len(), 2);

        let diff = store.previous_diff("100", &mk_eff(d2, 62.0, 64.0)).unwrap();
        assert_eq!(diff.components[0], 7.0);
        assert_eq!(diff.components[1], 4.0);

        // Cap at MAX_POINTS
        for i in 0..35 {
            let d = d2.unwrap() + chrono::Duration::days(i + 1);
            store.record("100", &mk_eff(Some(d), 60.0 + (i % 10) as f32, 65.0));
        }
        assert_eq!(store.points("100").len(), MAX_POINTS);
    }

    #[test]
    fn rejects_zero_components_and_diff_skips_corrupt_entries() {
        let mut store = HistoryStore::default();
        let d1 = NaiveDate::from_ymd_opt(2026, 8, 1);
        let d2 = NaiveDate::from_ymd_opt(2026, 8, 2);

        // Kaydedilen geçerli ilk okuma
        assert!(store.record("100", &mk_eff(d1, 60.0, 68.0)));

        // Bileşenlerden biri 0 ise geçmişe kaydedilmez
        let mut bad = mk_eff(d2, 62.0, 69.0);
        bad.set_component(Component::NextMatchInfo, 0.0);
        assert!(!store.record("100", &bad));
        assert_eq!(store.points("100").len(), 1);

        // Eski geçmiş dosyasında bozuk (0'lı) bir kayıt varsa sanitize / previous_diff onu atlar
        let mut corrupt = HistoryPoint::from_eff(&mk_eff(d2, 61.0, 68.0));
        corrupt.components[3] = 0.0;
        store.clubs.get_mut("100").unwrap().push(corrupt);

        let cur = mk_eff(NaiveDate::from_ymd_opt(2026, 8, 3), 65.0, 70.0);
        let diff = store.previous_diff("100", &cur).unwrap();
        assert_eq!(diff.components[0], 5.0);
        assert_eq!(diff.components[1], 2.0);
        assert_eq!(diff.components[3], 0.0);

        store.sanitize();
        assert_eq!(store.points("100").len(), 1);
    }
}
