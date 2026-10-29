//! The four campaign endings (docs, section 10), derived from the final choice and the Rôle.

use serde::{Deserialize, Serialize};

use super::save::Save;
use crate::i18n::t;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Ending {
    TraitUnique,
    FeuilleBrulee,
    LegendeCommune,
    Oubrenne,
}

impl Ending {
    pub fn label(self) -> String {
        match self {
            Self::TraitUnique => t("ending.trait_unique").to_string(),
            Self::FeuilleBrulee => t("ending.feuille_brulee").to_string(),
            Self::LegendeCommune => t("ending.legende_commune").to_string(),
            Self::Oubrenne => t("ending.oubrenne").to_string(),
        }
    }

    pub fn postgame(self) -> String {
        match self {
            Self::TraitUnique => t("ending.trait_unique.postgame").to_string(),
            Self::FeuilleBrulee => t("ending.feuille_brulee.postgame").to_string(),
            Self::LegendeCommune => t("ending.legende_commune.postgame").to_string(),
            Self::Oubrenne => t("ending.oubrenne.postgame").to_string(),
        }
    }
}

pub fn determine(save: &Save) -> Option<Ending> {
    if !save.campaign_won() {
        return None;
    }
    if save.heroic && save.names() >= 6 {
        return Some(Ending::Oubrenne);
    }
    match save.choices.get("ending").map(String::as_str) {
        Some("trait_unique") => Some(Ending::TraitUnique),
        Some("feuille_brulee") => Some(Ending::FeuilleBrulee),
        Some("legende_commune") => Some(Ending::LegendeCommune),
        _ if save.names() >= 4
            && (save.choices.get("deserteurs").map(String::as_str) == Some("escorter")
                || save.choices.get("noyer").map(String::as_str) == Some("epargner")) =>
        {
            Some(Ending::LegendeCommune)
        }
        _ => Some(Ending::TraitUnique),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn endings_follow_the_final_choice_and_secret_condition() {
        let mut save = Save { reached: 24, ..Save::default() };
        save.record_mut(24).won = true;
        save.choices.insert("ending".into(), "feuille_brulee".into());
        assert_eq!(determine(&save), Some(Ending::FeuilleBrulee));
        save.heroic = true;
        for index in 0..6 {
            save.record_mut(index).names = 1;
        }
        assert_eq!(determine(&save), Some(Ending::Oubrenne));
    }
}
