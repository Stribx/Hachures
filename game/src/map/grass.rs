//! The growth on the board: one mesh of felt tufts, wildflowers and wheat that sways in the wind,
//! grown again with the board.

use bevy::prelude::*;

use super::biome::Biome;
use super::cells::Cell;
use super::fields::ground;
use super::growth::{Crop, Sprig, sprigs};
use super::props::merge;
use crate::level::Level;
use crate::look::{InkMaterial, fine, weighed};
use crate::settings::Settings;
use crate::theme::{HEDGE, WHEAT, wheat_ear};

/// The growth now on the board.
#[derive(Component)]
pub(super) struct Growth;

/// How far a tip leans in the wind (`Hachure::wind`): a few hundredths of a cell.
const GRASS_WIND: f32 = 0.05;
/// The tallest thing that grows, for the sway's weight: wheat, which stays well under a barricade.
const TALLEST: f32 = 0.24;
/// The alpine flowers' colours: gentian, edelweiss, alpenrose, arnica.
const ALPINE_BLOOMS: [Color; 4] = [
    Color::srgb(0.22, 0.36, 0.82),
    Color::srgb(0.95, 0.95, 0.9),
    Color::srgb(0.86, 0.42, 0.56),
    Color::srgb(0.96, 0.7, 0.22),
];
const KARST_BLOOMS: [Color; 4] = [
    Color::srgb(0.52, 0.58, 0.48),
    Color::srgb(0.78, 0.72, 0.52),
    Color::srgb(0.42, 0.48, 0.52),
    Color::srgb(0.7, 0.64, 0.58),
];
/// The wildflowers' colours: poppy, cornflower, ox-eye daisy, buttercup.
const BLOOMS: [Color; 4] = [
    Color::srgb(0.82, 0.26, 0.2),
    Color::srgb(0.36, 0.47, 0.82),
    Color::srgb(0.96, 0.94, 0.86),
    Color::srgb(0.95, 0.8, 0.28),
];

/// The board was redrawn, or the settings changed: the growth is grown again, as much as the
/// machine allows. Low, the one preset without shadows, grows a third; reduced motion stills the
/// wind.
pub(super) fn grow(
    redrawn: Query<(), Added<Cell>>,
    old: Query<Entity, With<Growth>>,
    level: Res<Level>,
    settings: Res<Settings>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<InkMaterial>>,
) {
    if redrawn.is_empty() && !settings.is_changed() {
        return;
    }
    for entity in &old {
        commands.entity(entity).despawn();
    }
    let grid = level.terrain.grid();
    let density = if settings.shadows { 1.0 } else { 0.35 };
    let washes = ground(grid);
    let biome = Biome::of(level.world);
    let pieces = sprigs(grid, density, biome).into_iter().flat_map(|sprig| {
        let (x, z) = grid.coords(sprig.cell);
        let foot = Vec3::new(
            x as f32 + sprig.at.x,
            super::look(grid.tiles[sprig.cell]).height,
            z as f32 + sprig.at.y,
        );
        sprig_parts(&sprig, washes[sprig.cell], biome)
            .into_iter()
            .map(move |part| part.translated_by(foot))
    });
    let Some(mesh) = merge(pieces) else { return };
    let mut felt = fine(Color::WHITE);
    felt.extension.uniform.wind = if settings.reduced_motion { 0.0 } else { GRASS_WIND };
    commands.spawn((Growth, Mesh3d(meshes.add(mesh)), MeshMaterial3d(materials.add(felt))));
}

/// One sprig's parts, standing at the origin, coloured, each vertex weighted for the wind by its
/// height.
fn sprig_parts(sprig: &Sprig, wash: Color, biome: Biome) -> Vec<Mesh> {
    let roll = sprig.roll;
    let turn = (roll >> 16 & 255) as f32 / 255.0 * std::f32::consts::TAU;
    let grass = wash.mix(&HEDGE, 0.45);
    let stalk = |h: f32, w: f32, tilt: f32, color: Color| {
        part(Vec3::new(w, h, w * 0.5), h / 2.0, turn, tilt, color)
    };
    match sprig.crop {
        // A tuft: five blades splayed.
        Crop::Grass => (0..5)
            .map(|k| {
                let h = 0.09 + (roll >> (k * 3) & 7) as f32 * 0.009;
                part(Vec3::new(0.022, h, 0.008), h / 2.0, turn + k as f32 * 1.26, 0.4, grass)
            })
            .collect(),
        Crop::Flowers => {
            let h = 0.07 + (roll >> 3 & 7) as f32 * 0.006;
            let blooms = match biome {
                Biome::Cols => &ALPINE_BLOOMS,
                Biome::Karst => &KARST_BLOOMS,
                Biome::Bocage => &BLOOMS,
                Biome::Desert => &KARST_BLOOMS,
            };
            let bloom = blooms[(roll >> 24) as usize % blooms.len()];
            vec![stalk(h, 0.014, 0.1, grass), part(Vec3::splat(0.042), h, turn, 0.1, bloom)]
        }
        Crop::Wheat => {
            let h = 0.17 + (roll >> 3 & 7) as f32 * 0.008;
            let ear = wheat_ear();
            vec![
                stalk(h, 0.018, 0.05, WHEAT),
                part(Vec3::new(0.034, 0.07, 0.034), h, turn, 0.05, ear),
            ]
        }
    }
}

/// A box `size`, its centre `up` above the foot, turned and tilted, coloured, its vertices' alpha
/// the share of the wind they take (0 at the ground).
fn part(size: Vec3, up: f32, turn: f32, tilt: f32, color: Color) -> Mesh {
    let mesh = Mesh::from(Cuboid::from_size(size))
        .translated_by(Vec3::Y * up)
        .rotated_by(Quat::from_rotation_y(turn) * Quat::from_rotation_x(tilt));
    weighed(mesh, color, TALLEST)
}
