//! The figurine kit: which primitives, where and in what paint make up a species' figurine (pure data).

use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use super::props::props;
use crate::map::species::{Carry, Shape};

/// How far the Grande Chaîne's cart is pushed back so its oxen fit in front.
const BOSS_LEAD: f32 = -0.18;
/// The Grande Chaîne's box is lower than a convoy's: at its size a tall one walls off the board.
const BOSS_BOX: f32 = 0.14;
/// How far behind the root the Grande Chaîne's back lies.
pub const BOSS_REAR: f32 = 0.25 - BOSS_LEAD;

/// A primitive of the kit, sized in figure units.
#[derive(Clone, Copy, Debug)]
pub enum Form {
    Frustum {
        bottom: f32,
        top: f32,
        height: f32,
    },
    Sphere(f32),
    /// Radius, height.
    Cylinder(f32, f32),
    /// Full lengths.
    Cuboid(Vec3),
    Torus {
        minor: f32,
        major: f32,
    },
}

/// What a part is painted with.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Paint {
    /// The species' own colour.
    Coat,
    Felt,
    Wood,
    Linen,
    Seal,
    Iron,
    Hide,
    Slate,
    Lamp,
}

/// What the gait may move on its own: wheels spin, legs swing.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Role {
    Body,
    Wheel,
    Leg,
    /// The oxen's yoke, which knocks when they bolt.
    Yoke,
}

/// One primitive placed on the figurine. Figure units: the ground is y = 0 (the bottom of the felt), +X is forward.
#[derive(Clone, Copy, Debug)]
pub struct Part {
    pub form: Form,
    pub at: Transform,
    pub paint: Paint,
    pub role: Role,
    /// Which way a leg swings first, +1 or -1 (0 for what is not a leg): legs of one animal alternate.
    pub side: f32,
}

impl Form {
    #[must_use]
    fn scaled(self, k: f32) -> Form {
        match self {
            Form::Frustum { bottom, top, height } => {
                Form::Frustum { bottom: bottom * k, top: top * k, height: height * k }
            }
            Form::Sphere(r) => Form::Sphere(r * k),
            Form::Cylinder(r, height) => Form::Cylinder(r * k, height * k),
            Form::Cuboid(size) => Form::Cuboid(size * k),
            Form::Torus { minor, major } => Form::Torus { minor: minor * k, major: major * k },
        }
    }
}

/// Parts no bigger than this (their longest full side, in figure units) cast no shadow: seals, horns, bands, small
/// heads. Just above the 0.06 of a seal's or a child's head.
const SMALL: f32 = 0.065;

/// Whether a part of this form is big enough to cast a shadow (the shadow pass is costly during a wave).
pub fn casts_shadow(form: &Form) -> bool {
    let longest = match *form {
        Form::Frustum { bottom, top, height } => (2.0 * bottom.max(top)).max(height),
        Form::Sphere(r) => 2.0 * r,
        Form::Cylinder(r, height) => (2.0 * r).max(height),
        Form::Cuboid(size) => size.max_element(),
        Form::Torus { minor, major } => 2.0 * (major + minor),
    };
    longest >= SMALL
}

impl Part {
    /// A part of the body, painted `paint`, at `(x, y, z)`.
    pub(super) fn at(form: Form, x: f32, y: f32, z: f32, paint: Paint) -> Part {
        Part { form, at: Transform::from_xyz(x, y, z), paint, role: Role::Body, side: 0.0 }
    }

    #[must_use]
    pub(super) fn turned(mut self, rotation: Quat) -> Part {
        self.at.rotation = rotation;
        self
    }

    /// The part `k` times the size, from the figure's origin.
    #[must_use]
    pub(super) fn scaled(mut self, k: f32) -> Part {
        self.form = self.form.scaled(k);
        self.at.translation *= k;
        self
    }

    #[must_use]
    pub(super) fn shifted(mut self, by: Vec3) -> Part {
        self.at.translation += by;
        self
    }

    #[must_use]
    pub(super) fn painted(mut self, paint: Paint) -> Part {
        self.paint = paint;
        self
    }

    pub(super) fn role(mut self, role: Role) -> Part {
        self.role = role;
        self
    }

    /// A leg that swings first toward `side`.
    fn leg(mut self, side: f32) -> Part {
        self.side = side;
        self.role(Role::Leg)
    }

    /// The box, along the world axes, that holds the part once placed by `at` (an over-estimate when it is turned).
    #[cfg(test)]
    pub fn bounds(&self) -> (Vec3, Vec3) {
        let half = match self.form {
            Form::Frustum { bottom, top, height } => Vec3::new(bottom.max(top), height / 2.0, bottom.max(top)),
            Form::Sphere(r) => Vec3::splat(r),
            Form::Cylinder(r, height) => Vec3::new(r, height / 2.0, r),
            Form::Cuboid(size) => size / 2.0,
            Form::Torus { minor, major } => Vec3::new(major + minor, minor, major + minor),
        };
        let reach = (self.at.rotation * Vec3::X * half.x).abs()
            + (self.at.rotation * Vec3::Y * half.y).abs()
            + (self.at.rotation * Vec3::Z * half.z).abs();
        (self.at.translation - reach, self.at.translation + reach)
    }
}

/// Where the props go on a body: the middle of the chest, the head, and the top of the head.
pub(super) struct Anchors {
    pub chest: Vec3,
    /// How far the front of the body (and the seal on it) lies from the middle, at chest height.
    pub front: f32,
    pub head: Vec3,
    pub crown: f32,
}

/// The bare body of `shape`, and where it takes its props.
pub(super) fn body(shape: Shape) -> (Vec<Part>, Anchors) {
    use Paint::{Coat, Felt, Wood};
    let at = Part::at;
    match shape {
        Shape::Pied => (
            vec![
                at(Form::Cylinder(0.14, 0.03), 0.0, 0.015, 0.0, Felt),
                at(Form::Frustum { bottom: 0.10, top: 0.055, height: 0.26 }, 0.0, 0.16, 0.0, Coat),
                at(Form::Sphere(0.065), 0.0, 0.35, 0.0, Wood),
            ],
            Anchors { chest: Vec3::new(0.0, 0.20, 0.0), front: 0.075, head: Vec3::new(0.0, 0.35, 0.0), crown: 0.415 },
        ),
        Shape::Echasses => (
            vec![
                at(Form::Cylinder(0.14, 0.03), 0.0, 0.015, 0.0, Felt),
                at(Form::Cylinder(0.018, 0.30), 0.0, 0.18, 0.05, Wood).leg(1.0),
                at(Form::Cylinder(0.018, 0.30), 0.0, 0.18, -0.05, Wood).leg(-1.0),
                at(Form::Frustum { bottom: 0.08, top: 0.05, height: 0.16 }, 0.0, 0.41, 0.0, Coat),
                at(Form::Sphere(0.06), 0.0, 0.55, 0.0, Wood),
            ],
            Anchors { chest: Vec3::new(0.0, 0.42, 0.0), front: 0.068, head: Vec3::new(0.0, 0.55, 0.0), crown: 0.61 },
        ),
        Shape::Bete => {
            let mut parts = vec![
                at(Form::Cuboid(Vec3::new(0.46, 0.02, 0.28)), 0.0, 0.01, 0.0, Felt),
                at(Form::Cuboid(Vec3::new(0.42, 0.18, 0.24)), 0.0, 0.23, 0.0, Coat),
                at(Form::Cuboid(Vec3::new(0.13, 0.13, 0.12)), 0.25, 0.31, 0.0, Coat),
            ];
            for (x, z) in [(0.15, 0.09), (0.15, -0.09), (-0.15, 0.09), (-0.15, -0.09)] {
                // Diagonal pairs swing together, as in a trot.
                parts.push(at(Form::Cylinder(0.025, 0.12), x, 0.08, z, Coat).leg(x.signum() * z.signum()));
            }
            (
                parts,
                Anchors {
                    chest: Vec3::new(0.0, 0.20, 0.0),
                    front: 0.215,
                    head: Vec3::new(0.25, 0.31, 0.0),
                    crown: 0.375,
                },
            )
        }
        Shape::Chariot => chariot(0.0, 0.22),
    }
}

/// The cart, its box `lead` further along than the Convoi's (the Grande Chaîne's is pushed back for its oxen).
fn chariot(lead: f32, height: f32) -> (Vec<Part>, Anchors) {
    let at = Part::at;
    // The box rests on the axles, 0.14 up.
    let (middle, top) = (0.14 + height / 2.0, 0.14 + height);
    let mut parts = vec![at(Form::Cuboid(Vec3::new(0.5, height, 0.3)), lead, middle, 0.0, Paint::Coat)];
    for (x, z) in [(0.15, 0.17), (0.15, -0.17), (-0.15, 0.17), (-0.15, -0.17)] {
        let wheel = at(Form::Cylinder(0.1, 0.04), lead + x, 0.1, z, Paint::Wood);
        parts.push(wheel.turned(Quat::from_rotation_x(FRAC_PI_2)).role(Role::Wheel));
    }
    // No head: the props sit on the box.
    let anchors =
        Anchors { chest: Vec3::new(lead, middle, 0.0), front: 0.255, head: Vec3::new(lead, top, 0.0), crown: top };
    (parts, anchors)
}

/// The parts of a figurine of `shape` wearing `carries`, with its seal if it is `hostile`.
pub fn parts(shape: Shape, carries: &[Carry], hostile: bool) -> Vec<Part> {
    let boss = shape == Shape::Chariot && carries.contains(&Carry::Boeufs);
    let (mut parts, anchors) = if boss { chariot(BOSS_LEAD, BOSS_BOX) } else { body(shape) };
    if hostile {
        // A disc of wax pressed on the front of the chest, its flat face looking forward.
        let seal =
            Part::at(Form::Cylinder(0.03, 0.015), anchors.chest.x + anchors.front, anchors.chest.y, 0.0, Paint::Seal);
        parts.push(seal.turned(Quat::from_rotation_z(-FRAC_PI_2)));
    }
    for &carry in carries {
        parts.extend(props(carry, &anchors));
    }
    parts
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combat::enemies::{BOSS_SCALE, SCALE};
    use crate::map::species;

    const SHAPES: [Shape; 4] = [Shape::Pied, Shape::Echasses, Shape::Bete, Shape::Chariot];

    fn count(parts: &[Part], paint: Paint) -> usize {
        parts.iter().filter(|p| p.paint == paint).count()
    }

    #[test]
    fn a_hostile_wears_one_seal_an_ally_none() {
        for shape in SHAPES {
            assert_eq!(count(&parts(shape, &[], true), Paint::Seal), 1, "{shape:?}");
            assert_eq!(count(&parts(shape, &[], false), Paint::Seal), 0, "{shape:?}");
        }
    }

    #[test]
    fn parts_stay_on_their_cell_and_above_ground() {
        for s in species::load() {
            // The Grande Chaîne spans two cells in length, and one across.
            let (long, scale) = if s.carries.contains(&Carry::Boeufs) { (1.0, BOSS_SCALE) } else { (0.45, SCALE) };
            for part in parts(s.shape, &s.carries, s.hostile) {
                let (min, max) = part.bounds();
                assert!(min.y >= -0.001, "{} {part:?} sinks to {}", s.name, min.y);
                for v in [min.x, max.x] {
                    assert!(v.abs() <= long / scale, "{} {part:?} reaches x {v}", s.name);
                }
                for v in [min.z, max.z] {
                    assert!(v.abs() <= 0.45 / scale, "{} {part:?} reaches z {v}", s.name);
                }
            }
        }
    }

    #[test]
    fn the_grande_chaine_is_assembled() {
        let all = species::load();
        let boss = all.iter().find(|s| s.name == "grande_chaine").unwrap();
        let parts = parts(boss.shape, &boss.carries, boss.hostile);
        let boxy = parts.iter().find(|p| p.paint == Paint::Coat).unwrap();
        let ahead = |p: &&Part| p.at.translation.x > boxy.at.translation.x;
        let hide: Vec<&Part> = parts.iter().filter(|p| p.paint == Paint::Hide).collect();
        assert_eq!(hide.iter().filter(|p| p.role == Role::Leg).count(), 8, "two oxen of four legs");
        assert_eq!(hide.iter().filter(|p| matches!(p.form, Form::Cuboid(_))).count(), 4, "a body and a head each");
        assert!(hide.iter().all(ahead), "the oxen walk in front of the cart: {hide:?}");
        assert_eq!(parts.iter().filter(|p| p.paint == Paint::Linen).count(), 4, "two horns each");
        let yoke = parts.iter().find(|p| p.paint == Paint::Wood && matches!(p.form, Form::Cuboid(c) if c.z > 0.2));
        let yoke = yoke.expect("a yoke across both necks");
        assert!(ahead(&yoke) && yoke.at.translation.z == 0.0, "{yoke:?}");
        let lamp = parts.iter().find(|p| p.paint == Paint::Lamp && matches!(p.form, Form::Sphere(_)));
        assert!(lamp.expect("a sighting lantern").at.translation.x > boxy.at.translation.x);
        let top = boxy.bounds().1.y;
        let head = parts.iter().find(|p| p.paint == Paint::Wood && matches!(p.form, Form::Sphere(_)));
        assert!(head.expect("Gast's head").bounds().0.y >= top, "Gast stands on the box");
        assert!(parts.iter().any(|p| p.paint == Paint::Slate), "in the Chancellerie's cloth");
        // The rompue phase keeps the same cart.
        let broken = all.iter().find(|s| s.name == "grande_chaine_rompue").unwrap();
        assert_eq!(broken.carries, boss.carries);
    }

    /// The legs of one animal: two swing each way, and the diagonal ones swing together.
    fn assert_trots(legs: &[&Part], who: &str) {
        let n = legs.len() as f32;
        let (mx, mz) = (
            legs.iter().map(|p| p.at.translation.x).sum::<f32>() / n,
            legs.iter().map(|p| p.at.translation.z).sum::<f32>() / n,
        );
        assert_eq!(legs.len(), 4, "{who}");
        assert_eq!(legs.iter().filter(|p| p.side == 1.0).count(), 2, "{who}");
        assert_eq!(legs.iter().filter(|p| p.side == -1.0).count(), 2, "{who}");
        for p in legs {
            let diagonal = (p.at.translation.x - mx).signum() * (p.at.translation.z - mz).signum();
            assert_eq!(p.side, diagonal, "{who}: {p:?}");
        }
    }

    #[test]
    fn legs_alternate_per_animal() {
        let all = species::load();
        let of = |name: &str| {
            let s = all.iter().find(|s| s.name == name).unwrap();
            parts(s.shape, &s.carries, s.hostile)
        };
        let harde = of("harde");
        assert_trots(&harde.iter().filter(|p| p.role == Role::Leg).collect::<Vec<_>>(), "harde");
        let boss = of("grande_chaine");
        let legs: Vec<&Part> = boss.iter().filter(|p| p.role == Role::Leg).collect();
        for sign in [1.0, -1.0] {
            let ox: Vec<&Part> = legs.iter().copied().filter(|p| p.at.translation.z * sign > 0.0).collect();
            assert_trots(&ox, "ox");
        }
        let stilts = of("echassier");
        let legs: Vec<&Part> = stilts.iter().filter(|p| p.role == Role::Leg).collect();
        assert_eq!(legs.iter().map(|p| p.side).sum::<f32>(), 0.0);
        assert!(legs.iter().all(|p| p.side == p.at.translation.z.signum()), "{legs:?}");
    }

    #[test]
    fn small_parts_cast_no_shadow() {
        assert!(!casts_shadow(&Form::Cylinder(0.03, 0.015)), "a seal");
        assert!(!casts_shadow(&Form::Cylinder(0.008, 0.05)), "a horn");
        assert!(casts_shadow(&Form::Frustum { bottom: 0.10, top: 0.055, height: 0.26 }), "a coat");
    }

    #[test]
    fn figurines_stand_on_felt_carts_on_wheels() {
        for shape in [Shape::Pied, Shape::Echasses, Shape::Bete] {
            assert_eq!(count(&parts(shape, &[], true), Paint::Felt), 1, "{shape:?}");
        }
        let cart = parts(Shape::Chariot, &[], true);
        assert_eq!(count(&cart, Paint::Felt), 0);
        assert_eq!(cart.iter().filter(|p| p.role == Role::Wheel).count(), 4);
    }

    #[test]
    fn the_chaineur_wears_its_chain_and_bicorne() {
        let all = species::load();
        let chaineur = all.iter().find(|s| s.name == "chaineur").unwrap();
        let parts = parts(chaineur.shape, &chaineur.carries, chaineur.hostile);
        assert!(parts.iter().any(|p| p.paint == Paint::Iron && matches!(p.form, Form::Torus { .. })), "{parts:?}");
        let head = parts.iter().find(|p| matches!(p.form, Form::Sphere(_))).unwrap();
        let hat = parts.iter().find(|p| matches!(p.form, Form::Cuboid(_)) && p.paint == Paint::Wood).unwrap();
        assert!(hat.bounds().0.y >= head.at.translation.y, "hat {hat:?} under head {head:?}");
    }

    /// (number of parts, bounding box rounded to 0.01) of a species' figurine.
    fn signature(s: &species::Species) -> (usize, [i32; 6]) {
        let parts = parts(s.shape, &s.carries, s.hostile);
        let (mut min, mut max) = (Vec3::splat(f32::MAX), Vec3::splat(f32::MIN));
        for part in &parts {
            let (lo, hi) = part.bounds();
            (min, max) = (min.min(lo), max.max(hi));
        }
        let r = |v: f32| (v * 100.0).round() as i32;
        (parts.len(), [r(min.x), r(min.y), r(min.z), r(max.x), r(max.y), r(max.z)])
    }

    #[test]
    fn every_species_has_its_own_silhouette() {
        let all = species::load();
        for (i, a) in all.iter().enumerate() {
            for b in &all[i + 1..] {
                // The boss's phases share one cart.
                if a.name.starts_with(&b.name) || b.name.starts_with(&a.name) {
                    continue;
                }
                assert_ne!(signature(a), signature(b), "{} and {} look alike", a.name, b.name);
            }
        }
    }

    /// WCAG contrast ratio between two colours.
    fn contrast(a: Color, b: Color) -> f32 {
        let luminance = |c: Color| {
            let c = c.to_linear();
            0.2126 * c.red + 0.7152 * c.green + 0.0722 * c.blue
        };
        let (a, b) = (luminance(a), luminance(b));
        (a.max(b) + 0.05) / (a.min(b) + 0.05)
    }

    #[test]
    fn every_coat_reads_on_the_fields() {
        use crate::map::grid::Tile;
        use crate::theme::{MEADOW, PASTURE, WHEAT};
        let fields = [PASTURE, MEADOW, WHEAT, crate::map::look(Tile::Glace).1];
        for s in species::load() {
            let coat = Color::srgb(s.color.0, s.color.1, s.color.2);
            for field in fields {
                let ratio = contrast(coat, field);
                assert!(ratio >= 2.0, "{} on {field:?}: contrast {ratio:.2}", s.name);
            }
        }
    }

    #[test]
    fn no_coat_is_sealing_wax() {
        let seal = crate::theme::SEAL.to_srgba();
        for s in species::load() {
            let d =
                ((s.color.0 - seal.red).powi(2) + (s.color.1 - seal.green).powi(2) + (s.color.2 - seal.blue).powi(2))
                    .sqrt();
            assert!(d > 0.25, "{} is {d:.2} from the seal", s.name);
        }
    }

    #[test]
    fn props_of_the_people() {
        let all = species::load();
        let of = |name: &str| {
            let s = all.iter().find(|s| s.name == name).unwrap();
            parts(s.shape, &s.carries, s.hostile)
        };
        let heads = |parts: &[Part]| -> Vec<Part> {
            parts.iter().filter(|p| p.paint == Paint::Wood && matches!(p.form, Form::Sphere(_))).copied().collect()
        };
        let fuyard = of("fuyard");
        assert_eq!(count(&fuyard, Paint::Seal), 0);
        let heads = heads(&fuyard);
        assert_eq!(heads.len(), 2, "a head and a child's");
        let own = heads.iter().map(|h| h.at.translation.y).fold(f32::MIN, f32::max);
        assert!(heads.iter().any(|h| h.at.translation.y < own), "{heads:?}");

        let jalonneur = of("jalonneur");
        let top = |p: &Part| p.bounds().1.y;
        let head_top = jalonneur.iter().filter(|p| matches!(p.form, Form::Sphere(_))).map(top).fold(f32::MIN, f32::max);
        assert!(jalonneur.iter().any(|p| top(p) > head_top + 0.05), "the jalon stands above the head");
    }
}
