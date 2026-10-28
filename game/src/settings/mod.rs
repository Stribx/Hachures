//! The player's settings: sound, display, play, accessibility and language. Their own resource, kept in their own
//! file (`storage`); the page that edits them is `menu::settings_page`, `apply` puts them into effect.

mod apply;
mod model;

use bevy::prelude::*;

pub(crate) use model::PRESETS;
pub use model::Settings;

pub fn plugin(app: &mut App) {
    let settings = Settings::load();
    // Before the first page is written.
    crate::i18n::set(&settings.language);
    app.insert_resource(settings).add_systems(Startup, play_music).add_plugins(apply::plugin);
}

/// The music playing, whose volume follows the settings.
#[derive(Component)]
pub struct Music;

fn play_music(mut commands: Commands, assets: Res<AssetServer>, settings: Res<Settings>) {
    commands.spawn((
        Music,
        AudioPlayer::new(assets.load("audio/music/lonelyhood.ogg")),
        PlaybackSettings::LOOP.with_volume(settings.music_volume(true)),
    ));
}
