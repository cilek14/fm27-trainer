//! Datamodel: de vier onderdelen van FM27 Tactical Effectiveness, de 17 key factors en de
//! trainingssessies. Namen zijn letterlijk zoals FM27 ze toont (uit de game-binary gehaald),
//! zodat ze in het spel één-op-één terug te vinden zijn.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Component {
    TacticalUnderstanding,
    PlayerAbility,
    PlayerBuyIn,
    NextMatchInfo,
}

impl Component {
    pub const ALL: [Component; 4] = [
        Component::TacticalUnderstanding,
        Component::PlayerAbility,
        Component::PlayerBuyIn,
        Component::NextMatchInfo,
    ];

    /// Titel zoals in FM27.
    pub fn label(self) -> &'static str {
        match self {
            Component::TacticalUnderstanding => "Tactical Understanding",
            Component::PlayerAbility => "Player Ability",
            Component::PlayerBuyIn => "Player Buy-In",
            Component::NextMatchInfo => "Next Match Info",
        }
    }

    /// Arayüzde seçili dile göre gösterilen bileşen adı.
    pub fn display_label_for(self, lang: crate::i18n::Lang) -> &'static str {
        use crate::i18n::Lang::*;
        match (lang, self) {
            (Nl, Component::TacticalUnderstanding) => "Tactisch begrip",
            (Nl, Component::PlayerAbility) => "Spelerskwaliteit",
            (Nl, Component::PlayerBuyIn) => "Spelersdraagvlak",
            (Nl, Component::NextMatchInfo) => "Volgende wedstrijd",
            (Tr, Component::TacticalUnderstanding) => "Taktiksel Anlayış",
            (Tr, Component::PlayerAbility) => "Oyuncu Yeteneği",
            (Tr, Component::PlayerBuyIn) => "Oyuncu Benimsemesi",
            (Tr, Component::NextMatchInfo) => "Sonraki Maç Bilgisi",
            (Fr, Component::TacticalUnderstanding) => "Compréhension tactique",
            (Fr, Component::PlayerAbility) => "Niveau des joueurs",
            (Fr, Component::PlayerBuyIn) => "Adhésion des joueurs",
            (Fr, Component::NextMatchInfo) => "Infos prochain match",
            (It, Component::TacticalUnderstanding) => "Comprensione tattica",
            (It, Component::PlayerAbility) => "Abilità giocatori",
            (It, Component::PlayerBuyIn) => "Coinvolgimento squadra",
            (It, Component::NextMatchInfo) => "Info prossima partita",
            (Es, Component::TacticalUnderstanding) => "Comprensión táctica",
            (Es, Component::PlayerAbility) => "Calidad de jugadores",
            (Es, Component::PlayerBuyIn) => "Compromiso del equipo",
            (Es, Component::NextMatchInfo) => "Info próximo partido",
            (En, _) => self.label(),
        }
    }

    pub fn display_label(self) -> &'static str {
        self.display_label_for(crate::i18n::get())
    }

}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Factor {
    // Tactical Understanding → Tactical Areas (in balbezit)
    InPossessionShape,
    PassingStyle,
    BuildUp,
    ChanceCreation,
    // Tactical Understanding → Tactical Areas (zonder bal)
    OutOfPossessionShape,
    LineOfEngagement,
    PressingIntensity,
    PressingTrap,
    // Tactical Understanding → Tactical Comprehension
    PlayerTacticalAbility,
    TacticalComplexity,
    // Player Ability
    PlayerCapability,
    PlayerRoleSuitability,
    // Player Buy-In
    ManagerialSupport,
    TeammatesAndStaffRelationships,
    ManagerTendencies,
    // Next Match Info
    QualityOfNextOppositionReport,
    OppositionPlayerKnowledge,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    InPossession,
    OutOfPossession,
}

impl Factor {
    pub const ALL: [Factor; 17] = [
        Factor::InPossessionShape,
        Factor::PassingStyle,
        Factor::BuildUp,
        Factor::ChanceCreation,
        Factor::OutOfPossessionShape,
        Factor::LineOfEngagement,
        Factor::PressingIntensity,
        Factor::PressingTrap,
        Factor::PlayerTacticalAbility,
        Factor::TacticalComplexity,
        Factor::PlayerCapability,
        Factor::PlayerRoleSuitability,
        Factor::ManagerialSupport,
        Factor::TeammatesAndStaffRelationships,
        Factor::ManagerTendencies,
        Factor::QualityOfNextOppositionReport,
        Factor::OppositionPlayerKnowledge,
    ];

    pub const TACTICAL_AREAS: [Factor; 8] = [
        Factor::InPossessionShape,
        Factor::PassingStyle,
        Factor::BuildUp,
        Factor::ChanceCreation,
        Factor::OutOfPossessionShape,
        Factor::LineOfEngagement,
        Factor::PressingIntensity,
        Factor::PressingTrap,
    ];

    /// Label zoals in het Breakdown-scherm van FM27.
    pub fn label(self) -> &'static str {
        match self {
            Factor::InPossessionShape => "In Possession Shape",
            Factor::PassingStyle => "Passing Style",
            Factor::BuildUp => "Build Up",
            Factor::ChanceCreation => "Chance Creation",
            Factor::OutOfPossessionShape => "Out Of Possession Shape",
            Factor::LineOfEngagement => "Line of Engagement",
            Factor::PressingIntensity => "Pressing Intensity",
            Factor::PressingTrap => "Pressing Trap",
            Factor::PlayerTacticalAbility => "Player Tactical Ability",
            Factor::TacticalComplexity => "Tactical Complexity",
            Factor::PlayerCapability => "Player Capability",
            Factor::PlayerRoleSuitability => "Player Role Suitability",
            Factor::ManagerialSupport => "Managerial Support",
            Factor::TeammatesAndStaffRelationships => "Teammates and Staff Relationships",
            Factor::ManagerTendencies => "Manager Tendencies",
            Factor::QualityOfNextOppositionReport => "Quality of Next Opposition Report",
            Factor::OppositionPlayerKnowledge => "Opposition Player Knowledge",
        }
    }

    /// Arayüzde seçili dile göre gösterilen faktör adı.
    pub fn display_label_for(self, lang: crate::i18n::Lang) -> &'static str {
        use crate::i18n::Lang::*;
        match lang {
            Tr => match self {
                Factor::InPossessionShape => "Topa Sahipken Diziliş",
                Factor::PassingStyle => "Pas Stili",
                Factor::BuildUp => "Oyun Kurma",
                Factor::ChanceCreation => "Pozisyon Üretme",
                Factor::OutOfPossessionShape => "Topsuz Alanda Diziliş",
                Factor::LineOfEngagement => "Karşılama Çizgisi",
                Factor::PressingIntensity => "Pres Şiddeti",
                Factor::PressingTrap => "Pres Tuzağı",
                Factor::PlayerTacticalAbility => "Oyuncu Taktik Yeteneği",
                Factor::TacticalComplexity => "Taktiksel Karmaşıklık",
                Factor::PlayerCapability => "Oyuncu Yeterliliği",
                Factor::PlayerRoleSuitability => "Oyuncu Rol Uygunluğu",
                Factor::ManagerialSupport => "Menajer Desteği",
                Factor::TeammatesAndStaffRelationships => "Takım & Personel İlişkileri",
                Factor::ManagerTendencies => "Menajer Eğilimleri",
                Factor::QualityOfNextOppositionReport => "Rakip Raporu Kalitesi",
                Factor::OppositionPlayerKnowledge => "Rakip Oyuncu Bilgisi",
            },
            Fr => match self {
                Factor::InPossessionShape => "Placement en possession",
                Factor::PassingStyle => "Style de passes",
                Factor::BuildUp => "Construction",
                Factor::ChanceCreation => "Création d'occasions",
                Factor::OutOfPossessionShape => "Placement sans ballon",
                Factor::LineOfEngagement => "Ligne d'engagement",
                Factor::PressingIntensity => "Intensité du pressing",
                Factor::PressingTrap => "Piège de pressing",
                Factor::PlayerTacticalAbility => "Capacité tactique",
                Factor::TacticalComplexity => "Complexité tactique",
                Factor::PlayerCapability => "Niveau des joueurs",
                Factor::PlayerRoleSuitability => "Adéquation aux rôles",
                Factor::ManagerialSupport => "Soutien à l'entraîneur",
                Factor::TeammatesAndStaffRelationships => "Cohésion groupe & staff",
                Factor::ManagerTendencies => "Tendances de l'entraîneur",
                Factor::QualityOfNextOppositionReport => "Qualité du rapport adverse",
                Factor::OppositionPlayerKnowledge => "Connaissance joueurs adverses",
            },
            It => match self {
                Factor::InPossessionShape => "Disposizione in possesso",
                Factor::PassingStyle => "Stile di passaggio",
                Factor::BuildUp => "Costruzione",
                Factor::ChanceCreation => "Creazione occasioni",
                Factor::OutOfPossessionShape => "Disposizione senza palla",
                Factor::LineOfEngagement => "Linea di ingaggio",
                Factor::PressingIntensity => "Intensità pressing",
                Factor::PressingTrap => "Trappola di pressing",
                Factor::PlayerTacticalAbility => "Abilità tattica giocatori",
                Factor::TacticalComplexity => "Complessità tattica",
                Factor::PlayerCapability => "Capacità dei giocatori",
                Factor::PlayerRoleSuitability => "Idoneità ai ruoli",
                Factor::ManagerialSupport => "Sostegno all'allenatore",
                Factor::TeammatesAndStaffRelationships => "Rapporti squadra e staff",
                Factor::ManagerTendencies => "Tendenze dell'allenatore",
                Factor::QualityOfNextOppositionReport => "Qualità report avversario",
                Factor::OppositionPlayerKnowledge => "Conoscenza giocatori avversari",
            },
            Es => match self {
                Factor::InPossessionShape => "Disposición con balón",
                Factor::PassingStyle => "Estilo de pases",
                Factor::BuildUp => "Salida de balón",
                Factor::ChanceCreation => "Creación de ocasiones",
                Factor::OutOfPossessionShape => "Disposición sin balón",
                Factor::LineOfEngagement => "Línea de presión",
                Factor::PressingIntensity => "Intensidad de presión",
                Factor::PressingTrap => "Trampa de presión",
                Factor::PlayerTacticalAbility => "Capacidad táctica",
                Factor::TacticalComplexity => "Complejidad táctica",
                Factor::PlayerCapability => "Capacidad del jugador",
                Factor::PlayerRoleSuitability => "Idoneidad en el rol",
                Factor::ManagerialSupport => "Respaldo al mánager",
                Factor::TeammatesAndStaffRelationships => "Relación plantilla y cuerpo técnico",
                Factor::ManagerTendencies => "Tendencias del mánager",
                Factor::QualityOfNextOppositionReport => "Calidad del informe rival",
                Factor::OppositionPlayerKnowledge => "Conocimiento jugadores rivales",
            },
            En | Nl => self.label(),
        }
    }

    pub fn display_label(self) -> &'static str {
        self.display_label_for(crate::i18n::get())
    }

    pub fn component(self) -> Component {
        use Factor::*;
        match self {
            InPossessionShape | PassingStyle | BuildUp | ChanceCreation | OutOfPossessionShape
            | LineOfEngagement | PressingIntensity | PressingTrap | PlayerTacticalAbility
            | TacticalComplexity => Component::TacticalUnderstanding,
            PlayerCapability | PlayerRoleSuitability => Component::PlayerAbility,
            ManagerialSupport | TeammatesAndStaffRelationships | ManagerTendencies => {
                Component::PlayerBuyIn
            }
            QualityOfNextOppositionReport | OppositionPlayerKnowledge => Component::NextMatchInfo,
        }
    }

    pub fn is_tactical_area(self) -> bool {
        Factor::TACTICAL_AREAS.contains(&self)
    }

    pub fn phase(self) -> Option<Phase> {
        use Factor::*;
        match self {
            InPossessionShape | PassingStyle | BuildUp | ChanceCreation => Some(Phase::InPossession),
            OutOfPossessionShape | LineOfEngagement | PressingIntensity | PressingTrap => {
                Some(Phase::OutOfPossession)
            }
            _ => None,
        }
    }

    /// Wat je eraan kunt doen (kort).
    pub fn advice(self) -> &'static str {
        use Factor::*;
        match self {
            InPossessionShape | PassingStyle | BuildUp | ChanceCreation | OutOfPossessionShape
            | LineOfEngagement | PressingIntensity | PressingTrap => crate::t!(
                "Train de sessie met dezelfde naam (groep Tactical Understanding). Zet je Primary Trained Tactic op de tactiek die je speelt.",
                "Train the session with the same name (Tactical Understanding group). Set your Primary Trained Tactic to the tactic you play."
            ),
            PlayerTacticalAbility => crate::t!(
                "Hangt af van de spelers zelf (beslissingen, anticipatie, teamwork). Speel spelers met een hogere Tactic Ability of houd de instructies eenvoudiger.",
                "Depends on the players themselves (decisions, anticipation, teamwork). Pick players with higher Tactic Ability or keep instructions simpler."
            ),
            TacticalComplexity => crate::t!(
                "Te veel team-, speler- en tegenstanderinstructies. Schrap instructies die weinig opleveren en voeg ze later weer toe.",
                "Too many team, player and opposition instructions. Drop the ones that add little and bring them back later."
            ),
            PlayerCapability => crate::t!(
                "Spelers kunnen de rollen niet goed genoeg uitvoeren. Kies rollen die bij hun attributen passen of versterk de selectie.",
                "Players struggle to carry out their roles. Pick roles that suit their attributes or strengthen the squad."
            ),
            PlayerRoleSuitability => crate::t!(
                "Speel spelers op hun natuurlijke positie en rol. Check in Player View wie het cijfer omlaag trekt.",
                "Play people in their natural position and role. Use Player View to see who drags the rating down."
            ),
            ManagerialSupport => crate::t!(
                "Resultaten, goede teamtalks en positieve gesprekken met spelers verhogen de steun.",
                "Results, good team talks and positive player interactions raise support."
            ),
            TeammatesAndStaffRelationships => crate::t!(
                "Plan Team Bonding (Extra-Curricular). Houd de kern van de selectie bij elkaar en vermijd onrust.",
                "Schedule Team Bonding (Extra-Curricular). Keep the core of the squad together and avoid unrest."
            ),
            ManagerTendencies => crate::t!(
                "Spelers verwachten de speelstijl van je managerprofiel. Hoe meer je tactiek afwijkt, hoe lager dit.",
                "Players expect the style of your manager profile. The further your tactic strays from it, the lower this is."
            ),
            QualityOfNextOppositionReport => crate::t!(
                "Laat je analist/scouts de volgende tegenstander rapporteren en neem goede analisten aan.",
                "Have your analysts/scouts report on the next opponent, and hire good analysts."
            ),
            OppositionPlayerKnowledge => crate::t!(
                "Groeit door scouting van de tegenstander en hun spelers. Laat onbekende (Europese) tegenstanders vooraf scouten.",
                "Grows by scouting the opponent and their players. Scout unknown (European) opponents in advance."
            ),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Category {
    TacticalUnderstanding,
    MatchPreparation,
    FootballingDevelopment,
    PhysicalRecovery,
    ExtraCurricular,
    Fixed,
}

impl Category {
    pub fn label(self) -> &'static str {
        match self {
            Category::TacticalUnderstanding => "Tactical Understanding",
            Category::MatchPreparation => "Match Preparation",
            Category::FootballingDevelopment => "Footballing Development",
            Category::PhysicalRecovery => "Physical and Recovery",
            Category::ExtraCurricular => "Extra-Curricular",
            Category::Fixed => "Match / Travel",
        }
    }

    pub fn display_label_for(self, lang: crate::i18n::Lang) -> &'static str {
        use crate::i18n::Lang::*;
        match lang {
            Tr => match self {
                Category::TacticalUnderstanding => "Taktiksel Anlayış",
                Category::MatchPreparation => "Maç Hazırlığı",
                Category::FootballingDevelopment => "Futbol Gelişimi",
                Category::PhysicalRecovery => "Fiziksel ve Toparlanma",
                Category::ExtraCurricular => "Takım Dışı / Sosyal",
                Category::Fixed => "Maç / Seyahat",
            },
            Fr => match self {
                Category::TacticalUnderstanding => "Compréhension tactique",
                Category::MatchPreparation => "Préparation de match",
                Category::FootballingDevelopment => "Développement football",
                Category::PhysicalRecovery => "Physique et récupération",
                Category::ExtraCurricular => "Extra-sportif",
                Category::Fixed => "Match / Déplacement",
            },
            It => match self {
                Category::TacticalUnderstanding => "Comprensione tattica",
                Category::MatchPreparation => "Preparazione partita",
                Category::FootballingDevelopment => "Sviluppo calcistico",
                Category::PhysicalRecovery => "Fisica e recupero",
                Category::ExtraCurricular => "Attività extra",
                Category::Fixed => "Partita / Viaggio",
            },
            Es => match self {
                Category::TacticalUnderstanding => "Comprensión táctica",
                Category::MatchPreparation => "Preparación de partido",
                Category::FootballingDevelopment => "Desarrollo futbolístico",
                Category::PhysicalRecovery => "Físico y recuperación",
                Category::ExtraCurricular => "Extra-deportivo",
                Category::Fixed => "Partido / Viaje",
            },
            En | Nl => self.label(),
        }
    }

    pub fn display_label(self) -> &'static str {
        self.display_label_for(crate::i18n::get())
    }

    /// Kleur voor de GUI en de HTML-export (RGB).
    pub fn color(self) -> [u8; 3] {
        match self {
            Category::TacticalUnderstanding => [16, 185, 129],
            Category::MatchPreparation => [56, 189, 248],
            Category::FootballingDevelopment => [163, 230, 53],
            Category::PhysicalRecovery => [245, 158, 11],
            Category::ExtraCurricular => [244, 114, 182],
            Category::Fixed => [244, 63, 94],
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize)]
pub struct Session {
    pub name: &'static str,
    pub category: Category,
    /// Maximaal aantal keer per week (uit de game-data).
    pub max_per_week: i32,
    /// Effect op [conditie, vermoeidheid, geluk, blessurerisico, scherpte], −3…+3, zoals
    /// FM27 het in de sessie-info toont (positief = gunstig; uit de game-data).
    pub effects: [i8; 5],
}

const fn s(name: &'static str, category: Category, max_per_week: i32, effects: [i8; 5]) -> Session {
    Session { name, category, max_per_week, effects }
}

/// Alle 59 sessies van FM27, letterlijk uit `training/training_sessions` in de game-data
/// (game_plugin.bundle/…/game_simulation/training.fmf), in dezelfde volgorde.
pub const SESSIONS: &[Session] = &[
    s("Passing Style", Category::TacticalUnderstanding, 8, [-1, 0, 0, -1, 1]),
    s("Build Up", Category::TacticalUnderstanding, 8, [-1, 0, 0, -1, 1]),
    s("Chance Creation", Category::TacticalUnderstanding, 8, [-1, 0, 0, -1, 1]),
    s("In Possession Shape", Category::TacticalUnderstanding, 8, [-1, 0, 0, -1, 1]),
    s("Line of Engagement", Category::TacticalUnderstanding, 8, [-1, 0, 0, -1, 1]),
    s("Pressing Intensity", Category::TacticalUnderstanding, 8, [-1, 0, 0, -1, 1]),
    s("Pressing Trap", Category::TacticalUnderstanding, 8, [-1, 0, 0, -1, 1]),
    s("Out Of Possession Shape", Category::TacticalUnderstanding, 8, [-1, 0, 0, -1, 1]),
    s("Press Avoidance", Category::MatchPreparation, 1, [-1, 0, 0, -1, 2]),
    s("Ball Progression", Category::MatchPreparation, 1, [-1, 0, 0, -1, 2]),
    s("Final Third", Category::MatchPreparation, 1, [-1, 0, 0, -1, 2]),
    s("Defensive Block", Category::MatchPreparation, 1, [-1, 0, 0, -1, 2]),
    s("Defensive Positioning", Category::MatchPreparation, 1, [-1, 0, 0, -1, 2]),
    s("Defensive Engagement", Category::MatchPreparation, 1, [-1, 0, 0, -1, 2]),
    s("Corners", Category::MatchPreparation, 1, [-1, 0, 0, -1, 2]),
    s("Free Kicks", Category::MatchPreparation, 1, [-1, 0, 0, -1, 2]),
    s("Penalties", Category::MatchPreparation, 1, [-1, 0, 0, -1, 2]),
    s("Individual Development", Category::FootballingDevelopment, 7, [-1, -1, 1, -1, 0]),
    s("Attacking Ball Work", Category::FootballingDevelopment, 7, [-1, -1, 1, -1, 0]),
    s("Defensive Ball Work", Category::FootballingDevelopment, 7, [-1, -1, 1, -1, 1]),
    s("Possession Ball Work", Category::FootballingDevelopment, 7, [-1, -1, 1, -1, 0]),
    s("Attacking - Balanced", Category::FootballingDevelopment, 7, [-1, -1, 1, -1, 0]),
    s("Attacking - Patient", Category::FootballingDevelopment, 7, [-1, -1, 1, -1, 0]),
    s("Attacking - Direct", Category::FootballingDevelopment, 7, [-1, -1, 1, -1, 0]),
    s("Attacking - Wings", Category::FootballingDevelopment, 7, [-1, -1, 1, -1, 0]),
    s("Defending - Balanced", Category::FootballingDevelopment, 7, [-1, -1, 1, -1, 0]),
    s("Defending - Engaged", Category::FootballingDevelopment, 7, [-1, -1, 1, -1, 0]),
    s("Defending - Disengaged", Category::FootballingDevelopment, 7, [-1, -1, 1, -1, 0]),
    s("Defending - Wide", Category::FootballingDevelopment, 7, [-1, -1, 1, -1, 0]),
    s("Theory - Attacking Shadow Play", Category::FootballingDevelopment, 7, [-1, -1, 1, -1, 0]),
    s("Theory - Defensive Shadow Play", Category::FootballingDevelopment, 7, [-1, -1, 1, -1, 0]),
    s("Technical - Chance Creation", Category::FootballingDevelopment, 7, [-1, -1, 1, -1, 0]),
    s("Technical - Chance Conversion", Category::FootballingDevelopment, 7, [-1, -1, 1, -1, 0]),
    s("Technical - Ball Retention", Category::FootballingDevelopment, 7, [-1, -1, 1, -1, 0]),
    s("Technical - Ground Defence", Category::FootballingDevelopment, 7, [-1, -1, 1, -1, 0]),
    s("Technical - Aerial Defence", Category::FootballingDevelopment, 7, [-1, -1, 1, -1, 0]),
    s("Transition - Restrict", Category::FootballingDevelopment, 7, [-1, -1, 1, -1, 0]),
    s("Transition - Press", Category::FootballingDevelopment, 7, [-1, -1, 1, -1, 0]),
    s("Transition - Progress", Category::FootballingDevelopment, 7, [-1, -1, 1, -1, 0]),
    s("GK - Balanced", Category::FootballingDevelopment, 7, [-1, -1, 1, -1, 0]),
    s("GK - Shot Stopping", Category::FootballingDevelopment, 7, [-1, -1, 1, -1, 0]),
    s("GK - Handling", Category::FootballingDevelopment, 7, [-1, -1, 1, -1, 0]),
    s("GK - One On Ones", Category::FootballingDevelopment, 7, [-1, -1, 1, -1, 0]),
    s("GK - Distribution", Category::FootballingDevelopment, 7, [-1, -1, 1, -1, 0]),
    s("Set Pieces - Delivery", Category::FootballingDevelopment, 7, [-1, -1, 1, -1, 0]),
    s("Set Pieces - Penalties", Category::FootballingDevelopment, 7, [-1, -1, 1, -1, 0]),
    s("Rest", Category::PhysicalRecovery, 21, [3, 1, 2, 0, -3]),
    s("Recovery", Category::PhysicalRecovery, 7, [1, 2, 1, 1, -2]),
    s("Light Conditioning", Category::PhysicalRecovery, 7, [-1, -1, -1, -2, -1]),
    s("Conditioning", Category::PhysicalRecovery, 7, [-3, -2, -2, -2, -1]),
    s("Heavy Conditioning", Category::PhysicalRecovery, 7, [-3, -3, -3, -2, -1]),
    s("Rest (IM)", Category::PhysicalRecovery, 21, [0, 0, 0, 0, 0]),
    s("Recovery (IM)", Category::PhysicalRecovery, 7, [0, 0, 0, 0, 0]),
    s("Light Conditioning (IM)", Category::PhysicalRecovery, 7, [0, 0, 0, 0, 0]),
    s("Team Bonding", Category::ExtraCurricular, 2, [1, 0, 3, 0, -3]),
    s("Community Outreach", Category::ExtraCurricular, 2, [1, 0, 3, 0, -3]),
    s("Travel", Category::ExtraCurricular, 21, [1, 0, 0, 0, -3]),
    s("Match", Category::Fixed, 7, [0, 0, 0, 0, 0]),
    s("Not Available", Category::PhysicalRecovery, 21, [0, 0, 0, 0, 0]),
    // Algemene keuzes die de SI-presets gebruiken: FM vult zelf de specifieke sessie in
    s("Match Preparation", Category::MatchPreparation, 9, [-1, 0, 0, -1, 2]),
    s("Tactical Understanding IP", Category::TacticalUnderstanding, 8, [-1, 0, 0, -1, 1]),
    s("Tactical Understanding OOP", Category::TacticalUnderstanding, 8, [-1, 0, 0, -1, 1]),
];

impl Session {
    /// Belasting 0 (rust) … 6 (zwaar): hoe negatief het effect op conditie en vermoeidheid is.
    pub fn load(&self) -> i32 {
        (-(self.effects[0] as i32) - self.effects[1] as i32).max(0)
    }
}

pub fn session(name: &str) -> Option<&'static Session> {
    SESSIONS.iter().find(|s| s.name == name)
}

/// Interne sessiecode van FM27 (`game_plugin.dll` doğrulaması):
/// - Tactical Understanding: 0–7 (genel IP/OOP/TU: 8–10)
/// - Match Preparation: 100–108 (genel: 109)
/// - Footballing Development: 200–203 (Individual + Ball Work), 220–244 (diğer 25 seans; 204–219 boş!)
/// - Physical & Recovery: 300–304, Inter-match (IM): 320–322 (305–319 boş!)
/// - Extra-Curricular: 400–402, Match: 500, Not Available: 600
pub fn session_code(name: &str) -> Option<u16> {
    let idx = SESSIONS.iter().position(|s| s.name == name)?;
    match idx {
        0..=7 => Some(idx as u16),
        8..=16 => Some(100 + (idx - 8) as u16),
        17..=20 => Some(200 + (idx - 17) as u16),
        21..=45 => Some(220 + (idx - 21) as u16),
        46..=50 => Some(300 + (idx - 46) as u16),
        51..=53 => Some(320 + (idx - 51) as u16),
        54..=56 => Some(400 + (idx - 54) as u16),
        57 => Some(500),
        58 => Some(600),
        _ => None,
    }
}

pub fn session_by_code(code: u16) -> Option<&'static str> {
    SESSIONS.iter().map(|s| s.name).find(|n| session_code(n) == Some(code))
}

/// 62 FM27 antrenman seansının kullanıcı arayüzünde gösterilecek dil karşılığı.
/// Canonical İngilizce ad (`SESSIONS[i].name`) planlayıcı ve bellek kodları için aynen korunur.
pub fn session_display_name_for<'a>(canonical: &'a str, lang: crate::i18n::Lang) -> &'a str {
    use crate::i18n::Lang::*;
    match lang {
        Tr => match canonical {
            "Passing Style" => "Pas Stili",
            "Build Up" => "Oyun Kurma",
            "Chance Creation" => "Pozisyon Üretme",
            "In Possession Shape" => "Topa Sahipken Diziliş",
            "Line of Engagement" => "Karşılama Çizgisi",
            "Pressing Intensity" => "Pres Şiddeti",
            "Pressing Trap" => "Pres Tuzağı",
            "Out Of Possession Shape" => "Topsuz Alanda Diziliş",
            "Press Avoidance" => "Presten Çıkış",
            "Ball Progression" => "Topu İleri Taşıma",
            "Final Third" => "Üçüncü Bölge",
            "Defensive Block" => "Savunma Bloğu",
            "Defensive Positioning" => "Savunma Pozisyonu",
            "Defensive Engagement" => "Savunma Baskısı",
            "Corners" => "Kornerler",
            "Free Kicks" => "Serbest Vuruşlar",
            "Penalties" => "Penaltılar",
            "Individual Development" => "Bireysel Gelişim",
            "Attacking Ball Work" => "Hücum Topla Çalışma",
            "Defensive Ball Work" => "Savunma Topla Çalışma",
            "Possession Ball Work" => "Topa Sahip Olma",
            "Attacking - Balanced" => "Hücum - Dengeli",
            "Attacking - Patient" => "Hücum - Sabırlı",
            "Attacking - Direct" => "Hücum - Direkt",
            "Attacking - Wings" => "Hücum - Kanatlar",
            "Defending - Balanced" => "Savunma - Dengeli",
            "Defending - Engaged" => "Savunma - Baskılı",
            "Defending - Disengaged" => "Savunma - Geri Çekilen",
            "Defending - Wide" => "Savunma - Geniş",
            "Theory - Attacking Shadow Play" => "Teori - Hücum Gölge Oyunu",
            "Theory - Defensive Shadow Play" => "Teori - Savunma Gölge Oyunu",
            "Technical - Chance Creation" => "Teknik - Pozisyon Üretme",
            "Technical - Chance Conversion" => "Teknik - Gol Vuruşu",
            "Technical - Ball Retention" => "Teknik - Top Tutma",
            "Technical - Ground Defence" => "Teknik - Yerden Savunma",
            "Technical - Aerial Defence" => "Teknik - Hava Savunması",
            "Transition - Restrict" => "Geçiş - Kısıtla",
            "Transition - Press" => "Geçiş - Pres",
            "Transition - Progress" => "Geçiş - İlerle",
            "GK - Balanced" => "Kaleci - Dengeli",
            "GK - Shot Stopping" => "Kaleci - Şut Kurtarma",
            "GK - Handling" => "Kaleci - Elle Kontrol",
            "GK - One On Ones" => "Kaleci - Birebir",
            "GK - Distribution" => "Kaleci - Oyuna Sokma",
            "Set Pieces - Delivery" => "Duran Top - Orta",
            "Set Pieces - Penalties" => "Duran Top - Penaltılar",
            "Rest" => "Dinlenme",
            "Recovery" => "Rejenerasyon",
            "Light Conditioning" => "Hafif Kondisyon",
            "Conditioning" => "Kondisyon",
            "Heavy Conditioning" => "Ağır Kondisyon",
            "Rest (IM)" => "Dinlenme (Maç Arası)",
            "Recovery (IM)" => "Rejenerasyon (Maç Arası)",
            "Light Conditioning (IM)" => "Hafif Kondisyon (Maç Arası)",
            "Team Bonding" => "Takım Kaynaşması",
            "Community Outreach" => "Sosyal Yardımlaşma",
            "Travel" => "Seyahat",
            "Match" => "Maç",
            "Not Available" => "Müsait Değil",
            "Match Preparation" => "Maç Hazırlığı",
            "Tactical Understanding IP" => "Taktik Anlayış (Topla)",
            "Tactical Understanding OOP" => "Taktik Anlayış (Topsuz)",
            _ => canonical,
        },
        Fr => match canonical {
            "Passing Style" => "Style de passes",
            "Build Up" => "Construction",
            "Chance Creation" => "Création d'occasions",
            "In Possession Shape" => "Placement en possession",
            "Line of Engagement" => "Ligne d'engagement",
            "Pressing Intensity" => "Intensité du pressing",
            "Pressing Trap" => "Piège de pressing",
            "Out Of Possession Shape" => "Placement sans ballon",
            "Press Avoidance" => "Sortie de pressing",
            "Ball Progression" => "Progression du ballon",
            "Final Third" => "Dernier tiers",
            "Defensive Block" => "Bloc défensif",
            "Defensive Positioning" => "Placement défensif",
            "Defensive Engagement" => "Engagement défensif",
            "Corners" => "Corners",
            "Free Kicks" => "Coups francs",
            "Penalties" => "Penaltys",
            "Individual Development" => "Développement individuel",
            "Attacking Ball Work" => "Travail offensif avec ballon",
            "Defensive Ball Work" => "Travail défensif avec ballon",
            "Possession Ball Work" => "Conservation du ballon",
            "Attacking - Balanced" => "Attaque - Équilibrée",
            "Attacking - Patient" => "Attaque - Patiente",
            "Attacking - Direct" => "Attaque - Directe",
            "Attacking - Wings" => "Attaque - Ailes",
            "Defending - Balanced" => "Défense - Équilibrée",
            "Defending - Engaged" => "Défense - Haute",
            "Defending - Disengaged" => "Défense - Basse",
            "Defending - Wide" => "Défense - Large",
            "Theory - Attacking Shadow Play" => "Théorie - Jeu d'ombre offensif",
            "Theory - Defensive Shadow Play" => "Théorie - Jeu d'ombre défensif",
            "Technical - Chance Creation" => "Technique - Création d'occasions",
            "Technical - Chance Conversion" => "Technique - Finition",
            "Technical - Ball Retention" => "Technique - Conservation",
            "Technical - Ground Defence" => "Technique - Défense au sol",
            "Technical - Aerial Defence" => "Technique - Défense aérienne",
            "Transition - Restrict" => "Transition - Contenir",
            "Transition - Press" => "Transition - Pressing",
            "Transition - Progress" => "Transition - Projection",
            "GK - Balanced" => "Gardien - Équilibré",
            "GK - Shot Stopping" => "Gardien - Arrêts",
            "GK - Handling" => "Gardien - Prise de balle",
            "GK - One On Ones" => "Gardien - Un contre un",
            "GK - Distribution" => "Gardien - Relance",
            "Set Pieces - Delivery" => "Coups de pied arrêtés - Centres",
            "Set Pieces - Penalties" => "Coups de pied arrêtés - Penaltys",
            "Rest" => "Repos",
            "Recovery" => "Récupération",
            "Light Conditioning" => "Physique léger",
            "Conditioning" => "Physique",
            "Heavy Conditioning" => "Physique intense",
            "Rest (IM)" => "Repos (Entre-matchs)",
            "Recovery (IM)" => "Récupération (Entre-matchs)",
            "Light Conditioning (IM)" => "Physique léger (Entre-matchs)",
            "Team Bonding" => "Cohésion d'équipe",
            "Community Outreach" => "Actions communautaires",
            "Travel" => "Déplacement",
            "Match" => "Match",
            "Not Available" => "Indisponible",
            "Match Preparation" => "Préparation de match",
            "Tactical Understanding IP" => "Compréhension tactique (Avec ballon)",
            "Tactical Understanding OOP" => "Compréhension tactique (Sans ballon)",
            _ => canonical,
        },
        It => match canonical {
            "Passing Style" => "Stile di passaggio",
            "Build Up" => "Costruzione",
            "Chance Creation" => "Creazione occasioni",
            "In Possession Shape" => "Disposizione in possesso",
            "Line of Engagement" => "Linea di ingaggio",
            "Pressing Intensity" => "Intensità pressing",
            "Pressing Trap" => "Trappola di pressing",
            "Out Of Possession Shape" => "Disposizione senza palla",
            "Press Avoidance" => "Uscita dal pressing",
            "Ball Progression" => "Progressione palla",
            "Final Third" => "Ultimo terzo",
            "Defensive Block" => "Blocco difensivo",
            "Defensive Positioning" => "Posizionamento difensivo",
            "Defensive Engagement" => "Pressione difensiva",
            "Corners" => "Calci d'angolo",
            "Free Kicks" => "Calci di punizione",
            "Penalties" => "Rigori",
            "Individual Development" => "Sviluppo individuale",
            "Attacking Ball Work" => "Lavoro offensivo con palla",
            "Defensive Ball Work" => "Lavoro difensivo con palla",
            "Possession Ball Work" => "Possesso palla",
            "Attacking - Balanced" => "Attacco - Bilanciato",
            "Attacking - Patient" => "Attacco - Paziente",
            "Attacking - Direct" => "Attacco - Diretto",
            "Attacking - Wings" => "Attacco - Fasce",
            "Defending - Balanced" => "Difesa - Bilanciata",
            "Defending - Engaged" => "Difesa - Alta",
            "Defending - Disengaged" => "Difesa - Bassa",
            "Defending - Wide" => "Difesa - Ampia",
            "Theory - Attacking Shadow Play" => "Teoria - Gioco ombra offensivo",
            "Theory - Defensive Shadow Play" => "Teoria - Gioco ombra difensivo",
            "Technical - Chance Creation" => "Tecnica - Creazione occasioni",
            "Technical - Chance Conversion" => "Tecnica - Finalizzazione",
            "Technical - Ball Retention" => "Tecnica - Mantenimento palla",
            "Technical - Ground Defence" => "Tecnica - Difesa a terra",
            "Technical - Aerial Defence" => "Tecnica - Difesa aerea",
            "Transition - Restrict" => "Transizione - Contenimento",
            "Transition - Press" => "Transizione - Pressing",
            "Transition - Progress" => "Transizione - Ripartenza",
            "GK - Balanced" => "Portiere - Bilanciato",
            "GK - Shot Stopping" => "Portiere - Parate",
            "GK - Handling" => "Portiere - Presa",
            "GK - One On Ones" => "Portiere - Uno contro uno",
            "GK - Distribution" => "Portiere - Rilanci",
            "Set Pieces - Delivery" => "Palle inattive - Cross",
            "Set Pieces - Penalties" => "Palle inattive - Rigori",
            "Rest" => "Riposo",
            "Recovery" => "Recupero",
            "Light Conditioning" => "Condizionamento leggero",
            "Conditioning" => "Condizionamento",
            "Heavy Conditioning" => "Condizionamento intenso",
            "Rest (IM)" => "Riposo (Tra partite)",
            "Recovery (IM)" => "Recupero (Tra partite)",
            "Light Conditioning (IM)" => "Condizionamento leggero (Tra partite)",
            "Team Bonding" => "Coesione di squadra",
            "Community Outreach" => "Iniziative sociali",
            "Travel" => "Viaggio",
            "Match" => "Partita",
            "Not Available" => "Non disponibile",
            "Match Preparation" => "Preparazione partita",
            "Tactical Understanding IP" => "Comprensione tattica (Con palla)",
            "Tactical Understanding OOP" => "Comprensione tattica (Senza palla)",
            _ => canonical,
        },
        Es => match canonical {
            "Passing Style" => "Estilo de pases",
            "Build Up" => "Salida de balón",
            "Chance Creation" => "Creación de ocasiones",
            "In Possession Shape" => "Disposición con balón",
            "Line of Engagement" => "Línea de presión",
            "Pressing Intensity" => "Intensidad de presión",
            "Pressing Trap" => "Trampa de presión",
            "Out Of Possession Shape" => "Disposición sin balón",
            "Press Avoidance" => "Salida de presión",
            "Ball Progression" => "Progresión con balón",
            "Final Third" => "Último tercio",
            "Defensive Block" => "Bloque defensivo",
            "Defensive Positioning" => "Colocación defensiva",
            "Defensive Engagement" => "Anticipación defensiva",
            "Corners" => "Saques de esquina",
            "Free Kicks" => "Faltas",
            "Penalties" => "Penaltis",
            "Individual Development" => "Desarrollo individual",
            "Attacking Ball Work" => "Trabajo ofensivo con balón",
            "Defensive Ball Work" => "Trabajo defensivo con balón",
            "Possession Ball Work" => "Posesión de balón",
            "Attacking - Balanced" => "Ataque - Equilibrado",
            "Attacking - Patient" => "Ataque - Paciente",
            "Attacking - Direct" => "Ataque - Directo",
            "Attacking - Wings" => "Ataque - Bandas",
            "Defending - Balanced" => "Defensa - Equilibrada",
            "Defending - Engaged" => "Defensa - Adelantada",
            "Defending - Disengaged" => "Defensa - Replegada",
            "Defending - Wide" => "Defensa - Amplia",
            "Theory - Attacking Shadow Play" => "Teoría - Juego posicional ofensivo",
            "Theory - Defensive Shadow Play" => "Teoría - Juego posicional defensivo",
            "Technical - Chance Creation" => "Técnica - Creación de ocasiones",
            "Technical - Chance Conversion" => "Técnica - Definición",
            "Technical - Ball Retention" => "Técnica - Conservación del balón",
            "Technical - Ground Defence" => "Técnica - Defensa raso",
            "Technical - Aerial Defence" => "Técnica - Juego aéreo defensivo",
            "Transition - Restrict" => "Transición - Contener",
            "Transition - Press" => "Transición - Presionar",
            "Transition - Progress" => "Transición - Progresar",
            "GK - Balanced" => "Portero - Equilibrado",
            "GK - Shot Stopping" => "Portero - Paradas",
            "GK - Handling" => "Portero - Blocaje",
            "GK - One On Ones" => "Portero - Uno contra uno",
            "GK - Distribution" => "Portero - Distribución",
            "Set Pieces - Delivery" => "Balón parado - Centros",
            "Set Pieces - Penalties" => "Balón parado - Penaltis",
            "Rest" => "Descanso",
            "Recovery" => "Recuperación",
            "Light Conditioning" => "Físico ligero",
            "Conditioning" => "Físico",
            "Heavy Conditioning" => "Físico intenso",
            "Rest (IM)" => "Descanso (Entre partidos)",
            "Recovery (IM)" => "Recuperación (Entre partidos)",
            "Light Conditioning (IM)" => "Físico ligero (Entre partidos)",
            "Team Bonding" => "Cohesión de equipo",
            "Community Outreach" => "Acción comunitaria",
            "Travel" => "Viaje",
            "Match" => "Partido",
            "Not Available" => "No disponible",
            "Match Preparation" => "Preparación de partido",
            "Tactical Understanding IP" => "Comprensión táctica (Con balón)",
            "Tactical Understanding OOP" => "Comprensión táctica (Sin balón)",
            _ => canonical,
        },
        En | Nl => canonical,
    }
}

pub fn session_display_name(canonical: &str) -> &str {
    session_display_name_for(canonical, crate::i18n::get())
}

pub fn category_of(name: &str) -> Category {
    session(name).map(|s| s.category).unwrap_or(Category::Fixed)
}

/// Uitgelezen of handmatig ingevulde stand van alle factors (0–100) en onderdelen.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Effectiveness {
    pub factors: Vec<(Factor, f32)>,
    pub components: Vec<(Component, f32)>,
    pub source: String,
    /// Spel-datum waarop de Breakdown is uitgelezen (als FM met de module draaide).
    #[serde(default)]
    pub game_date: Option<chrono::NaiveDate>,
}

impl Default for Effectiveness {
    fn default() -> Self {
        Effectiveness {
            factors: Factor::ALL.iter().map(|f| (*f, 50.0)).collect(),
            components: Component::ALL.iter().map(|c| (*c, 50.0)).collect(),
            source: String::new(),
            game_date: None,
        }
    }
}

impl Effectiveness {
    pub fn get(&self, f: Factor) -> f32 {
        self.factors.iter().find(|(x, _)| *x == f).map(|(_, v)| *v).unwrap_or(50.0)
    }

    pub fn set(&mut self, f: Factor, v: f32) {
        if let Some(e) = self.factors.iter_mut().find(|(x, _)| *x == f) {
            e.1 = v.clamp(0.0, 100.0);
        } else {
            self.factors.push((f, v.clamp(0.0, 100.0)));
        }
    }

    pub fn set_component(&mut self, c: Component, v: f32) {
        if let Some(e) = self.components.iter_mut().find(|(x, _)| *x == c) {
            e.1 = v.clamp(0.0, 100.0);
        } else {
            self.components.push((c, v.clamp(0.0, 100.0)));
        }
    }

    pub fn component(&self, c: Component) -> f32 {
        if let Some((_, v)) = self.components.iter().find(|(x, _)| *x == c) {
            return *v;
        }
        let vals: Vec<f32> =
            Factor::ALL.iter().filter(|f| f.component() == c).map(|f| self.get(*f)).collect();
        vals.iter().sum::<f32>() / vals.len().max(1) as f32
    }

    /// Factors gesorteerd van zwak naar sterk.
    pub fn weakest(&self) -> Vec<(Factor, f32)> {
        let mut v: Vec<(Factor, f32)> = Factor::ALL.iter().map(|f| (*f, self.get(*f))).collect();
        v.sort_by(|a, b| a.1.total_cmp(&b.1));
        v
    }

    pub fn phase_score(&self, p: Phase) -> f32 {
        let v: Vec<f32> = Factor::TACTICAL_AREAS
            .iter()
            .filter(|f| f.phase() == Some(p))
            .map(|f| self.get(*f))
            .collect();
        v.iter().sum::<f32>() / v.len().max(1) as f32
    }

    /// Antrenmanla artırılabilen Oyuncu Benimsemesi alt faktörü: Takım Arkadaşları ve Personel İlişkileri.
    pub fn buy_in(&self) -> f32 {
        self.get(Factor::TeammatesAndStaffRelationships)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Band {
    VeryLow,
    Low,
    Fair,
    Good,
    Excellent,
}

/// Skorun ait olduğu tek kaynaklı performans bandı (<25, <45, <60, <80, ≥80).
pub fn band(score: f32) -> Band {
    match score {
        s if s < 25.0 => Band::VeryLow,
        s if s < 45.0 => Band::Low,
        s if s < 60.0 => Band::Fair,
        s if s < 80.0 => Band::Good,
        _ => Band::Excellent,
    }
}

/// Performans bandının yerelleştirilmiş etiketi.
pub fn level_name(score: f32) -> &'static str {
    match band(score) {
        Band::VeryLow => crate::t!("zeer laag", "very low"),
        Band::Low => crate::t!("laag", "low"),
        Band::Fair => crate::t!("matig", "fair"),
        Band::Good => crate::t!("goed", "good"),
        Band::Excellent => crate::t!("uitstekend", "excellent"),
    }
}

/// Performans bandı için sabit, etiketiyle tutarlı RGB renk.
pub fn level_color(score: f32) -> [u8; 3] {
    match band(score) {
        Band::VeryLow => [244, 63, 94],
        Band::Low => [240, 101, 90],
        Band::Fair => [245, 158, 11],
        Band::Good => [132, 204, 22],
        Band::Excellent => [34, 197, 94],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn localized_labels_cover_all_languages() {
        use crate::i18n::Lang;

        for lang in [Lang::Tr, Lang::Fr, Lang::It, Lang::Es] {
            for c in Component::ALL {
                let lbl = c.display_label_for(lang);
                assert!(!lbl.is_empty());
                assert_ne!(lbl, c.label(), "component {} missing {:?}", c.label(), lang);
            }
            for f in Factor::ALL {
                let lbl = f.display_label_for(lang);
                assert!(!lbl.is_empty());
                assert_ne!(lbl, f.label(), "factor {} missing {:?}", f.label(), lang);
            }
            for cat in [
                Category::TacticalUnderstanding,
                Category::MatchPreparation,
                Category::FootballingDevelopment,
                Category::PhysicalRecovery,
                Category::ExtraCurricular,
                Category::Fixed,
            ] {
                let lbl = cat.display_label_for(lang);
                assert!(!lbl.is_empty());
                assert_ne!(lbl, cat.label(), "category {} missing {:?}", cat.label(), lang);
            }
        }
        // Canonical session keys must remain unchanged for FM27 session lookups.
        for f in Factor::TACTICAL_AREAS {
            assert!(session(f.label()).is_some(), "missing session for {}", f.label());
        }
    }

    #[test]
    fn localized_session_names_cover_all_sessions() {
        use crate::i18n::Lang;

        for s in SESSIONS {
            assert_eq!(session_display_name_for(s.name, Lang::En), s.name);
            for lang in [Lang::Tr, Lang::Fr, Lang::It, Lang::Es] {
                let loc = session_display_name_for(s.name, lang);
                assert!(!loc.is_empty(), "empty {:?} name for {}", lang, s.name);
                // "Corners" ve "Match" Fransızca futbolda aynı yazılır.
                if !(lang == Lang::Fr && matches!(s.name, "Corners" | "Match")) {
                    assert_ne!(loc, s.name, "session {} missing {:?} translation", s.name, lang);
                }
            }
        }
    }

    #[test]
    fn session_codes_match_fm27_binary() {
        assert_eq!(session_code("Passing Style"), Some(0));
        assert_eq!(session_code("Out Of Possession Shape"), Some(7));
        assert_eq!(session_code("Press Avoidance"), Some(100));
        assert_eq!(session_code("Penalties"), Some(108));
        assert_eq!(session_code("Individual Development"), Some(200));
        assert_eq!(session_code("Attacking Ball Work"), Some(201));
        assert_eq!(session_code("Defensive Ball Work"), Some(202));
        assert_eq!(session_code("Possession Ball Work"), Some(203));
        assert_eq!(session_code("Attacking - Balanced"), Some(220));
        assert_eq!(session_code("Attacking - Patient"), Some(221));
        assert_eq!(session_code("Technical - Ball Retention"), Some(232));
        assert_eq!(session_code("Transition - Press"), Some(236));
        assert_eq!(session_code("Set Pieces - Penalties"), Some(244));
        assert_eq!(session_code("Rest"), Some(300));
        assert_eq!(session_code("Heavy Conditioning"), Some(304));
        assert_eq!(session_code("Rest (IM)"), Some(320));
        assert_eq!(session_code("Light Conditioning (IM)"), Some(322));
        assert_eq!(session_code("Team Bonding"), Some(400));
        assert_eq!(session_code("Travel"), Some(402));
        assert_eq!(session_code("Match"), Some(500));
        assert_eq!(session_code("Not Available"), Some(600));
        for gap in 204..=219 {
            assert_eq!(session_by_code(gap), None, "unexpected code in 204..=219 gap: {gap}");
        }
        for gap in 305..=319 {
            assert_eq!(session_by_code(gap), None, "unexpected code in 305..=319 gap: {gap}");
        }
    }

    #[test]
    fn band_thresholds_and_colors_are_consistent() {
        assert_eq!(band(0.0), Band::VeryLow);
        assert_eq!(band(24.9), Band::VeryLow);
        assert_eq!(band(25.0), Band::Low);
        assert_eq!(band(44.9), Band::Low);
        assert_eq!(band(45.0), Band::Fair);
        assert_eq!(band(59.9), Band::Fair);
        assert_eq!(band(60.0), Band::Good);
        assert_eq!(band(79.9), Band::Good);
        assert_eq!(band(80.0), Band::Excellent);
        assert_eq!(band(100.0), Band::Excellent);

        assert_eq!(level_color(60.0), level_color(79.9));
        assert_eq!(level_color(45.0), level_color(59.9));
        assert_ne!(level_color(59.9), level_color(60.0));
        assert_ne!(level_color(79.9), level_color(80.0));
    }
}
