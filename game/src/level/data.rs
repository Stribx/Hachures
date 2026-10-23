//! The campaign's level files, as written in `assets/data/levels/`.

use serde::Deserialize;

use super::story::{Challenge, Choice, Note};
use crate::map::grid::{Grid, Tile};

/// The campaign, in order.
pub const LEVELS: [&str; 7] = [
    include_str!("../../assets/data/levels/1-1.ron"),
    include_str!("../../assets/data/levels/1-2.ron"),
    include_str!("../../assets/data/levels/1-3.ron"),
    include_str!("../../assets/data/levels/1-4.ron"),
    include_str!("../../assets/data/levels/2-1.ron"),
    include_str!("../../assets/data/levels/2-2.ron"),
    include_str!("../../assets/data/levels/2-3.ron"),
];

/// Enemies reaching the Cible or the Borne-Mère before the Rôle is lost (not in the docs; set with the balance bot).
pub const START_LIVES: u32 = 20;

/// The name of level `index`, for the menus.
pub fn level_name(index: usize) -> String {
    #[derive(Deserialize)]
    struct Named {
        name: String,
    }
    ron::from_str::<Named>(LEVELS[index]).expect("invalid level file").name
}

/// A level file.
#[derive(Deserialize)]
pub struct LevelData {
    pub name: String,
    /// Its world, from 1 (`assets/data/worlds/1.ron`).
    pub world: usize,
    pub sepia: u32,
    pub tools: Vec<String>,
    /// Waves the world event starts at.
    pub events: Vec<u32>,
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
}

/// A scripted change of the map [T]: as wave `wave` starts, the cells `at` (x, y) become `becomes`.
/// Announced by a shiver the wave before; le Droit de Passage already counts it before then.
#[derive(Clone, Deserialize)]
pub struct Change {
    pub wave: u32,
    pub at: Vec<(usize, usize)>,
    pub becomes: Tile,
}

impl Change {
    /// The cells it changes, on a map `width` wide.
    pub fn cells(&self, width: usize) -> impl Iterator<Item = usize> + '_ {
        self.at.iter().map(move |&(x, y)| y * width + x)
    }

    /// Lets it fall on `grid`.
    pub fn apply(&self, grid: &mut Grid) {
        for i in self.cells(grid.width) {
            grid.tiles[i] = self.becomes;
        }
    }
}
