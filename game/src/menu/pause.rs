//! Esc during play: the game stands still behind this page.

use bevy::prelude::*;

use super::settings::open_settings;
use super::{Screen, VEIL, button, go, label, page, text};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Paused), spawn);
}

fn spawn(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(page(Screen::Paused, VEIL)).with_children(|p| {
        p.spawn(text(&assets, "Pause", 56.0));
        p.spawn(button(label(&assets, "Reprendre"))).observe(go(Screen::Playing));
        p.spawn(button(label(&assets, "Réglages"))).observe(open_settings(Screen::Paused));
        // The level stays as it is: Continuer picks it up again.
        p.spawn(button(label(&assets, "Menu principal"))).observe(go(Screen::Title));
    });
}
