//! A level: its map as inked, its world's event, waves and sépia, and what it leaves in the campaign.
//! `Level` is one resource; each feature adds its own `impl Level` (inking in `ink`, tower reach in `combat`).

pub mod data;
pub mod story;
pub mod waves;
pub mod world;

use std::collections::{BTreeMap, HashMap};

use bevy::prelude::*;

use crate::Step;
use crate::campaign::save::Save;
use crate::campaign::skills::Skill;
use crate::combat::boss::boss_phases;
use crate::combat::towers::{self, TowerStats, Weapon};
use crate::ink::{TOOLS, tool_index};
use crate::map::flow::{self, FlowField};
use crate::map::grid::{Grid, Tile};
use crate::map::species::{self, Species};
use data::{Change, LEVELS, LevelData, START_LIVES};
use story::{Challenge, Choice, Note, Trigger, holds};
use waves::{Wave, spawn_order};
use world::{WORLDS, World};

/// Chaîneur HP at world 1, wave 1 (docs, section 8).
const BASE_HP: f32 = 100.0;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, finish_level.in_set(Step::Simulate).after(boss_phases));
}

/// Tags what belongs to the level being played, to clear it when another one loads.
#[derive(Component)]
pub struct LevelEntity;

#[derive(Resource)]
pub struct Level {
    /// Position in `LEVELS`.
    pub index: usize,
    /// Position in `WORLDS`.
    pub world: usize,
    pub name: String,
    /// The map as Nève inked it.
    pub base: Grid,
    /// What the species walk on now: `base`, or `base` under the world event while it lasts.
    pub grid: Grid,
    /// Waves the world event starts at; it lasts `event.lasts` waves.
    pub events: Vec<u32>,
    /// The world event is under way.
    pub event_on: bool,
    /// The next wave brings the event: its map is checked too before inking.
    pub event_next: bool,
    /// Terrain blocks must be rebuilt from `grid`.
    pub redraw: bool,
    pub species: Vec<Species>,
    pub fields: Vec<FlowField>,
    /// The species this level sends (the boss's phases included), sorted: le Droit de Passage and Tab only
    /// consider these.
    pub present: Vec<usize>,
    /// Spawn order (species indices) of every wave.
    pub waves: Vec<Vec<usize>>,
    /// Scripted changes still to come.
    pub changes: Vec<Change>,
    pub wave: Wave,
    pub tower_stats: HashMap<Weapon, TowerStats>,
    /// (cell, weapon) of every tower inked.
    pub towers: Vec<(usize, Weapon)>,
    /// Name of the boss phase under way, while the boss is on the map.
    pub boss: Option<String>,
    /// Indices in `TOOLS` this level allows.
    pub tools: Vec<usize>,
    pub tool: usize,
    /// Tools inked at least once, for the challenge.
    pub used: Vec<usize>,
    pub shown: usize,
    pub sepia: u32,
    /// Fractions of sépia owed by Sceaux brisés.
    loot_carry: f32,
    pub lives: u32,
    /// Names kept in the Rôle: Fuyards reaching the Refuge, and notes.
    pub names: i32,
    pub fuyard_hurt: bool,
    /// (species, cell) of every enemy on the map and the cell it walks to.
    pub walkers: Vec<(usize, usize)>,
    pub calque: bool,
    pub hovered: Option<usize>,
    /// Notes still to come, and the margin written so far.
    notes: Vec<Note>,
    pub margin: Vec<String>,
    pub challenge: Option<Challenge>,
    pub choice: Option<Choice>,
    /// The choice was made on an earlier run: replaying the level can't change it.
    pub choice_locked: bool,
    /// The campaign's choices so far (from the save).
    pub choices: BTreeMap<String, String>,
    pub skills: Vec<Skill>,
    /// Won, and its sceaux are in the save.
    pub finished: bool,
    /// La Tension de la feuille (docs, section 9), hidden from the player: from -2 (the sheet
    /// buckles) to 2 (the sheet is taut). Waves won clean tighten it; waves that cost Rôle loosen it.
    pub tension: i32,
    lives_at_wave_start: u32,
}

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
            wave: Wave::default(),
            tower_stats: towers::load_stats(),
            towers: Vec::new(),
            boss: None,
            tools: (0..TOOLS.len()).collect(),
            tool: 0,
            used: Vec::new(),
            shown: 0,
            sepia: 120,
            loot_carry: 0.0,
            lives: START_LIVES,
            names: 0,
            fuyard_hurt: false,
            walkers: Vec::new(),
            calque: false,
            hovered: None,
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
        let data: LevelData = ron::from_str(LEVELS[index]).expect("invalid level file");
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
        let tools: Vec<usize> =
            data.tools.iter().map(|t| tool_index(t).unwrap_or_else(|| panic!("unknown tool {t:?}"))).collect();
        let mut map = data.map.join("\n");
        map = match &data.alt_refuge {
            Some(cond) if holds(&save.choices, cond) => map.replace('R', ".").replace('W', "R"),
            _ => map.replace('W', "."),
        };

        let mut level = Level::new(Grid::parse(&map), species);
        level.index = index;
        level.world = data.world - 1;
        level.name = data.name;
        level.sepia = data.sepia;
        level.tool = tools[0];
        level.tools = tools;
        level.events = data.events;
        level.event_next = level.event_at(1);
        level.waves = waves;
        level.shown = present[0];
        level.present = present;
        level.changes = data.changes;
        level.notes = data.notes;
        level.challenge = data.challenge;
        level.choice_locked = data.choice.as_ref().is_some_and(|c| save.choices.contains_key(&c.id));
        level.choice = data.choice;
        level.choices = save.choices.clone();
        level.skills = save.skills.clone();
        level.note(Trigger::Start);
        level
    }

    pub fn world(&self) -> &'static World {
        &WORLDS[self.world]
    }

    /// HP of an enemy `relative` to the Chaîneur's 100 HP at world 1: ×1.5 per world after the
    /// first (docs, section 8).
    pub fn hp(&self, relative: f32) -> f32 {
        BASE_HP * 1.5f32.powi(self.world as i32) * relative
    }

    /// Whether the world event is under way during wave `wave`.
    pub fn event_at(&self, wave: u32) -> bool {
        let lasts = self.world().event.lasts;
        self.events.iter().any(|&start| (start..start + lasts).contains(&wave))
    }

    /// `base` once every scripted change still to come has fallen.
    pub fn after_changes(&self) -> Grid {
        let mut grid = self.base.clone();
        self.changes.iter().for_each(|c| c.apply(&mut grid));
        grid
    }

    pub fn has(&self, skill: Skill) -> bool {
        self.skills.contains(&skill)
    }

    /// Every wave sent and beaten.
    pub fn won(&self) -> bool {
        self.lives > 0 && !self.waves.is_empty() && self.wave.number as usize == self.waves.len() && !self.wave.running
    }

    /// Once won: the sceaux and Names go to the save, and the next level opens.
    pub fn finish(&mut self, save: &mut Save) {
        self.finished = true;
        self.note(Trigger::Won);
        let challenge = self.challenge_met();
        let record = save.record(self.index);
        record.won = true;
        record.challenge |= challenge;
        record.names = self.names;
        // A level with a choice opens the next one only once it is made: see `choose`.
        if self.chosen() {
            self.unlock_next(save);
        }
    }

    fn unlock_next(&self, save: &mut Save) {
        save.reached = save.reached.max((self.index + 1).min(LEVELS.len() - 1));
    }

    /// Rebuild `grid` and the flow fields after `base`, the towers or `event_on` changed.
    pub fn refresh(&mut self) {
        let weight = if self.has(Skill::OmbrePortee) { 1.5 } else { 1.0 };
        let cover = (0..self.base.tiles.len())
            .map(|i| {
                let covering = self.towers.iter().filter(|&&(t, weapon)| self.base.within(i, t, self.range(t, weapon)));
                covering.count() as f32 * weight
            })
            .collect();
        self.base.cover = cover;
        self.grid = if self.event_on { self.base.under(&self.world().event) } else { self.base.clone() };
        self.fields = flow::fields(&self.grid, &self.species);
        self.redraw = true;
    }
}

/// Once the last wave is beaten: sceaux, Names and the next level go to the save.
fn finish_level(mut level: ResMut<Level>, mut save: ResMut<Save>) {
    if level.won() && !level.finished {
        level.finish(&mut save);
        save.store();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every level loads, and every species it sends can reach its goal, with and without the world event,
    /// whatever was chosen.
    #[test]
    fn levels_are_playable() {
        let mut other = Save::default();
        other.choices.insert("refuge".into(), "bois".into());
        other.choices.insert("deserteurs".into(), "livrer".into());
        for save in [Save::default(), other] {
            for index in 0..LEVELS.len() {
                let level = Level::load(index, &save);
                assert!((1..=12).contains(&level.waves.len()), "{}: up to 12 waves (docs)", level.name);
                for (k, species) in level.species.iter().enumerate() {
                    if !level.waves.iter().flatten().any(|&s| s == k) {
                        continue;
                    }
                    let spawns = level.base.spawns_for(species);
                    assert!(!spawns.is_empty(), "{}: nowhere for {} to spawn", level.name, species.name);
                    let event = &level.world().event;
                    let later = level.after_changes();
                    for map in [level.base.under(event), later.under(event), level.base.clone(), later] {
                        let field = FlowField::new(&map, species);
                        assert!(
                            spawns.iter().all(|&s| field.dist[s].is_finite()),
                            "{}: {} can't reach its goal",
                            level.name,
                            species.name
                        );
                    }
                }
            }
        }
    }

    /// Le Droit de Passage and Tab only consider the species a level sends, boss phases included.
    #[test]
    fn present_species() {
        fn names(level: &Level) -> Vec<&str> {
            level.present.iter().map(|&k| level.species[k].name.as_str()).collect()
        }
        let at = |prefix: &str| {
            let index = (0..LEVELS.len()).find(|&i| Level::load(i, &Save::default()).name.starts_with(prefix)).unwrap();
            Level::load(index, &Save::default())
        };
        let (col, boss_level) = (at("2-1"), at("1-4"));
        assert_eq!(names(&col), ["chaineur", "gens_du_col", "harde"]);
        let boss = names(&boss_level);
        assert!(boss.contains(&"grande_chaine") && boss.contains(&"grande_chaine_rompue"), "{boss:?}");
    }

    /// Handing the deserters over in 2-2 drops the last two waves of 2-3, and costs a Name.
    #[test]
    fn deserters_choice_shortens_the_escort() {
        let escort = (0..LEVELS.len()).find(|&i| Level::load(i, &Save::default()).name.starts_with("2-3")).unwrap();
        let mut save = Save::default();
        save.choices.insert("deserteurs".into(), "escorter".into());
        let full = Level::load(escort, &save);
        save.choices.insert("deserteurs".into(), "livrer".into());
        let short = Level::load(escort, &save);
        assert_eq!(short.waves.len(), full.waves.len() - 2);
        assert_eq!((full.names, short.names), (0, -1));
    }
}
