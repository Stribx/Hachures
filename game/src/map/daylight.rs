//! The day goes on with the waves: the sun lowers into the afternoon and warms, eased from one wave
//! to the next.

use bevy::prelude::*;

use crate::level::Level;
use crate::look::Sun;
use crate::theme::SUNLIGHT;

/// Where the sun stands, seen from the board, and its colour, as wave `wave` of `waves` begins:
/// early afternoon before the first, a lower and warmer sun by the last, never so low that a
/// piece's shadow hides its neighbour.
pub fn daylight(wave: u32, waves: usize) -> (Vec3, Color) {
    let t = (wave as f32 / waves.max(1) as f32).clamp(0.0, 1.0);
    let noon = Vec3::new(-4.0, 9.0, -3.0);
    let (high, low) = ((noon.y / noon.length()).asin(), 43f32.to_radians());
    let elevation = high + (low - high) * t;
    // Swinging a little toward the west as it lowers.
    let azimuth = noon.z.atan2(noon.x) + 0.25 * t;
    let at = Vec3::new(
        elevation.cos() * azimuth.cos(),
        elevation.sin(),
        elevation.cos() * azimuth.sin(),
    ) * noon.length();
    (at, SUNLIGHT.mix(&EVENING, t * 0.8))
}

/// The low sun's colour toward the end of a level.
const EVENING: Color = Color::srgb(1.0, 0.82, 0.62);

/// The sun follows the waves, eased over a couple of seconds.
pub(super) fn follow_the_waves(
    level: Res<Level>,
    time: Res<Time>,
    sun: Single<(&mut Transform, &mut DirectionalLight), With<Sun>>,
) {
    let (at, color) = daylight(level.wave.number, level.waves.len());
    let (mut transform, mut light) = sun.into_inner();
    let target = Transform::from_translation(at).looking_at(Vec3::ZERO, Vec3::Y);
    if transform.rotation.angle_between(target.rotation) < 0.0005 {
        return;
    }
    let k = 1.0 - (-time.delta_secs() / 2.0).exp();
    transform.translation = transform.translation.lerp(target.translation, k);
    transform.rotation = transform.rotation.slerp(target.rotation, k);
    light.color = light.color.mix(&color, k);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn elevation(at: Vec3) -> f32 {
        (at.y / at.length()).asin().to_degrees()
    }

    #[test]
    fn the_first_wave_has_the_afternoon_sun() {
        let (at, color) = daylight(0, 12);
        assert!((elevation(at) - elevation(Vec3::new(-4.0, 9.0, -3.0))).abs() < 1.0);
        assert_eq!(color, SUNLIGHT);
    }

    /// By the last wave the sun is lower and warmer, but high enough for short shadows.
    #[test]
    fn the_last_wave_has_a_lower_warmer_sun() {
        let ((first, morning), (last, evening)) = (daylight(0, 12), daylight(12, 12));
        assert!(elevation(last) < elevation(first) - 10.0);
        assert!(elevation(last) > 35.0, "{}", elevation(last));
        let (m, e) = (morning.to_srgba(), evening.to_srgba());
        assert!(e.blue < m.blue && e.red >= m.red, "{e:?}");
    }

    /// Each wave moves the sun on, and a level with few waves gets the whole afternoon too.
    #[test]
    fn the_sun_moves_on_with_every_wave() {
        for waves in [3, 12] {
            let elevations: Vec<f32> =
                (0..=waves as u32).map(|w| elevation(daylight(w, waves).0)).collect();
            assert!(elevations.windows(2).all(|e| e[1] < e[0]), "{elevations:?}");
        }
    }
}
