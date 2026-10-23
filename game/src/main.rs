//! Hachures: a cartographic tower defense. One folder per feature, each with its own plugin.

mod campaign;
mod combat;
mod ink;
mod level;
mod map;
mod menu;
mod ui;

use bevy::prelude::*;

use campaign::save::Save;
use level::Level;
use level::data::LEVELS;
use menu::{Music, Screen};

/// The three steps of a frame; every feature puts its systems in one of them.
#[derive(SystemSet, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Step {
    /// Keys and mouse.
    Input,
    /// Waves, enemies, towers.
    Simulate,
    /// Terrain blocks, gizmos and text.
    Render,
}

fn main() {
    let save = Save::load();
    let level = Level::load(save.reached.min(LEVELS.len() - 1), &save);
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Hachures".into(),
                // In the browser, fill the page.
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(steps)
        .insert_resource(level)
        .insert_resource(save)
        .add_plugins((map::plugin, ink::plugin, campaign::plugin, level::plugin, combat::plugin, ui::plugin, menu::plugin))
        .add_systems(Startup, setup)
        .run();
}

/// The frame's steps, in order, and when they run. Paused is simply: no input, no simulation; the map
/// is still drawn behind the menus. Once the Rôle is lost, the sheet stands still until Enter plays again.
fn steps(app: &mut App) {
    app.configure_sets(
        Update,
        (
            Step::Input.run_if(in_state(Screen::Playing)),
            Step::Simulate.run_if(in_state(Screen::Playing)).run_if(|level: Res<Level>| level.lives > 0),
            Step::Render,
        )
            .chain(),
    );
}

/// Camera, light and music.
fn setup(mut commands: Commands, assets: Res<AssetServer>, save: Res<Save>) {
    // Placed over the map by `map::redraw_cells`.
    commands.spawn(Camera3d::default());
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(3.0, 5.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Music,
        AudioPlayer::new(assets.load("audio/music/lonelyhood.ogg")),
        PlaybackSettings::LOOP.with_volume(save.settings.music_volume()),
    ));
}

#[cfg(test)]
mod tests {
    use bevy::state::app::StatesPlugin;

    use super::*;
    use crate::map::grid::Grid;
    use crate::map::species;

    #[derive(Resource, Default)]
    struct Ticks(u32);

    /// The simulation runs only during play: not on the splash, not paused, not once the Rôle is lost.
    #[test]
    fn simulation_runs_only_in_play() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, StatesPlugin, steps))
            .init_state::<Screen>()
            .init_resource::<Ticks>()
            .insert_resource(Level::new(Grid::parse("S.M"), species::load()))
            .add_systems(Update, (|mut ticks: ResMut<Ticks>| ticks.0 += 1).in_set(Step::Simulate));
        let mut frame = |screen: Screen| {
            app.world_mut().resource_mut::<NextState<Screen>>().set(screen);
            app.update();
            app.world().resource::<Ticks>().0
        };
        assert_eq!(frame(Screen::Splash), 0);
        assert_eq!(frame(Screen::Playing), 1);
        assert_eq!(frame(Screen::Paused), 1, "paused");
        assert_eq!(frame(Screen::Playing), 2);
        app.world_mut().resource_mut::<Level>().lives = 0;
        app.update();
        assert_eq!(app.world().resource::<Ticks>().0, 2, "the Rôle is lost");
    }
}
