use std::f32::consts::FRAC_PI_2;

use bevy::state::app::StatesPlugin;

use super::*;
use crate::combat::figure;
use crate::level::Level;
use crate::look::{InkMaterial, PipelinesReady};

fn app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, StatesPlugin, AssetPlugin::default()))
        .init_asset::<Image>()
        .init_asset::<Mesh>()
        .init_asset::<InkMaterial>()
        .insert_state(Screen::Title)
        .init_resource::<PipelinesReady>()
        .insert_resource(Level::parse("S.M"))
        .init_resource::<Filming>()
        .add_systems(Startup, figure::setup)
        .add_systems(PostStartup, super::species::setup)
        .add_systems(OnEnter(Screen::Lexique), film_everything)
        .add_systems(Update, film);
    app.update();
    app
}

/// Whether each studio camera of `reel` is filming.
fn filming(app: &mut App, reel: Reel) -> Vec<bool> {
    let world = app.world_mut();
    let mut cameras = world.query::<(&StudioCamera, &Camera)>();
    cameras.iter(world).filter(|(c, _)| c.0 == reel).map(|(_, c)| c.is_active).collect()
}

fn with_a_tool_camera(app: &mut App) {
    app.world_mut().spawn((StudioCamera(Reel::Tools), Camera { is_active: false, ..default() }));
}

#[test]
fn there_is_one_species_picture_per_species() {
    let mut app = app();
    let species = app.world().resource::<Level>().species.len();
    assert!(species > 0);
    assert_eq!(app.world().resource::<SpeciesIcons>().0.len(), species);
    assert_eq!(filming(&mut app, Reel::Species).len(), species);
}

#[test]
fn the_bar_films_the_tools_only() {
    let mut app = app();
    with_a_tool_camera(&mut app);
    app.world_mut().spawn(Bar);
    app.update();
    assert_eq!(filming(&mut app, Reel::Tools), [true]);
    assert!(filming(&mut app, Reel::Species).iter().all(|on| !on));
}

#[test]
fn entering_the_lexique_films_every_camera_for_a_while() {
    let mut app = app();
    with_a_tool_camera(&mut app);
    app.world_mut().resource_mut::<NextState<Screen>>().set(Screen::Lexique);
    app.update();
    assert_eq!(filming(&mut app, Reel::Tools), [true]);
    assert!(filming(&mut app, Reel::Species).iter().all(|&on| on));
    for _ in 0..FRAMES {
        app.update();
    }
    assert!(filming(&mut app, Reel::Species).iter().all(|on| !on));
    assert_eq!(filming(&mut app, Reel::Tools), [false]);
}

#[test]
fn a_figure_is_framed_whole_around_its_middle() {
    let mut meshes = Assets::<Mesh>::default();
    let cube = meshes.add(Cuboid::from_length(1.0));
    let raised = Transform::from_xyz(0.0, 0.5, 0.0);
    let framing = super::species::framing([(&cube, raised)], &meshes, Quat::IDENTITY);
    assert!((framing.aim - Vec3::Y * 0.5).length() < 1e-5, "{:?}", framing.aim);
    // A unit cube seen from any side fits in its diagonal, and not much less.
    assert!((3f32.sqrt()..=3f32.sqrt() * 1.2).contains(&framing.side), "{}", framing.side);
    let turn = Quat::from_rotation_y(FRAC_PI_2);
    let aside =
        super::species::framing([(&cube, Transform::from_xyz(1.0, 0.0, 0.0))], &meshes, turn);
    assert!((aside.aim - turn * Vec3::X).length() < 1e-5, "{:?}", aside.aim);
}

/// Behind the splash every camera films; once the splash has taken every picture, the bar and
/// the Lexique film no more.
#[test]
fn the_splash_takes_the_pictures_for_good() {
    let mut app = app();
    with_a_tool_camera(&mut app);
    app.add_systems(OnExit(Screen::Splash), taken);
    app.world_mut().resource_mut::<NextState<Screen>>().set(Screen::Splash);
    app.update();
    assert_eq!(filming(&mut app, Reel::Tools), [true]);
    assert!(filming(&mut app, Reel::Species).iter().all(|&on| on));
    app.world_mut().resource_mut::<PipelinesReady>().0 = true;
    app.world_mut().resource_mut::<NextState<Screen>>().set(Screen::Title);
    app.update();
    app.world_mut().spawn(Bar);
    app.world_mut().resource_mut::<NextState<Screen>>().set(Screen::Lexique);
    app.update();
    assert_eq!(filming(&mut app, Reel::Tools), [false]);
    assert!(filming(&mut app, Reel::Species).iter().all(|on| !on));
}
