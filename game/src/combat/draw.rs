//! What combat draws each frame with gizmos: health bars over the enemies and the shots just fired.

use bevy::prelude::*;

use super::Enemy;

/// Shots drawn for a moment.
#[derive(Resource, Default)]
pub(super) struct Shots(pub(super) Vec<Shot>);

pub(super) struct Shot {
    pub(super) from: Vec3,
    pub(super) to: Vec3,
    /// Seconds left on screen.
    pub(super) left: f32,
}

/// Health bars over the enemies, and the shots fired.
pub(super) fn draw(
    time: Res<Time>,
    mut shots: ResMut<Shots>,
    enemies: Query<(&Enemy, &Transform)>,
    mut gizmos: Gizmos,
) {
    for (enemy, t) in &enemies {
        let left = t.translation + Vec3::new(-0.25, 0.35 * t.scale.y, 0.0);
        let frac = (enemy.hp / enemy.max_hp).clamp(0.0, 1.0);
        // Ink on a strip of paper; the seal's red once it is nearly broken.
        gizmos.line(left, left + Vec3::X * 0.5, crate::theme::SHEET);
        let ink = if frac < 0.35 { crate::theme::SEAL } else { crate::theme::WORLD_INK };
        gizmos.line(left + Vec3::Z * 0.01, left + Vec3::new(0.5 * frac, 0.0, 0.01), ink);
    }
    for shot in &mut shots.0 {
        gizmos.line(shot.from, shot.to, Color::srgb(0.2, 0.1, 0.05));
        shot.left -= time.delta_secs();
    }
    shots.0.retain(|s| s.left > 0.0);
}
