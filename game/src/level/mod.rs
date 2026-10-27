//! A level: its map as inked, its world's event, waves and sépia, and what it leaves in the campaign.
//! `Level` is one resource; each feature adds its own `impl Level` (inking in `ink`, tower reach in `combat`).

pub mod data;
mod load;
pub mod story;
pub mod waves;
pub mod world;

use std::collections::{BTreeMap, HashMap};

use bevy::prelude::*;

use crate::Step;
use crate::campaign::save::Save;
use crate::campaign::skills::Skill;
use crate::combat::towers::{TowerStats, Weapon};
use crate::map::flow::{self, FlowField};
use crate::map::grid::Grid;
use crate::map::species::Species;
use data::{Change, LEVELS, Place};
use story::{Challenge, Choice, MarginNote, Note, Trigger};
use waves::Wave;
use world::{WORLDS, World};

/// Chaîneur HP at world 1, wave 1 (docs, section 8).
const BASE_HP: f32 = 100.0;

pub fn plugin(app: &mut App) {
    // Once the Rôle is lost, the sheet stands still until Enter plays again.
    app.configure_sets(Update, Step::Simulate.run_if(alive)).add_systems(Update, finish_level.in_set(Step::Simulate));
}

/// Some of the Rôle is left: the simulation goes on.
fn alive(level: Res<Level>) -> bool {
    level.lives > 0
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
    pub places: Vec<Place>,
    /// Indices in `places` of the names scratched out.
    pub erased: Vec<usize>,
    pub wave: Wave,
    pub tower_stats: HashMap<Weapon, TowerStats>,
    /// (cell, weapon) of every tower inked.
    pub towers: Vec<(usize, Weapon)>,
    /// Name of the boss phase under way, while the boss is on the map.
    pub boss: Option<String>,
    /// Indices in `TOOLS` this level allows.
    pub tools: Vec<usize>,
    /// Tools inked at least once, for the challenge.
    pub used: Vec<usize>,
    pub sepia: u32,
    /// Fractions of sépia owed by Sceaux brisés.
    loot_carry: f32,
    pub lives: u32,
    /// Names kept in the Rôle: Fuyards reaching the Refuge, and notes.
    pub names: i32,
    pub fuyard_hurt: bool,
    /// (species, cell) of every enemy on the map and the cell it walks to.
    pub walkers: Vec<(usize, usize)>,
    /// Notes still to come, and the margin written so far.
    notes: Vec<Note>,
    pub margin: Vec<MarginNote>,
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

    /// The place at cell `i` whose name is still on the sheet, if any.
    pub fn place_at(&self, i: usize) -> Option<usize> {
        (0..self.places.len()).find(|&p| !self.erased.contains(&p) && self.places[p].covers(i, self.base.width))
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
        record.erased = self.erased.iter().map(|&p| self.places[p].name.clone()).collect();
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
pub(crate) fn finish_level(mut level: ResMut<Level>, mut save: ResMut<Save>) {
    if level.won() && !level.finished {
        level.finish(&mut save);
        save.store();
    }
}

#[cfg(test)]
mod tests {
    use bevy::state::app::StatesPlugin;

    use super::*;
    use crate::map::species;
    use crate::schedule::Screen;

    /// Once the Rôle is lost, the simulation stands still, even during play.
    #[test]
    fn a_lost_level_stands_still() {
        #[derive(Resource, Default)]
        struct Ticks(u32);
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, StatesPlugin, crate::schedule::plugin, plugin))
            .init_resource::<Ticks>()
            .insert_resource(Save::default())
            .insert_resource(Level::new(Grid::parse("S.M"), species::load()))
            .add_systems(Update, (|mut ticks: ResMut<Ticks>| ticks.0 += 1).in_set(Step::Simulate));
        app.world_mut().resource_mut::<NextState<Screen>>().set(Screen::Playing);
        app.update();
        assert_eq!(app.world().resource::<Ticks>().0, 1, "playing");
        app.world_mut().resource_mut::<Level>().lives = 0;
        app.update();
        assert_eq!(app.world().resource::<Ticks>().0, 1, "the Rôle is lost");
    }

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

    /// Every named place stands on ground Nève can ink: otherwise its name could never cost anything.
    #[test]
    fn places_can_be_inked() {
        for index in 0..LEVELS.len() {
            let level = Level::load(index, &Save::default());
            for place in &level.places {
                for &(x, y) in &place.at {
                    let i = y * level.base.width + x;
                    assert!(level.base.tiles[i].buildable(), "{}: {} at ({x}, {y})", level.name, place.name);
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
            let index = (0..LEVELS.len())
                .find(|&i| Level::load(i, &Save::default()).name == format!("level.{prefix}.name"))
                .unwrap();
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
        let escort = (0..LEVELS.len()).find(|&i| Level::load(i, &Save::default()).name == "level.2-3.name").unwrap();
        let mut save = Save::default();
        save.choices.insert("deserteurs".into(), "escorter".into());
        let full = Level::load(escort, &save);
        save.choices.insert("deserteurs".into(), "livrer".into());
        let short = Level::load(escort, &save);
        assert_eq!(short.waves.len(), full.waves.len() - 2);
        assert_eq!((full.names, short.names), (0, -1));
    }
}
