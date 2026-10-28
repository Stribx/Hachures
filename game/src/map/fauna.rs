//! The board's small life: butterflies over the wildflowers, and now and then a flock of birds
//! crossing the sky.

use bevy::prelude::*;

use super::biome::Biome;
use super::cells::Cell;
use super::fields::mix;
use super::grid::Grid;
use super::growth::{Crop, sprigs};
use crate::level::Level;
use crate::look::{Hachure, InkMaterial};
use crate::schedule::Screen;
use crate::settings::Settings;
use crate::theme::WORLD_INK;

/// At most this many butterflies on a board.
const BUTTERFLIES: usize = 6;
/// The colours of their wings: brimstone, cabbage white, orange tip.
const WINGS: [Color; 3] =
    [Color::srgb(0.95, 0.88, 0.4), Color::srgb(0.96, 0.95, 0.9), Color::srgb(0.95, 0.6, 0.3)];

/// Where the butterflies flit: over cells in flower, a few of them spread over the board, the same
/// each time.
pub fn butterfly_homes(grid: &Grid, biome: Biome) -> Vec<Vec3> {
    let mut cells: Vec<usize> = sprigs(grid, 1.0, biome)
        .into_iter()
        .filter(|s| s.crop == Crop::Flowers)
        .map(|s| s.cell)
        .collect();
    cells.dedup();
    cells.sort_by_key(|&c| mix(c as u32 ^ 0xb177));
    cells.truncate(BUTTERFLIES);
    cells
        .into_iter()
        .map(|c| {
            let (x, z) = grid.coords(c);
            Vec3::new(x as f32, super::look(grid.tiles[c]).height + 0.25, z as f32)
        })
        .collect()
}

/// A butterfly, flitting around `home`, in its own time.
#[derive(Component)]
pub(super) struct Butterfly {
    home: Vec3,
    phase: f32,
}

/// A wing, on the `side` of its body (-1 or 1), beating `speed` times a second or so.
#[derive(Component)]
pub(super) struct Wing {
    side: f32,
    speed: f32,
}

/// A flock of birds crossing the sky at `velocity`, `left` seconds before it is gone.
#[derive(Component)]
pub(super) struct Flock {
    velocity: Vec3,
    left: f32,
}

/// The pieces a creature is made of: its wing's mesh and its colour's ink.
fn wings(
    commands: &mut ChildSpawnerCommands,
    mesh: &Handle<Mesh>,
    material: &Handle<InkMaterial>,
    span: f32,
    speed: f32,
) {
    for side in [-1.0, 1.0] {
        commands.spawn((
            Wing { side, speed },
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material.clone()),
            Transform::from_xyz(side * span, 0.0, 0.0),
            Pickable::IGNORE,
        ));
    }
}

fn felt(color: Color) -> InkMaterial {
    let mut material = crate::look::ink(color);
    material.extension = Hachure::fine();
    material
}

/// The board was redrawn or the settings changed: its butterflies are set out again over its
/// flowers, none on Low (the one preset without shadows).
pub(super) fn settle(
    redrawn: Query<(), Added<Cell>>,
    old: Query<Entity, With<Butterfly>>,
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
    if !settings.shadows {
        return;
    }
    let wing = meshes.add(Cuboid::new(0.1, 0.006, 0.09));
    let biome = Biome::of(level.world);
    for (k, home) in butterfly_homes(level.terrain.grid(), biome).into_iter().enumerate() {
        let colour = materials.add(felt(WINGS[k % WINGS.len()]));
        commands
            .spawn((
                Butterfly { home, phase: k as f32 * 1.7 },
                Transform::from_translation(home),
                Visibility::default(),
            ))
            .with_children(|body| wings(body, &wing, &colour, 0.05, 18.0));
    }
}

/// Butterflies wander about their flowers; every wing beats.
pub(super) fn flutter(
    time: Res<Time>,
    screen: Res<State<Screen>>,
    mut butterflies: Query<(&Butterfly, &mut Transform), Without<Wing>>,
    mut wings: Query<(&Wing, &mut Transform), Without<Butterfly>>,
) {
    if *screen.get() == Screen::Paused {
        return;
    }
    let t = time.elapsed_secs();
    for (b, mut transform) in &mut butterflies {
        let (p, s) = (b.phase, t);
        let wander = Vec3::new(
            (s * 0.9 + p).sin() * 0.35 + (s * 2.3 + p).sin() * 0.1,
            (s * 3.1 + p).sin() * 0.06,
            (s * 0.7 + p * 1.3).cos() * 0.3,
        );
        let ahead = Vec3::new((s * 0.9 + p).cos(), 0.0, -(s * 0.7 + p * 1.3).sin());
        transform.translation = b.home + wander;
        transform.rotation = Transform::IDENTITY.looking_to(ahead, Vec3::Y).rotation;
    }
    for (wing, mut transform) in &mut wings {
        transform.rotation = Quat::from_rotation_z(wing.side * (t * wing.speed).sin() * 0.9);
    }
}

/// Every 20 to 40 seconds a flock of three to seven birds sets off across the sky over the board,
/// in a V, with the wind; none on Low.
pub(super) fn launch(
    time: Res<Time>,
    mut next: Local<f32>,
    level: Res<Level>,
    settings: Res<Settings>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<InkMaterial>>,
) {
    *next -= time.delta_secs();
    if *next > 0.0 {
        return;
    }
    let roll = mix(time.elapsed_secs().to_bits());
    *next = 20.0 + (roll & 255) as f32 / 255.0 * 20.0;
    if !settings.shadows {
        return;
    }
    let grid = level.terrain.grid();
    let centre = Vec3::new(grid.width as f32 / 2.0, 4.5, grid.height as f32 / 2.0);
    let heading = (roll >> 8 & 255) as f32 / 255.0 - 0.5 + 0.35f32.atan2(0.94);
    let way = Vec3::new(heading.cos(), 0.0, heading.sin());
    let across = Vec3::new(-way.z, 0.0, way.x) * ((roll >> 16 & 255) as f32 / 255.0 - 0.5) * 8.0;
    let (wing, ink) = (meshes.add(Cuboid::new(0.16, 0.012, 0.05)), materials.add(felt(WORLD_INK)));
    let flock = commands
        .spawn((
            Flock { velocity: way * 2.2, left: 40.0 / 2.2 },
            Transform::from_translation(centre - way * 20.0 + across).looking_to(way, Vec3::Y),
            Visibility::default(),
        ))
        .id();
    for i in 0..3 + (roll >> 24) as usize % 5 {
        let (rank, side) = (i.div_ceil(2), if i % 2 == 0 { 1.0 } else { -1.0 });
        let at = Vec3::new(side * rank as f32 * 0.45, 0.0, rank as f32 * 0.5);
        commands
            .spawn((Transform::from_translation(at), Visibility::default(), ChildOf(flock)))
            .with_children(|bird| wings(bird, &wing, &ink, 0.08, 5.0 + i as f32 * 0.3));
    }
}

/// Flocks fly on and are gone once across.
pub(super) fn cross(
    time: Res<Time>,
    screen: Res<State<Screen>>,
    mut flocks: Query<(Entity, &mut Flock, &mut Transform)>,
    mut commands: Commands,
) {
    if *screen.get() == Screen::Paused {
        return;
    }
    let dt = time.delta_secs();
    for (entity, mut flock, mut transform) in &mut flocks {
        transform.translation += flock.velocity * dt;
        flock.left -= dt;
        if flock.left < 0.0 {
            commands.entity(entity).despawn();
        }
    }
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

    /// Butterflies keep to the flowers, and to a few of them.
    #[test]
    fn butterflies_keep_to_the_flowers() {
        let grid = Grid::parse(MAP);
        let flowers: Vec<usize> = sprigs(&grid, 1.0, Biome::Bocage)
            .into_iter()
            .filter(|s| s.crop == Crop::Flowers)
            .map(|s| s.cell)
            .collect();
        let homes = butterfly_homes(&grid, Biome::Bocage);
        assert!(!homes.is_empty() && homes.len() <= BUTTERFLIES, "{homes:?}");
        for home in homes {
            let cell = home.z.round() as usize * grid.width + home.x.round() as usize;
            assert!(flowers.contains(&cell), "cell {cell}");
        }
        assert_eq!(butterfly_homes(&grid, Biome::Bocage), butterfly_homes(&grid, Biome::Bocage));
    }

    #[test]
    fn a_board_without_flowers_has_no_butterflies() {
        assert!(butterfly_homes(&Grid::parse("S^^^M"), Biome::Bocage).is_empty());
    }
}
