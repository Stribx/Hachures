//! The studio, off the board: each piece of the plan-relief and each species' figurine stands on a
//! little square of ground, on its own render layer, and a camera takes its picture into a
//! texture. The bar shows the tools' pictures; the Lexique shows them all.

mod species;
#[cfg(test)]
mod tests;
mod tools;

use bevy::camera::visibility::RenderLayers;
use bevy::camera::{RenderTarget, ScalingMode};
use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::prelude::*;
use bevy::render::render_resource::TextureFormat;
use bevy::world_serialization::WorldInstanceReady;

use super::bar::Bar;
use crate::look::{InkMaterial, PipelinesReady};
use crate::schedule::Screen;
use crate::theme::SUNLIGHT;
pub use species::SpeciesIcons;
pub use tools::ToolIcons;

/// What a studio camera films: a tool, whose picture the bar shows, or a species.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Reel {
    Tools,
    Species,
}

/// A camera of the studio.
#[derive(Component)]
struct StudioCamera(Reel);

/// A model in the studio, filmed once its scene is in.
#[derive(Component)]
struct StudioModel;

/// How many more frames the studio films, for the bar (the tools) and for the Lexique (all), and
/// whether every picture is taken already. Each studio camera costs a whole pass (about 1.5 ms
/// here, measured), so it films behind the splash, which waits for the models and the shaders;
/// else when a page that shows its picture comes up, long enough for the shaders to be ready, a
/// missed picture retaken the next time.
#[derive(Resource, Default)]
struct Filming {
    bar: u32,
    lexique: u32,
    taken: bool,
}

/// How long the studio films, in frames.
const FRAMES: u32 = 60;
/// Side of a picture, in pixels: the Lexique shows it large.
const SIZE: u32 = 256;
/// Where the studio stands, far under the world.
const STUDIO: Vec3 = Vec3::new(0.0, -500.0, 0.0);

pub fn plugin(app: &mut App) {
    app.init_resource::<Filming>()
        .add_systems(PreStartup, tools::setup)
        // After the figurines are made (`combat`, in `Startup`).
        .add_systems(PostStartup, species::setup)
        .add_systems(OnExit(Screen::Splash), taken)
        .add_systems(OnEnter(Screen::Lexique), film_everything)
        .add_systems(Update, film)
        .add_observer(into_layer);
}

/// A spot of the studio: its number, where it stands, its layer, and the picture it is filmed
/// into.
struct Spot {
    n: usize,
    at: Vec3,
    layer: RenderLayers,
    image: Handle<Image>,
}

/// Spot `n`, its picture blank until filmed.
fn stand(n: usize, images: &mut Assets<Image>) -> Spot {
    let image =
        images.add(Image::new_target_texture(SIZE, SIZE, TextureFormat::Rgba8UnormSrgb, None));
    Spot { n, at: STUDIO + Vec3::X * 10.0 * n as f32, layer: RenderLayers::layer(n + 1), image }
}

/// The studio's own sun on spots `spots`, from where the board's comes, on their layers only.
fn sun(commands: &mut Commands, spots: std::ops::Range<usize>) {
    let layers: Vec<usize> = spots.map(|n| n + 1).collect();
    commands.spawn((
        DirectionalLight { color: SUNLIGHT, illuminance: 10_000.0, ..default() },
        Transform::from_xyz(-4.0, 9.0, -3.0).looking_at(Vec3::ZERO, Vec3::Y),
        RenderLayers::from_layers(&layers),
    ));
}

/// How a camera frames its piece: the side of the square it sees, and where it looks, from the
/// spot.
#[derive(Debug)]
struct Framing {
    side: f32,
    aim: Vec3,
}

/// The camera of `spot`, filming `reel` into its picture.
fn spawn_camera(commands: &mut Commands, spot: &Spot, reel: Reel, framing: Framing) {
    let (Spot { n, at, layer, image }, Framing { side, aim }) = (spot, framing);
    commands.spawn((
        StudioCamera(reel),
        Camera3d::default(),
        Camera {
            order: -1 - *n as isize,
            is_active: false,
            clear_color: ClearColorConfig::Custom(Color::NONE),
            ..default()
        },
        RenderTarget::from(image.clone()),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::Fixed { width: side, height: side },
            ..OrthographicProjection::default_3d()
        }),
        Tonemapping::None,
        // Without tonemapping, dithering has nothing to do.
        DebandDither::Disabled,
        // From where the player sees the board.
        Transform::from_translation(*at + Vec3::new(0.0, 13.0, 9.0).normalize() * 10.0)
            .looking_at(*at + aim, Vec3::Y),
        layer.clone(),
    ));
}

/// The square of ground the piece of `spot` stands on, in `ground` (a tool's: a figurine stands on
/// the page itself).
fn spawn_ground(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    ground: Handle<InkMaterial>,
    spot: &Spot,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 0.2, 1.0))),
        MeshMaterial3d(ground),
        Transform::from_translation(spot.at - Vec3::Y * 0.1),
        spot.layer.clone(),
    ));
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

fn film_everything(mut filming: ResMut<Filming>) {
    if !filming.taken {
        filming.lexique = FRAMES;
    }
}

/// The splash is over: if every model was in and every shader compiled, its last frames took
/// every picture, for good. (A harness run skips the splash; a loading that hung cut it short.)
fn taken(
    models: Query<Has<Children>, With<StudioModel>>,
    ready: Res<PipelinesReady>,
    mut filming: ResMut<Filming>,
) {
    filming.taken = ready.0 && models.iter().all(|in_scene| in_scene);
}

/// Behind the splash, film everything. The bar is up: film the tools for a second's worth of
/// frames, then rest. The Lexique films every camera (`film_everything`).
fn film(
    screen: Res<State<Screen>>,
    bar: Query<(), Added<Bar>>,
    mut filming: ResMut<Filming>,
    mut cameras: Query<(&StudioCamera, &mut Camera)>,
) {
    if !bar.is_empty() && !filming.taken {
        filming.bar = FRAMES;
    }
    let species = filming.lexique > 0 || *screen.get() == Screen::Splash;
    let tools = species || filming.bar > 0;
    filming.bar = filming.bar.saturating_sub(1);
    filming.lexique = filming.lexique.saturating_sub(1);
    for (StudioCamera(reel), mut camera) in &mut cameras {
        let on = match reel {
            Reel::Tools => tools,
            Reel::Species => species,
        };
        if camera.is_active != on {
            camera.is_active = on;
        }
    }
}
