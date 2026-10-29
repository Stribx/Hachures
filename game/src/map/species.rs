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

/// The silhouette a species is drawn with (docs, section 9: one template, told apart by silhouette
/// and colour).
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

/// A prop a species wears on its figurine (docs, direction artistique, phase 7).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Deserialize)]
pub enum Carry {
    /// The Chancellerie's flat hat.
    Bicorne,
    /// The surveying chain, coiled over the shoulder.
    Chaine,
    /// The ranging pole, striped, taller than its bearer.
    Jalon,
    /// The Échassiers' wide straw hat.
    Paille,
    /// The Marches' kerchief.
    Fichu,
    /// A bundle on the back.
    Ballot,
    /// A child carried on the hip.
    Enfant,
    /// The Gens du Col's chalk-white hood.
    Capuche,
    /// A coil of rope on the back.
    Corde,
    /// The alpenstock: an ice axe.
    Piolet,
    /// Pack boxes hanging at a beast's flanks.
    Bats,
    /// The convoy's canvas hood over the cart.
    Bache,
    /// The little figure who leads the cart.
    Meneur,
    /// The Grande Chaîne's two oxen walking in front of the cart.
    Boeufs,
    /// The yoke across the oxen's necks, a part of its own.
    Joug,
    /// The Chancellerie's cartographer standing on the cart's box.
    Gast,
    /// The sighting lantern on a post at the front of the box.
    Visee,
    /// The Foreuse's toothed drill head.
    Foreuse,
}

/// What a species walks towards.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Deserialize)]
pub enum Goal {
    /// What the hostiles attack and Nève defends (`M` on the map; `World.defends` names it): the
    /// Moulin, the Refuge Sarre, the Puits d'Air…
    Cible,
    BorneMere,
    Refuge,
    /// Where an escorted convoy leaves the map (`X`).
    Sortie,
}

#[derive(Clone, Deserialize)]
pub struct Species {
    /// Its key in `species.ron`, which the code and the level files use.
    #[serde(skip)]
    pub name: String,
    /// What the player reads: a key of `assets/data/i18n/`.
    pub label: String,
    pub goal: Goal,
    #[serde(default)]
    pub reading: Reading,
    /// Share of every hit it shrugs off, except from the weapon it is weak to (the Étayeur's
    /// armour).
    #[serde(default)]
    pub armor: f32,
    /// Walks under the ground: only a weapon that strikes there (the Sonde) reaches it.
    #[serde(default)]
    pub underground: bool,
    /// Share of a damaged borne's HP it gives back each second, beside it (the Étayeur).
    #[serde(default)]
    pub mends: f32,
    /// Towers only aim at hostiles (a mortar splash still hurts the others).
    pub hostile: bool,
    /// Whether a weapon can damage this species.
    #[serde(default = "default_damageable")]
    pub damageable: bool,
    /// Relative to the Chaîneur's 100 HP.
    pub hp: f32,
    /// Tiles per second on a cost-1 tile.
    pub speed: f32,
    pub loot: u32,
    pub weak_to: Option<Weapon>,
    pub color: (f32, f32, f32),
    #[serde(default)]
    pub shape: Shape,
    /// The props its figurine wears, besides its body and (for a hostile) its seal.
    #[serde(default)]
    pub carries: Vec<Carry>,
    pub costs: HashMap<Tile, f32>,
}

fn default_damageable() -> bool {
    true
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_species_walks_plain_sol() {
        for s in load() {
            assert!(s.costs.contains_key(&Tile::Sol), "{} can't walk Sol", s.name);
        }
    }

    #[test]
    fn no_species_walks_a_palissade() {
        for s in load() {
            assert!(!s.costs.contains_key(&Tile::Palissade), "{} walks a Palissade", s.name);
        }
    }

    #[test]
    fn carries_default_to_none() {
        let species: Species = ron::from_str(
            "(label: \"x\", goal: Cible, hostile: true, hp: 1.0, speed: 1.0, loot: 0, weak_to: None, \
             color: (0.5, 0.5, 0.5), costs: {Sol: 1.0})",
        )
        .unwrap();
        assert!(species.carries.is_empty());
    }
}
