//! The land's sounds: the wind always, birds over the bocage, rain with la Crue, the river by the
//! water, the cold wind of the Cols; each loop fading in and out as the board and its weather
//! change.

use bevy::audio::{AudioSinkPlayback, Volume};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::biome::Biome;
use super::grid::Tile;
use crate::level::Level;
use crate::settings::Settings;

/// The loops, in the order `ambience` gives their loudness.
const LOOPS: [&str; 5] = ["wind", "birds", "rain", "river", "snow-wind"];

/// How loud each loop of `LOOPS` is, from 0 to 1, in world `world` (an index of `WORLDS`), with its
/// event under way or not, on a board with water (`wet`) or without.
pub fn ambience(world: usize, event_on: bool, wet: bool) -> [f32; 5] {
    let biome = Biome::of(world);
    let (bocage, cols) = (biome == Biome::Bocage, biome == Biome::Cols);
    let on = |yes: bool, loud: f32| if yes { loud } else { 0.0 };
    [
        0.55,
        on(bocage && !event_on, 0.6),
        on(bocage && event_on, 0.8),
        on(wet, 0.45),
        on(cols || biome == Biome::Karst, if event_on { 0.9 } else { 0.5 }),
    ]
}

/// One of the land's loops, by its index in `LOOPS`.
#[derive(Component)]
pub(super) struct Ambient(usize);

/// Every loop starts at once, silent, and keeps playing: `fade` brings each in when it is heard.
pub(super) fn start(mut commands: Commands, assets: Res<AssetServer>) {
    for (k, name) in LOOPS.iter().enumerate() {
        commands.spawn((
            Ambient(k),
            AudioPlayer::new(assets.load(format!("audio/ambient/{name}.ogg"))),
            PlaybackSettings::LOOP.with_volume(Volume::SILENT),
        ));
    }
}

/// Each loop eases toward its loudness over a couple of seconds, under the ambient volume; the wind
/// swells and eases slowly, so it never sits flat.
pub(super) fn fade(
    level: Res<Level>,
    settings: Res<Settings>,
    time: Res<Time>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut sinks: Query<(&Ambient, &mut AudioSink)>,
) {
    let wet = level.terrain.grid().tiles.iter().any(|&t| matches!(t, Tile::Eau | Tile::Gue));
    let loudness = ambience(level.world, level.event_on, wet);
    let t = time.elapsed_secs();
    let swell = 0.8 + 0.2 * ((t * 0.21).sin() * (t * 0.13).cos()).abs();
    let volume = settings.ambient_volume(window.focused).to_linear();
    let k = 1.0 - (-time.delta_secs() / 2.0).exp();
    for (ambient, mut sink) in &mut sinks {
        let target = loudness[ambient.0] * if ambient.0 == 0 { swell } else { 1.0 } * volume;
        let now = sink.volume().to_linear();
        sink.set_volume(Volume::Linear(now + (target - now) * k));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const WIND: usize = 0;
    const BIRDS: usize = 1;
    const RAIN: usize = 2;
    const RIVER: usize = 3;
    const SNOW: usize = 4;

    #[test]
    fn the_wind_always_blows() {
        for (world, event_on, wet) in
            [(0, false, false), (0, true, true), (1, false, true), (1, true, false)]
        {
            assert!(ambience(world, event_on, wet)[WIND] > 0.0);
        }
    }

    #[test]
    fn birds_sing_over_the_bocage_until_la_crue() {
        assert!(ambience(0, false, false)[BIRDS] > 0.0);
        assert_eq!(ambience(0, true, false)[BIRDS], 0.0, "they hush in the flood");
        assert_eq!(ambience(1, false, false)[BIRDS], 0.0, "none up in the Cols");
    }

    #[test]
    fn la_crue_brings_rain() {
        assert!(ambience(0, true, false)[RAIN] > 0.0);
        assert_eq!(ambience(0, false, false)[RAIN], 0.0);
    }

    #[test]
    fn the_river_runs_where_there_is_water() {
        assert!(ambience(0, false, true)[RIVER] > 0.0);
        assert_eq!(ambience(0, false, false)[RIVER], 0.0);
    }

    #[test]
    fn the_cols_have_their_cold_wind() {
        assert!(ambience(1, false, false)[SNOW] > 0.0);
        assert!(
            ambience(1, true, false)[SNOW] > ambience(1, false, false)[SNOW],
            "louder in le Gel"
        );
        assert_eq!(ambience(0, true, true)[SNOW], 0.0);
    }
}
