//! The pale preview of the selected tool under the cursor.

use std::collections::HashMap;

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use super::{Act, Selection, TOOLS};
use crate::campaign::skills::Skill;
use crate::level::Level;
use crate::look::{Hachure, InkMaterial, ink};
use crate::map::grid::Tile;
use crate::map::look;
use crate::map::props::{barricade_shape, prop};

/// The assets the ghost's piece is made of.
#[derive(SystemParam)]
pub(super) struct Stock<'w> {
    meshes: ResMut<'w, Assets<Mesh>>,
    materials: ResMut<'w, Assets<InkMaterial>>,
}

/// The piece the hand holds, pale, where it would stand: shown on any cell it may be inked on.
///
/// One ghost that stays, moved from cell to cell, its mesh made once per tool and shape: a mesh just made is drawn
/// a frame or more later, so a ghost made anew on every cell blinked out as the mouse moved. And one mesh, its
/// parts' colours on its vertices: as separate see-through meshes, the wood and the string (centred alike) swapped
/// their drawing order from frame to frame, and the ghost trembled.
pub(super) fn ghost(
    level: Res<Level>,
    selection: Res<Selection>,
    mut ghost: Local<Option<Entity>>,
    mut shown: Local<Option<(Tile, u8)>>,
    mut made: Local<HashMap<(Tile, u8), Handle<Mesh>>>,
    mut commands: Commands,
    mut stock: Stock,
) {
    let grid = &level.grid;
    // Only a tile has a ghost: a tower shows its range, a scrape the route it opens.
    let tile = if let Act::Lay(tile) = TOOLS[selection.tool].act { Some(tile) } else { None };
    let at =
        selection.hovered.zip(tile).filter(|&(i, _)| !level.finished && level.refusal(i, selection.tool).is_none());
    let entity = *ghost.get_or_insert_with(|| {
        let mut pale = ink(Color::WHITE.with_alpha(0.45));
        pale.base.alpha_mode = AlphaMode::Blend;
        pale.extension = Hachure::fine();
        commands.spawn((MeshMaterial3d(stock.materials.add(pale)), Transform::default(), Visibility::Hidden)).id()
    });
    let Some((i, tile)) = at else {
        commands.entity(entity).insert(Visibility::Hidden);
        return;
    };
    let shape = if tile == Tile::Barricade { barricade_shape(grid, i, level.has(Skill::Fascines)) } else { 0 };
    let (x, z) = grid.coords(i);
    let height = if tile == Tile::Boue { look(Tile::Boue).0 } else { look(Tile::Sol).0 };
    commands.entity(entity).insert((Transform::from_xyz(x as f32, height, z as f32), Visibility::Inherited));
    if *shown == Some((tile, shape)) {
        return;
    }
    *shown = Some((tile, shape));
    let mesh = made.entry((tile, shape)).or_insert_with(|| {
        let parts = prop(tile, shape).into_iter().map(|(mesh, color)| {
            let n = mesh.count_vertices();
            mesh.with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, vec![color.to_linear().to_f32_array(); n])
        });
        let whole = parts.reduce(|mut all, part| {
            all.merge(&part).expect("the parts share their attributes");
            all
        });
        stock.meshes.add(whole.expect("a tool's piece has parts"))
    });
    commands.entity(entity).insert(Mesh3d(mesh.clone()));
}
