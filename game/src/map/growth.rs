//! What grows on the fields: felt tufts on the pastures, wildflowers in the meadows, wheat on the crop fields.

use bevy::prelude::*;

use super::fields::{fields, mix};
use super::grid::{Grid, Tile};

/// What a field grows.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Crop {
    Grass,
    Flowers,
    Wheat,
}

/// One thing growing: in `cell`, `at` from its centre, of `crop`, its own `roll` for its shape and colour.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Sprig {
    pub cell: usize,
    pub at: Vec2,
    pub crop: Crop,
    pub roll: u32,
}

/// Everything growing on `grid`, `density` of it (1 in full, less on a modest machine): only on its fields' open
/// ground as it is now, so nothing on a marker, under a tower or an inked piece, or in a drowned meadow. Up in the
/// mountains (`alpine`) nothing is sown: the crop fields are meadows in flower. The same every time the same map is
/// drawn.
pub fn sprigs(grid: &Grid, density: f32, alpine: bool) -> Vec<Sprig> {
    let fields = fields(grid);
    let mut sprigs = Vec::new();
    for (cell, field) in fields.iter().enumerate() {
        let Some(field) = *field else { continue };
        if !matches!(grid.tiles[cell], Tile::Sol | Tile::SolBas) {
            continue;
        }
        let crop = match (grid.map[cell], mix(field as u32 ^ 0xc3a5) % 6) {
            (Tile::SolBas, 0..=3) | (_, 0..=2) => Crop::Grass,
            (Tile::SolBas, _) | (_, 3) => Crop::Flowers,
            _ if alpine => Crop::Flowers,
            _ => Crop::Wheat,
        };
        // Wheat stands in rows, thick; grass and flowers are scattered.
        let full = match crop {
            Crop::Grass => 7,
            Crop::Flowers => 8,
            Crop::Wheat => 9,
        };
        let count = ((full as f32 * density).round() as usize).max(1);
        for k in 0..count {
            let roll = mix((cell as u32) << 4 ^ k as u32);
            let jitter = |bits: u32| ((roll >> bits) & 255) as f32 / 255.0 - 0.5;
            let at = match crop {
                Crop::Wheat => {
                    let (row, col) = ((k % 3) as f32 - 1.0, (k / 3 % 3) as f32 - 1.0);
                    Vec2::new(col * 0.28 + jitter(0) * 0.08, row * 0.28 + jitter(8) * 0.08)
                }
                _ => Vec2::new(jitter(0), jitter(8)) * 0.84,
            };
            sprigs.push(Sprig { cell, at, crop, roll });
        }
    }
    sprigs
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAP: &str = "
        S...,,,,....,,,,....M
        ..r..,,,,....rr.,,,..
        ...^^^...~~~.........
        F.......,,,,,,...,,,R
        ";

    #[test]
    fn the_same_map_grows_the_same() {
        assert_eq!(sprigs(&Grid::parse(MAP), 1.0, false), sprigs(&Grid::parse(MAP), 1.0, false));
    }

    /// Things grow on open ground only: never on a marker, rock, water or brambles.
    #[test]
    fn only_open_ground_grows() {
        let grid = Grid::parse(MAP);
        let marked: Vec<usize> =
            grid.spawns.iter().chain(&grid.fuyard_spawns).chain(grid.goals.values()).copied().collect();
        let sprigs = sprigs(&grid, 1.0, false);
        assert!(sprigs.len() > 100, "{}", sprigs.len());
        for s in sprigs {
            assert!(matches!(grid.tiles[s.cell], Tile::Sol | Tile::SolBas), "cell {}", s.cell);
            assert!(!marked.contains(&s.cell), "cell {}", s.cell);
            assert!(s.at.x.abs() < 0.5 && s.at.y.abs() < 0.5, "{:?}", s.at);
        }
    }

    /// An inked piece clears its cell: a barricade or a tower stands on bare ground.
    #[test]
    fn an_inked_cell_has_nothing_growing() {
        let mut grid = Grid::parse(MAP);
        let before = sprigs(&grid, 1.0, false);
        let cell = before[0].cell;
        grid.tiles[cell] = Tile::Barricade;
        let after = sprigs(&grid, 1.0, false);
        assert!(
            after.iter().all(|s| s.cell != cell),
            "cell {cell} still grows {:?}",
            after.iter().find(|s| s.cell == cell)
        );
    }

    /// A modest machine grows less, but still some on every field.
    #[test]
    fn less_density_grows_less() {
        let grid = Grid::parse(MAP);
        let (full, sparse) = (sprigs(&grid, 1.0, false).len(), sprigs(&grid, 0.35, false).len());
        assert!(sparse * 2 < full && sparse > 0, "{sparse} of {full}");
    }

    /// Wheat grows on pasture only: the wet meadows have grass and flowers.
    #[test]
    fn no_wheat_in_the_meadows() {
        let grid = Grid::parse(MAP);
        let wheat: Vec<_> = sprigs(&grid, 1.0, false).into_iter().filter(|s| s.crop == Crop::Wheat).collect();
        assert!(wheat.iter().all(|s| grid.map[s.cell] != Tile::SolBas), "{wheat:?}");
    }

    /// Up in the Cols nothing is sown: the wheat fields are alpine meadows in flower.
    #[test]
    fn the_cols_grow_flowers_not_wheat() {
        let grid = Grid::parse(MAP);
        let (bocage, alpine) = (sprigs(&grid, 1.0, false), sprigs(&grid, 1.0, true));
        assert!(bocage.iter().any(|s| s.crop == Crop::Wheat), "the bocage has wheat to replace");
        assert!(alpine.iter().all(|s| s.crop != Crop::Wheat), "{alpine:?}");
        let flowers = |sprigs: &[Sprig]| sprigs.iter().filter(|s| s.crop == Crop::Flowers).count();
        assert!(flowers(&alpine) > flowers(&bocage), "{} against {}", flowers(&alpine), flowers(&bocage));
    }
}
