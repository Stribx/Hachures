//! The terrain grid, parsed from a level's map. Pure logic, no Bevy.

use std::collections::HashMap;

use serde::Deserialize;

use super::species::{Goal, Species};
use crate::level::world::Event;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Deserialize)]
pub enum Tile {
    Sol,
    /// Low ground: La Crue drowns it.
    SolBas,
    Boue,
    Ronces,
    Eau,
    Gue,
    Barricade,
    Roc,
    /// Under a tower; in no profile, so impassable for everyone.
    Tour,
    /// Imperial road laid by La Grande Chaîne; cheap for the imperials.
    Chaussee,
    /// Frozen water, during Le Gel (world 2).
    Glace,
}

impl Tile {
    /// Nève can only ink over open, dry ground.
    pub fn buildable(self) -> bool {
        matches!(self, Tile::Sol | Tile::SolBas | Tile::Chaussee)
    }
}

#[derive(Clone)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Tile>,
    /// The tiles as drawn on the level's map, before any inking: what the world event reads.
    pub map: Vec<Tile>,
    /// Where hostile species enter.
    pub spawns: Vec<usize>,
    /// Where the allies (Fuyards, a convoy: non-hostile) set off from.
    pub fuyard_spawns: Vec<usize>,
    pub goals: HashMap<Goal, usize>,
    /// Imperial bornes planted by La Grande Chaîne: more targets for whoever seeks the Borne-Mère.
    pub bornes: Vec<usize>,
    /// Towers covering each cell (weighted), for the species that read l'Ombre.
    pub cover: Vec<f32>,
}

impl Grid {
    /// One char per tile. Markers stand on Sol: S spawn, F allies' start (Fuyards, a convoy), M the Cible
    /// (Moulin…), B Borne-Mère, R Refuge, X Sortie.
    pub fn parse(map: &str) -> Grid {
        let rows: Vec<&str> = map.lines().map(str::trim).filter(|l| !l.is_empty()).collect();
        let width = rows[0].len();
        assert!(rows.iter().all(|r| r.len() == width), "map rows must have the same length");
        let mut grid = Grid {
            width,
            height: rows.len(),
            tiles: Vec::with_capacity(width * rows.len()),
            map: Vec::new(),
            spawns: Vec::new(),
            fuyard_spawns: Vec::new(),
            goals: HashMap::new(),
            bornes: Vec::new(),
            cover: vec![0.0; width * rows.len()],
        };
        for (i, c) in rows.concat().chars().enumerate() {
            let tile = match c {
                '.' => Tile::Sol,
                ',' => Tile::SolBas,
                'b' => Tile::Boue,
                'r' => Tile::Ronces,
                '~' => Tile::Eau,
                'g' => Tile::Gue,
                '#' => Tile::Barricade,
                '^' => Tile::Roc,
                'S' => {
                    grid.spawns.push(i);
                    Tile::Sol
                }
                'F' => {
                    grid.fuyard_spawns.push(i);
                    Tile::Sol
                }
                'M' | 'B' | 'R' | 'X' => {
                    let goal = match c {
                        'M' => Goal::Cible,
                        'B' => Goal::BorneMere,
                        'R' => Goal::Refuge,
                        _ => Goal::Sortie,
                    };
                    grid.goals.insert(goal, i);
                    Tile::Sol
                }
                _ => panic!("unknown map char {c:?}"),
            };
            grid.tiles.push(tile);
        }
        grid.map = grid.tiles.clone();
        grid
    }

    /// Whether the world event changes cell `i`.
    pub fn hit_by(&self, event: &Event, i: usize) -> bool {
        event.cells.contains(&self.map[i])
    }

    /// The map while the world event lasts: its cells turn to `event.becomes`, unless inked with one of `event.spares`.
    pub fn under(&self, event: &Event) -> Grid {
        let mut grid = self.clone();
        for i in 0..grid.tiles.len() {
            if self.hit_by(event, i) && !event.spares.contains(&grid.tiles[i]) {
                grid.tiles[i] = event.becomes;
            }
        }
        grid
    }

    pub fn spawns_for(&self, species: &Species) -> &[usize] {
        if species.hostile { &self.spawns } else { &self.fuyard_spawns }
    }

    pub fn coords(&self, i: usize) -> (usize, usize) {
        (i % self.width, i / self.width)
    }

    /// Whether cells `a` and `b` are at most `range` apart, center to center.
    pub fn within(&self, a: usize, b: usize, range: f32) -> bool {
        let ((ax, az), (bx, bz)) = (self.coords(a), self.coords(b));
        (ax as f32 - bx as f32).powi(2) + (az as f32 - bz as f32).powi(2) <= range.powi(2)
    }

    pub fn neighbours(&self, i: usize) -> impl Iterator<Item = usize> {
        let (x, y) = self.coords(i);
        let w = self.width;
        [
            (x > 0).then(|| i - 1),
            (x + 1 < w).then(|| i + 1),
            (y > 0).then(|| i - w),
            (y + 1 < self.height).then(|| i + w),
        ]
        .into_iter()
        .flatten()
    }
}
