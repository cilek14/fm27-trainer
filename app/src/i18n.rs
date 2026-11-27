//! Nederlands / English / Türkçe / Deutsch / Français / Italiano / Español.
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
    De,
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
        Lang::De => 6,
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
        6 => Lang::De,
        _ => Lang::En,
    }
}

pub fn code(l: Lang) -> &'static str {
    match l {
        Lang::Nl => "nl",
        Lang::En => "en",
        Lang::Tr => "tr",
        Lang::Fr => "fr",
        Lang::It => "it",
        Lang::Es => "es",
        Lang::De => "de",
    }
}

pub fn is_nl() -> bool {
    get() == Lang::Nl
}

pub fn is_tr() -> bool {
    get() == Lang::Tr
}

/// Sistem dil kodunu uygulama diline eşler; desteklenmiyorsa İngilizce döner.
pub fn from_locale(locale: Option<&str>) -> Lang {
    let Some(raw) = locale else {
        return Lang::En;
    };
    let primary = raw
        .trim()
        .split(['-', '_', '.'])
        .next()
        .unwrap_or("")
        .to_ascii_lowercase();
    match primary.as_str() {
        "tr" => Lang::Tr,
        "nl" => Lang::Nl,
        "de" => Lang::De,
        "fr" => Lang::Fr,
        "it" => Lang::It,
        "es" => Lang::Es,
        "en" => Lang::En,
        _ => Lang::En,
    }
}

/// Sistem dilini algılar: TR / NL / DE / FR / IT / ES, diğer -> EN.
pub fn detect() -> Lang {
    from_locale(sys_locale::get_locale().as_deref())
}

pub trait Translatable {
    fn translate(self, lang: Lang) -> Self;
}

impl Translatable for &'static str {
    fn translate(self, lang: Lang) -> Self {
        match lang {
            Lang::Tr => crate::i18n_tr::translate_static(self),
            Lang::De => crate::i18n_de::translate_static(self),
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
            Lang::De => crate::i18n_de::translate_dynamic(self),
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
    const DE: [&str; 12] = ["Jan", "Feb", "Mär", "Apr", "Mai", "Jun", "Jul", "Aug", "Sep", "Okt", "Nov", "Dez"];
    const FR: [&str; 12] = ["jan", "fév", "mar", "avr", "mai", "juin", "juil", "aoû", "sep", "oct", "nov", "déc"];
    const IT: [&str; 12] = ["gen", "feb", "mar", "apr", "mag", "giu", "lug", "ago", "set", "ott", "nov", "dic"];
    const ES: [&str; 12] = ["ene", "feb", "mar", "abr", "may", "jun", "jul", "ago", "sep", "oct", "nov", "dic"];
    let i = (m.clamp(1, 12) - 1) as usize;
    match get() {
        Lang::Nl => NL[i],
        Lang::En => EN[i],
        Lang::Tr => TR[i],
        Lang::De => DE[i],
        Lang::Fr => FR[i],
        Lang::It => IT[i],
        Lang::Es => ES[i],
    }
}

pub fn weekday_short(i: usize) -> &'static str {
    const NL: [&str; 7] = ["ma", "di", "wo", "do", "vr", "za", "zo"];
    const EN: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    const TR: [&str; 7] = ["Pzt", "Sal", "Çar", "Per", "Cum", "Cmt", "Paz"];
    const DE: [&str; 7] = ["Mo", "Di", "Mi", "Do", "Fr", "Sa", "So"];
    const FR: [&str; 7] = ["Lun", "Mar", "Mer", "Jeu", "Ven", "Sam", "Dim"];
    const IT: [&str; 7] = ["Lun", "Mar", "Mer", "Gio", "Ven", "Sab", "Dom"];
    const ES: [&str; 7] = ["Lun", "Mar", "Mié", "Jue", "Vie", "Sáb", "Dom"];
    match get() {
        Lang::Nl => NL[i % 7],
        Lang::En => EN[i % 7],
        Lang::Tr => TR[i % 7],
        Lang::De => DE[i % 7],
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_locale_maps_supported_languages_and_defaults_to_english() {
        assert_eq!(from_locale(Some("de-DE")), Lang::De);
        assert_eq!(from_locale(Some("de_AT.UTF-8")), Lang::De);
        assert_eq!(from_locale(Some("de-CH")), Lang::De);
        assert_eq!(from_locale(Some("tr-TR")), Lang::Tr);
        assert_eq!(from_locale(Some("nl-NL")), Lang::Nl);
        assert_eq!(from_locale(Some("nl-BE")), Lang::Nl);
        assert_eq!(from_locale(Some("fr-FR")), Lang::Fr);
        assert_eq!(from_locale(Some("it-IT")), Lang::It);
        assert_eq!(from_locale(Some("es-ES")), Lang::Es);
        assert_eq!(from_locale(Some("en-GB")), Lang::En);
        assert_eq!(from_locale(Some("ja-JP")), Lang::En);
        assert_eq!(from_locale(Some("pl-PL")), Lang::En);
        assert_eq!(from_locale(Some("")), Lang::En);
        assert_eq!(from_locale(None), Lang::En);

        assert_eq!(code(Lang::De), "de");
        assert_eq!(code(Lang::Tr), "tr");
        assert_eq!(code(Lang::En), "en");
    }
}
