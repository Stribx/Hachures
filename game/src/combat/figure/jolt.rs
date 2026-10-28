//! What a blow does to a standing figurine: the jolt of a hit and the stagger of a pin.

use bevy::prelude::*;

/// Seconds a jolt takes to die out.
pub const JOLT: f32 = 0.2;

/// The body's offset `t` seconds after a hit: it tips back, squashes a little, and settles.
pub fn jolt(t: f32) -> Transform {
    let k = (1.0 - t / JOLT).max(0.0).powi(2);
    // The forward axis is X: a pitch about Z that sends the head toward -X.
    Transform::from_rotation(Quat::from_rotation_z(0.35 * k)).with_scale(Vec3::new(
        1.0 + 0.08 * k,
        1.0 - 0.15 * k,
        1.0 + 0.08 * k,
    ))
}

/// The sway of a pinned figurine on the spot, at `clock` seconds.
pub fn stagger(clock: f32) -> Transform {
    Transform::from_rotation(Quat::from_rotation_x((clock * 25.0).sin() * 0.08))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_identity(t: Transform) -> bool {
        t.translation.length() < 1e-5 && t.rotation.xyz().length() < 1e-5 && t.scale.distance(Vec3::ONE) < 1e-5
    }

    #[test]
    fn a_jolt_fades_in_a_fifth_of_a_second() {
        let angle = |t: f32| jolt(t).rotation.xyz().length();
        assert!(angle(0.0) > 0.1, "it tips back at once");
        assert!(jolt(0.0).scale.y < 0.9, "and squashes");
        assert!(is_identity(jolt(JOLT)), "gone at 0.2 s");
        assert!(is_identity(jolt(1.0)));
        let mut last = angle(0.0);
        for i in 1..=25 {
            let now = angle(i as f32 * 0.01);
            assert!(now <= last + 1e-6, "the tilt grew at {i}");
            assert!(jolt(i as f32 * 0.01).scale.y >= jolt(0.0).scale.y);
            last = now;
        }
    }
}
