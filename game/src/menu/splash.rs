//! The title card at launch, over the loading: it stays two seconds (any key or click skips them),
//! and until everything the menus and play draw is loaded and compiled, as a shipped game's does.

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use super::{Screen, page, text};
use crate::look::PipelinesReady;
use crate::theme::{PAPER, SUBTITLE_SIZE};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Splash), spawn)
        .add_systems(Update, leave.run_if(in_state(Screen::Splash)));
}

fn spawn(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(page(Screen::Splash, PAPER)).with_children(|p| {
        p.spawn(text(&assets, crate::NAME, 96.0));
        p.spawn(text(&assets, crate::i18n::t("splash.subtitle"), SUBTITLE_SIZE));
    });
}

/// How long the card stays at least, in seconds, unless skipped.
const SHOWN: f32 = 2.0;
/// Frames in a row without a pipeline to compile: a model just loaded asks for its own a frame or
/// two after it lands.
const CALM: u32 = 10;
/// The longest the card waits for the loading, in seconds: past it, the title comes anyway.
const LONGEST: f32 = 10.0;

/// How far the splash has got.
#[derive(Default)]
struct Waited {
    shown: f32,
    skipped: bool,
    calm: u32,
}

/// What the loading has got to: the models, and the GPU's pipelines.
#[derive(SystemParam)]
struct Loading<'w, 's> {
    pipelines: Res<'w, PipelinesReady>,
    roots: Query<'w, 's, &'static WorldAssetRoot>,
    assets: Res<'w, AssetServer>,
}

impl Loading<'_, '_> {
    /// Every model asked for is in, with what it is made of.
    fn models_loaded(&self) -> bool {
        self.roots.iter().all(|root| self.assets.is_loaded_with_dependencies(&root.0))
    }
}

/// The title may come: everything loaded and compiled, and the card shown or skipped; or the
/// loading took too long.
fn may_leave(waited: &Waited, loaded: bool) -> bool {
    let ready = loaded && waited.calm >= CALM;
    waited.shown >= LONGEST || (ready && (waited.skipped || waited.shown >= SHOWN))
}

fn leave(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    loading: Loading,
    mut waited: Local<Waited>,
    mut next: ResMut<NextState<Screen>>,
) {
    waited.shown += time.delta_secs();
    waited.skipped |=
        keys.get_just_pressed().next().is_some() || mouse.get_just_pressed().next().is_some();
    waited.calm = if loading.pipelines.0 { waited.calm + 1 } else { 0 };
    if may_leave(&waited, loading.models_loaded()) {
        next.set(Screen::Title);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A skip only cuts the card's two seconds short, never the loading; and a loading that hangs
    /// still gives way to the title.
    #[test]
    fn the_splash_waits_for_the_loading() {
        let at = |shown, skipped, calm| Waited { shown, skipped, calm };
        assert!(!may_leave(&at(1.0, false, CALM), true), "the card's two seconds");
        assert!(may_leave(&at(1.0, true, CALM), true), "skipped, and ready");
        assert!(!may_leave(&at(5.0, true, CALM), false), "still loading");
        assert!(!may_leave(&at(5.0, false, 3), true), "still compiling");
        assert!(may_leave(&at(2.5, false, CALM), true));
        assert!(may_leave(&at(LONGEST, false, 0), false), "a loading that hangs");
    }
}
