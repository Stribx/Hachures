//! Esc during play: the game stands still behind this page.

use bevy::prelude::*;
use bevy::ui_widgets::Activate;

use super::controls::open_controls;
use super::lexique::open_lexique;
use super::settings_page::open_settings;
use super::{Screen, button, go, label, page, text};
use crate::i18n::{n_, t, tf};
use crate::level::Level;
use crate::overworld::travel::Travel;
use crate::theme::{INK, INK_SOFT, SEAL, SUBTITLE_SIZE, TITLE_SIZE, VEIL};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Paused), spawn)
        .add_systems(Update, show_confirms.run_if(in_state(Screen::Paused)));
}

/// A button that asks twice, for what loses the level under way: the first press arms it ("Sûr ? …" in the seal's
/// red), a second within `ARMED` seconds acts.
#[derive(Component)]
struct Confirm {
    label: &'static str,
    /// Seconds since it was armed.
    armed: Option<f32>,
}

const ARMED: f32 = 3.0;

/// Whether `button` may act now: a confirm button acts only once armed, and the first press arms it.
fn confirmed(button: Entity, confirms: &mut Query<&mut Confirm>) -> bool {
    let Ok(mut confirm) = confirms.get_mut(button) else { return true };
    if confirm.armed.is_some() {
        return true;
    }
    confirm.armed = Some(0.0);
    false
}

fn confirm(assets: &AssetServer, words: &'static str) -> impl Bundle {
    (button(label(assets, t(words))), Confirm { label: words, armed: None })
}

fn spawn(mut commands: Commands, assets: Res<AssetServer>, level: Res<Level>) {
    commands.spawn(page(Screen::Paused, VEIL)).with_children(|p| {
        p.spawn(text(&assets, t("pause.title"), TITLE_SIZE));
        p.spawn(text(&assets, t(&level.name), SUBTITLE_SIZE)).insert(TextColor(INK_SOFT));
        p.spawn(button(label(&assets, t("pause.resume")))).observe(go(Screen::Playing));
        p.spawn(confirm(&assets, n_("pause.restart"))).observe(
            |act: On<Activate>,
             mut confirms: Query<&mut Confirm>,
             level: Res<Level>,
             mut travel: MessageWriter<Travel>| {
                if confirmed(act.entity, &mut confirms) {
                    travel.write(Travel { to: level.index, again: true });
                }
            },
        );
        // The level stays on its sheet: tracing it again from the Plan picks it up where it was.
        p.spawn(button(label(&assets, t("plan.title")))).observe(go(Screen::Plan));
        p.spawn(button(label(&assets, t("menu.lexique")))).observe(open_lexique(Screen::Paused));
        p.spawn(button(label(&assets, t("menu.controls")))).observe(open_controls(Screen::Paused));
        p.spawn(button(label(&assets, t("menu.settings")))).observe(open_settings(Screen::Paused));
        // The level stays as it is: Continuer picks it up again.
        p.spawn(button(label(&assets, t("pause.main_menu")))).observe(go(Screen::Title));
        // A browser tab is closed, not quit.
        #[cfg(not(target_arch = "wasm32"))]
        p.spawn(confirm(&assets, n_("pause.quit"))).observe(
            |act: On<Activate>,
             mut confirms: Query<&mut Confirm>,
             mut exit: MessageWriter<AppExit>| {
                if confirmed(act.entity, &mut confirms) {
                    exit.write(AppExit::Success);
                }
            },
        );
    });
}

/// An armed button says "Sûr ?" in red, and disarms after `ARMED` seconds.
fn show_confirms(
    time: Res<Time>,
    mut confirms: Query<(&mut Confirm, &Children)>,
    mut texts: Query<(&mut Text, &mut TextColor)>,
) {
    for (mut confirm, children) in &mut confirms {
        if let Some(t) = &mut confirm.armed {
            *t += time.delta_secs();
            if *t > ARMED {
                confirm.armed = None;
            }
        }
        let Some(&child) = children.first() else { continue };
        let Ok((mut text, mut color)) = texts.get_mut(child) else { continue };
        let (words, ink) = match confirm.armed {
            Some(_) => (tf("pause.confirm", &[&t(confirm.label)]), SEAL),
            None => (t(confirm.label).to_string(), INK),
        };
        if text.0 != words {
            text.0 = words;
            color.0 = ink;
        }
    }
}
