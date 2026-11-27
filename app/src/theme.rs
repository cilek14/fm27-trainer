//! Ontwerpsysteem: lettertype (Inter + Phosphor-iconen), kleuren en bouwstenen.

use eframe::egui::{
    self, Color32, CornerRadius, FontData, FontDefinitions, FontFamily, FontId, Frame, Margin, Response, RichText,
    Sense, Shadow, Stroke, TextStyle, Ui, Vec2, vec2,
};
use std::sync::Arc;

pub use egui_phosphor::regular as icon;

// ---- kleuren (Pitch Obsidian & Emerald Telemetry) ----------------------------
pub const BG: Color32 = Color32::from_rgb(11, 17, 20);
pub const SIDEBAR: Color32 = Color32::from_rgb(15, 23, 28);
pub const SURFACE: Color32 = Color32::from_rgb(18, 28, 34);
pub const SURFACE2: Color32 = Color32::from_rgb(25, 38, 46);
pub const HOVER: Color32 = Color32::from_rgb(33, 50, 61);
pub const BORDER: Color32 = Color32::from_rgb(33, 50, 59);
pub const TRACK: Color32 = Color32::from_rgb(28, 42, 50);
pub const TEXT: Color32 = Color32::from_rgb(236, 244, 247);
pub const TEXT_DIM: Color32 = Color32::from_rgb(148, 169, 180);
pub const TEXT_FAINT: Color32 = Color32::from_rgb(122, 134, 150);
pub const ACCENT: Color32 = Color32::from_rgb(16, 185, 129);
pub const ACCENT_HI: Color32 = Color32::from_rgb(52, 211, 153);
pub const ACCENT_SOFT: Color32 = Color32::from_rgb(14, 51, 42);
pub const SUCCESS: Color32 = Color32::from_rgb(34, 197, 94);
pub const WARNING: Color32 = Color32::from_rgb(245, 158, 11);
pub const DANGER: Color32 = Color32::from_rgb(244, 63, 94);
pub const INFO: Color32 = Color32::from_rgb(56, 189, 248);

pub const CONTENT_MAX: f32 = 1400.0;
pub const SETTINGS_MAX: f32 = 960.0;

/// Mengt twee kleuren (t = 0: a, t = 1: b).
pub fn mix(a: Color32, b: Color32, t: f32) -> Color32 {
    let m = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * t.clamp(0.0, 1.0)).round() as u8;
    Color32::from_rgb(m(a.r(), b.r()), m(a.g(), b.g()), m(a.b(), b.b()))
}

// ---- beweging -------------------------------------------------------------------
/// Duur van een hover-/keuze-overgang.
const QUICK: f32 = 0.14;

fn enter_id() -> egui::Id {
    egui::Id::new("fm27-page-enter")
}

/// Start de intrede-animatie opnieuw (bij het wisselen van pagina of na een nieuwe uitlezing).
pub fn restart_enter(ctx: &egui::Context) {
    let now = ctx.input(|i| i.time);
    ctx.data_mut(|d| d.insert_temp(enter_id(), now));
}

/// Voortgang van de intrede-animatie over `secs` seconden, 0…1 met een zachte uitloop. Vraagt zelf
/// om een volgend beeld zolang hij loopt.
pub fn enter(ctx: &egui::Context, secs: f32) -> f32 {
    let now = ctx.input(|i| i.time);
    let start = ctx.data_mut(|d| *d.get_temp_mut_or(enter_id(), now));
    let t = (((now - start) as f32) / secs).clamp(0.0, 1.0);
    if t < 1.0 {
        ctx.request_repaint();
    }
    1.0 - (1.0 - t).powi(3)
}

pub fn tint(c: Color32, a: f32) -> Color32 {
    // kleur gemengd met de kaartachtergrond (geen echte transparantie: rustiger in tabellen)
    let m = |x: u8, y: u8| (x as f32 * a + y as f32 * (1.0 - a)) as u8;
    Color32::from_rgb(m(c.r(), SURFACE.r()), m(c.g(), SURFACE.g()), m(c.b(), SURFACE.b()))
}

pub fn rgb(c: [u8; 3]) -> Color32 {
    Color32::from_rgb(c[0], c[1], c[2])
}

// ---- lettertypen ----------------------------------------------------------------
pub fn medium() -> FontFamily {
    FontFamily::Name("medium".into())
}
pub fn semibold() -> FontFamily {
    FontFamily::Name("semibold".into())
}
pub fn bold() -> FontFamily {
    FontFamily::Name("bold".into())
}
pub fn icons() -> FontFamily {
    FontFamily::Name("icons".into())
}

/// Icoon als tekst (in de iconenfamilie).
pub fn ic(s: &str) -> RichText {
    RichText::new(s).family(icons())
}

/// Icoon + tekst in één label (twee lettertypes).
pub fn icon_label(icon_s: &str, text: &str, font: FontId, color: Color32) -> egui::text::LayoutJob {
    use egui::text::{LayoutJob, TextFormat};
    let mut job = LayoutJob::default();
    let fmt = |f: FontId| TextFormat { font_id: f, color, valign: egui::Align::Center, ..Default::default() };
    if !icon_s.is_empty() {
        job.append(icon_s, 0.0, fmt(FontId::new(font.size + 2.0, icons())));
        job.append("  ", 0.0, fmt(font.clone()));
    }
    job.append(text, 0.0, fmt(font));
    job
}

fn install_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    let add = |fonts: &mut FontDefinitions, name: &str, bytes: &'static [u8]| {
        fonts.font_data.insert(name.into(), Arc::new(FontData::from_static(bytes)));
    };
    add(&mut fonts, "Inter-Regular", include_bytes!("../assets/fonts/Inter-Regular.ttf"));
    add(&mut fonts, "Inter-Medium", include_bytes!("../assets/fonts/Inter-Medium.ttf"));
    add(&mut fonts, "Inter-SemiBold", include_bytes!("../assets/fonts/Inter-SemiBold.ttf"));
    add(&mut fonts, "Inter-Bold", include_bytes!("../assets/fonts/Inter-Bold.ttf"));
    egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);

    let fallback: Vec<String> = fonts.families.get(&FontFamily::Proportional).cloned().unwrap_or_default();
    let family = |main: &str| {
        let mut v = vec![main.to_string(), "phosphor".to_string()];
        v.extend(fallback.iter().filter(|f| *f != "phosphor").cloned());
        v
    };
    fonts.families.insert(FontFamily::Proportional, family("Inter-Regular"));
    fonts.families.insert(medium(), family("Inter-Medium"));
    fonts.families.insert(semibold(), family("Inter-SemiBold"));
    fonts.families.insert(bold(), family("Inter-Bold"));
    // Iconen apart: Inter heeft eigen tekens in hetzelfde privégebied (U+E000…) als Phosphor,
    // en Phosphor heeft lege tekens voor letters. Dus een eigen familie met Phosphor voorop.
    fonts.families.insert(icons(), vec!["phosphor".to_string(), "Inter-Regular".to_string()]);
    ctx.set_fonts(fonts);
}

pub fn install(ctx: &egui::Context) {
    install_fonts(ctx);
    ctx.set_theme(egui::Theme::Dark);
    ctx.style_mut_of(egui::Theme::Dark, |style| {
        style.text_styles = [
            (TextStyle::Heading, FontId::new(24.0, bold())),
            (TextStyle::Body, FontId::new(14.0, FontFamily::Proportional)),
            (TextStyle::Button, FontId::new(14.0, medium())),
            (TextStyle::Small, FontId::new(12.0, FontFamily::Proportional)),
            (TextStyle::Monospace, FontId::new(13.0, FontFamily::Monospace)),
        ]
        .into();
        let s = &mut style.spacing;
        s.item_spacing = vec2(8.0, 8.0);
        s.button_padding = vec2(14.0, 8.0);
        s.interact_size = vec2(36.0, 32.0);
        s.slider_width = 160.0;
        s.combo_width = 160.0;
        s.window_margin = Margin::same(16);
        s.menu_margin = Margin::same(8);
        s.scroll.bar_width = 8.0;
        s.scroll.floating = true;
        style.animation_time = 0.14;

        let v = &mut style.visuals;
        *v = egui::Visuals::dark();
        v.panel_fill = BG;
        v.window_fill = SURFACE;
        v.extreme_bg_color = Color32::from_rgb(9, 14, 17);
        v.faint_bg_color = Color32::from_rgb(21, 32, 39);
        v.code_bg_color = SURFACE2;
        v.window_stroke = Stroke::new(1.0_f32, BORDER);
        v.window_corner_radius = CornerRadius::same(12);
        v.menu_corner_radius = CornerRadius::same(10);
        v.window_shadow = Shadow { offset: [0, 10], blur: 28, spread: 0, color: Color32::from_black_alpha(120) };
        v.popup_shadow = Shadow { offset: [0, 6], blur: 18, spread: 0, color: Color32::from_black_alpha(110) };
        v.selection.bg_fill = ACCENT;
        v.selection.stroke = Stroke::new(1.0_f32, Color32::WHITE);
        v.hyperlink_color = ACCENT_HI;
        v.striped = false;
        let r = CornerRadius::same(8);
        let w = &mut v.widgets;
        w.noninteractive.bg_fill = SURFACE;
        w.noninteractive.weak_bg_fill = SURFACE;
        w.noninteractive.bg_stroke = Stroke::new(1.0_f32, BORDER);
        w.noninteractive.fg_stroke = Stroke::new(1.0_f32, TEXT);
        w.noninteractive.corner_radius = r;
        w.inactive.bg_fill = SURFACE2;
        w.inactive.weak_bg_fill = SURFACE2;
        w.inactive.bg_stroke = Stroke::new(1.0_f32, BORDER);
        w.inactive.fg_stroke = Stroke::new(1.0_f32, TEXT);
        w.inactive.corner_radius = r;
        w.hovered.bg_fill = HOVER;
        w.hovered.weak_bg_fill = HOVER;
        w.hovered.bg_stroke = Stroke::new(1.0_f32, Color32::from_rgb(47, 73, 86));
        w.hovered.fg_stroke = Stroke::new(1.5_f32, Color32::WHITE);
        w.hovered.corner_radius = r;
        w.hovered.expansion = 0.0;
        w.active.bg_fill = ACCENT_SOFT;
        w.active.weak_bg_fill = ACCENT_SOFT;
        w.active.bg_stroke = Stroke::new(1.0_f32, ACCENT);
        w.active.fg_stroke = Stroke::new(1.5_f32, Color32::WHITE);
        w.active.corner_radius = r;
        w.active.expansion = 0.0;
        w.open = w.active;
    });
}

// ---- tekst ----------------------------------------------------------------------
pub fn title(s: impl Into<String>) -> RichText {
    RichText::new(s).font(FontId::new(27.0, bold())).color(TEXT)
}
pub fn h2(s: impl Into<String>) -> RichText {
    RichText::new(s).font(FontId::new(16.0, semibold())).color(TEXT)
}
pub fn label_strong(s: impl Into<String>) -> RichText {
    RichText::new(s).font(FontId::new(14.0, semibold())).color(TEXT)
}
pub fn dim(s: impl Into<String>) -> RichText {
    RichText::new(s).color(TEXT_DIM)
}
pub fn faint(s: impl Into<String>) -> RichText {
    RichText::new(s).size(12.0).color(TEXT_FAINT)
}
pub fn caps(s: impl Into<String>) -> RichText {
    let raw = s.into();
    let up = if crate::i18n::is_tr() {
        raw.replace('i', "İ").replace('ı', "I").to_uppercase()
    } else {
        raw.to_uppercase()
    };
    RichText::new(up).font(FontId::new(12.5, semibold())).color(TEXT_FAINT)
}

/// Tablo sütun ve gün başlıkları (sentence case, 12.5 px).
pub fn col_header(s: impl Into<String>) -> RichText {
    RichText::new(s).font(FontId::new(12.5, semibold())).color(TEXT_DIM)
}

/// Özel çizilmiş taktik saha arması logosu ("FM27 Training Assistant").
pub fn brand_logo(ui: &mut Ui, size: f32) {
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
    let p = ui.painter();
    let cr = CornerRadius::same((size * 0.27).round() as u8);

    // Koyu zümrüt taktik tahtası zemini
    let bg = Color32::from_rgb(17, 52, 43);
    let top_hi = Color32::from_rgb(24, 72, 58);
    p.rect_filled(rect, cr, bg);
    let hi_rect = egui::Rect::from_min_max(rect.min, egui::pos2(rect.max.x, rect.center().y));
    p.rect_filled(hi_rect, CornerRadius { nw: cr.nw, ne: cr.ne, sw: 0, se: 0 }, mix(bg, top_hi, 0.55));
    p.rect_stroke(rect, cr, Stroke::new(1.2_f32, mix(SUCCESS, Color32::WHITE, 0.2)), egui::StrokeKind::Inside);

    // Saha orta çizgisi ve dairesi (silik)
    let pitch_col = Color32::from_rgba_unmultiplied(61, 214, 140, 70);
    let pad = size * 0.16;
    p.line_segment(
        [egui::pos2(rect.left() + pad, rect.center().y), egui::pos2(rect.right() - pad, rect.center().y)],
        Stroke::new(1.0_f32, pitch_col),
    );
    p.circle_stroke(rect.center(), size * 0.18, Stroke::new(1.0_f32, pitch_col));

    // 3 taktik noktası + yükselen gelişim oku
    let w = rect.width();
    let h = rect.height();
    let pt = |nx: f32, ny: f32| egui::pos2(rect.left() + w * nx, rect.top() + h * ny);
    let bl = pt(0.27, 0.71);
    let br = pt(0.68, 0.71);
    let mid = pt(0.47, 0.52);
    let tip = pt(0.75, 0.25);

    p.line_segment([br, mid], Stroke::new(1.6_f32, Color32::from_rgba_unmultiplied(238, 240, 245, 130)));
    p.line_segment([bl, mid], Stroke::new(2.2_f32, SUCCESS));
    p.line_segment([mid, tip], Stroke::new(2.4_f32, Color32::WHITE));

    // Ok ucu
    let wing_l = pt(0.57, 0.27);
    let wing_r = pt(0.73, 0.43);
    p.line_segment([tip, wing_l], Stroke::new(2.2_f32, Color32::WHITE));
    p.line_segment([tip, wing_r], Stroke::new(2.2_f32, Color32::WHITE));

    p.circle_filled(bl, (size * 0.068).max(2.2), SUCCESS);
    p.circle_filled(br, (size * 0.058).max(2.0), Color32::from_rgba_unmultiplied(238, 240, 245, 190));
    p.circle_filled(mid, (size * 0.062).max(2.0), Color32::WHITE);
    p.circle_filled(tip, (size * 0.065).max(2.2), INFO);
}

// ---- bouwstenen -----------------------------------------------------------------
pub fn card<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> R {
    Frame::new()
        .fill(SURFACE)
        .stroke(Stroke::new(1.0_f32, BORDER))
        .corner_radius(CornerRadius::same(12))
        .inner_margin(Margin::same(18))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            add(ui)
        })
        .inner
}

/// Üst kenar vurgulu özel telemetri kartı.
pub fn card_accent<R>(ui: &mut Ui, accent: Color32, add: impl FnOnce(&mut Ui) -> R) -> R {
    let resp = Frame::new()
        .fill(SURFACE)
        .stroke(Stroke::new(1.0_f32, BORDER))
        .corner_radius(CornerRadius::same(12))
        .inner_margin(Margin { left: 16, right: 16, top: 15, bottom: 15 })
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            add(ui)
        });
    let r = resp.response.rect;
    let top_bar = egui::Rect::from_min_max(
        egui::pos2(r.left() + 10.0, r.top() + 0.5),
        egui::pos2(r.right() - 10.0, r.top() + 2.5),
    );
    ui.painter().rect_filled(top_bar, CornerRadius::same(2), mix(accent, Color32::WHITE, 0.12));
    resp.inner
}

/// Paginakop: titel + acties op de eerste rij, ondertitel eronder zodat niets overlapt.
pub fn page_header(ui: &mut Ui, title_text: &str, subtitle: &str, actions: impl FnOnce(&mut Ui)) {
    ui.horizontal(|ui| {
        ui.label(title(title_text));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), actions);
    });
    if !subtitle.is_empty() {
        ui.add_space(2.0);
        ui.add(egui::Label::new(dim(subtitle)).wrap());
    }
    ui.add_space(10.0);
    let (sep_rect, _) = ui.allocate_exact_size(vec2(ui.available_width().max(20.0), 1.0), Sense::hover());
    ui.painter().hline(sep_rect.x_range(), sep_rect.center().y, Stroke::new(1.0_f32, BORDER));
    ui.add_space(14.0);
}

pub fn progress(ui: &mut Ui, value: f32, height: f32, color: Color32) {
    let w = ui.available_width().max(40.0);
    let (rect, _) = ui.allocate_exact_size(vec2(w, height), Sense::hover());
    let p = ui.painter();
    let r = CornerRadius::same((height / 2.0) as u8);
    p.rect_filled(rect, r, TRACK);
    // de balk groeit bij het openen van de pagina en glijdt mee als de waarde verandert
    let shown = ui.ctx().animate_value_with_time(ui.next_auto_id().with("bar"), value, 0.35) * enter(ui.ctx(), 0.6);
    let mut f = rect;
    f.set_width((rect.width() * (shown / 100.0).clamp(0.0, 1.0)).max(if value > 0.0 { height } else { 0.0 }));
    p.rect_filled(f, r, color);
}

/// Ring die met de klok mee vult vanaf 12 uur (0…100).
pub fn ring(ui: &mut Ui, value: f32, size: f32, color: Color32) {
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
    let p = ui.painter();
    let (c, w) = (rect.center(), (size * 0.11).max(3.0));
    let r = size / 2.0 - w / 2.0;
    p.circle_stroke(c, r, Stroke::new(w, TRACK));
    let shown = ui.ctx().animate_value_with_time(ui.next_auto_id().with("ring"), value, 0.35) * enter(ui.ctx(), 0.8);
    let sweep = (shown / 100.0).clamp(0.0, 1.0) * std::f32::consts::TAU;
    if sweep > 0.001 {
        let n = ((sweep / std::f32::consts::TAU) * 96.0).ceil().max(2.0) as usize;
        let at = |a: f32| c + vec2(a.sin(), -a.cos()) * r;
        let pts: Vec<egui::Pos2> = (0..=n).map(|i| at(sweep * i as f32 / n as f32)).collect();
        p.circle_filled(pts[0], w / 2.0, color);
        p.circle_filled(*pts.last().unwrap(), w / 2.0, color);
        p.add(egui::Shape::line(pts, Stroke::new(w, color)));
    }
}

/// Tegel met een groot getal; met `bar` een ring rechts die de waarde toont.
pub fn stat_tile(ui: &mut Ui, icon_s: &str, label: &str, value: &str, sub: &str, color: Color32, bar: Option<f32>) {
    stat_tile_delta(ui, icon_s, label, value, sub, color, bar, None);
}

pub fn stat_tile_delta(
    ui: &mut Ui,
    icon_s: &str,
    label: &str,
    value: &str,
    sub: &str,
    color: Color32,
    bar: Option<f32>,
    delta: Option<f32>,
) {
    card_accent(ui, color, |ui| {
        ui.horizontal(|ui| {
            ui.label(ic(icon_s).size(15.0).color(color));
            ui.add(egui::Label::new(caps(label)).truncate());
        });
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(value).font(FontId::new(30.0, bold())).color(TEXT));
                    if let Some(d) = delta.filter(|x| x.abs() >= 1.0) {
                        let up = d > 0.0;
                        let dc = if up { SUCCESS } else { DANGER };
                        let txt = if up { format!("▲+{:.0}", d) } else { format!("▼{:.0}", d) };
                        ui.label(RichText::new(txt).font(FontId::new(12.0, semibold())).color(dc));
                    }
                });
                if !sub.is_empty() {
                    ui.label(RichText::new(sub).size(12.5).color(if bar.is_some() { color } else { TEXT_FAINT }));
                }
            });
            if let Some(b) = bar {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| ring(ui, b, 52.0, color));
            }
        });
    });
}

/// Klein gekleurd label.
pub fn pill(ui: &mut Ui, text: &str, color: Color32) -> Response {
    let galley = ui.fonts(|f| f.layout_no_wrap(text.to_owned(), FontId::new(12.0, medium()), color));
    let size = vec2(galley.size().x + 18.0, (galley.size().y + 6.0).max(22.0));
    let (rect, resp) = ui.allocate_exact_size(size, Sense::hover());
    if ui.is_rect_visible(rect) {
        let p = ui.painter();
        let r = (rect.height() * 0.5).round().clamp(4.0, 20.0) as u8;
        p.rect_filled(rect, CornerRadius::same(r), tint(color, 0.18));
        let text_pos = egui::pos2(
            rect.center().x - galley.size().x * 0.5,
            rect.center().y - galley.size().y * 0.5,
        );
        p.galley(text_pos, galley, color);
    }
    resp
}

pub fn primary_button(ui: &mut Ui, icon_s: &str, text: &str, enabled: bool) -> Response {
    let base_fill = Color32::from_rgb(5, 150, 105);
    let job = icon_label(icon_s, text, FontId::new(13.5, semibold()), if enabled { Color32::WHITE } else { TEXT_FAINT });
    let galley = ui.fonts(|f| f.layout_job(job));
    let size = vec2(galley.size().x + 28.0, 35.0);
    let (rect, resp) = ui.allocate_exact_size(size, if enabled { Sense::click() } else { Sense::hover() });
    let hover = ui.ctx().animate_bool_with_time(resp.id.with("h"), enabled && resp.hovered(), QUICK);
    let down = enabled && resp.is_pointer_button_down_on();
    let fill = if !enabled { SURFACE2 } else if down { mix(base_fill, BG, 0.22) } else { mix(base_fill, ACCENT, hover) };
    let p = ui.painter();
    if enabled {
        p.rect_filled(rect.translate(vec2(0.0, 2.5)).shrink2(vec2(6.0, 2.0)), CornerRadius::same(10), mix(BG, ACCENT, 0.15 + 0.12 * hover));
    }
    p.rect_filled(rect, CornerRadius::same(9), fill);
    p.galley(rect.center() - galley.size() / 2.0, galley, Color32::WHITE);
    if enabled { resp.on_hover_cursor(egui::CursorIcon::PointingHand) } else { resp }
}

pub fn button(ui: &mut Ui, icon_s: &str, text: &str, enabled: bool) -> Response {
    let col = if enabled { TEXT } else { TEXT_FAINT };
    let b = egui::Button::new(icon_label(icon_s, text, FontId::new(14.0, medium()), col))
        .corner_radius(CornerRadius::same(10))
        .min_size(vec2(0.0, 36.0));
    ui.add_enabled(enabled, b).on_hover_cursor(egui::CursorIcon::PointingHand)
}

pub fn small_button(ui: &mut Ui, text: &str, enabled: bool) -> Response {
    let b = egui::Button::new(RichText::new(text).size(12.5).font(FontId::new(12.5, medium())))
        .corner_radius(CornerRadius::same(8))
        .min_size(vec2(0.0, 26.0));
    ui.add_enabled(enabled, b)
}

/// Sıcak kehribar tonlu destek / Buy Me a Coffee butonu (`full_width = true` yan menü için).
pub fn support_button(ui: &mut Ui, icon_s: &str, text: &str, full_width: bool) -> Response {
    let fg = Color32::from_rgb(252, 214, 134);
    let galley = if text.is_empty() {
        let mut job = egui::text::LayoutJob::default();
        job.append(
            icon_s,
            0.0,
            egui::text::TextFormat { font_id: FontId::new(15.5, icons()), color: fg, valign: egui::Align::Center, ..Default::default() },
        );
        ui.fonts(|f| f.layout_job(job))
    } else {
        let job = icon_label(icon_s, text, FontId::new(13.0, semibold()), fg);
        ui.fonts(|f| f.layout_job(job))
    };
    let w = if full_width {
        ui.available_width().max(galley.size().x + 24.0)
    } else if text.is_empty() {
        36.0
    } else {
        galley.size().x + 28.0
    };
    let h = if full_width { 32.0 } else { 34.0 };
    let (rect, resp) = ui.allocate_exact_size(vec2(w, h), Sense::click());
    let hover = ui.ctx().animate_bool_with_time(resp.id.with("h"), resp.hovered(), QUICK);
    let fill = tint(WARNING, 0.13 + 0.11 * hover);
    let stroke = tint(WARNING, 0.38 + 0.25 * hover);
    let p = ui.painter();
    p.rect_filled(rect, CornerRadius::same(9), fill);
    p.rect_stroke(rect, CornerRadius::same(9), Stroke::new(1.0_f32, stroke), egui::StrokeKind::Inside);
    p.galley(rect.center() - galley.size() / 2.0, galley, fg);
    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// Üst yatay navigasyon sekmesi (Top Command Bar için).
pub fn nav_tab(ui: &mut Ui, icon_s: &str, label: &str, selected: bool) -> Response {
    let font = FontId::new(13.0, medium());
    let mut measure = egui::text::LayoutJob::default();
    let fmt = |f: FontId, c: Color32| egui::text::TextFormat { font_id: f, color: c, valign: egui::Align::Center, ..Default::default() };
    measure.append(icon_s, 0.0, fmt(FontId::new(15.0, icons()), TEXT));
    measure.append(" ", 0.0, fmt(font.clone(), TEXT));
    measure.append(label, 0.0, fmt(font.clone(), TEXT));
    let text_galley = ui.fonts(|f| f.layout_job(measure));
    let w = text_galley.size().x + 20.0;
    let (rect, resp) = ui.allocate_exact_size(vec2(w, 34.0), Sense::click());
    let sel = ui.ctx().animate_bool_with_time(resp.id.with("s"), selected, 0.2);
    let hov = ui.ctx().animate_bool_with_time(resp.id.with("h"), resp.hovered() && !selected, QUICK);
    let p = ui.painter();
    let fill = mix(mix(SIDEBAR, SURFACE2, hov), ACCENT_SOFT, sel);
    p.rect_filled(rect, CornerRadius::same(9), fill);
    if sel > 0.01 {
        let bar_w = (w - 14.0) * sel;
        let bar = egui::Rect::from_center_size(
            egui::pos2(rect.center().x, rect.bottom() - 2.0),
            vec2(bar_w.max(6.0), 2.5),
        );
        p.rect_filled(bar, CornerRadius::same(2), ACCENT_HI);
    }
    let text_col = mix(mix(TEXT_DIM, TEXT, hov), Color32::WHITE, sel);
    let icon_col = mix(mix(TEXT_DIM, TEXT, hov), ACCENT_HI, sel);
    let mut drawn = egui::text::LayoutJob::default();
    drawn.append(icon_s, 0.0, fmt(FontId::new(15.0, icons()), icon_col));
    drawn.append(" ", 0.0, fmt(font.clone(), text_col));
    drawn.append(label, 0.0, fmt(font, text_col));
    let galley = ui.fonts(|f| f.layout_job(drawn));
    p.galley(rect.center() - galley.size() / 2.0, galley, text_col);
    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// Gesegmenteerde keuze (bv. NL | EN, Week | Seizoen). Zelf getekend op een vaste maat, zodat
/// hij in elke omringende layout (ook rechts-naar-links) hetzelfde uitziet.
pub fn segmented<T: PartialEq + Copy>(ui: &mut Ui, value: &mut T, options: &[(T, &str)]) -> bool {
    let font = FontId::new(13.0, medium());
    let (pad, h, gap) = (12.0, 32.0, 2.0);
    let widths: Vec<f32> = options
        .iter()
        .map(|(_, l)| ui.fonts(|f| f.layout_no_wrap(l.to_string(), font.clone(), TEXT).size().x) + 2.0 * pad)
        .collect();
    let total = widths.iter().sum::<f32>() + 6.0 + gap * (options.len().saturating_sub(1)) as f32;
    let (rect, _) = ui.allocate_exact_size(vec2(total, h), Sense::hover());
    let p = ui.painter().clone();
    p.rect_filled(rect, CornerRadius::same(10), SURFACE2);
    let mut x = rect.left() + 3.0;
    let mut changed = false;
    for ((v, label), w) in options.iter().zip(widths) {
        let r = egui::Rect::from_min_size(egui::pos2(x, rect.top() + 3.0), vec2(w, h - 6.0));
        let resp = ui.interact(r, ui.id().with(("segmented", *label)), Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand);
        let sel = *value == *v;
        if sel {
            p.rect_filled(r, CornerRadius::same(8), ACCENT);
        } else if resp.hovered() {
            p.rect_filled(r, CornerRadius::same(8), HOVER);
        }
        p.text(r.center(), egui::Align2::CENTER_CENTER, *label, font.clone(), if sel { Color32::WHITE } else { TEXT_DIM });
        if resp.clicked() && !sel {
            *value = *v;
            changed = true;
        }
        x += w + gap;
    }
    changed
}

/// Nötr zeminli, solda 3 px kategori şeritli, 12 px metinli, 24 px yüksekliğinde seans çipi.
pub fn session_chip_bar(ui: &mut Ui, text: &str, color: Color32, highlight: bool, bold_text: bool) -> Response {
    let w = ui.available_width().max(48.0);
    let h = 24.0;
    let (rect, resp) = ui.allocate_exact_size(vec2(w, h), Sense::hover());
    if ui.is_rect_visible(rect) {
        let p = ui.painter();
        let bg = if highlight { tint(color, 0.16) } else { SURFACE2 };
        let stroke = if highlight { Stroke::new(1.0_f32, color) } else { Stroke::new(1.0_f32, BORDER) };
        p.rect_filled(rect, CornerRadius::same(6), bg);
        p.rect_stroke(rect, CornerRadius::same(6), stroke, egui::StrokeKind::Inside);
        let bar = egui::Rect::from_min_max(rect.min, egui::pos2(rect.left() + 3.0, rect.bottom()));
        p.rect_filled(bar, CornerRadius { nw: 6, sw: 6, ne: 0, se: 0 }, color);
        let font = FontId::new(12.0, if bold_text { bold() } else { medium() });
        let galley = ui.fonts(|f| {
            let mut job = egui::text::LayoutJob::single_section(
                text.to_owned(),
                egui::text::TextFormat { font_id: font, color: TEXT, ..Default::default() },
            );
            job.wrap = egui::text::TextWrapping {
                max_width: (w - 14.0).max(20.0),
                max_rows: 1,
                break_anywhere: true,
                overflow_character: Some('…'),
            };
            f.layout_job(job)
        });
        let pos = egui::pos2(rect.left() + 9.0, rect.center().y - galley.size().y * 0.5);
        p.galley(pos, galley, TEXT);
    }
    resp
}

pub fn status_dot(ui: &mut Ui, color: Color32) {
    let (r, _) = ui.allocate_exact_size(Vec2::splat(10.0), Sense::hover());
    ui.painter().circle_filled(r.center(), 5.0, mix(SURFACE, color, 0.28));
    ui.painter().circle_filled(r.center(), 3.2, color);
}

/// Ringless 2x2 component summary card for Team Analysis (icon, label, big score, band, delta).
pub fn component_tile(
    ui: &mut Ui,
    icon_glyph: &str,
    label: &str,
    value: f32,
    band_label: &str,
    color: Color32,
    delta: Option<f32>,
) {
    let d_opt = delta.filter(|x| x.abs() >= 1.0);
    card(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(ic(icon_glyph).color(color).size(15.0));
            ui.add(egui::Label::new(caps(label)).truncate()).on_hover_text(label);
        });
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.label(
                RichText::new(format!("{value:.0}"))
                    .font(FontId::new(26.0, semibold()))
                    .color(color),
            );
            ui.add_space(4.0);
            ui.label(dim(band_label));
            if let Some(d) = d_opt {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let up = d > 0.0;
                    let dc = if up { SUCCESS } else { DANGER };
                    let txt = if up { format!("▲+{:.0}", d) } else { format!("▼{:.0}", d) };
                    ui.label(RichText::new(txt).font(FontId::new(11.5, semibold())).color(dc));
                });
            }
        });
    });
}

/// Compact 1-row factor bar: name + inline progress bar + right-aligned fixed score + fixed delta.
pub fn factor_bar_row(ui: &mut Ui, name: &str, value: f32, color: Color32, delta: Option<f32>) {
    let d_opt = delta.filter(|x| x.abs() >= 1.0);
    let total_w = ui.available_width().max(200.0);
    let score_w = 32.0;
    let delta_w = 36.0;
    let gap = 8.0;
    let name_w = (total_w * 0.44).clamp(115.0, 215.0);
    let bar_w = (total_w - name_w - score_w - delta_w - gap * 3.0).max(44.0);

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = gap;
        ui.allocate_ui_with_layout(
            vec2(name_w, 20.0),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                ui.add(egui::Label::new(RichText::new(name).size(13.0).color(TEXT_DIM)).truncate())
                    .on_hover_text(name);
            },
        );
        ui.allocate_ui_with_layout(
            vec2(bar_w, 20.0),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                progress(ui, value, 6.0, color);
            },
        );
        ui.allocate_ui_with_layout(
            vec2(score_w, 20.0),
            egui::Layout::right_to_left(egui::Align::Center),
            |ui| {
                ui.label(
                    RichText::new(format!("{value:.0}"))
                        .font(FontId::new(13.5, semibold()))
                        .color(color),
                );
            },
        );
        ui.allocate_ui_with_layout(
            vec2(delta_w, 20.0),
            egui::Layout::right_to_left(egui::Align::Center),
            |ui| {
                if let Some(d) = d_opt {
                    let up = d > 0.0;
                    let dc = if up { SUCCESS } else { DANGER };
                    let txt = if up { format!("▲+{:.0}", d) } else { format!("▼{:.0}", d) };
                    ui.label(RichText::new(txt).font(FontId::new(11.0, semibold())).color(dc));
                }
            },
        );
    });
    ui.add_space(3.0);
}

/// Aan/uit-schakelaar.
pub fn toggle(ui: &mut Ui, on: &mut bool) -> Response {
    let size = vec2(46.0, 26.0);
    let (rect, mut resp) = ui.allocate_exact_size(size, Sense::click());
    if resp.clicked() {
        *on = !*on;
        resp.mark_changed();
    }
    let t = ui.ctx().animate_bool(resp.id, *on);
    let p = ui.painter();
    let bg = mix(Color32::from_rgb(52, 57, 72), SUCCESS, t);
    p.rect_filled(rect, CornerRadius::same(13), bg);
    let x = egui::lerp((rect.left() + 13.0)..=(rect.right() - 13.0), t);
    p.circle_filled(egui::pos2(x, rect.center().y), 10.0, Color32::WHITE);
    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// Yatayda ortalanmış maksimum genişlikli içerik sütunu (geniş ekranlarda sola yaslanmayı önler).
pub fn centered_column(ui: &mut Ui, max_w: f32, add: impl FnOnce(&mut Ui)) {
    let avail = ui.available_width().max(1.0);
    let w = avail.min(max_w);
    let pad = ((avail - w) * 0.5).max(0.0);
    let saved_gap = ui.spacing().item_spacing.x;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        if pad > 0.5 {
            ui.add_space(pad);
        }
        ui.allocate_ui_with_layout(
            vec2(w, 0.0),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                ui.spacing_mut().item_spacing.x = saved_gap;
                ui.set_min_width(w);
                ui.set_max_width(w);
                add(ui);
            },
        );
    });
}

/// İçeriği kadar yer kaplayan sıkı bağlantı durumu hapı.
pub fn status_pill(ui: &mut Ui, dot_color: Color32, text: &str, suffix: Option<&str>) -> Response {
    let font_main = FontId::new(12.5, FontFamily::Proportional);
    let font_sub = FontId::new(11.5, FontFamily::Proportional);
    let main_galley = ui.fonts(|f| f.layout_no_wrap(text.to_owned(), font_main, TEXT));
    let suf_text = suffix.map(|s| format!(" · {s}"));
    let suf_galley = suf_text.as_ref().map(|s| ui.fonts(|f| f.layout_no_wrap(s.clone(), font_sub, TEXT_FAINT)));
    let max_text_w = 190.0_f32;
    let main_w = main_galley.size().x.min(max_text_w);
    let suf_w = suf_galley.as_ref().map_or(0.0, |g| g.size().x);
    let total_w = 12.0 + 10.0 + 6.0 + main_w + suf_w + 12.0;
    let (rect, resp) = ui.allocate_exact_size(vec2(total_w, 28.0), Sense::hover());
    if ui.is_rect_visible(rect) {
        let p = ui.painter();
        p.rect_filled(rect, CornerRadius::same(14), SURFACE);
        p.rect_stroke(rect, CornerRadius::same(14), Stroke::new(1.0_f32, BORDER), egui::StrokeKind::Inside);
        let dot_c = egui::pos2(rect.left() + 15.0, rect.center().y);
        p.circle_filled(dot_c, 5.0, mix(SURFACE, dot_color, 0.28));
        p.circle_filled(dot_c, 3.2, dot_color);
        let text_x = rect.left() + 26.0;
        let main_y = rect.center().y - main_galley.size().y * 0.5;
        p.galley(egui::pos2(text_x, main_y), main_galley, TEXT);
        if let Some(sg) = suf_galley {
            let suf_y = rect.center().y - sg.size().y * 0.5;
            p.galley(egui::pos2(text_x + main_w, suf_y), sg, TEXT_FAINT);
        }
    }
    resp
}

/// Ayarlar sayfası ortak satırı: solda etiket + tek satırlık yardım metni, sağda kontrol ve altta ayırıcı çizgi.
#[allow(dead_code)]
pub fn setting_row(ui: &mut Ui, label: &str, help: Option<&str>, control: impl FnOnce(&mut Ui)) {
    setting_row_tip(ui, label, help, None, true, control);
}

pub fn setting_row_tip(
    ui: &mut Ui,
    label: &str,
    help: Option<&str>,
    tip: Option<&str>,
    divider: bool,
    control: impl FnOnce(&mut Ui),
) {
    let w = ui.available_width().max(260.0);
    let gap = 16.0;
    let right_w = (w * 0.34).clamp(160.0, 240.0);
    let left_w = (w - right_w - gap).max(120.0);
    let saved_gap = ui.spacing().item_spacing.x;

    ui.add_space(6.0);
    ui.allocate_ui_with_layout(
        vec2(w, 0.0),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.set_max_width(w);
            ui.allocate_ui_with_layout(
                vec2(left_w, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    ui.spacing_mut().item_spacing.x = saved_gap;
                    ui.set_min_width(left_w);
                    ui.set_max_width(left_w);
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new(label).font(FontId::new(13.0, medium())).color(TEXT));
                        if let Some(t) = tip {
                            ui.label(ic(icon::INFO).color(TEXT_FAINT).size(13.5)).on_hover_text(t);
                        }
                    });
                    if let Some(h) = help {
                        ui.add(egui::Label::new(RichText::new(h).size(12.0).color(TEXT_DIM)).wrap());
                    }
                },
            );
            ui.add_space(gap);
            ui.allocate_ui_with_layout(
                vec2(right_w, 0.0),
                egui::Layout::right_to_left(egui::Align::Center),
                |ui| {
                    ui.spacing_mut().item_spacing.x = saved_gap;
                    ui.set_min_width(right_w);
                    ui.set_max_width(right_w);
                    control(ui);
                },
            );
        },
    );
    ui.add_space(6.0);
    if divider {
        let (sep_rect, _) = ui.allocate_exact_size(vec2(w, 1.0), Sense::hover());
        ui.painter().hline(sep_rect.x_range(), sep_rect.center().y, Stroke::new(1.0_f32, mix(SURFACE, BORDER, 0.75)));
    }
}
