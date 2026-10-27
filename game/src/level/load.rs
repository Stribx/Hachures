//! Making a `Level`: bare on a grid, or from a level's data file shaped by the save.

use std::collections::BTreeMap;

use super::Level;
use super::data::{LEVELS, LevelData, START_LIVES};
use super::story::{Trigger, holds};
use super::waves::{Wave, spawn_order};
use super::world::WORLDS;
use crate::campaign::save::Save;
use crate::campaign::skills::Skill;
use crate::combat::towers;
use crate::ink::{TOOLS, tool_index};
use crate::map::flow;
use crate::map::grid::{Grid, Tile};
use crate::map::species::{self, Species};

impl Level {
    /// A bare level of world 1 on `base`: every tool, no waves, no event, no skills.
    pub fn new(base: Grid, species: Vec<Species>) -> Level {
        let fields = flow::fields(&base, &species);
        let present = (0..species.len()).collect();
        Level {
            index: 0,
            world: 0,
            name: String::new(),
            grid: base.clone(),
            base,
            events: Vec::new(),
            event_on: false,
            event_next: false,
            redraw: true,
            species,
            fields,
            present,
            waves: Vec::new(),
            changes: Vec::new(),
            places: Vec::new(),
            erased: Vec::new(),
            wave: Wave::default(),
            tower_stats: towers::load_stats(),
            towers: Vec::new(),
            boss: None,
            tools: (0..TOOLS.len()).collect(),
            used: Vec::new(),
            sepia: 120,
            loot_carry: 0.0,
            lives: START_LIVES,
            names: 0,
            fuyard_hurt: false,
            walkers: Vec::new(),
            notes: Vec::new(),
            margin: Vec::new(),
            challenge: None,
            choice: None,
            choice_locked: false,
            choices: BTreeMap::new(),
            skills: Vec::new(),
            finished: false,
            tension: 0,
            lives_at_wave_start: START_LIVES,
        }
    }

    /// Level `index` of the campaign, from its data file, shaped by the save's choices and skills.
    pub fn load(index: usize, save: &Save) -> Level {
        let data = LevelData::of(index);
        let mut species = species::load();
        if save.skills.contains(&Skill::BoueProfonde) {
            for s in species.iter_mut().filter(|s| s.hostile) {
                if let Some(cost) = s.costs.get_mut(&Tile::Boue) {
                    *cost += 1.0;
                }
            }
        }
        let find = |name: &str| species::index_of(&species, name).unwrap_or_else(|| panic!("unknown species {name:?}"));
        let mut waves: Vec<Vec<usize>> = data
            .waves
            .iter()
            .map(|groups| spawn_order(groups.iter().map(|(name, count)| (find(name), *count)).collect()))
            .collect();
        if let Some((cond, by)) = &data.shorter
            && holds(&save.choices, cond)
        {
            waves.truncate(waves.len() - by);
        }
        let mut present: Vec<usize> = waves.iter().flatten().copied().collect();
        // A boss changes species with its phases.
        if let Some(boss) = &WORLDS[data.world - 1].boss
            && present.contains(&find(&boss.phases[0].species))
        {
            present.extend(boss.phases.iter().map(|p| find(&p.species)));
        }
        present.sort_unstable();
        present.dedup();
        // The level's tools, then the powers earned by the levels already won, in campaign order.
        let won = (0..LEVELS.len()).filter(|&k| save.levels.get(k).is_some_and(|r| r.won));
        let mut tools: Vec<usize> = Vec::new();
        for name in data.tools.iter().cloned().chain(won.flat_map(|k| LevelData::of(k).grants)) {
            let t = tool_index(&name).unwrap_or_else(|| panic!("unknown tool {name:?}"));
            if !tools.contains(&t) {
                tools.push(t);
            }
        }
        let alt = data.alt_refuge.as_ref().is_some_and(|cond| holds(&save.choices, cond));
        let map = data.map_text(alt);

        let mut level = Level::new(Grid::parse(&map), species);
        level.index = index;
        level.world = data.world - 1;
        level.name = data.name;
        level.sepia = data.sepia;
        level.tools = tools;
        level.events = data.events;
        level.event_next = level.event_at(1);
        level.waves = waves;
        level.present = present;
        level.changes = data.changes;
        level.places = data.places;
        level.notes = data.notes;
        level.challenge = data.challenge;
        level.choice_locked = data.choice.as_ref().is_some_and(|c| save.choices.contains_key(&c.id));
        level.choice = data.choice;
        level.choices = save.choices.clone();
        level.skills = save.skills.clone();
        level.note(Trigger::Start);
        level
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::campaign::save::Record;

    /// Winning the world-1 boss earns Gratter for good: it joins the tools of a later world's levels.
    #[test]
    fn winning_the_boss_earns_gratter() {
        let gratter = tool_index("gratter").unwrap();
        let boss = (0..LEVELS.len()).find(|&k| LevelData::of(k).name == "level.1-4.name").unwrap();
        let later = (0..LEVELS.len()).find(|&k| LevelData::of(k).world == 2).unwrap();
        assert!(!Level::load(later, &Save::default()).tools.contains(&gratter), "not before");
        let mut save = Save::default();
        *save.record(boss) = Record { won: true, ..Default::default() };
        assert!(Level::load(later, &save).tools.contains(&gratter), "once the boss is beaten");
    }
}
