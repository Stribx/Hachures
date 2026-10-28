//! The surveying chain the Grande Chaîne drags: twenty links lying along the road its cart has
//! driven.

use bevy::light::NotShadowCaster;
use bevy::prelude::*;

use std::f32::consts::FRAC_PI_2;

use super::kit::BOSS_REAR;
use super::look::EnemyLook;
use super::spawn::FLOAT;
use crate::combat::boss::Boss;
use crate::level::LevelEntity;

/// Links in the chain, how far apart they lie, and the radius of the iron of one (`look`'s torus).
const LINKS: usize = 20;
const SPACING: f32 = 0.07;
pub const LINK_MINOR: f32 = 0.012;
pub const LINK_MAJOR: f32 = 0.035;

/// Where the chain lies: `links` points, the first at `history[0]` (the newest), each `spacing`
/// further back along the path (arc length). A road shorter than the chain bunches the last links
/// up at its oldest point.
pub fn trail(history: &[Vec3], links: usize, spacing: f32) -> Vec<Vec3> {
    let Some(&first) = history.first() else { return Vec::new() };
    let mut road = vec![first];
    // Where the walk along the history stands: the segment it is on, the point, and the length owed
    // to the next link.
    let (mut seg, mut at, mut owed) = (0, first, spacing);
    while road.len() < links {
        let Some(&next) = history.get(seg + 1) else {
            road.push(at);
            continue;
        };
        let gap = at.distance(next);
        if gap >= owed {
            at += (next - at) / gap * owed;
            road.push(at);
            owed = spacing;
        } else {
            (owed, at, seg) = (owed - gap, next, seg + 1);
        }
    }
    road
}

/// The road a boss's cart has driven, newest rear point first, and the chain laid along it.
#[derive(Component)]
pub struct Rope {
    history: Vec<Vec3>,
    road: Vec<Vec3>,
}

/// One link of a boss's chain, numbered from the cart. It is not an `Enemy`: nothing aims at it.
#[derive(Component)]
pub struct Link {
    boss: Entity,
    index: usize,
}

/// Where link `index` of the `road` lies: on the ground, turned about the chain's direction every
/// other link.
fn link_pose(road: &[Vec3], index: usize) -> Transform {
    let heading = match index {
        0 => road[0] - road[1],
        _ => road[index - 1] - road[index],
    };
    let yaw = Quat::from_rotation_y((-heading.z).atan2(heading.x));
    let (turn, lift) = if index.is_multiple_of(2) {
        (Quat::IDENTITY, LINK_MINOR)
    } else {
        (Quat::from_rotation_x(FRAC_PI_2), LINK_MAJOR)
    };
    let ground = road[index] - Vec3::Y * (FLOAT - lift);
    // A link that has not moved (the chain just laid, or bunched) keeps its pose from the heading
    // it had.
    let rotation = if heading.length_squared() > 1e-8 { yaw * turn } else { turn };
    Transform::from_translation(ground).with_rotation(rotation)
}

/// Keeps each boss's road, lays its chain along it (spawning the links with the boss) and takes the
/// chain away with a boss that is gone.
pub fn drag_chain(
    look: Res<EnemyLook>,
    mut bosses: Query<(Entity, &Transform, Option<&mut Rope>), With<Boss>>,
    mut links: Query<(Entity, &Link, &mut Transform), Without<Boss>>,
    mut commands: Commands,
) {
    for (boss, at, rope) in &mut bosses {
        // The back of the cart, behind the root along the heading.
        let rear = at.translation - at.rotation * Vec3::X * BOSS_REAR * at.scale.x;
        let Some(mut rope) = rope else {
            let road = trail(&[rear], LINKS, SPACING);
            for index in 0..LINKS {
                commands.spawn((
                    LevelEntity,
                    NotShadowCaster,
                    Link { boss, index },
                    Mesh3d(look.link_mesh.clone()),
                    MeshMaterial3d(look.link_material.clone()),
                    link_pose(&road, index),
                ));
            }
            commands.entity(boss).insert(Rope { history: vec![rear], road });
            continue;
        };
        if rope.history[0].distance(rear) >= SPACING / 2.0 {
            rope.history.insert(0, rear);
            // Twenty links never need more than a point every half link.
            rope.history.truncate(2 * LINKS);
            rope.road = trail(&rope.history, LINKS, SPACING);
        }
    }
    for (entity, link, mut at) in &mut links {
        match bosses.get(link.boss) {
            Ok((_, _, Some(rope))) => *at = link_pose(&rope.road, link.index),
            Ok(_) => {}
            Err(_) => commands.entity(entity).despawn(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A road 0.1 a step: along +X, then +Z, newest first.
    fn l_road() -> Vec<Vec3> {
        let along_x = (0..10).map(|i| Vec3::new(i as f32 * 0.1, 0.0, 0.0));
        let along_z = (1..10).map(|i| Vec3::new(0.9, 0.0, i as f32 * 0.1));
        let mut road: Vec<Vec3> = along_x.chain(along_z).collect();
        road.reverse();
        road
    }

    /// How far along the road (from the newest point) `p` lies.
    fn arc(road: &[Vec3], p: Vec3) -> f32 {
        let mut walked = 0.0;
        for pair in road.windows(2) {
            let (a, b) = (pair[0], pair[1]);
            let on = (a.distance(p) + p.distance(b) - a.distance(b)).abs() < 1e-4;
            if on {
                return walked + a.distance(p);
            }
            walked += a.distance(b);
        }
        panic!("{p} is off the road");
    }

    #[test]
    fn the_chain_lies_along_the_road() {
        let road = l_road();
        let links = trail(&road, 20, 0.07);
        assert_eq!(links.len(), 20);
        assert_eq!(links[0], road[0]);
        for pair in links.windows(2) {
            let step = arc(&road, pair[1]) - arc(&road, pair[0]);
            assert!((step - 0.07).abs() < 1e-3, "{step}");
        }
    }

    #[test]
    fn a_short_road_bunches_the_chain() {
        let road = [Vec3::new(0.1, 0.0, 0.0), Vec3::ZERO];
        let links = trail(&road, 20, 0.07);
        assert_eq!(links.len(), 20);
        assert!(links[2..].iter().all(|&l| l == Vec3::ZERO), "{links:?}");
    }

    #[test]
    fn no_road_no_chain_spread() {
        let links = trail(&[Vec3::ONE], 20, 0.07);
        assert_eq!(links, vec![Vec3::ONE; 20]);
    }
}
