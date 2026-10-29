//! How enemies look: each species' parts as meshes and inks, made once, and the shard and borne
//! handles.

use bevy::prelude::*;

use super::chain::{LINK_MAJOR, LINK_MINOR};
use super::kit::{Form, Paint, Role, casts_shadow, parts};
use super::spawn::FLOAT;
use crate::level::Level;
use crate::look::InkMaterial;
use crate::theme::{FELT, HIDE, IRON, LAMP, LIMESTONE, LINEN, SEAL, SEPIA, SLATE, WOOD};

/// One part of a figurine, ready to spawn: its mesh, place, ink, role, side and whether it casts a
/// shadow.
pub struct Piece {
    pub mesh: Handle<Mesh>,
    pub at: Transform,
    pub material: Handle<InkMaterial>,
    pub role: Role,
    pub side: f32,
    pub casts_shadow: bool,
}

#[derive(Resource, Default)]
pub struct EnemyLook {
    /// The pieces of each species (by its index), placed around the figure's ground.
    pub figures: Vec<Vec<Piece>>,
    /// The coat's ink of each species, the one its figure is painted in.
    pub coats: Vec<Handle<InkMaterial>>,
    /// The pieces of a borne, placed around its cell's centre.
    pub borne: Vec<Piece>,
    pub link_mesh: Handle<Mesh>,
    pub link_material: Handle<InkMaterial>,
    pub shard_mesh: Handle<Mesh>,
    pub shard_material: Handle<InkMaterial>,
    pub drop_mesh: Handle<Mesh>,
    pub blot_mesh: Handle<Mesh>,
    /// The sépia of the droplets and the blot.
    pub sepia_material: Handle<InkMaterial>,
}

/// Fine strokes, so a small piece does not turn to scribble; `gloss` for the varnished wood.
fn figurine_ink(color: Color, gloss: f32) -> InkMaterial {
    let mut material = crate::look::fine(color);
    material.extension.uniform.gloss = gloss;
    material
}

fn mesh(form: Form) -> Mesh {
    match form {
        Form::Frustum { bottom, top, height } => {
            ConicalFrustum { radius_top: top, radius_bottom: bottom, height }.into()
        }
        Form::Sphere(radius) => Sphere::new(radius).into(),
        Form::Cylinder { radius, height } => Cylinder::new(radius, height).into(),
        Form::Cuboid(size) => Cuboid::from_size(size).into(),
        Form::Torus { minor, major } => Torus { minor_radius: minor, major_radius: major }.into(),
    }
}

/// A limestone marker standing on the ground, capped in seal-red.
fn borne(
    meshes: &mut Assets<Mesh>,
    stone: Handle<InkMaterial>,
    cap: Handle<InkMaterial>,
) -> Vec<Piece> {
    let at = |y: f32| Transform::from_xyz(0.0, y - FLOAT, 0.0);
    vec![
        Piece {
            mesh: meshes.add(Cuboid::new(0.16, 0.36, 0.16)),
            at: at(0.18),
            material: stone,
            role: Role::Body,
            side: 0.0,
            casts_shadow: true,
        },
        Piece {
            mesh: meshes.add(Cylinder::new(0.09, 0.04)),
            at: at(0.38),
            material: cap,
            role: Role::Body,
            side: 0.0,
            casts_shadow: true,
        },
    ]
}

pub fn setup(
    mut commands: Commands,
    level: Res<Level>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<InkMaterial>>,
) {
    let felt = materials.add(figurine_ink(FELT, 0.0));
    let wood = materials.add(figurine_ink(WOOD, 0.5));
    let linen = materials.add(figurine_ink(LINEN, 0.0));
    let seal = materials.add(figurine_ink(SEAL, 0.0));
    let iron = materials.add(figurine_ink(IRON, 0.0));
    let hide = materials.add(figurine_ink(HIDE, 0.0));
    let slate = materials.add(figurine_ink(SLATE, 0.0));
    let lamp = materials.add(figurine_ink(LAMP, 0.5));
    let mut coats = Vec::new();
    let mut figures = Vec::new();
    for s in &level.species {
        let coat = materials.add(figurine_ink(Color::srgb(s.color.0, s.color.1, s.color.2), 0.0));
        let mut pieces = Vec::new();
        for part in parts(s.shape, &s.carries, s.hostile) {
            let paint = match part.paint {
                Paint::Coat => &coat,
                Paint::Felt => &felt,
                Paint::Wood => &wood,
                Paint::Linen => &linen,
                Paint::Seal => &seal,
                Paint::Iron => &iron,
                Paint::Hide => &hide,
                Paint::Slate => &slate,
                Paint::Lamp => &lamp,
            };
            pieces.push(Piece {
                mesh: meshes.add(mesh(part.form)),
                at: part.at,
                material: paint.clone(),
                role: part.role,
                side: part.side,
                casts_shadow: casts_shadow(&part.form),
            });
        }
        coats.push(coat);
        figures.push(pieces);
    }
    let stone = materials.add(figurine_ink(LIMESTONE, 0.0));
    let borne = borne(&mut meshes, stone, seal.clone());
    commands.insert_resource(EnemyLook {
        figures,
        coats,
        borne,
        link_mesh: meshes.add(Torus { minor_radius: LINK_MINOR, major_radius: LINK_MAJOR }),
        link_material: iron,
        shard_mesh: meshes.add(Cuboid::from_length(0.06)),
        shard_material: materials.add(crate::look::ink(Color::srgb(0.12, 0.08, 0.05))),
        drop_mesh: meshes.add(Sphere::new(0.04)),
        blot_mesh: meshes.add(Cylinder::new(0.22, 0.005)),
        sepia_material: materials.add(crate::look::ink(SEPIA)),
    });
}
