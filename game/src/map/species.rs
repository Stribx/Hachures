//! Who walks the map, where to, and how each species prices the ground.

use std::collections::{BTreeMap, HashMap};

use serde::Deserialize;

use super::grid::{Grid, Tile};
use crate::combat::towers::Weapon;

/// How a species prices the ground (docs, les trois lectures; le But is its `goal`).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Deserialize)]
pub enum Reading {
    /// The terrain itself: `costs`.
    #[default]
    Pente,
    /// Fear: every walkable cell costs 1 + the towers covering it.
    Ombre,
}

/// The silhouette a species is drawn with (docs, section 9: one template, told apart by silhouette and colour).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Deserialize)]
pub enum Shape {
    /// On foot.
    #[default]
    Pied,
    /// On stilts: the Échassiers.
    Echasses,
    /// A pack beast: the Hardes.
    Bete,
    /// A wagon: the Grande Chaîne, the convoy.
    Chariot,
}

/// What a species walks towards.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Deserialize)]
pub enum Goal {
    /// What the hostiles attack and Nève defends (`M` on the map): the Moulin in world 1, the Refuge Sarre in world 2.
    Cible,
    BorneMere,
    Refuge,
    /// Where an escorted convoy leaves the map (`X`).
    Sortie,
}

#[derive(Deserialize)]
pub struct Species {
    /// Its key in `species.ron`, which the code and the level files use.
    #[serde(skip)]
    pub name: String,
    /// What the player reads, in French.
    pub label: String,
    pub goal: Goal,
    #[serde(default)]
    pub reading: Reading,
    /// Towers only aim at hostiles (a mortar splash still hurts the others).
    pub hostile: bool,
    /// Relative to the Chaîneur's 100 HP.
    pub hp: f32,
    /// Tiles per second on a cost-1 tile.
    pub speed: f32,
    pub loot: u32,
    pub weak_to: Option<Weapon>,
    pub color: (f32, f32, f32),
    #[serde(default)]
    pub shape: Shape,
    pub costs: HashMap<Tile, f32>,
}

impl Species {
    /// Cost of walking off `tile`; tiles missing from the profile are impassable.
    pub fn cost(&self, tile: Tile) -> f32 {
        self.costs.get(&tile).copied().unwrap_or(f32::INFINITY)
    }

    /// Cost of walking off cell `i`, as this species reads it.
    pub fn step(&self, grid: &Grid, i: usize) -> f32 {
        let cost = self.cost(grid.tiles[i]);
        match self.reading {
            Reading::Ombre if cost.is_finite() => 1.0 + grid.cover[i],
            _ => cost,
        }
    }
}

/// Every species of `assets/data/species.ron`, sorted by name; their index is used everywhere else.
pub fn load() -> Vec<Species> {
    let map: BTreeMap<String, Species> =
        ron::from_str(include_str!("../../assets/data/species.ron")).expect("invalid species.ron");
    map.into_iter().map(|(name, s)| Species { name, ..s }).collect()
}

/// Index of the species called `name`.
pub fn index_of(species: &[Species], name: &str) -> Option<usize> {
    species.iter().position(|s| s.name == name)
}
