//! The bocage's fields: the open ground of a board cut into a quilt of small fields, each washed
//! its own colour.

use bevy::prelude::*;

use super::cell_color;
use super::grid::{Grid, Tile};
use crate::theme::{MEADOW, PASTURE};

/// The washes of dry pasture, the Sol's family: sap greens and a touch of gold, never as yellow as
/// straw.
const PASTURES: [Color; 4] = [
    PASTURE,
    Color::srgb(0.72, 0.77, 0.5),
    Color::srgb(0.59, 0.71, 0.44),
    Color::srgb(0.7, 0.73, 0.47),
];
/// The washes of wet meadow, the Sol bas's family: bluer greens, darker than any pasture.
const MEADOWS: [Color; 3] = [MEADOW, Color::srgb(0.48, 0.65, 0.55), Color::srgb(0.57, 0.7, 0.56)];

/// The field each cell of `grid` belongs to, by index: only open ground as drawn (Sol, Sol bas, and
/// the pasture under drawn brambles), never a marker. A field is all dry pasture or all wet meadow,
/// a few cells to a dozen, the same every time the same map is drawn.
pub fn fields(grid: &Grid) -> Vec<Option<usize>> {
    let n = grid.map.len();
    let marked: Vec<usize> =
        grid.spawns.iter().chain(&grid.fuyard_spawns).chain(grid.goals.values()).copied().collect();
    let open = |i: usize| {
        matches!(grid.map[i], Tile::Sol | Tile::SolBas | Tile::Ronces) && !marked.contains(&i)
    };
    let seed = grid
        .map
        .iter()
        .enumerate()
        .fold(grid.width as u32, |h, (i, &t)| mix(h ^ (t as u32 + 1) ^ i as u32));
    // Fields are started in an order of their own, scattered over the board, and grown cell by cell
    // up to a size of their own, over the same ground.
    let mut order: Vec<usize> = (0..n).filter(|&i| open(i)).collect();
    order.sort_by_key(|&i| mix(seed ^ i as u32));
    let mut field = vec![None; n];
    let mut sizes = Vec::new();
    for start in order {
        if field[start].is_some() {
            continue;
        }
        let (id, cap) = (sizes.len(), 6 + (mix(seed ^ start as u32 ^ 0x9e37) % 7) as usize);
        let mut queue = std::collections::VecDeque::from([start]);
        let mut size = 0;
        while let Some(i) = queue.pop_front() {
            if size == cap || field[i].is_some() {
                continue;
            }
            field[i] = Some(id);
            size += 1;
            queue.extend(grid.neighbours(i).filter(|&j| {
                open(j) && field[j].is_none() && kind(grid.map[j]) == kind(grid.map[start])
            }));
        }
        sizes.push(size);
    }
    absorb_crumbs(grid, &mut field, &mut sizes);
    field
}

/// The colour of each cell's ground: its field's wash where the field still shows (open ground, or
/// a barricade or brambles inked on it; la Crue drowns a meadow's), else what `cell_color` says.
pub fn ground(grid: &Grid) -> Vec<Color> {
    let fields = fields(grid);
    let count = fields.iter().flatten().max().map_or(0, |&field| field + 1);
    let mut wash: Vec<Option<usize>> = vec![None; count];
    for i in 0..fields.len() {
        let Some(field) = fields[i] else { continue };
        if wash[field].is_some() {
            continue;
        }
        let washes = if kind(grid.map[i]) == Tile::Sol { PASTURES.len() } else { MEADOWS.len() };
        // The washes its neighbours on the same ground already have.
        let taken: Vec<usize> = (0..fields.len())
            .filter(|&member| fields[member] == Some(field))
            .flat_map(|member| grid.neighbours(member))
            .filter_map(|neighbour| {
                fields[neighbour]
                    .filter(|&other| {
                        other != field && kind(grid.map[neighbour]) == kind(grid.map[i])
                    })
                    .and_then(|other| wash[other])
            })
            .collect();
        let first = mix(field as u32 ^ 0x5bd1) as usize;
        wash[field] = (0..washes)
            .map(|offset| (first + offset) % washes)
            .find(|candidate| !taken.contains(candidate))
            .or(Some(first % washes));
    }
    (0..fields.len())
        .map(|i| match (fields[i].and_then(|field| wash[field]), grid.tiles[i]) {
            (Some(pick), Tile::Sol | Tile::SolBas | Tile::Barricade | Tile::Ronces)
                if kind(grid.map[i]) == Tile::Sol =>
            {
                PASTURES[pick]
            }
            (Some(pick), Tile::Sol | Tile::SolBas | Tile::Barricade | Tile::Ronces) => {
                MEADOWS[pick]
            }
            _ => cell_color(grid, i),
        })
        .collect()
}

/// A field of one or two cells, left over between bigger ones, joins a neighbour of its ground if
/// that one has room.
fn absorb_crumbs(grid: &Grid, field: &mut [Option<usize>], sizes: &mut [usize]) {
    for i in 0..field.len() {
        let Some(f) = field[i] else { continue };
        if sizes[f] >= 3 {
            continue;
        }
        let joins = grid
            .neighbours(i)
            .filter_map(|j| field[j].filter(|&g| g != f && kind(grid.map[j]) == kind(grid.map[i])));
        if let Some(g) = joins.min_by_key(|&g| sizes[g]).filter(|&g| sizes[g] + sizes[f] <= 12) {
            for cell in field.iter_mut().filter(|c| **c == Some(f)) {
                *cell = Some(g);
            }
            sizes[g] += sizes[f];
            sizes[f] = 0;
        }
    }
}

/// The ground a drawn tile lies on, for the fields: wet meadow for Sol bas, dry pasture for the
/// rest.
fn kind(drawn: Tile) -> Tile {
    if drawn == Tile::SolBas { Tile::SolBas } else { Tile::Sol }
}

/// A well-stirred hash of `h`, for the fields' order and sizes, and what grows on them.
pub(super) fn mix(h: u32) -> u32 {
    let h = (h ^ (h >> 16)).wrapping_mul(0x7feb_352d);
    let h = (h ^ (h >> 15)).wrapping_mul(0x846c_a68b);
    h ^ (h >> 16)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    const MAP: &str = "
        S...,,,,....,,,,....M
        ..r..,,,,....rr.,,,..
        ...^^^...~~~.........
        F.......,,,,,,...,,,R
        ";

    fn sizes(fields: &[Option<usize>]) -> HashMap<usize, usize> {
        let mut sizes = HashMap::new();
        for f in fields.iter().flatten() {
            *sizes.entry(*f).or_insert(0) += 1;
        }
        sizes
    }

    #[test]
    fn the_same_map_gives_the_same_fields() {
        assert_eq!(fields(&Grid::parse(MAP)), fields(&Grid::parse(MAP)));
    }

    /// Only open ground is cut into fields: no rock, water or marker belongs to one.
    #[test]
    fn fields_lie_on_open_ground_only() {
        let grid = Grid::parse(MAP);
        let marked: Vec<usize> = grid
            .spawns
            .iter()
            .chain(&grid.fuyard_spawns)
            .chain(grid.goals.values())
            .copied()
            .collect();
        for (i, field) in fields(&grid).iter().enumerate() {
            let open = matches!(grid.map[i], Tile::Sol | Tile::SolBas | Tile::Ronces)
                && !marked.contains(&i);
            assert_eq!(field.is_some(), open, "cell {i}");
        }
    }

    /// A field never mixes dry pasture and wet meadow, so each keeps its colour family.
    #[test]
    fn a_field_is_all_sol_or_all_sol_bas() {
        let grid = Grid::parse(MAP);
        let fields = fields(&grid);
        let mut kinds = HashMap::new();
        for (i, f) in fields.iter().enumerate() {
            if let Some(f) = f {
                assert_eq!(
                    *kinds.entry(*f).or_insert(kind(grid.map[i])),
                    kind(grid.map[i]),
                    "field {f}"
                );
            }
        }
    }

    /// Fields are a few cells to a dozen: a quilt, not confetti nor one sheet.
    #[test]
    fn fields_are_a_few_cells_to_a_dozen() {
        let grid = Grid::parse(MAP);
        let sizes = sizes(&fields(&grid));
        assert!(sizes.len() >= 4, "{sizes:?}");
        assert!(sizes.values().all(|&n| n <= 12), "{sizes:?}");
        let singles = sizes.values().filter(|&&n| n < 3).count();
        assert!(singles * 4 <= sizes.len(), "mostly real fields: {sizes:?}");
    }

    /// Two fields side by side on the same ground never share a wash: the seam always shows.
    #[test]
    fn neighbouring_fields_differ() {
        let grid = Grid::parse(MAP);
        let (fields, ground) = (fields(&grid), ground(&grid));
        for i in 0..grid.map.len() {
            for j in grid.neighbours(i) {
                if fields[i].is_some()
                    && fields[j].is_some()
                    && fields[i] != fields[j]
                    && kind(grid.map[i]) == kind(grid.map[j])
                {
                    assert_ne!(ground[i], ground[j], "cells {i} and {j}");
                }
            }
        }
    }

    /// Each field takes a wash from its ground's family; what is not a field keeps its own colour.
    #[test]
    fn washes_stay_in_their_family() {
        let grid = Grid::parse(MAP);
        let fields = fields(&grid);
        for (i, wash) in ground(&grid).into_iter().enumerate() {
            match (fields[i], grid.tiles[i]) {
                (Some(_), Tile::Sol | Tile::Ronces) => {
                    assert!(PASTURES.contains(&wash), "cell {i}")
                }
                (Some(_), Tile::SolBas) => assert!(MEADOWS.contains(&wash), "cell {i}"),
                _ => assert_eq!(wash, cell_color(&grid, i), "cell {i}"),
            }
        }
    }
}
