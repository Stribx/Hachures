//! Towers standing on the board: their models, raised when inked and taken down when biffed.

use bevy::prelude::*;

use super::towers::Weapon;
use crate::ink::{TowerBiffed, TowerInked};
use crate::level::{Level, LevelEntity};
use crate::look::Carved;
use crate::map::grid::Grid;
use crate::map::land::Land;

#[derive(Component)]
pub struct Tower {
    pub cell: usize,
    pub weapon: Weapon,
    pub ready_in: f32,
}

/// The base under the tower on a cell.
#[derive(Component)]
pub struct TowerBase(pub usize);

/// A tower's base model; it stands 0.6 high, and the weapon sits on top.
pub const TOWER_BASE: &str = "models/tower-round-bottom-a.glb";

/// Carved for the plan-relief: a base of plain limewood, the weapon in walnut with its parts still
/// told apart.
pub const BASE_WOOD: Carved = Carved { rgb: [214, 186, 140], painted: false };
pub const WEAPON_WOOD: Carved = Carved { rgb: [196, 150, 110], painted: true };

/// The glTF asset path of the weapon's model.
pub fn weapon_model(weapon: Weapon) -> &'static str {
    match weapon {
        Weapon::Arbalete => "models/weapon-ballista.glb",
        Weapon::Mortier => "models/weapon-cannon.glb",
        Weapon::Harpon => "models/weapon-catapult.glb",
        Weapon::Lanterne => "models/weapon-turret.glb",
    }
}

/// Stands up the towers just inked.
pub(super) fn raise_towers(
    mut inked: MessageReader<TowerInked>,
    level: Res<Level>,
    assets: Res<AssetServer>,
    mut commands: Commands,
) {
    for &TowerInked { cell, weapon } in inked.read() {
        spawn_tower(&mut commands, &assets, level.terrain.grid(), cell, weapon);
    }
}

/// Takes down the towers just biffed, base and weapon.
pub(super) fn take_down(
    mut biffed: MessageReader<TowerBiffed>,
    towers: Query<(Entity, &Tower)>,
    bases: Query<(Entity, &TowerBase)>,
    mut commands: Commands,
) {
    for &TowerBiffed { cell } in biffed.read() {
        let weapon = towers.iter().filter(|(_, t)| t.cell == cell).map(|(e, _)| e);
        let base = bases.iter().filter(|(_, b)| b.0 == cell).map(|(e, _)| e);
        for entity in weapon.chain(base) {
            commands.entity(entity).despawn();
        }
    }
}

/// The tower's base and its weapon, on top of `cell`.
pub fn spawn_tower(
    commands: &mut Commands,
    assets: &AssetServer,
    grid: &Grid,
    cell: usize,
    weapon: Weapon,
) {
    let (x, z) = grid.coords(cell);
    let scene =
        |path: &'static str| WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset(path)));
    // Both land in the cell like the other pieces; the base makes the sound of a wooden figure set
    // down.
    let (base, top) = (
        Transform::from_xyz(x as f32, 0.2, z as f32),
        Transform::from_xyz(x as f32, 0.8, z as f32),
    );
    commands.spawn((
        LevelEntity,
        TowerBase(cell),
        scene(TOWER_BASE),
        BASE_WOOD,
        base,
        Land::new(base, Some("audio/sfx/impact/impactPlank_medium_000.ogg")),
    ));
    commands.spawn((
        LevelEntity,
        Tower { cell, weapon, ready_in: 0.0 },
        scene(weapon_model(weapon)),
        WEAPON_WOOD,
        top,
        Land::new(top, None),
    ));
}
