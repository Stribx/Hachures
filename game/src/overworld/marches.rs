//! The Marches' land, spawned once under `OverworldRoot`, and the shift that keeps the played sheet
//! at the origin.

use bevy::light::{NotShadowCaster, NotShadowReceiver};
use bevy::prelude::*;

use super::props::props;
use super::sheets::{SHEETS, sheet_mesh};
use super::terrain::{EAST, NORTH, SOUTH, WATER, WEST, height, land_mesh, road_mesh};
use super::{OverworldRoot, camera, pawn};
use crate::campaign::save::Save;
use crate::level::Level;
use crate::look::{InkMaterial, Swaying, ink, shore_image, water};

/// How far a tree leans in the wind, per world unit of height squared: a canopy a few units up
/// sways a few hundredths, its trunk not at all.
const TREE_WIND: f32 = 0.012;

/// The miniature of sheet `.0`, hidden while that sheet is the one on the board.
#[derive(Component)]
pub struct Miniature(pub usize);

pub(super) fn spawn(
    save: Res<Save>,
    assets: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<InkMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let mut land = ink(Color::WHITE);
    land.extension.uniform.params.x = 0.16;
    land.extension.uniform.paper.alpha = 0.9;
    let land = materials.add(land);
    // The same resin as the board's water, lined from the lakes' shores; seen from higher, the
    // lines spaced wider.
    let (min, size) = (Vec2::new(WEST, NORTH), Vec2::new(EAST - WEST, SOUTH - NORTH));
    let (shore, rect) = shore_image(min, size, 4.0, |p| height(p) < WATER);
    let mut lakes = water(crate::theme::WATER, images.add(shore), rect, 1.0);
    lakes.extension.uniform.params.x = 0.4;
    let water = materials.add(lakes);
    let root = commands
        .spawn((OverworldRoot, Transform::default(), Visibility::default()))
        .with_children(|root| {
            root.spawn((Mesh3d(meshes.add(land_mesh())), MeshMaterial3d(land.clone())));
            // Lying on the land, it would catch the land's own shadow as stripes.
            root.spawn((
                Mesh3d(meshes.add(road_mesh())),
                MeshMaterial3d(materials.add(ink(Color::srgb(0.86, 0.8, 0.66)))),
                NotShadowReceiver,
                NotShadowCaster,
            ));
            let (size, middle) = (
                Vec2::new(EAST - WEST, SOUTH - NORTH),
                Vec2::new(EAST + WEST, SOUTH + NORTH) / 2.0,
            );
            root.spawn((
                Mesh3d(meshes.add(Plane3d::new(Vec3::Y, size / 2.0))),
                MeshMaterial3d(water),
                Transform::from_xyz(middle.x, WATER, middle.y),
            ));
        })
        .id();

    let mut blank = ink(Color::WHITE);
    // A sheet not reached yet: blank paper, hatched all over.
    blank.extension.uniform.params.w = 1.0;
    let blank = materials.add(blank);
    for (index, sheet) in SHEETS.iter().enumerate() {
        let reached = index <= save.reached;
        let material = if reached { land.clone() } else { blank.clone() };
        commands.spawn((
            Miniature(index),
            Mesh3d(meshes.add(sheet_mesh(sheet, reached))),
            MeshMaterial3d(material),
            Transform::from_xyz(sheet.at.x, 0.0, sheet.at.y),
            ChildOf(root),
        ));
    }

    for (model, transform) in props() {
        let mut prop = commands.spawn((
            WorldAssetRoot(
                assets.load(GltfAssetLabel::Scene(0).from_asset(format!("models/{model}.glb"))),
            ),
            transform,
            ChildOf(root),
        ));
        if model.contains("tree") {
            prop.insert(Swaying(TREE_WIND));
        }
    }
}

/// A new level on the board: the world shifts so its sheet lies at the origin (the camera along
/// with it), and its miniature hides.
pub(super) fn follow_level(
    level: Res<Level>,
    mut shown: Local<Option<usize>>,
    mut root: Single<&mut Transform, With<OverworldRoot>>,
    mut miniatures: Query<(&Miniature, &mut Visibility)>,
    mut rig: ResMut<camera::CameraRig>,
    mut pawn: ResMut<pawn::Pawn>,
) {
    if *shown == Some(level.index) {
        return;
    }
    *shown = Some(level.index);
    let at = SHEETS[level.index].at;
    let shifted = Vec3::new(-at.x, 0.0, -at.y);
    // The camera moves with the world, so the swap can't be seen.
    rig.shift(shifted - root.translation);
    root.translation = shifted;
    // Nève stands on the sheet being played, however it was loaded.
    pawn.stand(level.index);
    for (miniature, mut visibility) in &mut miniatures {
        *visibility =
            if miniature.0 == level.index { Visibility::Hidden } else { Visibility::Inherited };
    }
}
