//! The pale preview of the selected tool under the cursor.

use std::collections::HashMap;

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use super::{Act, Selection, TOOLS, Verdict};
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

/// What the ghost keeps from frame to frame: its entity, the piece it shows, and the meshes made so far.
#[derive(Default)]
pub(super) struct Held {
    ghost: Option<Entity>,
    shown: Option<(Tile, u8)>,
    made: HashMap<(Tile, u8), Handle<Mesh>>,
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
    verdict: Res<Verdict>,
    mut held: Local<Held>,
    mut commands: Commands,
    mut stock: Stock,
) {
    let grid = level.terrain.grid();
    // Only a tile has a ghost: a tower's range is drawn by `combat`, a scrape the route once the cell is plain.
    let tile = if let Act::Lay(tile) = TOOLS[selection.tool].act { Some(tile) } else { None };
    let at = verdict.at.zip(tile).filter(|_| verdict.refusal.is_none());
    let entity = *held.ghost.get_or_insert_with(|| {
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
    if held.shown == Some((tile, shape)) {
        return;
    }
    held.shown = Some((tile, shape));
    let mesh = held.made.entry((tile, shape)).or_insert_with(|| {
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
