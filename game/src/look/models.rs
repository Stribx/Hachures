//! Kenney models spawn with their own materials; here they are swapped for ink as they appear.

use std::collections::HashMap;

use bevy::ecs::system::SystemParam;
use bevy::pbr::ExtendedMaterial;
use bevy::prelude::*;
use bevy::world_serialization::WorldInstanceReady;

use super::{Hachure, InkMaterial};

/// Ink materials already made from a model's own (and its carving, if any), so every tower shares them.
#[derive(Resource, Default)]
pub(super) struct Inked(HashMap<(AssetId<StandardMaterial>, Option<Carved>), Handle<InkMaterial>>);

/// A model carved for the plan-relief: its colour becomes `.0`'s wood. With `.1`, its painted texture shows
/// through the wood's tint, so its parts still read apart; without, it is one piece of plain wood.
#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Carved(pub [u8; 3], pub bool);

/// The materials a model's ink is made from, and where the made ones are kept.
#[derive(SystemParam)]
pub(super) struct Inking<'w> {
    standard: Res<'w, Assets<StandardMaterial>>,
    inks: ResMut<'w, Assets<InkMaterial>>,
    inked: ResMut<'w, Inked>,
}

impl Inking<'_> {
    /// The ink for a model's own material, made on first use; none if that material is gone.
    fn of(&mut self, material: &Handle<StandardMaterial>, carving: Option<Carved>) -> Option<Handle<InkMaterial>> {
        let base = self.standard.get(material)?;
        let handle = self
            .inked
            .0
            .entry((material.id(), carving))
            .or_insert_with(|| {
                let mut base = base.clone();
                if let Some(Carved([r, g, b], painted)) = carving {
                    base.base_color = Color::srgb_u8(r, g, b);
                    if !painted {
                        base.base_color_texture = None;
                    }
                }
                self.inks.add(ExtendedMaterial { base, extension: Hachure::fine() })
            })
            .clone();
        Some(handle)
    }
}

/// A model just spawned: its materials become ink, its texture kept, unless it is `Carved`.
pub(super) fn ink_models(
    ready: On<WorldInstanceReady>,
    carved: Query<&Carved>,
    children: Query<&Children>,
    meshes: Query<&MeshMaterial3d<StandardMaterial>>,
    mut inking: Inking,
    mut commands: Commands,
) {
    let carving = carved.get(ready.entity).ok().copied();
    for entity in children.iter_descendants(ready.entity) {
        let Ok(material) = meshes.get(entity) else { continue };
        let Some(handle) = inking.of(&material.0, carving) else { continue };
        commands.entity(entity).remove::<MeshMaterial3d<StandardMaterial>>().insert(MeshMaterial3d(handle));
    }
}
