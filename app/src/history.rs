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
        std::fs::read_to_string(history_file())
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
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
        let list = self.clubs.entry(club_key.to_string()).or_default();

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
        let pts = self.points(club_key);
        let cur = HistoryPoint::from_eff(eff);
        let prev = match pts.len() {
            0 => return None,
            1 => {
                if !pts[0].differs(&cur) {
                    return None;
                }
                &pts[0]
            }
            n => {
                if !pts[n - 1].differs(&cur) {
                    &pts[n - 2]
                } else {
                    &pts[n - 1]
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

pub fn render_trend_card(ui: &mut Ui, points: &[HistoryPoint]) {
    theme::card(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.label(theme::ic(icon::GAUGE).color(theme::ACCENT_HI).size(18.0));
            ui.label(theme::h2(t!(
                "Ontwikkeling over tijd",
                "Progress Over Time",
                "Zaman İçinde Gelişim Trendi"
            )));
            if points.len() >= 2 {
                theme::pill(
                    ui,
                    &t!(
                        format!("{} metingen", points.len()),
                        format!("{} readings", points.len()),
                        format!("{} ölçüm", points.len())
                    ),
                    theme::INFO,
                );
            }
        });
        ui.add_space(6.0);

        if points.len() < 2 {
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

        // Üst renkli efsane + ilk ölçüme göre net değişim
        let first = &points[0];
        let last = &points[points.len() - 1];
        ui.horizontal_wrapped(|ui| {
            for (i, c) in Component::ALL.iter().enumerate() {
                let net = last.components[i] - first.components[i];
                let sign = if net >= 0.5 {
                    format!(" (+{:.0})", net)
                } else if net <= -0.5 {
                    format!(" ({:.0})", net)
                } else {
                    String::new()
                };
                theme::status_dot(ui, COMP_COLORS[i]);
                ui.label(
                    egui::RichText::new(format!("{}: {:.0}{sign}", c.display_label(), last.components[i]))
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
        let p = ui.painter();

        let plot = Rect::from_min_max(
            Pos2::new(rect.left() + 30.0, rect.top() + 8.0),
            Pos2::new(rect.right() - 10.0, rect.bottom() - 22.0),
        );

        let y_of = |val: f32| -> f32 {
            let t = (val / 100.0).clamp(0.0, 1.0);
            plot.bottom() - t * plot.height()
        };

        let axis_font = FontId::new(10.5, egui::FontFamily::Proportional);
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

        let n = points.len();
        let x_of = |idx: usize| -> f32 {
            if n <= 1 {
                plot.center().x
            } else {
                plot.left() + (idx as f32 / (n - 1) as f32) * plot.width()
            }
        };

        // Alt eksende birkaç tarih etiketi
        let step = (n / 5).max(1);
        for (i, pt) in points.iter().enumerate() {
            if i == 0 || i == n - 1 || i % step == 0 {
                let align = if i == 0 {
                    egui::Align2::LEFT_TOP
                } else if i == n - 1 {
                    egui::Align2::RIGHT_TOP
                } else {
                    egui::Align2::CENTER_TOP
                };
                p.text(
                    Pos2::new(x_of(i), plot.bottom() + 5.0),
                    align,
                    &pt.label,
                    axis_font.clone(),
                    theme::TEXT_FAINT,
                );
            }
        }

        // 4 ana bileşen çizgisi
        for (ci, col) in COMP_COLORS.iter().enumerate() {
            let pts: Vec<Pos2> = points
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
}
