//! The campaign save: levels reached, sceaux, skills, choices and Names.
//! One RON file (`storage`) in the user's data folder (localStorage in the browser), rewritten
//! whole on every change.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

use super::endings::{self, Ending};
use super::skills::Skill;
use crate::i18n::french_key;
use crate::ink::TOOLS;
use crate::level::Level;
use crate::level::data::LEVELS;
use crate::storage;

/// What a level left in the campaign.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Record {
    /// Sceau of the victory.
    pub won: bool,
    /// Sceau of the reading challenge.
    pub challenge: bool,
    /// Names kept in the Rôle by the first win: a replay can't rewrite them.
    pub names: i32,
    /// The places whose names the first win scratched out, by key (`place.pont_du_bief`).
    #[serde(default)]
    pub erased: Vec<String>,
}

#[derive(Resource, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Save {
    /// 0 for a save written before the prologue (its levels count from 1-1); see `upgrade`.
    /// A missing field reads as 0, a new save is `VERSION`.
    #[serde(default)]
    pub version: u32,
    /// Furthest level unlocked (index in `LEVELS`).
    pub reached: usize,
    /// One record per level, by index.
    pub levels: Vec<Record>,
    pub skills: Vec<Skill>,
    /// Choice id -> option picked, e.g. "refuge" -> "moulin".
    pub choices: BTreeMap<String, String>,
    /// What Nève has met on a sheet, for the Lexique: the label keys of tools and species
    /// (`tool.arbalete`).
    pub seen: BTreeSet<String>,
    /// Heroic mode was used for the final victory.
    pub heroic: bool,
}

/// The save's layout: 1 counts the prologue as level 0; 2 keeps the names scratched out by key, not
/// in French; 3 remembers what was seen, for the Lexique.
const VERSION: u32 = 4;

impl Default for Save {
    fn default() -> Save {
        Save {
            version: VERSION,
            reached: 0,
            levels: Vec::new(),
            skills: Vec::new(),
            choices: BTreeMap::new(),
            seen: BTreeSet::new(),
            heroic: false,
        }
    }
}

impl Save {
    /// Its name in `storage`: `save.ron`, or `hachures-save` in the browser.
    pub const FILE: &str = "save";

    /// A save read from before the prologue moves up one level, the prologue counted as won.
    /// One holding only the settings starts with the prologue.
    /// One from before the language files has its scratched-out names in French: they become their
    /// keys. One from before the Lexique has met what its won levels showed.
    #[must_use]
    pub fn upgrade(mut self) -> Save {
        if self.version == 0 && (self.reached > 0 || !self.levels.is_empty()) {
            self.reached += 1;
            self.levels.insert(0, Record { won: true, ..default() });
        }
        if self.version < 2 {
            for name in self.levels.iter_mut().flat_map(|r| &mut r.erased) {
                if let Some(key) = french_key("place.", name) {
                    *name = key.to_string();
                }
            }
        }
        if self.version < 3 {
            let won = (0..self.levels.len().min(LEVELS.len())).filter(|&k| self.levels[k].won);
            for k in won {
                let level = Level::load(k, &self);
                let tools = level.tools.iter().map(|&t| TOOLS[t].label);
                let species = level.present.iter().map(|&s| level.species[s].label.as_str());
                let met: Vec<String> = tools.chain(species).map(String::from).collect();
                self.seen.extend(met);
            }
        }
        self.version = VERSION;
        self
    }

    /// Notes that `id` (a label key) has been met; true the first time.
    pub fn discover(&mut self, id: &str) -> bool {
        self.seen.insert(id.to_string())
    }

    /// Nothing played yet: the Lexique may already hold the first sheet's tools.
    pub fn is_new(&self) -> bool {
        self.reached == 0
            && self.levels.is_empty()
            && self.skills.is_empty()
            && self.choices.is_empty()
    }

    pub fn sceaux(&self) -> u32 {
        let earned: usize = self.levels.iter().map(|r| r.won as usize + r.challenge as usize).sum();
        // Saturating: a hand-edited save may have spent more than it earned.
        (earned as u32).saturating_sub(self.skills.iter().map(|s| s.cost()).sum())
    }

    /// The Rôle des Noms over the whole campaign.
    pub fn names(&self) -> i32 {
        self.levels.iter().map(|r| r.names).sum()
    }

    pub fn campaign_won(&self) -> bool {
        self.levels.get(LEVELS.len() - 1).is_some_and(|r| r.won)
    }

    pub fn ending(&self) -> Option<Ending> {
        endings::determine(self)
    }

    pub fn record_mut(&mut self, level: usize) -> &mut Record {
        if self.levels.len() <= level {
            self.levels.resize(level + 1, Record::default());
        }
        &mut self.levels[level]
    }

    pub fn load() -> Save {
        storage::load(Save::FILE).map(Save::upgrade).unwrap_or_default()
    }

    pub fn store(&self) {
        storage::store(Save::FILE, self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_and_unreadable_save_kept() {
        let dir = std::env::temp_dir().join(format!("hachures-test-{}", std::process::id()));
        let path = dir.join("save.ron");
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(storage::read::<Save>(&path), None, "no save yet");

        let mut save = Save { reached: 2, skills: vec![Skill::MineDePlomb], ..default() };
        *save.record_mut(1) = Record {
            won: true,
            challenge: true,
            names: 3,
            erased: vec!["place.pre_commun".into()],
        };
        save.choices.insert("refuge".into(), "bois".into());
        save.seen.insert("tool.arbalete".into());
        storage::write(&path, &save).unwrap();
        assert_eq!(storage::read(&path).as_ref(), Some(&save));
        assert_eq!(save.sceaux(), 1, "2 earned, 1 spent");

        let old: Save = ron::from_str(
            "(reached: 2, levels: [(won: true, challenge: false, names: 0), (won: true, challenge: true, names: 0)])",
        )
        .unwrap();
        let old = old.upgrade();
        assert_eq!(
            (old.reached, old.levels.len(), old.sceaux()),
            (3, 3, 4),
            "before the prologue: one level up"
        );
        let settings_only: Save = ron::from_str("(settings: (music: 40))").unwrap();
        assert_eq!(settings_only.upgrade().reached, 0, "not started: the prologue comes first");
        assert_eq!(Save::default().upgrade(), Save::default(), "a new save is never moved");
        let french: Save = ron::from_str(
            "(version: 1, reached: 2, levels: [(won: true, challenge: false, names: 0, erased: [\"le Pont du Bief\"])])",
        )
        .unwrap();
        assert_eq!(
            french.upgrade().levels[0].erased,
            ["place.pont_du_bief"],
            "the French name becomes its key"
        );

        std::fs::write(&path, "not a save").unwrap();
        assert_eq!(storage::read::<Save>(&path), None);
        assert_eq!(
            std::fs::read_to_string(dir.join("save.ron.bak")).unwrap(),
            "not a save",
            "kept aside, not lost"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn discover_is_true_once() {
        let mut save = Save::default();
        assert!(save.discover("species.chaineur"));
        assert!(!save.discover("species.chaineur"));
    }

    #[test]
    fn meeting_tools_is_not_playing() {
        let mut save = Save::default();
        save.discover("tool.barricade");
        assert!(save.is_new(), "the title still offers a new game");
        save.record_mut(0).won = true;
        assert!(!save.is_new());
    }

    #[test]
    fn a_save_before_the_lexique_remembers_its_won_levels() {
        let old: Save =
            ron::from_str("(version: 2, reached: 2, levels: [(won: true, challenge: false, names: 0), (won: true, challenge: false, names: 0), (won: false, challenge: false, names: 0)])")
                .unwrap();
        let old = old.upgrade();
        assert_eq!(old.version, 4);
        for id in ["tool.arbalete", "tool.barricade", "species.chaineur", "species.echassier"] {
            assert!(old.seen.contains(id), "{id} was on level 1-1");
        }
        assert!(!old.seen.contains("tool.mortier"), "level 1-2 was never won");
    }
}
