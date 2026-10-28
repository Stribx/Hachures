//! Per-species flow fields and le Droit de Passage. Pure logic, no Bevy.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use super::grid::{Grid, Tile};
use super::species::{Goal, Species};

/// Walking cost from every cell to the species' goal (∞ = cut off).
pub struct FlowField {
    pub dist: Vec<f32>,
}

impl FlowField {
    /// Dijkstra from the goal outward.
    pub fn new(grid: &Grid, species: &Species) -> FlowField {
        let mut dist = vec![f32::INFINITY; grid.tiles.len()];
        let mut heap = BinaryHeap::new();
        let bornes = if species.goal == Goal::BorneMere { &grid.bornes[..] } else { &[] };
        for &source in grid.goals.get(&species.goal).into_iter().chain(bornes) {
            dist[source] = 0.0;
            heap.push(Reverse((0, source)));
        }
        // Distances are never negative, so their bits sort like the floats: a min-heap of (bits, cell).
        while let Some(Reverse((bits, i))) = heap.pop() {
            let d = f32::from_bits(bits);
            if d > dist[i] {
                continue;
            }
            for n in grid.neighbours(i) {
                let nd = d + species.step(grid, n);
                if nd < dist[n] {
                    dist[n] = nd;
                    heap.push(Reverse((nd.to_bits(), n)));
                }
            }
        }
        FlowField { dist }
    }

    /// Next cell to step on from `i`; None at the goal or when cut off.
    pub fn next_step(&self, grid: &Grid, i: usize) -> Option<usize> {
        grid.neighbours(i)
            .filter(|&n| self.dist[n] < self.dist[i])
            .min_by(|&a, &b| self.dist[a].total_cmp(&self.dist[b]))
    }

    /// Cells walked from `from`, `from` included, until the goal or a dead end.
    pub fn path(&self, grid: &Grid, from: usize) -> Vec<usize> {
        let mut path = vec![from];
        let mut at = from;
        while let Some(n) = self.next_step(grid, at) {
            path.push(n);
            at = n;
        }
        path
    }
}

/// One field per species, in the same order.
pub fn fields(grid: &Grid, species: &[Species]) -> Vec<FlowField> {
    species.iter().map(|s| FlowField::new(grid, s)).collect()
}

/// Le Droit de Passage: painting `tile` at `i` is refused if any `present` species (indices in `species`)
/// would lose every path to its goal, from its spawns or from where its `walkers` (species index, cell) stand.
pub fn can_place(
    grid: &Grid,
    species: &[Species],
    present: &[usize],
    i: usize,
    tile: Tile,
    walkers: &[(usize, usize)],
) -> bool {
    let marked = grid.spawns.iter().chain(&grid.fuyard_spawns).chain(grid.goals.values()).chain(&grid.bornes);
    if marked.copied().any(|m| m == i) {
        return false;
    }
    let mut next = grid.clone();
    next.tiles[i] = tile;
    // Bornes fall (shot or scraped): only the lasting goals vouch for a path.
    next.bornes.clear();
    present.iter().map(|&k| (k, &species[k])).filter(|(_, s)| next.goals.contains_key(&s.goal)).all(|(k, s)| {
        let field = FlowField::new(&next, s);
        let walking = walkers.iter().filter(|w| w.0 == k).map(|w| w.1);
        next.spawns_for(s).iter().copied().chain(walking).all(|c| field.dist[c].is_finite())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::species::{self, index_of};

    #[test]
    fn droit_de_passage() {
        let species = species::load();
        let all: Vec<usize> = (0..species.len()).collect();
        // One gap in the wall, at (2, 2).
        let grid = Grid::parse(
            "
            S.#.M
            ..#.B
            ....R
            ",
        );
        let gap = 2 * grid.width + 2;
        assert!(!can_place(&grid, &species, &all, gap, Tile::Barricade, &[]), "closing the only gap must be refused");
        assert!(can_place(&grid, &species, &all, gap, Tile::Boue, &[]), "mud slows but leaves a path");
        assert!(can_place(&grid, &species, &all, grid.width, Tile::Barricade, &[]), "blocking a side cell is fine");
        assert!(!can_place(&grid, &species, &all, grid.spawns[0], Tile::Barricade, &[]), "spawns stay free");

        // A Chaîneur at (2, 1), rock left, right and below: closing (2, 0) traps it,
        // even though the spawn still reaches the Moulin along the bottom row.
        let chaineur = index_of(&species, "chaineur").unwrap();
        let pocket = Grid::parse(
            "
            S...M
            .^.^.
            ..^..
            .....
            ",
        );
        let walker = (chaineur, pocket.width + 2);
        assert!(can_place(&pocket, &species, &all, 2, Tile::Tour, &[]));
        assert!(
            !can_place(&pocket, &species, &all, 2, Tile::Tour, &[walker]),
            "a walker already on the map keeps a way out"
        );

        // Flow field leads the Chaîneur from the spawn to the Moulin through the gap.
        let field = FlowField::new(&grid, &species[chaineur]);
        let path = field.path(&grid, grid.spawns[0]);
        assert_eq!(path.last(), Some(&grid.goals[&Goal::Cible]));
        assert!(path.contains(&gap));
    }

    #[test]
    fn ombre_flees_towers() {
        let species = species::load();
        let rompue = &species[index_of(&species, "grande_chaine_rompue").unwrap()];
        let mut grid = Grid::parse(
            "
            S...M
            .....
            ",
        );
        // Three towers cover the straight road: going round underneath is cheaper.
        grid.cover[1..4].fill(3.0);
        let path = FlowField::new(&grid, rompue).path(&grid, grid.spawns[0]);
        assert_eq!(path.last(), Some(&grid.goals[&Goal::Cible]));
        assert!(path.iter().all(|c| !(1..4).contains(c)), "path {path:?} stays out of the shadow");
    }

    /// A borne may fall at any time: it doesn't keep the Borne-Mère's last gap open.
    #[test]
    fn a_borne_is_no_way_out() {
        let species = species::load();
        let jalonneur = index_of(&species, "jalonneur").unwrap();
        let mut grid = Grid::parse(
            "
            S...#B
            ....#.
            ......
            ",
        );
        grid.bornes.push(2);
        let gap = 2 * grid.width + 4;
        assert!(!can_place(&grid, &species, &[jalonneur], gap, Tile::Barricade, &[]));
    }
}
