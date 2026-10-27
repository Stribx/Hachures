//! Hachures: a cartographic tower defense. One folder per feature, each with its own plugin.

// A function does one thing: at most 70 lines of code (`clippy.toml`).
#![warn(clippy::too_many_lines)]

mod campaign;
mod combat;
mod i18n;
mod ink;
mod level;
mod look;
mod map;
mod menu;
mod overworld;
mod schedule;
mod settings;
mod shot;
mod storage;
mod theme;
mod ui;

use bevy::prelude::*;
pub use schedule::Step;

/// The game's name, written the same in every language.
pub const NAME: &str = "Hachures";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: NAME.into(),
                // In the browser, fill the page.
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            schedule::plugin,
            settings::plugin,
            look::plugin,
            map::plugin,
            overworld::plugin,
            ink::plugin,
            campaign::plugin,
            level::plugin,
            combat::plugin,
            ui::plugin,
            menu::plugin,
            shot::plugin,
        ))
        .run();
}
