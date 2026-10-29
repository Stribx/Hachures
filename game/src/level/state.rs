//! The `Level` resource: everything a level on the board holds, and the rules that read and change
//! it.

use std::collections::{BTreeMap, HashMap};

use bevy::prelude::*;

use super::data::{Change, LEVELS, Place};
use super::story::{Challenge, Choice, MarginNote, Note, Trigger};
use super::waves::Wave;
use super::world::{WORLDS, World};
use crate::campaign::save::Save;
use crate::campaign::skills::Skill;
use crate::combat::towers::{TowerStats, Weapon};
use crate::map::grid::{Grid, Tile};
use crate::map::species::Species;
use crate::map::terrain::{Edit, Terrain};

/// Chaîneur HP at world 1, wave 1 (docs, section 8).
const BASE_HP: f32 = 100.0;

#[derive(Resource)]
pub struct Level {
    /// Position in `LEVELS`.
    pub index: usize,
    /// Position in `WORLDS`.
    pub world: usize,
    pub name: String,
    /// The map: as inked, as walked, the towers on it and the routes over it.
    pub terrain: Terrain,
    /// Waves the world event starts at; it lasts `event.lasts` waves.
    pub events: Vec<u32>,
    /// Wave and state of each lock gate: `true` means open water.
    pub vanne_schedule: Vec<(u32, bool)>,
    /// The world event is under way.
    pub event_on: bool,
    /// The next wave brings the event: its map is checked too before inking.
    pub event_next: bool,
    /// Dater has been used in this level.
    pub dated: bool,
    /// Dater cancels the current event instead of moving it.
    pub event_suppressed: bool,
    /// Nom gardé is consumed by the first Rename that would erase a place.
    pub rename_free: bool,
    /// The two-wave darkness interval of World 3 level 3-2, if any.
    pub darkness: Option<(u32, u32)>,
    pub wind_turn: Option<u32>,
    pub dune_direction: i32,
    pub vanne_open: bool,
    /// World 6's Mise au net: all species temporarily read the imperial profile.
    pub uniform_profile: bool,
    pub species: Vec<Species>,
    /// The species this level sends (the boss's phases included), sorted: le Droit de Passage and
    /// Tab only consider these.
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
    /// Name of the boss phase under way, while the boss is on the map.
    pub boss: Option<String>,
    /// Indices in `TOOLS` this level allows.
    pub tools: Vec<usize>,
    /// Tools inked at least once, for the challenge.
    pub used: Vec<usize>,
    /// Nève's strokes, in the order inked: what Biffer may take back.
    pub inked: Vec<Inked>,
    pub sepia: u32,
    /// Fractions of sépia owed by Sceaux brisés.
    pub(super) loot_carry: f32,
    pub lives: u32,
    /// Names kept in the Rôle: Fuyards reaching the Refuge, and notes.
    pub names: i32,
    /// One for the cell each enemy on the map stands on, and a second for the cell it walks to
    /// next, when it has one.
    pub walkers: Vec<Walker>,
    /// Notes still to come, and the margin written so far.
    pub(super) notes: Vec<Note>,
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
    /// buckles) to 2 (the sheet is taut). Waves won clean tighten it; waves that cost Rôle loosen
    /// it.
    pub tension: i32,
    pub(super) lives_at_wave_start: u32,
}

/// A species index and a cell where one of its walkers stands or is heading.
#[derive(Clone, Copy, Debug)]
pub struct Walker {
    pub species: usize,
    pub cell: usize,
}

/// One stroke Nève inked on the sheet.
#[derive(Clone, Copy, Debug)]
pub struct Inked {
    pub cell: usize,
    /// Index in `TOOLS`.
    pub tool: usize,
    /// The tile the cell was before.
    pub under: Tile,
    /// Sépia paid, skills included.
    pub paid: u32,
    /// `wave.number` when inked between waves; none when inked while a wave ran.
    pub laid_at: Option<u32>,
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
        let Some(event) = &self.world().event else { return false };
        if self.event_suppressed {
            return false;
        }
        self.events.iter().any(|&start| (start..start + event.lasts).contains(&wave))
    }

    pub fn darkness_on(&self) -> bool {
        self.darkness.is_some_and(|(from, to)| (from..=to).contains(&self.wave.number))
    }

    pub fn migrate_dunes(&mut self) {
        if self.world != 3 || self.wave.number == 0 || !self.wave.number.is_multiple_of(2) {
            return;
        }
        let width = self.terrain.base().width;
        let mut edits = Vec::new();
        for y in 0..self.terrain.base().height {
            let dunes: Vec<_> = (0..width)
                .map(|x| y * width + x)
                .filter(|&i| self.terrain.base().tiles[i] == Tile::Dune)
                .collect();
            for i in dunes {
                let x = i % width;
                let next_x = x as i32 + self.dune_direction;
                if (0..width as i32).contains(&next_x) {
                    let next = y * width + next_x as usize;
                    if self.terrain.base().tiles[next] == Tile::Sol {
                        edits.push((i, Edit::Land(Tile::Sol)));
                        edits.push((next, Edit::Land(Tile::Dune)));
                    }
                }
            }
        }
        self.alter(edits);
    }

    pub fn set_vanne(&mut self, open: bool) {
        if self.vanne_open != open {
            self.vanne_open = open;
            self.terrain.set_vanne(open, &self.species);
        }
    }

    /// `base` once every scripted change still to come has fallen.
    pub fn after_changes(&self) -> Grid {
        let width = self.terrain.base().width;
        self.terrain.ahead(self.changes.iter().flat_map(|c| c.edits(width)))
    }

    /// The place at cell `i` whose name is still on the sheet, if any.
    pub fn place_at(&self, i: usize) -> Option<usize> {
        (0..self.places.len()).find(|&p| {
            !self.erased.contains(&p) && self.places[p].covers(i, self.terrain.base().width)
        })
    }

    pub fn has(&self, skill: Skill) -> bool {
        self.skills.contains(&skill)
    }

    /// Dater: cancel an event already under way, or bring the announced event to this wave.
    pub fn date(&mut self) {
        if self.dated || self.world().event.is_none() || self.wave.running {
            return;
        }
        self.dated = true;
        if self.event_on {
            self.event_suppressed = true;
            self.event_on = false;
        } else if self.event_next {
            let wave = self.wave.number + 1;
            self.events = vec![wave];
            self.event_on = true;
        } else {
            return;
        }
        self.event_next = self.event_at(self.wave.number + 1);
        self.follow_event();
    }

    /// Every wave sent and beaten.
    pub fn won(&self) -> bool {
        self.lives > 0
            && !self.waves.is_empty()
            && self.wave.number as usize == self.waves.len()
            && !self.wave.running
    }

    /// Once won: the sceaux and Names go to the save, and the next level opens.
    pub fn finish(&mut self, save: &mut Save) {
        self.finished = true;
        self.note(Trigger::Won);
        let challenge = self.challenge_met();
        let record = save.record_mut(self.index);
        // A replay earns sceaux, but the story is written once, like the choice: the first win
        // keeps its Names.
        if !record.won {
            record.names = self.names;
            record.erased = self.erased.iter().map(|&p| self.places[p].name.clone()).collect();
        }
        record.won = true;
        record.challenge |= challenge;
        // A level with a choice opens the next one only once it is made: see `choose`.
        if self.chosen() {
            self.unlock_next(save);
        }
    }

    pub(super) fn unlock_next(&self, save: &mut Save) {
        save.reached = save.reached.max((self.index + 1).min(LEVELS.len() - 1));
    }

    /// Lets `edits` fall on the map: the one way a level's ground changes.
    pub fn alter(&mut self, edits: impl IntoIterator<Item = (usize, Edit)>) {
        self.terrain.alter(edits, &self.species);
    }

    /// The map follows `event_on`: under the world event while it lasts.
    pub(super) fn follow_event(&mut self) {
        let event = self.world().event.as_ref().filter(|_| self.event_on);
        self.terrain.set_event(event, &self.species);
    }
}

/// Once the last wave is beaten: sceaux, Names and the next level go to the save.
pub(crate) fn finish_level(mut level: ResMut<Level>, mut save: ResMut<Save>) {
    if level.won() && !level.finished {
        level.finish(&mut save);
        save.store();
    }
}
