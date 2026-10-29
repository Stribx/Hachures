//! The terrain: its grid, who walks it and how, and its blocks on screen.

pub mod flow;
pub mod grid;
pub mod species;
pub mod terrain;

pub mod ambience;
pub(crate) mod barricade;
pub mod biome;
mod cells;
pub mod daylight;
mod decor;
pub mod fauna;
pub mod fields;
mod grass;
pub mod growth;
pub(crate) mod land;
pub(crate) mod props;
pub(crate) mod scraper;
mod seams;
pub mod weather;

use bevy::prelude::*;

use crate::Step;
use crate::theme::{MEADOW, PASTURE, SEPIA, WATER, WOOD};
use cells::redraw_cells;
use grass::grow;
use grid::{Grid, Tile};
use land::{fibres, land};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (redraw_cells, grow, land, fibres).chain().in_set(Step::Render))
        .add_systems(Startup, ambience::start)
        .add_systems(
            Update,
            (daylight::follow_the_waves, weather::fall, ambience::fade).in_set(Step::Render),
        )
        .add_systems(
            Update,
            ((fauna::settle, fauna::flutter).chain(), fauna::launch, fauna::cross)
                .in_set(Step::Render),
        );
}

/// Center of cell `i`, just above the ground.
pub fn cell_pos(grid: &Grid, i: usize) -> Vec3 {
    let (x, z) = grid.coords(i);
    Vec3::new(x as f32, 0.45, z as f32)
}

/// The height and colour of a tile's block.
#[derive(Clone, Copy, Debug)]
pub struct Block {
    pub height: f32,
    pub color: Color,
}

/// Height and colour of a tile's block, washed in like a hand-coloured map: each tile keeps its own
/// family, so the ground still reads at a glance (docs/direction-artistique.md, phase 6).
pub fn look(tile: Tile) -> Block {
    match tile {
        Tile::Sol => Block { height: 0.2, color: PASTURE },
        Tile::SolBas => Block { height: 0.15, color: MEADOW },
        // Sunk below the ground: a trap, not a wall.
        Tile::Boue => Block { height: 0.14, color: Color::srgb(0.55, 0.4, 0.24) },
        Tile::Ronces => Block { height: 0.35, color: Color::srgb(0.36, 0.3, 0.2) },
        Tile::Eau => Block { height: 0.05, color: WATER },
        Tile::Gue => Block { height: 0.1, color: Color::srgb(0.62, 0.78, 0.82) },
        Tile::Barricade => Block { height: 0.6, color: WOOD },
        // Warm limestone, so the ridges stay stone among the greens.
        Tile::Roc => Block { height: 0.5, color: Color::srgb(0.72, 0.65, 0.52) },
        Tile::Tour => Block { height: 0.2, color: Color::srgb(0.8, 0.76, 0.66) },
        Tile::Chaussee => Block { height: 0.21, color: Color::srgb(0.95, 0.93, 0.86) },
        Tile::Glace => Block { height: 0.07, color: Color::srgb(0.88, 0.93, 0.95) },
        Tile::Eboulis => Block { height: 0.2, color: Color::srgb(0.6, 0.57, 0.52) },
        Tile::Palissade => Block { height: 0.2, color: Color::srgb(0.36, 0.26, 0.2) },
        // A pit: sunk to the walnut base and slate-black.
        Tile::Gouffre => Block { height: 0.02, color: Color::srgb(0.13, 0.12, 0.13) },
    }
}

/// Colour of cell `i`: its tile's, or that of what stands on it.
pub fn cell_color(grid: &Grid, i: usize) -> Color {
    if grid.spawns.contains(&i) {
        // Where the imperial columns enter: vermilion, the Chancellerie's seal.
        Color::srgb(0.7, 0.2, 0.15)
    } else if grid.fuyard_spawns.contains(&i) {
        Color::srgb(0.95, 0.9, 0.75)
    } else if grid.goals.values().any(|&g| g == i) {
        SEPIA
    } else {
        look(grid.tiles[i]).color
    }
}
