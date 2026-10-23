//! The terrain: its grid, who walks it and how, and its blocks on screen.

pub mod flow;
pub mod grid;
pub mod species;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::Step;
use crate::level::Level;
use crate::menu::Screen;
use grid::{Grid, Tile};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (hover.in_set(Step::Input), redraw_cells.in_set(Step::Render)))
        // No stale pencil line behind the menus.
        .add_systems(OnExit(Screen::Playing), |mut level: ResMut<Level>| level.hovered = None);
}

/// Center of cell `i`, just above the ground.
pub fn cell_pos(grid: &Grid, i: usize) -> Vec3 {
    let (x, z) = grid.coords(i);
    Vec3::new(x as f32, 0.45, z as f32)
}

/// A terrain block.
#[derive(Component)]
struct Cell;

/// Cell under the mouse cursor.
pub fn hover(
    mut level: ResMut<Level>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = *camera;
    let grid = &level.grid;
    let hit = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .and_then(|ray| ray.plane_intersection_point(Vec3::Y * 0.2, InfinitePlane3d::new(Vec3::Y)));
    let cell = hit.and_then(|p| {
        let (x, z) = (p.x.round(), p.z.round());
        (x >= 0.0 && z >= 0.0 && (x as usize) < grid.width && (z as usize) < grid.height)
            .then(|| z as usize * grid.width + x as usize)
    });
    level.hovered = cell;
}

/// Rebuild every terrain block when the map changed (inking, La Crue, new level).
fn redraw_cells(
    mut level: ResMut<Level>,
    cells: Query<Entity, With<Cell>>,
    mut camera: Single<&mut Transform, With<Camera3d>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !level.redraw {
        return;
    }
    level.redraw = false;
    for entity in &cells {
        commands.entity(entity).despawn();
    }
    let grid = &level.grid;
    for i in 0..grid.tiles.len() {
        let (height, color) = match grid.tiles[i] {
            Tile::Sol => (0.2, Color::srgb(0.55, 0.62, 0.45)),
            Tile::SolBas => (0.15, Color::srgb(0.45, 0.55, 0.42)),
            Tile::Boue => (0.18, Color::srgb(0.45, 0.33, 0.2)),
            Tile::Ronces => (0.35, Color::srgb(0.2, 0.35, 0.15)),
            Tile::Eau => (0.05, Color::srgb(0.25, 0.45, 0.7)),
            Tile::Gue => (0.1, Color::srgb(0.45, 0.65, 0.8)),
            Tile::Barricade => (0.6, Color::srgb(0.4, 0.25, 0.12)),
            Tile::Roc => (0.8, Color::srgb(0.5, 0.5, 0.5)),
            Tile::Tour => (0.2, Color::srgb(0.7, 0.68, 0.6)),
            Tile::Chaussee => (0.21, Color::srgb(0.82, 0.8, 0.72)),
            Tile::Glace => (0.07, Color::srgb(0.8, 0.9, 0.97)),
        };
        let color = if grid.spawns.contains(&i) {
            Color::srgb(0.8, 0.2, 0.2)
        } else if grid.fuyard_spawns.contains(&i) {
            Color::srgb(0.95, 0.9, 0.75)
        } else if grid.goals.values().any(|&g| g == i) {
            Color::srgb(0.9, 0.75, 0.3)
        } else {
            color
        };
        let (x, z) = grid.coords(i);
        commands.spawn((
            Cell,
            Mesh3d(meshes.add(Cuboid::new(0.95, height, 0.95))),
            MeshMaterial3d(materials.add(color)),
            Transform::from_xyz(x as f32, height / 2.0, z as f32),
        ));
    }
    let center = Vec3::new((grid.width - 1) as f32 / 2.0, 0.0, (grid.height - 1) as f32 / 2.0);
    **camera = Transform::from_translation(center + Vec3::new(0.0, 13.0, 9.0)).looking_at(center, Vec3::Y);
}
