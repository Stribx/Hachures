//! The player's settings, kept in the save: music and effects volume, fullscreen.

use bevy::audio::{AudioSinkPlayback, Volume};
use bevy::prelude::*;
use bevy::ui_widgets::Activate;
use bevy::window::{MonitorSelection, PrimaryWindow, WindowMode};
use serde::{Deserialize, Serialize};

use super::{Screen, VEIL, button, label, page, text};
use crate::campaign::save::Save;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// Percent, in steps of 10.
    pub music: u32,
    /// Percent, in steps of 10.
    pub effects: u32,
    pub fullscreen: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings { music: 100, effects: 100, fullscreen: false }
    }
}

impl Settings {
    pub fn music_volume(&self) -> Volume {
        Volume::Linear(self.music as f32 / 100.0)
    }

    pub fn effects_volume(&self) -> Volume {
        Volume::Linear(self.effects as f32 / 100.0)
    }
}

/// The music playing, whose volume follows the settings.
#[derive(Component)]
pub struct Music;

/// Where the settings page goes back to: the title or the pause menu.
#[derive(Resource)]
pub struct SettingsFrom(pub Screen);

/// A line of the page that shows a setting's value.
#[derive(Component, Clone, Copy)]
enum Field {
    Music,
    Effects,
    Fullscreen,
}

impl Field {
    fn text(self, s: &Settings) -> String {
        match self {
            Field::Music => format!("Musique : {} %", s.music),
            Field::Effects => format!("Effets : {} %", s.effects),
            Field::Fullscreen => format!("Plein écran : {}", if s.fullscreen { "oui" } else { "non" }),
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Settings), spawn)
        .add_systems(Update, (show_values.run_if(in_state(Screen::Settings)), apply));
}

/// Button action: open the settings page, coming back to `from`.
pub fn open_settings(from: Screen) -> impl Fn(On<Activate>, Commands, ResMut<NextState<Screen>>) {
    move |_, mut commands, mut next| {
        commands.insert_resource(SettingsFrom(from));
        next.set(Screen::Settings);
    }
}

/// Button action: change the settings with `edit`, then save.
fn change(edit: impl Fn(&mut Settings) + Send + Sync + 'static) -> impl Fn(On<Activate>, ResMut<Save>) {
    move |_, mut save| {
        edit(&mut save.settings);
        save.store();
    }
}

fn spawn(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(page(Screen::Settings, VEIL)).with_children(|p| {
        p.spawn(text(&assets, "Réglages", 56.0));
        for (field, volume) in
            [(Field::Music, (|s| &mut s.music) as fn(&mut Settings) -> &mut u32), (Field::Effects, |s| &mut s.effects)]
        {
            p.spawn(Node { column_gap: px(14), align_items: AlignItems::Center, ..default() }).with_children(|row| {
                row.spawn(button(label(&assets, "−"))).observe(change(move |s| {
                    let v = volume(s);
                    *v = v.saturating_sub(10);
                }));
                row.spawn((text(&assets, "", 28.0), field));
                row.spawn(button(label(&assets, "+"))).observe(change(move |s| {
                    let v = volume(s);
                    *v = (*v + 10).min(100);
                }));
            });
        }
        p.spawn(button((label(&assets, ""), Field::Fullscreen))).observe(change(|s| s.fullscreen = !s.fullscreen));
        p.spawn(button(label(&assets, "Retour")))
            .observe(|_: On<Activate>, from: Res<SettingsFrom>, mut next: ResMut<NextState<Screen>>| next.set(from.0));
    });
}

fn show_values(save: Res<Save>, mut fields: Query<(&Field, &mut Text)>) {
    for (field, mut text) in &mut fields {
        text.0 = field.text(&save.settings);
    }
}

/// Puts the saved settings into effect: on the music playing, and on the window.
fn apply(
    save: Res<Save>,
    mut music: Query<&mut AudioSink, With<Music>>,
    mut window: Single<&mut Window, With<PrimaryWindow>>,
) {
    if !save.is_changed() {
        return;
    }
    for mut sink in &mut music {
        sink.set_volume(save.settings.music_volume());
    }
    let mode = if save.settings.fullscreen {
        WindowMode::BorderlessFullscreen(MonitorSelection::Current)
    } else {
        WindowMode::Windowed
    };
    if window.mode != mode {
        window.mode = mode;
    }
}
