//! The tools' icons, filmed: each piece of the plan-relief stands on a little square of ground in a studio off the
//! board, on its own render layer, and a camera takes its picture into a texture. The bar shows the picture, so a
//! tool's icon is always its piece, in its wood and string.

use bevy::camera::visibility::RenderLayers;
use bevy::camera::{RenderTarget, ScalingMode};
use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::prelude::*;
use bevy::render::render_resource::TextureFormat;
use bevy::world_serialization::WorldInstanceReady;

use super::bar::Bar;
use crate::combat::towers::{BASE_WOOD, TOWER_BASE, WEAPON_WOOD, weapon_model};
use crate::ink::{Act, TOOLS, Tool};
use crate::look::{Hachure, InkMaterial, ink, varnished};
use crate::map::grid::Tile;
use crate::map::look;
use crate::map::props::prop;
use crate::map::scraper::scraper;
use crate::theme::SUNLIGHT;

/// The picture of each of `TOOLS`, in order.
#[derive(Resource)]
pub struct ToolIcons(pub Vec<Handle<Image>>);

/// A model in the studio, filmed once its scene is in.
#[derive(Component)]
struct StudioModel;

/// A camera of the studio.
#[derive(Component)]
struct StudioCamera;

/// How many more frames the studio films. Each studio camera costs a whole pass (about 1.5 ms here, measured), so
/// it films only when the bar comes up, long enough for the shaders to be ready; a missed picture is retaken with
/// the next level.
#[derive(Resource, Default)]
struct Filming(u32);

/// Side of an icon, in pixels.
const SIZE: u32 = 128;
/// Where the studio stands, far under the world.
const STUDIO: Vec3 = Vec3::new(0.0, -500.0, 0.0);

pub fn plugin(app: &mut App) {
    app.init_resource::<Filming>().add_systems(PreStartup, setup).add_systems(Update, film).add_observer(into_layer);
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<InkMaterial>>,
) {
    let layers: Vec<usize> = (1..=TOOLS.len()).collect();
    // The studio's own sun, from where the board's comes, on every studio layer and no other.
    commands.spawn((
        DirectionalLight { color: SUNLIGHT, illuminance: 10_000.0, ..default() },
        Transform::from_xyz(-4.0, 9.0, -3.0).looking_at(Vec3::ZERO, Vec3::Y),
        RenderLayers::from_layers(&layers),
    ));
    let mut icons = Vec::new();
    for (t, tool) in TOOLS.iter().enumerate() {
        let layer = RenderLayers::layer(t + 1);
        let at = STUDIO + Vec3::X * 10.0 * t as f32;
        let image = images.add(Image::new_target_texture(SIZE, SIZE, TextureFormat::Rgba8UnormSrgb, None));
        spawn_camera(&mut commands, &image, t, at, &layer);
        spawn_ground(&mut commands, &mut meshes, &mut materials, tool, at, &layer);
        spawn_piece(&mut commands, &assets, &mut meshes, &mut materials, tool, at, &layer);
        icons.push(image);
    }
    commands.insert_resource(ToolIcons(icons));
}

/// The camera of `TOOLS[t]`, filming into `image`.
fn spawn_camera(commands: &mut Commands, image: &Handle<Image>, t: usize, at: Vec3, layer: &RenderLayers) {
    // A tower stands taller than a piece of terrain: frame it wider, look higher.
    let (frame, aim) = if TOOLS[t].act.weapon().is_some() { (1.3, 0.5) } else { (1.05, 0.12) };
    commands.spawn((
        StudioCamera,
        Camera3d::default(),
        Camera {
            order: -1 - t as isize,
            is_active: false,
            clear_color: ClearColorConfig::Custom(Color::NONE),
            ..default()
        },
        RenderTarget::from(image.clone()),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::Fixed { width: frame, height: frame },
            ..OrthographicProjection::default_3d()
        }),
        Tonemapping::None,
        // Without tonemapping, dithering has nothing to do.
        DebandDither::Disabled,
        // From where the player sees the board.
        Transform::from_translation(at + Vec3::new(0.0, 13.0, 9.0).normalize() * 10.0)
            .looking_at(at + Vec3::Y * aim, Vec3::Y),
        layer.clone(),
    ));
}

/// The square of ground the piece stands on: mud is its own ground, a varnished pool.
fn spawn_ground(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<InkMaterial>,
    tool: &Tool,
    at: Vec3,
    layer: &RenderLayers,
) {
    let ground = match tool.act {
        Act::Lay(Tile::Boue) => varnished(look(Tile::Boue).1, 0.6),
        _ => ink(look(Tile::Sol).1),
    };
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 0.2, 1.0))),
        MeshMaterial3d(materials.add(ground)),
        Transform::from_translation(at - Vec3::Y * 0.1),
        layer.clone(),
    ));
}

/// The piece itself: the terrain's props, or a tower's base and weapon.
fn spawn_piece(
    commands: &mut Commands,
    assets: &AssetServer,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<InkMaterial>,
    tool: &Tool,
    at: Vec3,
    layer: &RenderLayers,
) {
    let fine = |color: Color| {
        let mut paint = ink(color);
        paint.extension = Hachure::fine();
        paint
    };
    let parts = match tool.act {
        Act::Lay(tile) => prop(tile, 0),
        Act::Scrape => scraper(),
        Act::Raise(weapon) => {
            let scene = |path: &'static str| WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset(path)));
            commands.spawn((StudioModel, scene(TOWER_BASE), BASE_WOOD, Transform::from_translation(at), layer.clone()));
            commands.spawn((
                StudioModel,
                scene(weapon_model(weapon)),
                WEAPON_WOOD,
                // Turned a little, so its shape shows rather than its front.
                Transform::from_translation(at + Vec3::Y * 0.6).with_rotation(Quat::from_rotation_y(0.6)),
                layer.clone(),
            ));
            return;
        }
    };
    for (mesh, color) in parts {
        commands.spawn((
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(materials.add(fine(color))),
            Transform::from_translation(at),
            layer.clone(),
        ));
    }
}

/// A model's parts are spawned with its scene, on the default layer: they join their root's.
fn into_layer(
    ready: On<WorldInstanceReady>,
    roots: Query<&RenderLayers, With<StudioModel>>,
    children: Query<&Children>,
    mut commands: Commands,
) {
    if let Ok(layer) = roots.get(ready.entity) {
        for part in children.iter_descendants(ready.entity) {
            commands.entity(part).insert(layer.clone());
        }
    }
}

/// The bar is up: film for a second's worth of frames, then rest.
fn film(bar: Query<(), Added<Bar>>, mut filming: ResMut<Filming>, mut cameras: Query<&mut Camera, With<StudioCamera>>) {
    if !bar.is_empty() {
        filming.0 = 60;
    }
    let on = filming.0 > 0;
    filming.0 = filming.0.saturating_sub(1);
    for mut camera in &mut cameras {
        if camera.is_active != on {
            camera.is_active = on;
        }
    }
}
