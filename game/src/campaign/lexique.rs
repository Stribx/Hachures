//! The Lexique's tabs, entries and the lines every reading aid shows.

use std::collections::HashMap;

use crate::combat::towers::{TowerStats, Weapon};
use crate::i18n::{n_, number, t, tf};
use crate::ink::{Act, TOOLS};
use crate::map::species::{Goal, Reading, Species};

/// A page of the Lexique.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Tab {
    #[default]
    Outils,
    Especes,
}

impl Tab {
    /// The tabs in order. A later tab is one more variant here and one more arm in `name`.
    pub const ALL: [Tab; 2] = [Tab::Outils, Tab::Especes];

    /// The key of its name.
    pub const fn name(self) -> &'static str {
        match self {
            Tab::Outils => n_("lexique.tab.tools"),
            Tab::Especes => n_("lexique.tab.species"),
        }
    }
}

/// What an entry describes, by index in `TOOLS` or in the species list.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EntryKind {
    Tool(usize),
    Species(usize),
}

/// A line of a tab. Its `id` is the label key, which is also what `Save.seen` holds.
pub struct Entry {
    pub id: String,
    pub kind: EntryKind,
}

impl Entry {
    /// The key of its description.
    pub fn text(&self) -> String {
        format!("{}.text", self.id)
    }
}

/// The entries of `tab`: the tools in bar order, the species in file order.
pub fn entries(tab: Tab, species: &[Species]) -> Vec<Entry> {
    match tab {
        Tab::Outils => TOOLS
            .iter()
            .enumerate()
            .map(|(t, tool)| Entry { id: tool.label.to_string(), kind: EntryKind::Tool(t) })
            .collect(),
        Tab::Especes => species
            .iter()
            .enumerate()
            .map(|(s, sp)| Entry { id: sp.label.clone(), kind: EntryKind::Species(s) })
            .collect(),
    }
}

/// A line to show: its key and the values for its slots, translated when shown.
#[derive(Debug, PartialEq)]
pub struct Line {
    pub key: &'static str,
    pub args: Vec<String>,
}

impl Line {
    fn new(key: &'static str, args: &[String]) -> Line {
        Line { key, args: args.to_vec() }
    }

    pub fn shown(&self) -> String {
        let args: Vec<&dyn std::fmt::Display> = self.args.iter().map(|a| a as _).collect();
        tf(self.key, &args)
    }
}

/// The hostile species `weapon` hits twice as hard.
pub fn strong_against(weapon: Weapon, species: &[Species]) -> Vec<&Species> {
    species.iter().filter(|s| s.hostile && s.weak_to == Some(weapon)).collect()
}

/// The lines of tool `t` at `cost` (skills counted). A `Lay` tool lists the step price of `species`, the ones to show.
pub fn tool_lines(
    tool: usize,
    cost: u32,
    stats: &HashMap<Weapon, TowerStats>,
    species: &[Species],
    equerre: bool,
) -> Vec<Line> {
    // Biffer costs nothing: what it gives back is its line (docs, section 7).
    if TOOLS[tool].act == Act::Biffer {
        return vec![
            Line::new(n_("lexique.refund_wet"), &[]),
            Line::new(n_("lexique.refund_dry"), &[]),
        ];
    }
    let mut lines = vec![Line::new(n_("lexique.cost"), &[cost.to_string()])];
    match TOOLS[tool].act {
        Act::Raise(weapon) => lines.extend(weapon_lines(weapon, &stats[&weapon], species, equerre)),
        // A tile no one can cross reads as one line, not a list of the same word.
        Act::Lay(_) if species.is_empty() => {}
        Act::Lay(tile) if species.iter().all(|s| !s.cost(tile).is_finite()) => {
            lines.push(Line::new(n_("lexique.impassable_all"), &[]));
        }
        Act::Lay(tile) => {
            let prices: Vec<String> = species
                .iter()
                .map(|s| {
                    let price = s.cost(tile);
                    let price = if price.is_finite() {
                        number(price)
                    } else {
                        t("lexique.impassable").to_string()
                    };
                    tf("lexique.step_one", &[&t(&s.label), &price])
                })
                .collect();
            lines.push(Line::new(n_("lexique.step"), &[prices.join(", ")]));
        }
        Act::Scrape | Act::Biffer => {}
    }
    lines
}

fn weapon_lines(
    weapon: Weapon,
    stats: &TowerStats,
    species: &[Species],
    equerre: bool,
) -> Vec<Line> {
    let range = if equerre { n_("lexique.range_equerre") } else { n_("lexique.range") };
    let mut lines = vec![
        Line::new(n_("lexique.damage"), &[number(stats.damage)]),
        Line::new(range, &[number(stats.range)]),
        Line::new(n_("lexique.cooldown"), &[number(stats.cooldown)]),
    ];
    for (key, value) in [
        (n_("lexique.splash"), stats.splash),
        (n_("lexique.pin"), stats.pin),
        (n_("lexique.slow"), stats.slow),
    ] {
        if value > 0.0 {
            lines.push(Line::new(key, &[number(value)]));
        }
    }
    let strong: Vec<&str> = strong_against(weapon, species).iter().map(|s| t(&s.label)).collect();
    lines.push(if strong.is_empty() {
        Line::new(n_("lexique.strong_none"), &[])
    } else {
        Line::new(n_("lexique.strong"), &[strong.join(", ")])
    });
    lines.push(Line::new(n_("lexique.aims"), &[]));
    lines
}

/// The lines of species `s`.
pub fn species_lines(s: &Species) -> Vec<Line> {
    let goal = match s.goal {
        Goal::Cible => n_("lexique.goal.cible"),
        Goal::BorneMere => n_("lexique.goal.borne_mere"),
        Goal::Refuge => n_("lexique.goal.refuge"),
        Goal::Sortie => n_("lexique.goal.sortie"),
    };
    let reading = match s.reading {
        Reading::Pente => n_("lexique.reading.pente"),
        Reading::Ombre => n_("lexique.reading.ombre"),
    };
    let mut lines = vec![
        Line::new(if s.hostile { n_("lexique.hostile") } else { n_("lexique.ally") }, &[]),
        Line::new(goal, &[]),
        Line::new(reading, &[]),
        Line::new(n_("lexique.hp"), &[number(s.hp)]),
        Line::new(n_("lexique.speed"), &[number(s.speed)]),
    ];
    if s.hostile {
        lines.push(Line::new(n_("lexique.loot"), &[s.loot.to_string()]));
    }
    lines.push(match s.weak_to.and_then(|w| TOOLS.iter().find(|tool| tool.act == Act::Raise(w))) {
        Some(tool) => Line::new(n_("lexique.weak"), &[t(tool.label).to_string()]),
        None => Line::new(n_("lexique.weak_none"), &[]),
    });
    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combat::towers::load_stats;
    use crate::ink::tool_index;
    use crate::map::grid::Tile;
    use crate::map::species::load;

    fn arbalete() -> usize {
        tool_index("arbalète").unwrap()
    }

    #[test]
    fn the_arbalete_is_strong_against_the_chaineur() {
        let species = load();
        let names: Vec<&str> =
            strong_against(Weapon::Arbalete, &species).iter().map(|s| s.name.as_str()).collect();
        assert_eq!(names, ["chaineur"]);
    }

    #[test]
    fn the_arbalete_shows_its_numbers() {
        let (species, stats) = (load(), load_stats());
        let lines = tool_lines(arbalete(), 45, &stats, &species, false);
        let find = |key| lines.iter().find(|l| l.key == key).unwrap_or_else(|| panic!("no {key}"));
        assert_eq!(find("lexique.cost").args, ["45"]);
        assert_eq!(find("lexique.damage").args, ["30"]);
        assert_eq!(find("lexique.range").args, ["3"]);
        assert_eq!(find("lexique.cooldown").args, ["0,8"]);
        assert!(
            lines.iter().all(|l| l.key != "lexique.splash" && l.key != "lexique.range_equerre"),
            "{:?}",
            lines.iter().map(|l| l.key).collect::<Vec<_>>()
        );
        let equerre = tool_lines(arbalete(), 45, &stats, &species, true);
        assert!(
            equerre.iter().any(|l| l.key == "lexique.range_equerre" && l.args == ["3"]),
            "{:?}",
            equerre.iter().map(|l| (l.key, &l.args)).collect::<Vec<_>>()
        );
        assert!(equerre.iter().all(|l| l.key != "lexique.range"));
    }

    #[test]
    fn no_species_met_says_nothing_of_the_path() {
        let stats = load_stats();
        let boue = tool_index("boue").unwrap();
        let keys: Vec<_> = tool_lines(boue, 15, &stats, &[], false).iter().map(|l| l.key).collect();
        assert_eq!(keys, ["lexique.cost"], "mud is not impassable to all");
    }

    #[test]
    fn a_barricade_lists_what_cannot_cross_it() {
        let (species, stats) = (load(), load_stats());
        let barricade = tool_index("barricade").unwrap();
        let line = &tool_lines(barricade, 10, &stats, &species, false)[1];
        assert_eq!(line.key, "lexique.impassable_all");
        let boue = tool_index("boue").unwrap();
        let line = &tool_lines(boue, 15, &stats, &species, false)[1];
        let blocked = line.args[0].matches("infranchissable").count();
        assert_eq!(blocked, species.iter().filter(|s| s.cost(Tile::Boue).is_infinite()).count());
    }

    #[test]
    fn every_tool_and_species_has_one_entry() {
        let species = load();
        let tools = entries(Tab::Outils, &species);
        assert_eq!(tools.len(), TOOLS.len());
        assert!(
            (0..TOOLS.len())
                .all(|t| tools.iter().filter(|e| e.kind == EntryKind::Tool(t)).count() == 1),
            "{:?}",
            tools.iter().map(|e| e.kind).collect::<Vec<_>>()
        );
        let all = entries(Tab::Especes, &species);
        assert_eq!(all.len(), species.len());
        assert!(
            (0..species.len()).all(|s| all
                .iter()
                .filter(|e| e.kind == EntryKind::Species(s))
                .count()
                == 1),
            "{:?}",
            all.iter().map(|e| e.kind).collect::<Vec<_>>()
        );
        assert_eq!(all[0].text(), format!("{}.text", species[0].label));
    }

    #[test]
    fn every_species_has_lines() {
        for s in load() {
            let lines = species_lines(&s);
            assert_eq!(lines.iter().any(|l| l.key == "lexique.loot"), s.hostile);
            assert!(lines.iter().all(|l| !l.shown().is_empty()));
        }
    }
}
