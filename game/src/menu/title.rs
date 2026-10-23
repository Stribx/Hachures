//! The main menu, the chapters to replay, and the confirmation before a new game erases the campaign.

use bevy::prelude::*;
use bevy::ui_widgets::Activate;

use super::settings::open_settings;
use super::{Screen, VEIL, button, go, label, page, text};
use crate::campaign::LoadLevel;
use crate::campaign::save::Save;
use crate::level::data::{LEVELS, level_name};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), spawn)
        .add_systems(OnEnter(Screen::Chapters), spawn_chapters)
        .add_systems(OnEnter(Screen::ConfirmNew), spawn_confirm);
}

/// A new campaign that keeps the player's settings.
fn fresh(save: &Save) -> Save {
    Save { settings: save.settings.clone(), ..default() }
}

fn spawn(mut commands: Commands, assets: Res<AssetServer>, save: Res<Save>) {
    let new = *save == fresh(&save);
    commands.spawn(page(Screen::Title, VEIL)).with_children(|p| {
        p.spawn(text(&assets, "Hachures", 72.0));
        p.spawn(button(label(&assets, if new { "Jouer" } else { "Continuer" }))).observe(go(Screen::Playing));
        if !new {
            p.spawn(button(label(&assets, "Chapitres"))).observe(go(Screen::Chapters));
            p.spawn(button(label(&assets, "Nouvelle partie"))).observe(go(Screen::ConfirmNew));
        }
        p.spawn(button(label(&assets, "Réglages"))).observe(open_settings(Screen::Title));
        // A browser tab is closed, not quit.
        #[cfg(not(target_arch = "wasm32"))]
        p.spawn(button(label(&assets, "Quitter"))).observe(|_: On<Activate>, mut exit: MessageWriter<AppExit>| {
            exit.write(AppExit::Success);
        });
    });
}

/// Every level reached, with its sceaux, to play again: a missed challenge can still be won.
fn spawn_chapters(mut commands: Commands, assets: Res<AssetServer>, save: Res<Save>) {
    commands.spawn(page(Screen::Chapters, VEIL)).with_children(|p| {
        p.spawn(text(&assets, "Chapitres", 56.0));
        for index in 0..=save.reached.min(LEVELS.len() - 1) {
            let sceaux = save.levels.get(index).map_or(0, |r| r.won as u32 + r.challenge as u32);
            p.spawn(button(label(&assets, format!("{} ({sceaux}/2)", level_name(index))))).observe(
                move |_: On<Activate>, mut load: MessageWriter<LoadLevel>, mut next: ResMut<NextState<Screen>>| {
                    load.write(LoadLevel(index));
                    next.set(Screen::Playing);
                },
            );
        }
        p.spawn(button(label(&assets, "Retour"))).observe(go(Screen::Title));
    });
}

fn spawn_confirm(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(page(Screen::ConfirmNew, VEIL)).with_children(|p| {
        p.spawn(text(&assets, "Effacer la campagne et recommencer ?", 36.0));
        p.spawn(button(label(&assets, "Oui, tout effacer"))).observe(new_game);
        p.spawn(button(label(&assets, "Non"))).observe(go(Screen::Title));
    });
}

fn new_game(
    _: On<Activate>,
    mut save: ResMut<Save>,
    mut load: MessageWriter<LoadLevel>,
    mut next: ResMut<NextState<Screen>>,
) {
    *save = fresh(&save);
    save.store();
    load.write(LoadLevel(0));
    next.set(Screen::Playing);
}
