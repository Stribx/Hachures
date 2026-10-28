//! When systems run: the three steps of a frame, and which screen is up.

use bevy::prelude::*;

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

/// Which screen is up. The game's input and simulation only run while `Playing`.
#[derive(States, Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Screen {
    #[default]
    Splash,
    Title,
    /// "Erase the campaign?" before a new game.
    ConfirmNew,
    /// Le Grand Plan: the Marches from above, Nève on the road from sheet to sheet.
    Plan,
    Settings,
    /// L'arbre des sceaux.
    Sceaux,
    /// Le Lexique: what the tools and the species do.
    Lexique,
    /// Le carnet de Nève, once the campaign is over.
    Carnet,
    /// The keys, read-only.
    Controls,
    Credits,
    Playing,
    Paused,
}

/// The camera is up over the Marches (title, Grand Plan): the board's readouts stay off.
pub fn over_world(screen: Res<State<Screen>>) -> bool {
    matches!(screen.get(), Screen::Splash | Screen::Title | Screen::ConfirmNew | Screen::Plan)
}

/// The frame's steps, in order, and when they run. Paused is simply: no input, no simulation; the map
/// is still drawn behind the menus. `level` adds its own condition: none once the Rôle is lost.
pub fn plugin(app: &mut App) {
    app.init_state::<Screen>().configure_sets(
        Update,
        (
            Step::Input.run_if(in_state(Screen::Playing)),
            Step::Simulate.run_if(in_state(Screen::Playing)),
            Step::Render,
        )
            .chain(),
    );
}

#[cfg(test)]
mod tests {
    use bevy::state::app::StatesPlugin;

    use super::*;

    #[derive(Resource, Default)]
    struct Ticks(u32);

    /// The simulation runs only during play: not on the splash, not paused.
    #[test]
    fn simulation_runs_only_in_play() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, StatesPlugin, plugin))
            .init_state::<Screen>()
            .init_resource::<Ticks>()
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
    }
}
