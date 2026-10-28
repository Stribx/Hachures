//! The terrain blocks: the cells, the water lines laid under them and the model's base.

use std::collections::HashMap;

use bevy::prelude::*;

use super::decor::palisade_shape;
use super::fields::ground;
use super::grid::{Grid, Tile};
use super::look;
use super::props::{Props, barricade_shape, prop_parts, spawn_prop};
use super::seams::spawn_seams;
use crate::campaign::skills::Skill;
use crate::level::Level;
use crate::look::{InkMaterial, ink, shore_image, varnished, water};
use crate::theme::WOOD;

/// A terrain block.
#[derive(Component)]
pub(super) struct Cell;

/// Block meshes and materials made so far, by height bits and colour.
type Made = HashMap<(u32, [u8; 4]), (Handle<Mesh>, Handle<InkMaterial>)>;

fn is_water(tile: Tile) -> bool {
    matches!(tile, Tile::Eau | Tile::Gue | Tile::Glace)
}

/// A strip of land the water runs under, for the water lines: water on both sides along one axis, land or the
/// board's edge along the other (a bridge, a causeway). A one-cell island, water all round, is not one: the lines
/// circle it.
fn crossing(grid: &Grid, i: usize) -> bool {
    let (x, z) = grid.coords(i);
    let wet = |dx: isize, dz: isize| {
        let (Some(nx), Some(nz)) = (x.checked_add_signed(dx), z.checked_add_signed(dz)) else { return false };
        nx < grid.width && nz < grid.height && is_water(grid.tiles[nz * grid.width + nx])
    };
    !is_water(grid.tiles[i])
        && ((wet(-1, 0) && wet(1, 0) && !wet(0, -1) && !wet(0, 1))
            || (wet(0, -1) && wet(0, 1) && !wet(-1, 0) && !wet(1, 0)))
}

/// What `redraw_cells` keeps from one board to the next: the shared meshes and materials, the water lines' image
/// and the tiles last drawn.
#[derive(Default)]
pub(super) struct Drawn {
    made: Made,
    props: Props,
    shore: Handle<Image>,
    before: Vec<Tile>,
}

/// Rebuild every terrain block when the map changed (inking, La Crue, new level). Meshes and materials are made
/// once per height and colour, and shared. A tool stands on plain ground and says what it does by its prop; a prop
/// just inked by hand lands in its cell (`Land`).
pub(super) fn redraw_cells(
    mut level: ResMut<Level>,
    cells: Query<Entity, With<Cell>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<InkMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut drawn: Local<Drawn>,
) {
    if !level.terrain.take_redraw() {
        return;
    }
    for entity in &cells {
        commands.entity(entity).despawn();
    }
    let fascines = level.has(Skill::Fascines);
    let grid = level.terrain.grid();
    let drawn = &mut *drawn;
    // Inked by hand: a cell or so changed. A new level, or the Crue, changes many and lands nothing.
    let by_hand = drawn.before.len() == grid.tiles.len()
        && grid.tiles.iter().zip(drawn.before.iter()).filter(|(a, b)| a != b).count() <= 3;
    let rect = lay_water_lines(grid, drawn, &mut images, &mut materials);
    for (i, color) in ground(grid).into_iter().enumerate() {
        let tile = grid.tiles[i];
        let (height, mesh, material) = block(tile, color, rect, drawn, &mut meshes, &mut materials);
        let (x, z) = grid.coords(i);
        let cell = commands
            .spawn((
                Cell,
                Mesh3d(mesh),
                MeshMaterial3d(material),
                Transform::from_xyz(x as f32, height / 2.0, z as f32),
            ))
            .id();
        let shape = match tile {
            Tile::Barricade => barricade_shape(grid, i, fascines),
            Tile::Palissade => palisade_shape(grid, i),
            _ => 0,
        };
        let parts = prop_parts(&mut drawn.props, tile, shape, &mut meshes, &mut materials);
        let new = by_hand && drawn.before[i] != tile;
        spawn_prop(&mut commands, cell, i, tile, height, parts, new);
    }
    drawn.before = grid.tiles.clone();
    spawn_seams(&mut commands, grid, &mut meshes, &mut materials);
    spawn_base(&mut commands, grid, &mut meshes, &mut materials);
}

/// How far each point of the board lies from the bank, for the water lines, which run on under the bridges. Water
/// made for another board has its lines laid over this one. Returns where the lines lie (`Hachure::shore`).
fn lay_water_lines(
    grid: &Grid,
    drawn: &mut Drawn,
    images: &mut Assets<Image>,
    materials: &mut Assets<InkMaterial>,
) -> Vec4 {
    let wet: Vec<bool> = (0..grid.tiles.len()).map(|i| is_water(grid.tiles[i]) || crossing(grid, i)).collect();
    let (image, rect) = shore_image(Vec2::splat(-0.5), Vec2::new(grid.width as f32, grid.height as f32), 8.0, |p| {
        let (x, z) = ((p.x + 0.5) as usize, (p.y + 0.5) as usize);
        x < grid.width && z < grid.height && wet[z * grid.width + x]
    });
    if drawn.shore == Handle::default() {
        drawn.shore = images.add(image);
    } else {
        images.insert(drawn.shore.id(), image).ok();
    }
    for (_, material) in drawn.made.values() {
        // `get_mut` marks the material changed, so only when it does.
        if materials.get(material).is_some_and(|m| m.extension.shore.is_some() && m.extension.uniform.shore != rect) {
            materials.get_mut(material).expect("read just above").extension.uniform.shore = rect;
        }
    }
    rect
}

/// The block of a cell of `tile` on ground washed `color`: its height, and its mesh and material, made the first
/// time that height and colour are seen. A tool's piece stands on plain ground: the field's, or the pasture's.
fn block(
    tile: Tile,
    color: Color,
    rect: Vec4,
    drawn: &mut Drawn,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<InkMaterial>,
) -> (f32, Handle<Mesh>, Handle<InkMaterial>) {
    let (height, color) = match tile {
        Tile::Barricade | Tile::Ronces => (look(Tile::Sol).0, color),
        Tile::Eboulis | Tile::Palissade => look(Tile::Sol),
        _ => (look(tile).0, color),
    };
    let shore = &drawn.shore;
    let (mesh, material) = drawn
        .made
        .entry((height.to_bits(), color.to_srgba().to_u8_array()))
        .or_insert_with(|| {
            // Edge to edge: one continuous relief, not a grid of tiles. Water is the model's varnished resin.
            let paint = match tile {
                Tile::Eau | Tile::Gue => water(color, shore.clone(), rect, 1.0),
                Tile::Glace => water(color, shore.clone(), rect, 0.0),
                // Mud is a pool of brown varnish.
                Tile::Boue => varnished(color, 0.6),
                _ => ink(color),
            };
            (meshes.add(Cuboid::new(1.0, height, 1.0)), materials.add(paint))
        })
        .clone();
    (height, mesh, material)
}

/// The model's base: a slab of waxed walnut under the relief.
fn spawn_base(commands: &mut Commands, grid: &Grid, meshes: &mut Assets<Mesh>, materials: &mut Assets<InkMaterial>) {
    let center = Vec3::new((grid.width - 1) as f32 / 2.0, 0.0, (grid.height - 1) as f32 / 2.0);
    let (w, h) = (grid.width as f32, grid.height as f32);
    commands.spawn((
        Cell,
        Mesh3d(meshes.add(Cuboid::new(w + 0.2, 0.3, h + 0.2))),
        MeshMaterial3d(materials.add(varnished(WOOD, 0.5))),
        Transform::from_translation(center - Vec3::Y * 0.12),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn water_runs_under_a_bridge_not_round_an_island() {
        let grid = Grid::parse(
            "
            .....
            ~~.~~
            .....
            ~~~~~
            ~~.~~
            ~~~~~
            ",
        );
        // The river runs left to right under the bridge at (2, 1).
        assert!(crossing(&grid, 7));
        // The island at (2, 4), water all round.
        assert!(!crossing(&grid, 22));
        // A bank, and water itself.
        assert!(!crossing(&grid, 1));
        assert!(!crossing(&grid, 5));
    }
}
