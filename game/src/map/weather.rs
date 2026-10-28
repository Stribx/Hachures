//! The weather follows the world: rain slanting in the wind while la Crue is under way, snow during
//! le Gel and, lightly, all over the Cols. Drawn in ink strokes over the board.

use bevy::prelude::*;

use std::f32::consts::FRAC_PI_2;

use super::fields::mix;
use super::grid::{Grid, Tile};
use super::lands::Land;
use crate::level::Level;
use crate::settings::Settings;
use crate::theme::WATER;

/// What falls on the board.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Weather {
    Clear,
    Rain,
    /// Snow, `.0` of it: 1 in a storm, less when it only drifts.
    Snow(f32),
}

/// The weather in world `world` (an index of `WORLDS`), with its event under way or not.
pub fn weather(world: usize, event_on: bool) -> Weather {
    match (Land::of(world), event_on) {
        (Land::Bocage, true) => Weather::Rain,
        (Land::Cols, true) => Weather::Snow(1.0),
        (Land::Cols, false) => Weather::Snow(0.3),
        (Land::Bocage, false) => Weather::Clear,
    }
}

/// The wind, as in `wind.wesl`: rain slants and snow drifts with it.
const WIND: Vec2 = Vec2::new(0.94, 0.34);
/// Drops at full strength; snowflakes in a storm.
const DROPS: f32 = 420.0;
const FLAKES: f32 = 320.0;
/// Rain is drawn in the cool grey of a wash of ink, snow in paper white.
const RAIN: Color = Color::srgba(0.3, 0.38, 0.5, 0.5);
const SNOW: Color = Color::srgba(0.98, 0.98, 1.0, 0.9);

/// Draws this frame's weather over the board: none on Low, a quarter of it under reduced motion.
pub(super) fn fall(
    level: Res<Level>,
    settings: Res<Settings>,
    time: Res<Time>,
    mut gizmos: Gizmos,
) {
    let weather = weather(level.world, level.event_on);
    if weather == Weather::Clear || !settings.shadows {
        return;
    }
    let grid = level.terrain.grid();
    let span = Vec2::new(grid.width as f32, grid.height as f32) + 4.0;
    let calm = if settings.reduced_motion { 0.25 } else { 1.0 };
    let t = time.elapsed_secs();
    // Where drop `k` falls, and when in its fall it is, the same every time.
    let spot = |k: usize| {
        let h = mix(k as u32 ^ 0x2545);
        let at = Vec2::new((h & 1023) as f32, (h >> 10 & 1023) as f32) / 1023.0 * span - 2.0;
        (at, (h >> 20 & 1023) as f32 / 1023.0)
    };
    match weather {
        Weather::Rain => {
            for (at, phase) in (0..(DROPS * calm) as usize).map(spot) {
                let y = 3.0 - (t * 1.6 + phase).fract() * 2.8;
                let top = Vec3::new(at.x, y, at.y);
                gizmos.line(top, top + Vec3::new(WIND.x * 0.12, -0.35, WIND.y * 0.12), RAIN);
            }
            ripples(&mut gizmos, grid, t);
        }
        Weather::Snow(amount) => {
            for (at, phase) in (0..(FLAKES * amount * calm) as usize).map(spot) {
                let fall = (t * 0.22 + phase).fract();
                let drift = WIND * fall * 1.2 + Vec2::splat((t * 1.3 + phase * 9.0).sin() * 0.08);
                let flake = Vec3::new(at.x + drift.x, 3.0 - fall * 2.8, at.y + drift.y);
                gizmos.line(flake - Vec3::X * 0.035, flake + Vec3::X * 0.035, SNOW);
                gizmos.line(flake - Vec3::Z * 0.035, flake + Vec3::Z * 0.035, SNOW);
            }
        }
        Weather::Clear => {}
    }
}

/// Rings spreading on the water where the rain falls: one at a time on each water cell, each in its
/// own time.
fn ripples(gizmos: &mut Gizmos, grid: &Grid, t: f32) {
    for i in (0..grid.tiles.len()).filter(|&i| matches!(grid.tiles[i], Tile::Eau | Tile::Gue)) {
        let h = mix(i as u32 ^ 0x51ed);
        let spread = (t * 0.7 + (h & 255) as f32 / 255.0).fract();
        let shift = Vec2::new((h >> 8 & 255) as f32, (h >> 16 & 255) as f32) / 255.0 - 0.5;
        let (x, z) = grid.coords(i);
        let at = Vec3::new(
            x as f32 + shift.x * 0.6,
            super::look(grid.tiles[i]).height + 0.01,
            z as f32 + shift.y * 0.6,
        );
        let ring = WATER.lighter(0.25).with_alpha(0.7 * (1.0 - spread));
        gizmos.circle(
            Isometry3d::new(at, Quat::from_rotation_x(FRAC_PI_2)),
            0.04 + spread * 0.22,
            ring,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn la_crue_brings_rain() {
        assert_eq!(weather(0, true), Weather::Rain);
        assert_eq!(weather(0, false), Weather::Clear, "the Marches are dry between floods");
    }

    #[test]
    fn the_cols_have_snow_and_le_gel_a_storm() {
        let (Weather::Snow(drift), Weather::Snow(storm)) = (weather(1, false), weather(1, true))
        else {
            panic!("snow in the Cols");
        };
        assert!(drift > 0.0 && drift < storm, "{drift} {storm}");
    }
}
