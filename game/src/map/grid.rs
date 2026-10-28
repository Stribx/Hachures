//! The terrain grid, parsed from a level's map. Pure logic, no Bevy.

use std::collections::HashMap;

use serde::Deserialize;

use super::species::{Goal, Species};
use crate::i18n::n_;
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
    /// Fallen stone: slow to walk; Gratter scrapes it back to Sol.
    Eboulis,
    /// An imperial barricade the level data lays: impassable; Gratter scrapes it back to Sol.
    Palissade,
}

impl Tile {
    /// What the Calque calls it, in French.
    pub fn name(self) -> &'static str {
        match self {
            Tile::Sol => n_("tile.sol"),
            Tile::SolBas => n_("tile.sol_bas"),
            Tile::Boue => n_("tile.boue"),
            Tile::Ronces => n_("tile.ronces"),
            Tile::Eau => n_("tile.eau"),
            Tile::Gue => n_("tile.gue"),
            Tile::Barricade => n_("tile.barricade"),
            Tile::Roc => n_("tile.roc"),
            Tile::Tour => n_("tile.tour"),
            Tile::Chaussee => n_("tile.chaussee"),
            Tile::Glace => n_("tile.glace"),
            Tile::Eboulis => n_("tile.eboulis"),
            Tile::Palissade => n_("tile.palissade"),
        }
    }

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
    ///
    /// # Panics
    ///
    /// If the map is empty, its rows differ in length, or it holds a character that is not a tile or a marker.
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
                'e' => Tile::Eboulis,
                'p' => Tile::Palissade,
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

    /// Whether Gratter has something to take off cell `i`: an imperial borne, an Éboulis or a Palissade.
    pub fn scrapable(&self, i: usize) -> bool {
        self.bornes.contains(&i) || matches!(self.tiles[i], Tile::Eboulis | Tile::Palissade)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_eboulis_is_parsed_as_drawn() {
        let grid = Grid::parse("S.e.M");
        assert_eq!(grid.tiles[2], Tile::Eboulis);
        assert_eq!(grid.map[2], Tile::Eboulis);
    }

    #[test]
    fn a_palissade_is_parsed_as_drawn() {
        let grid = Grid::parse("S.p.M");
        assert_eq!(grid.tiles[2], Tile::Palissade);
        assert_eq!(grid.map[2], Tile::Palissade);
    }
}
