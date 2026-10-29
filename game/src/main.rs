//! Hachures: a cartographic tower defense. One folder per feature, each with its own plugin.

// A function does one thing: at most 70 lines of code (`clippy.toml`).
#![warn(clippy::too_many_lines)]
// Google Rust style (docs/agents/rust-style.md): what clippy can check of it.
#![deny(unsafe_code)]
#![warn(clippy::unwrap_used, clippy::wildcard_imports, clippy::needless_for_each)]

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
use std::path::PathBuf;

/// The game's name, written the same in every language.
pub const NAME: &str = "Hachures";

fn asset_path() -> String {
    if std::env::var_os("BEVY_ASSET_ROOT").is_some() {
        return "assets".into();
    }

    let executable_assets = std::env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(PathBuf::from))
        .map(|path| path.join("assets"));
    if let Some(path) = executable_assets.filter(|path| path.is_dir()) {
        return path.to_string_lossy().into_owned();
    }

    let source_assets = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");
    if source_assets.is_dir() {
        return source_assets.to_string_lossy().into_owned();
    }

    "assets".into()
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin { file_path: asset_path(), ..default() }).set(
            WindowPlugin {
                primary_window: Some(Window {
                    title: NAME.into(),
                    // In the browser, fill the page.
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            },
        ))
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
