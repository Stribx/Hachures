//! What a level leaves in the campaign: Nève's margin notes, the reading challenge, the choice.

use std::collections::BTreeMap;

use serde::Deserialize;

use super::Level;
use crate::campaign::save::Save;
use crate::i18n::{fill, t};
use crate::ink::TOOLS;

/// When a margin note is written.
#[derive(Clone, PartialEq, Debug, Deserialize)]
pub enum Trigger {
    Start,
    Wave(u32),
    /// The world event starts (La Crue, Le Gel…).
    Event,
    /// A boss phase begins, by name.
    Boss(String),
    FuyardHurt,
    FuyardSaved,
    Won,
}

/// A line of Nève's notebook, written once in the margin.
#[derive(Clone, Deserialize)]
pub struct Note {
    on: Trigger,
    /// Only if a choice went this way: "id=option".
    #[serde(default)]
    when: Option<String>,
    text: String,
    /// Names the Rôle gains or loses with it.
    #[serde(default)]
    names: i32,
}

#[derive(Deserialize)]
enum Rule {
    /// Win without inking this tool.
    Without(String),
    NoFuyardHurt,
    /// Win with at least this much Rôle left.
    KeepRole(u32),
    /// Win without scratching out a single name.
    KeepNames,
}

/// The reading challenge: the level's second sceau.
#[derive(Deserialize)]
pub struct Challenge {
    pub text: String,
    rule: Rule,
}

/// A choice offered once the level is won.
#[derive(Deserialize)]
pub struct Choice {
    pub id: String,
    pub prompt: String,
    /// (option, label)
    pub options: Vec<(String, String)>,
}

/// A line in Nève's margin: its key, and the key of the place it speaks of if any (its `{}`). It is translated when
/// shown, so it reads in the language of the moment.
pub struct MarginNote {
    pub line: String,
    pub name: Option<String>,
}

impl MarginNote {
    pub fn shown(&self) -> String {
        match &self.name {
            Some(name) => fill(t(&self.line), &[&t(name)]),
            None => t(&self.line).to_string(),
        }
    }
}

impl Note {
    /// The line, in French (for the translation tests).
    #[cfg(test)]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Written on `trigger`, if its choice went its way.
    fn due(&self, trigger: &Trigger, choices: &BTreeMap<String, String>) -> bool {
        self.on == *trigger && self.when.as_deref().is_none_or(|c| holds(choices, c))
    }
}

/// How a level closes in the carnet: its `Won` lines, as `choices` let them through.
pub fn closing_lines(notes: &[Note], choices: &BTreeMap<String, String>) -> Vec<String> {
    notes.iter().filter(|n| n.due(&Trigger::Won, choices)).map(|n| n.text.clone()).collect()
}

/// Whether the choice "id=option" in `cond` went that way.
pub fn holds(choices: &BTreeMap<String, String>, cond: &str) -> bool {
    let (id, option) = cond.split_once('=').unwrap_or_else(|| panic!("condition {cond:?} is not id=option"));
    choices.get(id).is_some_and(|picked| picked == option)
}

impl Level {
    /// Writes in the margin the notes `trigger` calls for.
    pub fn note(&mut self, trigger: Trigger) {
        let (due, rest) = std::mem::take(&mut self.notes).into_iter().partition(|n| n.due(&trigger, &self.choices));
        self.notes = rest;
        for note in due {
            self.names += note.names;
            self.margin.push(MarginNote { line: note.text, name: None });
        }
    }

    pub fn challenge_met(&self) -> bool {
        self.challenge.as_ref().is_some_and(|c| match &c.rule {
            Rule::Without(tool) => !self.used.iter().any(|&t| TOOLS[t].name == tool),
            Rule::NoFuyardHurt => !self.fuyard_hurt,
            Rule::KeepRole(role) => self.lives >= *role,
            Rule::KeepNames => self.erased.is_empty(),
        })
    }

    /// The level offers no choice, or it has been made.
    pub fn chosen(&self) -> bool {
        self.choice.as_ref().is_none_or(|c| self.choices.contains_key(&c.id))
    }

    /// Picks option `k` of the level's choice, for this level and the campaign; the next level opens with it.
    pub fn choose(&mut self, k: usize, save: &mut Save) {
        if self.choice_locked {
            return;
        }
        let Some((id, option)) = self.choice.as_ref().and_then(|c| Some((c.id.clone(), c.options.get(k)?.0.clone())))
        else {
            return;
        };
        self.choices.insert(id.clone(), option.clone());
        save.choices.insert(id, option);
        self.unlock_next(save);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::level::data::LEVELS;
    use crate::map::species::Goal;

    /// Index of the level offering the Refuge choice (1-3).
    fn refuge_choice() -> usize {
        (0..LEVELS.len()).find(|&i| Level::load(i, &Save::default()).choice.is_some_and(|c| c.id == "refuge")).unwrap()
    }

    /// Winning a level with a choice opens the next one only once the choice is made,
    /// so quitting in between can't skip it.
    #[test]
    fn choice_gates_the_next_level() {
        let choice_level = refuge_choice();
        let mut save = Save { reached: choice_level, ..Default::default() };
        let mut level = Level::load(choice_level, &save);
        level.finish(&mut save);
        assert_eq!(save.reached, choice_level, "won, but the choice is still open");
        level.choose(1, &mut save);
        assert_eq!(save.reached, choice_level + 1);
        assert_eq!(save.choices["refuge"], level.choices["refuge"]);

        // Replayed later, the level keeps the choice made then.
        let mut replay = Level::load(choice_level, &save);
        let made = save.choices["refuge"].clone();
        replay.choose(0, &mut save);
        replay.choose(1, &mut save);
        assert_eq!(save.choices["refuge"], made, "a replay can't change the choice");
    }

    /// The carnet closes 1-4 on what the Refuge choice left: Oudine speaks, or she is silent.
    #[test]
    fn closing_lines_follow_the_choice() {
        let boss: crate::level::data::LevelData = ron::from_str(LEVELS[refuge_choice() + 1]).unwrap();
        let lines = |option: &str| closing_lines(&boss.notes, &[("refuge".into(), option.into())].into());
        let (moulin, bois) = (lines("moulin"), lines("bois"));
        assert_ne!(moulin, bois);
        let said = |lines: &[String], words: &str| lines.iter().any(|l| crate::i18n::t(l).contains(words));
        assert!(said(&moulin, "parlera") && said(&bois, "se tait"));
    }

    /// The Refuge choice moves the Refuge, and "Dans le Bois" costs a Name when Le Bornage begins.
    #[test]
    fn refuge_choice_has_consequences() {
        let boss_level = refuge_choice() + 1;
        let moulin = Level::load(boss_level, &Save::default());
        let mut save = Save::default();
        save.choices.insert("refuge".into(), "bois".into());
        let mut bois = Level::load(boss_level, &save);
        assert_ne!(moulin.base.goals[&Goal::Refuge], bois.base.goals[&Goal::Refuge]);

        bois.note(Trigger::Boss("boss.grande_chaine.bornage".into()));
        assert_eq!(bois.names, -1);
        assert!(bois.margin.iter().any(|note| note.shown().contains("meule")));
    }
}
