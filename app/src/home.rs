//! "Bu hafta" ana ekranı (varsayılan sayfa):
//! menajerin haftalık döngüsüne göre analiz özetini, öncelikleri,
//! kompakt 7 günlük takvimi ve dikkat isteyen oyuncuları tek ekranda toplar.

use chrono::{Datelike, Duration, Local, NaiveDate};
use eframe::egui::{self, Align, FontId, Frame, Layout, Margin, RichText, Stroke, Ui, vec2};
use std::collections::HashSet;

use crate::gui::chip_name;
use crate::history::HistoryDiff;
use crate::individual::{self, IndividualState, IntensityLevel};
use crate::live::LiveState;
use crate::model::{
    self, Component, Effectiveness, Factor, category_of, level_color, level_name, session_display_name,
};
use crate::planner::{self, CalendarWeek, PlannerSettings, WeekKind};
use crate::t;
use crate::theme::{self, icon};

pub struct HomeActions {
    pub refresh_data: bool,
    pub apply_plan: bool,
    pub apply_attention: Option<Vec<(u32, IntensityLevel)>>,
}

pub struct HomeDay {
    pub date: NaiveDate,
    pub slots: [String; 3],
    pub changed: bool,
    pub past: bool,
}

struct HomeWeek {
    start: NaiveDate,
    days: [HomeDay; 7],
    next_match: Option<NaiveDate>,
    focus: String,
    is_live: bool,
}

struct AttentionPlayer {
    runtime_id: u32,
    name: String,
    issue: String,
    suggestion: String,
    intensity_action: Option<IntensityLevel>,
    severity: u8,
    sort_cond: f32,
}

/// Hafta aralığını kısa biçimde yazar (ör. "23–29 Kas" veya "30 Kas – 6 Ara").
pub fn format_week_range(start: NaiveDate) -> String {
    let end = start + Duration::days(6);
    if start.month() == end.month() {
        format!("{}–{} {}", start.day(), end.day(), crate::i18n::month_short(end.month()))
    } else {
        format!(
            "{} {} – {} {}",
            start.day(),
            crate::i18n::month_short(start.month()),
            end.day(),
            crate::i18n::month_short(end.month())
        )
    }
}

/// Bir faktör için bu haftaki planda neler yapıldığını okunur metin olarak döner.
pub fn plan_action_for_factor(factor: Factor, days: Option<&[HomeDay; 7]>) -> String {
    let target_name = if factor.is_tactical_area() {
        Some(factor.label())
    } else if factor == Factor::TeammatesAndStaffRelationships {
        Some("Team Bonding")
    } else {
        None
    };

    if let (Some(canonical), Some(week_days)) = (target_name, days) {
        let hits: Vec<&'static str> = week_days
            .iter()
            .enumerate()
            .filter(|(_, d)| d.slots.iter().any(|s| s == canonical))
            .map(|(i, _)| crate::i18n::weekday_short(i))
            .collect();
        if !hits.is_empty() {
            return format!("{} · {}", hits.join(", "), session_display_name(canonical));
        }
    }
    factor.advice().to_string()
}

fn build_week_summary(eff: &Effectiveness, has_data: bool, settings: &PlannerSettings, live: &LiveState) -> Option<HomeWeek> {
    if let Some(o) = &live.training {
        let today = o.today;
        let active = o
            .weeks
            .iter()
            .find(|w| today.is_none_or(|t| w.start + Duration::days(6) >= t))
            .or_else(|| o.weeks.first());
        if let Some(w) = active {
            let mut vec_days = Vec::with_capacity(7);
            for d in 0..7 {
                let date = w.start + Duration::days(d as i64);
                let codes = w.planned.get(d).copied().unwrap_or([300, 300, 300]);
                let cur = w.current.get(d).copied().unwrap_or(codes);
                let slots = [
                    model::session_by_code(codes[0]).unwrap_or("Rest").to_string(),
                    model::session_by_code(codes[1]).unwrap_or("Rest").to_string(),
                    model::session_by_code(codes[2]).unwrap_or("Rest").to_string(),
                ];
                let past = today.is_some_and(|t| date <= t);
                let changed = cur != codes && w.editable[d] && !past;
                vec_days.push(HomeDay { date, slots, changed, past });
            }
            let days: [HomeDay; 7] = vec_days.try_into().ok()?;
            let mut next_match: Option<NaiveDate> = None;
            for wk in &o.weeks {
                for d in 0..7 {
                    let date = wk.start + Duration::days(d as i64);
                    if today.is_some_and(|t| date < t) {
                        continue;
                    }
                    if wk.planned.get(d).is_some_and(|c| c.contains(&500)) {
                        next_match = Some(match next_match {
                            Some(cur_m) if cur_m <= date => cur_m,
                            _ => date,
                        });
                    }
                }
            }
            return Some(HomeWeek {
                start: w.start,
                days,
                next_match,
                focus: w.focus.clone(),
                is_live: true,
            });
        }
    }

    if !has_data {
        return None;
    }

    let monday = planner::monday_of(Local::now().date_naive());
    let mut matches = [false; 7];
    matches[6] = true;
    let cal_week = CalendarWeek {
        start: monday,
        kind: WeekKind::Season,
        matches,
        note: String::new(),
    };
    let plan = planner::plan_week(&cal_week, false, false, eff, settings, 1);
    let vec_days: Vec<HomeDay> = plan
        .days
        .into_iter()
        .map(|dp| HomeDay {
            date: dp.date,
            slots: dp.slots,
            changed: false,
            past: false,
        })
        .collect();
    let days: [HomeDay; 7] = vec_days.try_into().ok()?;
    Some(HomeWeek {
        start: monday,
        days,
        next_match: None,
        focus: plan.focus.to_string(),
        is_live: false,
    })
}

fn build_attention_players(live: &LiveState, individual: &IndividualState) -> Vec<AttentionPlayer> {
    if !live.connected {
        return Vec::new();
    }
    let (recs, week_matches) = individual::build_squad_recommendations(live, &individual.current_intensities, individual.profile);
    let mut out = Vec::new();
    let mut seen_rids = HashSet::new();

    for r in &recs {
        if r.runtime_id != 0 {
            seen_rids.insert(r.runtime_id);
        }
        if r.injured {
            out.push(AttentionPlayer {
                runtime_id: r.runtime_id,
                name: r.name.clone(),
                issue: t!("Geblesseerd", "Injured", "Sakat").to_string(),
                suggestion: r.recommended.label().to_string(),
                intensity_action: Some(r.recommended),
                severity: 3,
                sort_cond: r.condition,
            });
        } else if r.rest_match_warning || r.condition < 75.0 {
            out.push(AttentionPlayer {
                runtime_id: r.runtime_id,
                name: r.name.clone(),
                issue: t!(
                    format!("Lage conditie ({:.0}%)", r.condition),
                    format!("Low condition ({:.0}%)", r.condition),
                    format!("Düşük kondisyon (%{:.0})", r.condition)
                ),
                suggestion: r.recommended.label().to_string(),
                intensity_action: Some(r.recommended),
                severity: 2,
                sort_cond: r.condition,
            });
        } else if r.age.is_some_and(|a| a >= 32) && week_matches >= 2 {
            let age = r.age.unwrap_or(32);
            out.push(AttentionPlayer {
                runtime_id: r.runtime_id,
                name: r.name.clone(),
                issue: t!(
                    format!("{age}j · drukke week"),
                    format!("{age}y · congested week"),
                    format!("{age} yaş · yoğun fikstür")
                ),
                suggestion: r.recommended.label().to_string(),
                intensity_action: Some(r.recommended),
                severity: 2,
                sort_cond: r.condition,
            });
        }
    }

    for b in &individual.bottlenecks {
        if b.overall >= 65.0 {
            continue;
        }
        if b.runtime_id != 0 && seen_rids.contains(&b.runtime_id) && out.iter().any(|x| x.runtime_id == b.runtime_id) {
            continue;
        }
        let rec_act = recs
            .iter()
            .find(|r| r.runtime_id != 0 && r.runtime_id == b.runtime_id)
            .map(|r| r.recommended);
        out.push(AttentionPlayer {
            runtime_id: b.runtime_id,
            name: b.name.clone(),
            issue: t!(
                format!("Tactisch knelpunt ({:.0})", b.overall),
                format!("Tactical bottleneck ({:.0})", b.overall),
                format!("Taktik darboğaz ({:.0})", b.overall)
            ),
            suggestion: b.weakest_label.to_string(),
            intensity_action: rec_act,
            severity: 1,
            sort_cond: b.overall,
        });
    }

    out.sort_by(|a, b| {
        b.severity
            .cmp(&a.severity)
            .then_with(|| a.sort_cond.total_cmp(&b.sort_cond))
    });
    out.truncate(5);
    out
}

#[allow(clippy::too_many_arguments)]
pub fn render(
    ui: &mut Ui,
    eff: &Effectiveness,
    has_data: bool,
    last_read_hhmm: Option<&str>,
    diff: Option<HistoryDiff>,
    planner_settings: &PlannerSettings,
    live: &LiveState,
    individual: &IndividualState,
    status_msg: &str,
    status_error: bool,
) -> HomeActions {
    let mut actions = HomeActions {
        refresh_data: false,
        apply_plan: false,
        apply_attention: None,
    };

    let week = build_week_summary(eff, has_data, planner_settings, live);
    let week_start = week
        .as_ref()
        .map(|w| w.start)
        .unwrap_or_else(|| planner::monday_of(Local::now().date_naive()));
    let busy = live.busy();
    let can_apply = live.connected && !busy;

    // 1. Başlık satırı
    ui.horizontal(|ui| {
        ui.label(theme::title(format!(
            "{} · {}",
            t!("Deze week", "This week", "Bu hafta"),
            format_week_range(week_start)
        )));
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if theme::primary_button(
                ui,
                icon::LIGHTNING,
                t!("Plan in FM zetten", "Apply plan to FM", "Planı FM'ye uygula"),
                can_apply,
            )
            .on_disabled_hover_text(t!("Verbind met FM27", "Connect to FM27", "FM27'ye bağlanın"))
            .clicked()
            {
                actions.apply_plan = true;
            }
            if theme::button(
                ui,
                icon::ARROW_COUNTER_CLOCKWISE,
                t!("Vernieuwen", "Refresh", "Güncelle"),
                !busy,
            )
            .clicked()
            {
                actions.refresh_data = true;
            }
            if live.fixed_total > 0
                && let Some(last_log) = live.log.first()
            {
                theme::pill(
                    ui,
                    &t!(
                        format!("Toegepast · {}", last_log.at.format("%H:%M")),
                        format!("Applied · {}", last_log.at.format("%H:%M")),
                        format!("Uygulandı · {}", last_log.at.format("%H:%M"))
                    ),
                    theme::SUCCESS,
                );
            }
            if busy {
                ui.spinner();
            }
        });
    });

    ui.add_space(2.0);
    ui.horizontal_wrapped(|ui| {
        let dot_col = if live.connected { theme::SUCCESS } else { theme::TEXT_FAINT };
        theme::status_dot(ui, dot_col);
        let conn_lbl = if live.connected {
            live.club_name()
                .unwrap_or_else(|| t!("Verbonden met FM27", "Connected to FM27", "FM27 bağlı").to_string())
        } else {
            t!("Niet verbonden", "Not connected", "Bağlı değil").to_string()
        };
        ui.label(theme::dim(conn_lbl));
        if let Some(hhmm) = last_read_hhmm {
            ui.label(theme::faint("·"));
            ui.label(theme::dim(t!(
                format!("Laatst gelezen {hhmm}"),
                format!("Last read {hhmm}"),
                format!("Son okuma {hhmm}")
            )));
        }
        if let Some(nm) = week.as_ref().and_then(|w| w.next_match) {
            ui.label(theme::faint("·"));
            let wd = crate::i18n::weekday_short(nm.weekday().num_days_from_monday() as usize);
            let ds = crate::i18n::date_short(nm);
            ui.label(theme::dim(t!(
                format!("Volgende wedstrijd: {wd} {ds}"),
                format!("Next match: {wd} {ds}"),
                format!("Sonraki maç: {wd} {ds}")
            )));
        }
    });

    ui.add_space(10.0);
    let (sep_rect, _) = ui.allocate_exact_size(vec2(ui.available_width().max(20.0), 1.0), egui::Sense::hover());
    ui.painter().hline(sep_rect.x_range(), sep_rect.center().y, Stroke::new(1.0_f32, theme::BORDER));
    ui.add_space(14.0);

    if !status_msg.is_empty() {
        let (c, ic) = if status_error { (theme::DANGER, icon::WARNING) } else { (theme::INFO, icon::CHECK_CIRCLE) };
        Frame::new()
            .fill(theme::tint(c, 0.12))
            .stroke(Stroke::new(1.0_f32, theme::tint(c, 0.4)))
            .corner_radius(egui::CornerRadius::same(10))
            .inner_margin(Margin::symmetric(12, 8))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.horizontal_wrapped(|ui| {
                    ui.label(theme::ic(ic).color(c));
                    ui.label(RichText::new(status_msg).color(theme::TEXT));
                });
            });
        ui.add_space(12.0);
    }

    // 2. 4 bileşen kutucuğu (halka olmadan)
    if !has_data {
        theme::card(ui, |ui| {
            ui.label(theme::dim(t!(
                "Nog geen gegevens uitgelezen. Klik rechtsboven op Vernieuwen om te beginnen.",
                "No Tactical Effectiveness data read yet. Click Refresh above to begin.",
                "Henüz veri okunmadı. Başlamak için sağ üstten Güncelle butonuna tıklayın."
            )));
        });
    } else {
        let draw_comp = |col_ui: &mut Ui, i: usize, c: Component| {
            let v = eff.component(c);
            let col = theme::rgb(level_color(v));
            let ic = [icon::TARGET, icon::SOCCER_BALL, icon::USERS_THREE, icon::EYE][i];
            let d_opt = diff.map(|d| d.components[i]).filter(|x| x.abs() >= 1.0);
            theme::card_accent(col_ui, col, |ui| {
                ui.horizontal(|ui| {
                    ui.label(theme::ic(ic).size(15.0).color(col));
                    ui.add(egui::Label::new(theme::caps(c.display_label())).truncate());
                });
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.label(RichText::new(format!("{v:.0}")).font(FontId::new(28.0, theme::bold())).color(theme::TEXT));
                    ui.label(RichText::new(level_name(v)).size(12.5).color(col));
                    if let Some(d) = d_opt {
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            let up = d > 0.0;
                            let dc = if up { theme::SUCCESS } else { theme::DANGER };
                            let txt = if up { format!("▲+{d:.0}") } else { format!("▼{d:.0}") };
                            ui.label(RichText::new(txt).font(FontId::new(12.0, theme::semibold())).color(dc));
                        });
                    }
                });
            });
        };
        if ui.available_width() >= 880.0 {
            ui.columns(4, |cols| {
                for (i, c) in Component::ALL.iter().enumerate() {
                    draw_comp(&mut cols[i], i, *c);
                }
            });
        } else {
            for row in 0..2 {
                ui.columns(2, |cols| {
                    for (col_idx, col_ui) in cols.iter_mut().enumerate() {
                        let i = row * 2 + col_idx;
                        draw_comp(col_ui, i, Component::ALL[i]);
                    }
                });
                if row == 0 {
                    ui.add_space(8.0);
                }
            }
        }
    }
    ui.add_space(14.0);

    // 3. "Öncelikler" kartı (en düşük 3 faktör + plan bunun için ne yapıyor)
    theme::card(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(theme::ic(icon::TARGET).color(theme::WARNING).size(18.0));
            ui.label(theme::h2(t!("Prioriteiten", "Priorities", "Öncelikler")));
        });
        ui.add_space(6.0);
        if !has_data {
            ui.label(theme::dim(t!(
                "De drie laagste factoren verschijnen hier zodra je de eerste meting hebt gelezen.",
                "Your lowest three factors appear here once you read Tactical Effectiveness.",
                "En düşük 3 faktör ve plan adımları veri okunduktan sonra burada görünür."
            )));
        } else {
            let days_ref = week.as_ref().map(|w| &w.days);
            let label_w = (ui.available_width() * 0.30).clamp(180.0, 270.0);
            for (f, v) in eff.weakest().into_iter().take(3) {
                let col = theme::rgb(level_color(v));
                let action_txt = plan_action_for_factor(f, days_ref);
                ui.horizontal(|ui| {
                    ui.allocate_ui_with_layout(vec2(label_w, 22.0), Layout::left_to_right(Align::Center), |ui| {
                        ui.add(egui::Label::new(theme::label_strong(f.display_label())).truncate());
                    });
                    ui.allocate_ui_with_layout(vec2(52.0, 22.0), Layout::right_to_left(Align::Center), |ui| {
                        theme::pill(ui, &format!("{v:.0}"), col);
                    });
                    ui.add_space(8.0);
                    ui.add(egui::Label::new(theme::dim(action_txt)).wrap());
                });
                ui.add_space(4.0);
            }
        }
    });
    ui.add_space(14.0);

    // 4. Kompakt 7 günlük takvim (Faz 5 çip stiliyle)
    theme::card(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(theme::ic(icon::CALENDAR_DOTS).color(theme::ACCENT_HI).size(18.0));
            ui.label(theme::h2(t!("Weekprogramma", "This week's schedule", "Bu haftanın programı")));
            if let Some(w) = &week {
                let focus_lbl = match w.focus.as_str() {
                    "In Possession" => t!("Balbezit", "In Possession", "Topa Sahipken"),
                    "Out Of Possession" => t!("Balverlies", "Out Of Possession", "Topsuz Alanda"),
                    "Full Week of Rest" => t!("Rustweek", "Full Week of Rest", "Tam Dinlenme"),
                    _ => t!("Gebalanceerd", "Balanced", "Dengeli"),
                };
                theme::pill(ui, focus_lbl, theme::ACCENT);
                if !w.is_live {
                    theme::pill(ui, t!("Voorbeeld", "Preview", "Önizleme"), theme::INFO);
                }
            }
        });
        ui.add_space(8.0);
        if let Some(w) = &week {
            ui.columns(7, |cols| {
                for (d, day) in w.days.iter().enumerate() {
                    let col_ui = &mut cols[d];
                    col_ui.label(theme::col_header(crate::i18n::weekday_short(d)));
                    col_ui.label(theme::faint(crate::i18n::date_short(day.date)));
                    col_ui.add_space(4.0);
                    for s in &day.slots {
                        let c = if s == "Match" {
                            theme::DANGER
                        } else {
                            theme::rgb(category_of(s).color())
                        };
                        let c = if day.past { theme::tint(c, 0.45) } else { c };
                        let disp = session_display_name(s);
                        let hover = if disp != s.as_str() {
                            format!("{disp} ({s})")
                        } else {
                            s.clone()
                        };
                        theme::session_chip_bar(col_ui, &chip_name(disp), c, day.changed, s == "Match")
                            .on_hover_text(hover);
                        col_ui.add_space(3.0);
                    }
                }
            });
        } else {
            ui.label(theme::dim(t!(
                "Er is nog geen weekschema om te tonen.",
                "No weekly schedule to show yet.",
                "Henüz gösterilecek haftalık program yok."
            )));
        }
    });
    ui.add_space(14.0);

    // 5. "Dikkat isteyen oyuncular" kartı (en fazla 5 oyuncu)
    let attention = build_attention_players(live, individual);
    let apply_batch: Vec<(u32, IntensityLevel)> = attention
        .iter()
        .filter_map(|p| (p.runtime_id != 0).then_some(p.intensity_action.map(|lvl| (p.runtime_id, lvl))).flatten())
        .collect();
    let can_apply_att = live.connected && !apply_batch.is_empty();

    theme::card(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(theme::ic(icon::USERS_THREE).color(theme::WARNING).size(18.0));
            ui.label(theme::h2(t!(
                "Spelers die aandacht vragen",
                "Players needing attention",
                "Dikkat isteyen oyuncular"
            )));
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let btn_lbl = t!(
                    format!("Advies toepassen ({})", apply_batch.len()),
                    format!("Apply suggestions ({})", apply_batch.len()),
                    format!("Önerileri uygula ({})", apply_batch.len())
                );
                let disabled_tip = if !live.connected {
                    t!("Verbind met FM27", "Connect to FM27", "FM27'ye bağlanın")
                } else {
                    t!(
                        "Geen openstaande intensiteitsvoorstellen",
                        "No intensity suggestions to apply",
                        "Uygulanacak yoğunluk önerisi yok"
                    )
                };
                if theme::small_button(ui, &btn_lbl, can_apply_att)
                    .on_disabled_hover_text(disabled_tip)
                    .clicked()
                {
                    actions.apply_attention = Some(apply_batch.clone());
                }
            });
        });
        ui.add_space(6.0);
        if !live.connected {
            ui.label(theme::dim(t!(
                "Spelerswaarschuwingen verschijnen zodra FM27 verbonden is.",
                "Player warnings appear once FM27 is connected.",
                "Oyuncu uyarıları FM27 bağlıyken burada görünür."
            )));
        } else if attention.is_empty() {
            ui.label(theme::dim(t!(
                "Geen spelers die extra aandacht vragen deze week.",
                "No players need special attention this week.",
                "Bu hafta özel dikkat gerektiren oyuncu yok."
            )));
        } else {
            egui::Grid::new("home_attention_grid")
                .num_columns(3)
                .spacing([18.0, 6.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label(theme::col_header(t!("Speler", "Player", "Oyuncu")));
                    ui.label(theme::col_header(t!("Probleem", "Issue", "Sorun")));
                    ui.label(theme::col_header(t!("Advies", "Suggestion", "Öneri")));
                    ui.end_row();
                    for p in &attention {
                        ui.label(theme::label_strong(&p.name));
                        let issue_col = match p.severity {
                            3 => theme::DANGER,
                            2 => theme::WARNING,
                            _ => theme::INFO,
                        };
                        theme::pill(ui, &p.issue, issue_col);
                        ui.label(theme::dim(&p.suggestion));
                        ui.end_row();
                    }
                });
        }
    });

    actions
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::{self, Lang};

    #[test]
    fn formats_week_range_same_and_cross_month() {
        i18n::set(Lang::Tr);
        let d1 = NaiveDate::from_ymd_opt(2026, 11, 23).unwrap();
        assert_eq!(format_week_range(d1), "23–29 Kas");
        let d2 = NaiveDate::from_ymd_opt(2026, 11, 30).unwrap();
        assert_eq!(format_week_range(d2), "30 Kas – 6 Ara");
    }

    #[test]
    fn plan_action_reports_scheduled_days_and_falls_back_for_non_trainable() {
        i18n::set(Lang::Tr);
        let start = NaiveDate::from_ymd_opt(2026, 11, 23).unwrap();
        let days: [HomeDay; 7] = [
            HomeDay { date: start, slots: ["Recovery".into(), "Rest".into(), "Team Bonding".into()], changed: false, past: false },
            HomeDay { date: start + Duration::days(1), slots: ["Line of Engagement".into(), "Pressing Intensity".into(), "Rest".into()], changed: false, past: false },
            HomeDay { date: start + Duration::days(2), slots: ["In Possession Shape".into(), "Passing Style".into(), "Rest".into()], changed: false, past: false },
            HomeDay { date: start + Duration::days(3), slots: ["Light Conditioning".into(), "In Possession Shape".into(), "Build Up".into()], changed: false, past: false },
            HomeDay { date: start + Duration::days(4), slots: ["Chance Creation".into(), "Passing Style".into(), "Rest".into()], changed: false, past: false },
            HomeDay { date: start + Duration::days(5), slots: ["Corners".into(), "Free Kicks".into(), "Penalties".into()], changed: false, past: false },
            HomeDay { date: start + Duration::days(6), slots: ["Rest".into(), "Match".into(), "Rest".into()], changed: false, past: false },
        ];
        let act = plan_action_for_factor(Factor::InPossessionShape, Some(&days));
        assert!(act.contains("Çar, Per"), "expected scheduled days in `{act}`");
        let non_train = plan_action_for_factor(Factor::TacticalComplexity, Some(&days));
        assert_eq!(non_train, Factor::TacticalComplexity.advice());
    }
}
