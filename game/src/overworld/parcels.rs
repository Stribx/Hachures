//! The Marches' bocage: the open land cut into parcels, each washed its own crop's colour, hedged along its seams.

use bevy::prelude::*;

use super::terrain::{WATER, cols, hash, height, road_distance, smoothstep, wood};
use crate::theme::{PASTURE, WHEAT};

/// How wide a parcel is, in world units.
const SIZE: f32 = 6.0;

/// The washes the parcels are coloured with: pasture, wheat, fallow, sainfoin in flower (pink: a blue crop would
/// read as water), a second pasture. Neighbours never share one (see `parcel`).
pub(super) const WASHES: [Color; 5] =
    [PASTURE, WHEAT, Color::srgb(0.76, 0.62, 0.44), Color::srgb(0.8, 0.64, 0.62), Color::srgb(0.6, 0.71, 0.44)];
/// Which of `WASHES` is the wheat, whose parcels grow clumps of it.
pub(super) const WHEAT_WASH: usize = 1;

/// The parcel at `p`: its id (the grid cell of its centre) and its wash. Defined everywhere; `openness` says where
/// the bocage actually shows.
pub(super) fn parcel(p: Vec2) -> (IVec2, usize) {
    let (id, _) = nearest(p);
    // Along a row the washes step by one, down a column by two: over the 3 x 3 cells around, never a repeat.
    (id, (id.x + 2 * id.y).rem_euclid(WASHES.len() as i32) as usize)
}

/// How far `p` lies from the nearest seam between two parcels, in world units.
pub(super) fn seam(p: Vec2) -> f32 {
    nearest(p).1
}

/// The parcel whose centre is nearest `p`, and how far `p` lies from its seam with the next nearest. Each parcel's
/// centre is its grid cell's, shifted a little by a hash so the fields are not a chequerboard.
fn nearest(p: Vec2) -> (IVec2, f32) {
    let cell = (p / SIZE).floor().as_ivec2();
    let centre = |c: IVec2| {
        let k = c.as_vec2();
        (k + 0.5 + (Vec2::new(hash(k), hash(k + 7.3)) - 0.5) * 0.7) * SIZE
    };
    let mut sites: Vec<(IVec2, Vec2)> =
        (-1..=1).flat_map(|dz| (-1..=1).map(move |dx| cell + IVec2::new(dx, dz))).map(|c| (c, centre(c))).collect();
    sites.sort_by(|a, b| a.1.distance_squared(p).total_cmp(&b.1.distance_squared(p)));
    let ((id, a), (_, b)) = (sites[0], sites[1]);
    // Distance to the perpendicular bisector of the two nearest centres.
    let to_seam = (b.distance_squared(p) - a.distance_squared(p)) / (2.0 * a.distance(b));
    (id, to_seam)
}

/// How much of the bocage shows at `p`, from 0 to 1: none in the woods, by the water, up the mountains or on the
/// road, fading in over a few units so the land's colour never cuts there.
pub(super) fn openness(p: Vec2) -> f32 {
    let h = height(p);
    let dry = smoothstep(WATER + 0.25, WATER + 0.6, h) * (1.0 - smoothstep(1.5, 2.5, h));
    let clearing = 1.0 - smoothstep(0.42, 0.5, wood(p));
    let lowland = 1.0 - smoothstep(0.1, 0.4, cols(p.x));
    let off_road = smoothstep(1.2, 2.5, road_distance(p));
    dry * clearing * lowland * off_road
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_same_point_lies_in_the_same_parcel() {
        for p in [Vec2::new(10.0, 5.0), Vec2::new(-33.3, 12.7), Vec2::new(150.0, -40.0)] {
            assert_eq!(parcel(p), parcel(p));
        }
    }

    /// Two parcels side by side never share a wash: the quilt always shows its seams.
    #[test]
    fn neighbouring_parcels_differ() {
        let mut checked = 0;
        for z in (-70..35).step_by(3) {
            for x in (-35..230).step_by(3) {
                let p = Vec2::new(x as f32, z as f32);
                let (a, b) = (parcel(p), parcel(p + Vec2::new(0.4, 0.3)));
                if a.0 != b.0 {
                    assert_ne!(a.1, b.1, "at {p}");
                    checked += 1;
                }
            }
        }
        assert!(checked > 50, "{checked} seams crossed");
    }

    /// The bocage stays off the water, the woods, the mountains and the road.
    #[test]
    fn the_bocage_lies_on_open_land_only() {
        for z in (-70..35).step_by(2) {
            for x in (-35..230).step_by(2) {
                let p = Vec2::new(x as f32, z as f32);
                if openness(p) > 0.5 {
                    assert!(height(p) > WATER, "water at {p}");
                    assert!(wood(p) < 0.52, "wood at {p}");
                    assert!(cols(p.x) < 0.5, "mountain at {p}");
                    assert!(road_distance(p) > 1.0, "road at {p}");
                }
            }
        }
    }

    /// A seam lies where the parcel changes.
    #[test]
    fn a_seam_parts_two_parcels() {
        let p = Vec2::new(20.0, 3.0);
        let mut x = p.x;
        while parcel(Vec2::new(x, p.y)).0 == parcel(p).0 {
            x += 0.01;
        }
        assert!(seam(Vec2::new(x, p.y)) < 0.05, "at x {x}");
        assert!(seam(p) >= 0.0, "at {p}");
    }
}
