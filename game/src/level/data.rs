//! The campaign's level files, as written in `assets/data/levels/`.

use serde::Deserialize;

use super::story::{Challenge, Choice, Note};
use crate::map::grid::Tile;
use crate::map::terrain::Edit;

/// The campaign, in order.
pub const LEVELS: [&str; 25] = [
    include_str!("../../assets/data/levels/0-0.ron"),
    include_str!("../../assets/data/levels/1-1.ron"),
    include_str!("../../assets/data/levels/1-2.ron"),
    include_str!("../../assets/data/levels/1-3.ron"),
    include_str!("../../assets/data/levels/1-4.ron"),
    include_str!("../../assets/data/levels/2-1.ron"),
    include_str!("../../assets/data/levels/2-2.ron"),
    include_str!("../../assets/data/levels/2-3.ron"),
    include_str!("../../assets/data/levels/2-4.ron"),
    include_str!("../../assets/data/levels/3-1.ron"),
    include_str!("../../assets/data/levels/3-2.ron"),
    include_str!("../../assets/data/levels/3-3.ron"),
    include_str!("../../assets/data/levels/3-4.ron"),
    include_str!("../../assets/data/levels/4-1.ron"),
    include_str!("../../assets/data/levels/4-2.ron"),
    include_str!("../../assets/data/levels/4-3.ron"),
    include_str!("../../assets/data/levels/4-4.ron"),
    include_str!("../../assets/data/levels/5-1.ron"),
    include_str!("../../assets/data/levels/5-2.ron"),
    include_str!("../../assets/data/levels/5-3.ron"),
    include_str!("../../assets/data/levels/5-4.ron"),
    include_str!("../../assets/data/levels/6-1.ron"),
    include_str!("../../assets/data/levels/6-2.ron"),
    include_str!("../../assets/data/levels/6-3.ron"),
    include_str!("../../assets/data/levels/6-4.ron"),
];

/// Enemies reaching the Cible or the Borne-Mère before the Rôle is lost (not in the docs; set with
/// the balance bot).
pub const START_LIVES: u32 = 20;

/// A level file.
#[derive(Deserialize)]
pub struct LevelData {
    pub name: String,
    /// Where the sheet lies on the Marches: world position of its cell (0, 0).
    #[serde(default)]
    pub at: (f32, f32),
    /// Its world, from 1 (`assets/data/worlds/1.ron`).
    pub world: usize,
    pub sepia: u32,
    pub tools: Vec<String>,
    /// Tools winning this level earns for good (a power): they join every level's tools once it is
    /// won.
    #[serde(default)]
    pub grants: Vec<String>,
    /// Waves the world event starts at.
    pub events: Vec<u32>,
    /// Waves during which the board is dimmed; the Calque remains readable.
    #[serde(default)]
    pub darkness: Option<(u32, u32)>,
    /// Wave at which world 4's wind reverses.
    #[serde(default)]
    pub wind_turn: Option<u32>,
    /// Wave and state of each lock gate: `true` means open water.
    #[serde(default)]
    pub vanne_schedule: Vec<(u32, bool)>,
    pub map: Vec<String>,
    pub waves: Vec<Vec<(String, u32)>>,
    #[serde(default)]
    pub notes: Vec<Note>,
    #[serde(default)]
    pub challenge: Option<Challenge>,
    #[serde(default)]
    pub choice: Option<Choice>,
    /// "id=option": if that choice went this way, the Refuge stands on `W` instead of `R`.
    #[serde(default)]
    pub alt_refuge: Option<String>,
    /// ("id=option", n): if that choice went this way, the level drops its last `n` waves.
    #[serde(default)]
    pub shorter: Option<(String, usize)>,
    /// Scripted changes of the map, such as an avalanche.
    #[serde(default)]
    pub changes: Vec<Change>,
    /// Named places, written on the sheet.
    #[serde(default)]
    pub places: Vec<Place>,
}

/// Where the Refuge stands on a map.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Refuge {
    Usual,
    /// On `W`, once the choice in `alt_refuge` went that way.
    Moved,
}

impl LevelData {
    /// Level `index` of `LEVELS`, read from its file.
    ///
    /// # Panics
    ///
    /// If `index` is not in `LEVELS`, or the file is not valid RON for a `LevelData`.
    pub fn of(index: usize) -> LevelData {
        ron::from_str(LEVELS[index]).expect("invalid level file")
    }

    /// The map as text for `Grid::parse`. The Refuge stands on `R`, or on `W` once `refuge` is
    /// `Moved`; the cell not used is plain ground.
    pub fn map_text(&self, refuge: Refuge) -> String {
        let map = self.map.join("\n");
        match refuge {
            Refuge::Moved => map.replace('R', ".").replace('W', "R"),
            Refuge::Usual => map.replace('W', "."),
        }
    }
}

/// A named place (docs, section 2: le prix du nom): inking any of its cells scratches the name out,
/// and the Rôle des Noms loses it.
#[derive(Clone, Deserialize)]
pub struct Place {
    pub name: String,
    pub at: Vec<(usize, usize)>,
}

impl Place {
    /// Whether it covers cell `i` on a map `width` wide.
    pub fn covers(&self, i: usize, width: usize) -> bool {
        self.at.iter().any(|&(x, y)| y * width + x == i)
    }
}

/// A scripted change of the map [T]: as wave `wave` starts, the cells `at` (x, y) become `becomes`.
/// Announced by a shiver the wave before; le Droit de Passage already counts it before then.
#[derive(Clone, Deserialize)]
pub struct Change {
    pub wave: u32,
    pub at: Vec<(usize, usize)>,
    pub becomes: Tile,
    /// What the HUD says the wave before (les Effondrements: `world.3.announce`), if anything.
    #[serde(default)]
    pub announce: Option<String>,
    /// The shiver's colour the wave before: white unless given (red cracks for a collapse).
    #[serde(default)]
    pub color: Option<(f32, f32, f32)>,
}

impl Change {
    /// The cells it changes, on a map `width` wide.
    pub fn cells(&self, width: usize) -> impl Iterator<Item = usize> + '_ {
        self.at.iter().map(move |&(x, y)| y * width + x)
    }

    /// What it does to the map, on a map `width` wide: the land itself changes.
    pub fn edits(&self, width: usize) -> impl Iterator<Item = (usize, Edit)> + '_ {
        self.cells(width).map(|i| (i, Edit::Land(self.becomes)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The Refuge stands on `R`, or on `W` once the choice went that way; the other cell is ground.
    #[test]
    fn the_refuge_moves_with_the_choice() {
        let mut data = LevelData::of(0);
        data.map = vec!["R.W".into()];
        assert_eq!(data.map_text(Refuge::Usual), "R..");
        assert_eq!(data.map_text(Refuge::Moved), "..R");
    }
}
