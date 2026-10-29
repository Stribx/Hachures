//! The tools' pieces in the studio, so a tool's icon is always its piece, in its wood and string.

use bevy::prelude::*;

use super::{Framing, Reel, Spot, StudioModel, spawn_camera, spawn_ground, stand, sun};
use crate::combat::drill::sonde;
use crate::combat::stand::{BASE_WOOD, TOWER_BASE, WEAPON_WOOD, weapon_model};
use crate::combat::towers::Weapon;
use crate::ink::{Act, TOOLS, Tool};
use crate::look::{Hachure, InkMaterial, ink, varnished};
use crate::map;
use crate::map::grid::Tile;
use crate::map::props::prop;
use crate::map::scraper::scraper;

/// The picture of each of `TOOLS`, in order.
#[derive(Resource)]
pub struct ToolIcons(pub Vec<Handle<Image>>);

/// The tools stand on the studio's first spots, one each.
pub(super) fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<InkMaterial>>,
) {
    sun(&mut commands, 0..TOOLS.len());
    let mut icons = Vec::new();
    for (t, tool) in TOOLS.iter().enumerate() {
        let spot = stand(t, &mut images);
        // A tower stands taller than a piece of terrain: frame it wider, look higher.
        let framing = if tool.act.weapon().is_some() {
            Framing { side: 1.3, aim: Vec3::Y * 0.5 }
        } else {
            Framing { side: 1.05, aim: Vec3::Y * 0.12 }
        };
        spawn_camera(&mut commands, &spot, Reel::Tools, framing);
        // Mud is its own ground, a varnished pool.
        let ground = match tool.act {
            Act::Lay(Tile::Boue) => varnished(map::look(Tile::Boue).color, 0.6),
            _ => ink(map::look(Tile::Sol).color),
        };
        spawn_ground(&mut commands, &mut meshes, materials.add(ground), &spot);
        spawn_piece(&mut commands, &assets, &mut meshes, &mut materials, tool, &spot);
        icons.push(spot.image);
    }
    commands.insert_resource(ToolIcons(icons));
}

/// The piece itself: the terrain's props, or a tower's base and weapon.
fn spawn_piece(
    commands: &mut Commands,
    assets: &AssetServer,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<InkMaterial>,
    tool: &Tool,
    spot: &Spot,
) {
    let fine = |color: Color| {
        let mut paint = ink(color);
        paint.extension = Hachure::fine();
        paint
    };
    let (at, layer) = (spot.at, &spot.layer);
    let parts = match tool.act {
        Act::Lay(tile) => prop(tile, 0),
        // ponytail: Renommer and Biffer borrow Gratter's scraper; their own pieces (a quill
        // rewriting, one striking out) when the art is drawn.
        Act::Scrape | Act::Rename | Act::Biffer => scraper(),
        Act::Raise(Weapon::Sonde) => sonde(),
        Act::Raise(weapon) => {
            // Every other weapon has its model; the Sonde was built just above.
            let Some(path) = weapon_model(weapon) else { return };
            let scene = |path: &'static str| {
                WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset(path)))
            };
            commands.spawn((
                StudioModel,
                scene(TOWER_BASE),
                BASE_WOOD,
                Transform::from_translation(at),
                layer.clone(),
            ));
            commands.spawn((
                StudioModel,
                scene(path),
                WEAPON_WOOD,
                // Turned a little, so its shape shows rather than its front.
                Transform::from_translation(at + Vec3::Y * 0.6)
                    .with_rotation(Quat::from_rotation_y(0.6)),
                layer.clone(),
            ));
            return;
        }
    };
    for part in parts {
        commands.spawn((
            Mesh3d(meshes.add(part.mesh)),
            MeshMaterial3d(materials.add(fine(part.color))),
            Transform::from_translation(at),
            layer.clone(),
        ));
    }
}
