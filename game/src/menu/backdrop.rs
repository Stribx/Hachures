//! The world behind a veiled page: drawn once, then held still, as a shipped game does behind its
//! menus. The veil leaves it at an eighth; drawn every frame, it cost a whole frame of the GPU.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::look::WorldCamera;
use crate::schedule::Screen;
use crate::settings::Settings;

pub fn plugin(app: &mut App) {
    app.add_systems(PostUpdate, hold);
}

/// The pages laid on a veil over the world: behind them, it holds still.
fn held(screen: Screen) -> bool {
    matches!(
        screen,
        Screen::Paused
            | Screen::Settings
            | Screen::Sceaux
            | Screen::Lexique
            | Screen::Carnet
            | Screen::Controls
            | Screen::Credits
            | Screen::ConfirmNew
    )
}

/// Frames still drawn after a change: one for the settings to reach the camera, one to draw it.
const REDRAW: u8 = 2;

/// The world camera draws only when its picture would change: off a veiled page, or for a moment
/// after a new page, setting or window size. Stopped, its picture stays under the page.
fn hold(
    screen: Res<State<Screen>>,
    settings: Res<Settings>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut camera: Single<&mut Camera, With<WorldCamera>>,
    mut redraw: Local<u8>,
    mut size: Local<UVec2>,
) {
    // The window's size, not its change: the cursor's moves change the window too.
    let resized = *size != window.physical_size();
    *size = window.physical_size();
    if screen.is_changed() || settings.is_changed() || resized {
        *redraw = REDRAW;
    }
    let active = !held(*screen.get()) || *redraw > 0;
    if camera.is_active != active {
        camera.is_active = active;
    }
    *redraw = redraw.saturating_sub(1);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The board stays live in play, on the title and on the Grand Plan; still behind the veils.
    #[test]
    fn only_veiled_pages_hold_the_world() {
        for screen in [Screen::Splash, Screen::Title, Screen::Plan, Screen::Playing] {
            assert!(!held(screen), "{screen:?}");
        }
        for screen in [Screen::Paused, Screen::Settings, Screen::Lexique, Screen::Credits] {
            assert!(held(screen), "{screen:?}");
        }
    }
}
