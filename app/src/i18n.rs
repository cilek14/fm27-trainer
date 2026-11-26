//! Nederlands / Engels / Türkçe / Français / Italiano / Español.
//! Teksten staan naast elkaar in de code: `t!("Nederlands", "English")`
//! (veya 3 argümanlı `t!("Nederlands", "English", "Türkçe")`).

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU8, Ordering};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Lang {
    Nl,
    En,
    Tr,
    Fr,
    It,
    Es,
}

static LANG: AtomicU8 = AtomicU8::new(1);

pub fn set(l: Lang) {
    let v = match l {
        Lang::Nl => 0,
        Lang::En => 1,
        Lang::Tr => 2,
        Lang::Fr => 3,
        Lang::It => 4,
        Lang::Es => 5,
    };
    LANG.store(v, Ordering::Relaxed);
}

pub fn get() -> Lang {
    match LANG.load(Ordering::Relaxed) {
        0 => Lang::Nl,
        2 => Lang::Tr,
        3 => Lang::Fr,
        4 => Lang::It,
        5 => Lang::Es,
        _ => Lang::En,
    }
}

pub fn is_nl() -> bool {
    get() == Lang::Nl
}

pub fn is_tr() -> bool {
    get() == Lang::Tr
}

/// Sistem dilini algılar: TR / NL / FR / IT / ES, diğer -> EN.
pub fn detect() -> Lang {
    match sys_locale::get_locale() {
        Some(l) if l.to_lowercase().starts_with("tr") => Lang::Tr,
        Some(l) if l.to_lowercase().starts_with("nl") => Lang::Nl,
        Some(l) if l.to_lowercase().starts_with("fr") => Lang::Fr,
        Some(l) if l.to_lowercase().starts_with("it") => Lang::It,
        Some(l) if l.to_lowercase().starts_with("es") => Lang::Es,
        _ => Lang::En,
    }
}

pub trait Translatable {
    fn translate(self, lang: Lang) -> Self;
}

impl Translatable for &'static str {
    fn translate(self, lang: Lang) -> Self {
        match lang {
            Lang::Tr => crate::i18n_tr::translate_static(self),
            Lang::Fr => crate::i18n_fr::translate_static(self),
            Lang::It => crate::i18n_it::translate_static(self),
            Lang::Es => crate::i18n_es::translate_static(self),
            _ => self,
        }
    }
}

impl Translatable for String {
    fn translate(self, lang: Lang) -> Self {
        match lang {
            Lang::Tr => crate::i18n_tr::translate_dynamic(self),
            Lang::Fr => crate::i18n_fr::translate_dynamic(self),
            Lang::It => crate::i18n_it::translate_dynamic(self),
            Lang::Es => crate::i18n_es::translate_dynamic(self),
            _ => self,
        }
    }
}

/// Seçili dile göre metni döner; `&'static str` ve `String` için çalışır.
#[macro_export]
macro_rules! t {
    ($nl:expr, $en:expr $(,)?) => {
        match $crate::i18n::get() {
            $crate::i18n::Lang::Nl => $nl,
            $crate::i18n::Lang::En => $en,
            other => $crate::i18n::Translatable::translate($en, other),
        }
    };
    ($nl:expr, $en:expr, $tr:expr $(,)?) => {
        match $crate::i18n::get() {
            $crate::i18n::Lang::Nl => $nl,
            $crate::i18n::Lang::En => $en,
            $crate::i18n::Lang::Tr => $tr,
            other => $crate::i18n::Translatable::translate($en, other),
        }
    };
}

/// Kısa ay adları.
pub fn month_short(m: u32) -> &'static str {
    const NL: [&str; 12] = ["jan", "feb", "mrt", "apr", "mei", "jun", "jul", "aug", "sep", "okt", "nov", "dec"];
    const EN: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    const TR: [&str; 12] = ["Oca", "Şub", "Mar", "Nis", "May", "Haz", "Tem", "Ağu", "Eyl", "Eki", "Kas", "Ara"];
    const FR: [&str; 12] = ["jan", "fév", "mar", "avr", "mai", "juin", "juil", "aoû", "sep", "oct", "nov", "déc"];
    const IT: [&str; 12] = ["gen", "feb", "mar", "apr", "mag", "giu", "lug", "ago", "set", "ott", "nov", "dic"];
    const ES: [&str; 12] = ["ene", "feb", "mar", "abr", "may", "jun", "jul", "ago", "sep", "oct", "nov", "dic"];
    let i = (m.clamp(1, 12) - 1) as usize;
    match get() {
        Lang::Nl => NL[i],
        Lang::En => EN[i],
        Lang::Tr => TR[i],
        Lang::Fr => FR[i],
        Lang::It => IT[i],
        Lang::Es => ES[i],
    }
}

pub fn weekday_short(i: usize) -> &'static str {
    const NL: [&str; 7] = ["ma", "di", "wo", "do", "vr", "za", "zo"];
    const EN: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    const TR: [&str; 7] = ["Pzt", "Sal", "Çar", "Per", "Cum", "Cmt", "Paz"];
    const FR: [&str; 7] = ["Lun", "Mar", "Mer", "Jeu", "Ven", "Sam", "Dim"];
    const IT: [&str; 7] = ["Lun", "Mar", "Mer", "Gio", "Ven", "Sab", "Dom"];
    const ES: [&str; 7] = ["Lun", "Mar", "Mié", "Jue", "Vie", "Sáb", "Dom"];
    match get() {
        Lang::Nl => NL[i % 7],
        Lang::En => EN[i % 7],
        Lang::Tr => TR[i % 7],
        Lang::Fr => FR[i % 7],
        Lang::It => IT[i % 7],
        Lang::Es => ES[i % 7],
    }
}

pub fn date(d: chrono::NaiveDate) -> String {
    use chrono::Datelike;
    if get() != Lang::En {
        format!("{} {} {}", d.day(), month_short(d.month()), d.year())
    } else {
        format!("{} {} {}", month_short(d.month()), d.day(), d.year())
    }
}

pub fn date_short(d: chrono::NaiveDate) -> String {
    use chrono::Datelike;
    if get() != Lang::En {
        format!("{} {}", d.day(), month_short(d.month()))
    } else {
        format!("{} {}", month_short(d.month()), d.day())
    }
}
