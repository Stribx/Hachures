//! Dynamic resolution, as a shipped game has it: in play and on the Grand Plan, the world's
//! picture shrinks a step when frames run late on the GPU, and grows back when they have room,
//! between 85% and all of the player's render scale.

use bevy::diagnostic::DiagnosticsStore;
use bevy::prelude::*;

use crate::look::{frame_time, gpu_time};
use crate::schedule::Screen;

/// The share of the player's render scale the world is drawn at, in percent.
#[derive(Resource, Clone, Copy, PartialEq, Eq, Debug)]
pub struct DynamicScale(pub u32);

impl Default for DynamicScale {
    fn default() -> Self {
        DynamicScale(100)
    }
}

/// The least share, where the hatching still reads; and a step.
const FLOOR: u32 = 85;
const STEP: u32 = 5;
/// The frame aimed for: 60 a second.
const TARGET_MS: f64 = 1000.0 / 60.0;
/// The least time between two steps, in seconds: the readings are smoothed over about as long.
const HOLD: f32 = 1.0;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<DynamicScale>().add_systems(Update, adjust);
}

/// The next share, from a frame's time and the GPU's: a step down when frames run late and the
/// GPU takes most of them (a smaller picture would not help a busy CPU); a step up when even a
/// step bigger would leave room.
fn next_share(share: u32, frame_ms: f64, gpu_ms: f64) -> u32 {
    if frame_ms > TARGET_MS * 1.05 && gpu_ms > frame_ms * 0.5 {
        return share.saturating_sub(STEP).max(FLOOR);
    }
    let up = (share + STEP).min(100);
    // Most of the GPU's time goes with the pixels, so with the share squared.
    let grown = frame_ms + gpu_ms * ((up as f64 / share as f64).powi(2) - 1.0);
    if grown < TARGET_MS * 0.9 { up } else { share }
}

/// Once a second at most, in play and on the Grand Plan, the share follows the frames. Real time:
/// fast play must not hurry it. Behind the pages the world is held still; on the title, its own.
fn adjust(
    screen: Res<State<Screen>>,
    time: Res<Time<Real>>,
    store: Res<DiagnosticsStore>,
    mut share: ResMut<DynamicScale>,
    mut since: Local<f32>,
) {
    if !matches!(screen.get(), Screen::Playing | Screen::Plan) {
        return;
    }
    *since += time.delta_secs();
    if *since < HOLD {
        return;
    }
    *since = 0.0;
    // No timings (WebGL2): the share stays.
    let (Some(frame_ms), Some(gpu_ms)) = (frame_time(&store), gpu_time(&store)) else { return };
    share.set_if_neq(DynamicScale(next_share(share.0, frame_ms, gpu_ms)));
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Late on the GPU: a step down, never under the floor. Late on the CPU: no help from fewer
    /// pixels. With room: a step up, never past all of it. Just in budget: it holds, so it never
    /// swings between two steps.
    #[test]
    fn the_share_follows_the_gpu_within_its_bounds() {
        assert_eq!(next_share(100, 24.0, 18.0), 95);
        assert_eq!(next_share(85, 24.0, 18.0), 85);
        assert_eq!(next_share(100, 24.0, 6.0), 100, "the CPU is late, not the GPU");
        assert_eq!(next_share(90, 10.0, 6.0), 95);
        assert_eq!(next_share(100, 10.0, 6.0), 100);
        assert_eq!(next_share(90, 16.0, 12.0), 90, "a step up would run late");
    }
}
