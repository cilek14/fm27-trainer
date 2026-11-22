//! Nederlands / Engels / Türkçe. Teksten staan naast elkaar in de code: `t!("Nederlands", "English")`
//! (veya 3 argümanlı `t!("Nederlands", "English", "Türkçe")`).

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU8, Ordering};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Lang {
    Nl,
    En,
    Tr,
}

static LANG: AtomicU8 = AtomicU8::new(1);

pub fn set(l: Lang) {
    let v = match l {
        Lang::Nl => 0,
        Lang::En => 1,
        Lang::Tr => 2,
    };
    LANG.store(v, Ordering::Relaxed);
}

pub fn get() -> Lang {
    match LANG.load(Ordering::Relaxed) {
        0 => Lang::Nl,
        2 => Lang::Tr,
        _ => Lang::En,
    }
}

pub fn is_nl() -> bool {
    get() == Lang::Nl
}

pub fn is_tr() -> bool {
    get() == Lang::Tr
}

/// Sistem dilini algılar: Türkçe -> Tr, Hollandaca -> Nl, diğer -> En.
pub fn detect() -> Lang {
    match sys_locale::get_locale() {
        Some(l) if l.to_lowercase().starts_with("tr") => Lang::Tr,
        Some(l) if l.to_lowercase().starts_with("nl") => Lang::Nl,
        _ => Lang::En,
    }
}

pub trait Translatable {
    fn tr(self) -> Self;
}

impl Translatable for &'static str {
    fn tr(self) -> Self {
        crate::i18n_tr::translate_static(self)
    }
}

impl Translatable for String {
    fn tr(self) -> Self {
        crate::i18n_tr::translate_dynamic(self)
    }
}

/// Seçili dile göre metni döner; `&'static str` ve `String` için çalışır.
#[macro_export]
macro_rules! t {
    ($nl:expr, $en:expr $(,)?) => {
        match $crate::i18n::get() {
            $crate::i18n::Lang::Nl => $nl,
            $crate::i18n::Lang::En => $en,
            $crate::i18n::Lang::Tr => $crate::i18n::Translatable::tr($en),
        }
    };
    ($nl:expr, $en:expr, $tr:expr $(,)?) => {
        match $crate::i18n::get() {
            $crate::i18n::Lang::Nl => $nl,
            $crate::i18n::Lang::En => $en,
            $crate::i18n::Lang::Tr => $tr,
        }
    };
}

/// Maandnamen voor datums in de UI.
pub fn month_short(m: u32) -> &'static str {
    const NL: [&str; 12] = ["jan", "feb", "mrt", "apr", "mei", "jun", "jul", "aug", "sep", "okt", "nov", "dec"];
    const EN: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    const TR: [&str; 12] = ["Oca", "Şub", "Mar", "Nis", "May", "Haz", "Tem", "Ağu", "Eyl", "Eki", "Kas", "Ara"];
    let i = (m.clamp(1, 12) - 1) as usize;
    match get() {
        Lang::Nl => NL[i],
        Lang::En => EN[i],
        Lang::Tr => TR[i],
    }
}

pub fn weekday_short(i: usize) -> &'static str {
    const NL: [&str; 7] = ["ma", "di", "wo", "do", "vr", "za", "zo"];
    const EN: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    const TR: [&str; 7] = ["Pzt", "Sal", "Çar", "Per", "Cum", "Cmt", "Paz"];
    match get() {
        Lang::Nl => NL[i % 7],
        Lang::En => EN[i % 7],
        Lang::Tr => TR[i % 7],
    }
}

pub fn date(d: chrono::NaiveDate) -> String {
    use chrono::Datelike;
    if is_nl() || is_tr() {
        format!("{} {} {}", d.day(), month_short(d.month()), d.year())
    } else {
        format!("{} {} {}", month_short(d.month()), d.day(), d.year())
    }
}

pub fn date_short(d: chrono::NaiveDate) -> String {
    use chrono::Datelike;
    if is_nl() || is_tr() {
        format!("{} {}", d.day(), month_short(d.month()))
    } else {
        format!("{} {}", month_short(d.month()), d.day())
    }
}
