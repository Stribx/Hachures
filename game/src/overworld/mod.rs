//! Les Marches: one long strip of land that every level's sheet is cut out of. Hills, lakes, fields and woods
//! around the boards, the road from sheet to sheet, and each sheet not being played drawn as a miniature.
//!
//! The level being played always stays at the origin (combat, picking and the overlay all count from there);
//! the rest of the world is shifted under it instead, through `OverworldRoot`.

mod bocage;
mod camera;
mod card;
mod clouds;
pub mod onward;
mod parcels;
pub mod plan;
mod props;
mod road;
mod terrain;
pub mod travel;

use std::sync::LazyLock;

use bevy::light::{NotShadowCaster, NotShadowReceiver};
use bevy::prelude::*;

use crate::Step;
use crate::campaign::save::Save;
use crate::level::Level;
use crate::level::data::{LEVELS, LevelData, Refuge};
use crate::look::{InkMaterial, Swaying, ink, shore_image, water};
use crate::map;
use crate::map::fields;
use crate::map::grid::Grid;
use props::props;
use terrain::{EAST, NORTH, SOUTH, WATER, WEST, height, land_mesh, road_mesh};

/// How far a tree leans in the wind, per world unit of height squared: a canopy a few units up sways a few
/// hundredths, its trunk not at all.
const TREE_WIND: f32 = 0.012;
pub fn plugin(app: &mut App) {
    app.add_plugins((travel::plugin, plan::plugin))
        .add_systems(Startup, (spawn, bocage::spawn_bocage).chain())
        .add_systems(Update, follow_level.before(camera::aim))
        .add_systems(Update, onward::between_levels.before(crate::campaign::load_level).in_set(Step::Input));
}

/// A level's sheet as it lies on the Marches.
pub struct Sheet {
    /// World position of its cell (0, 0).
    pub at: Vec2,
    pub grid: Grid,
}

impl Sheet {
    /// Width and depth in cells.
    pub fn size(&self) -> Vec2 {
        Vec2::new(self.grid.width as f32, self.grid.height as f32)
    }

    /// World position of the middle of the sheet.
    pub fn center(&self) -> Vec2 {
        self.at + (self.size() - 1.0) / 2.0
    }

    /// How far `p` lies outside the sheet's edge (0 on or inside it).
    fn distance(&self, p: Vec2) -> f32 {
        let min = self.at - 0.6;
        let max = self.at + self.size() - 0.4;
        (min - p).max(p - max).max(Vec2::ZERO).length()
    }
}

pub static SHEETS: LazyLock<Vec<Sheet>> = LazyLock::new(|| {
    (0..LEVELS.len())
        .map(|i| {
            let data = LevelData::of(i);
            // The alternative Refuge only shows once played.
            Sheet { at: Vec2::from(data.at), grid: Grid::parse(&data.map_text(Refuge::Usual)) }
        })
        .collect()
});

/// The road from each sheet to the next: from the east edge of its wooden bed to the west edge of the next one's,
/// leaving and entering each bed square to its edge, so the ribbon's ends lie flush against them.
pub static ROAD: LazyLock<Vec<Vec<Vec2>>> = LazyLock::new(|| {
    SHEETS
        .windows(2)
        .map(|pair| {
            let (a, b) = (&pair[0], &pair[1]);
            // A sheet's cells end half a cell past their centres, and its bed reaches 0.1 further.
            let start = Vec2::new(a.at.x + a.size().x - 0.4, a.center().y);
            let end = Vec2::new(b.at.x - 0.6, b.center().y);
            let reach = Vec2::new((end.x - start.x) / 2.0, 0.0);
            let (c1, c2) = (start + reach, end - reach);
            (0..=48)
                .map(|i| {
                    let t = i as f32 / 48.0;
                    let s = 1.0 - t;
                    start * (s * s * s) + c1 * (3.0 * s * s * t) + c2 * (3.0 * s * t * t) + end * (t * t * t)
                })
                .collect()
        })
        .collect()
});

/// A sheet's blocks merged into one mesh, as the map draws them; a sheet not reached yet is blank paper.
fn sheet_mesh(sheet: &Sheet, reached: bool) -> Mesh {
    let grid = &sheet.grid;
    let mut mesh =
        block(Vec3::new(grid.width as f32 + 0.2, 0.3, grid.height as f32 + 0.2), sheet_bed(sheet), crate::theme::FRAME);
    for (i, wash) in fields::ground(grid).into_iter().enumerate() {
        let (x, z) = grid.coords(i);
        let (height, color) =
            if reached { (map::look(grid.tiles[i]).height, wash) } else { (0.2, crate::theme::SHEET) };
        let at = Vec3::new(x as f32, height / 2.0, z as f32);
        mesh.merge(&block(Vec3::new(0.96, height, 0.96), at, color)).expect("same attributes");
    }
    mesh
}

/// Centre of a sheet's bed, in the sheet's own coordinates.
fn sheet_bed(sheet: &Sheet) -> Vec3 {
    let half = (sheet.size() - 1.0) / 2.0;
    Vec3::new(half.x, -0.12, half.y)
}

fn block(size: Vec3, at: Vec3, color: Color) -> Mesh {
    let mesh = Mesh::from(Cuboid::from_size(size)).transformed_by(Transform::from_translation(at));
    let count = mesh.count_vertices();
    mesh.with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, vec![color.to_linear().to_f32_array(); count])
}

/// Everything around the board being played; shifted so that board sits at the origin.
#[derive(Component)]
pub struct OverworldRoot;

/// The miniature of sheet `.0`, hidden while that sheet is the one on the board.
#[derive(Component)]
pub struct Miniature(pub usize);

pub(super) fn spawn(
    save: Res<Save>,
    assets: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<InkMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let mut land = ink(Color::WHITE);
    land.extension.uniform.params.x = 0.16;
    land.extension.uniform.paper.alpha = 0.9;
    let land = materials.add(land);
    // The same resin as the board's water, lined from the lakes' shores; seen from higher, the lines spaced wider.
    let (min, size) = (Vec2::new(WEST, NORTH), Vec2::new(EAST - WEST, SOUTH - NORTH));
    let (shore, rect) = shore_image(min, size, 4.0, |p| height(p) < WATER);
    let mut lakes = water(crate::theme::WATER, images.add(shore), rect, 1.0);
    lakes.extension.uniform.params.x = 0.4;
    let water = materials.add(lakes);
    let root = commands
        .spawn((OverworldRoot, Transform::default(), Visibility::default()))
        .with_children(|root| {
            root.spawn((Mesh3d(meshes.add(land_mesh())), MeshMaterial3d(land.clone())));
            // Lying on the land, it would catch the land's own shadow as stripes.
            root.spawn((
                Mesh3d(meshes.add(road_mesh())),
                MeshMaterial3d(materials.add(ink(Color::srgb(0.86, 0.8, 0.66)))),
                NotShadowReceiver,
                NotShadowCaster,
            ));
            let (size, middle) = (Vec2::new(EAST - WEST, SOUTH - NORTH), Vec2::new(EAST + WEST, SOUTH + NORTH) / 2.0);
            root.spawn((
                Mesh3d(meshes.add(Plane3d::new(Vec3::Y, size / 2.0))),
                MeshMaterial3d(water),
                Transform::from_xyz(middle.x, WATER, middle.y),
            ));
        })
        .id();

    let mut blank = ink(Color::WHITE);
    // A sheet not reached yet: blank paper, hatched all over.
    blank.extension.uniform.params.w = 1.0;
    let blank = materials.add(blank);
    for (index, sheet) in SHEETS.iter().enumerate() {
        let reached = index <= save.reached;
        let material = if reached { land.clone() } else { blank.clone() };
        commands.spawn((
            Miniature(index),
            Mesh3d(meshes.add(sheet_mesh(sheet, reached))),
            MeshMaterial3d(material),
            Transform::from_xyz(sheet.at.x, 0.0, sheet.at.y),
            ChildOf(root),
        ));
    }

    for (model, transform) in props() {
        let mut prop = commands.spawn((
            WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset(format!("models/{model}.glb")))),
            transform,
            ChildOf(root),
        ));
        if model.contains("tree") {
            prop.insert(Swaying(TREE_WIND));
        }
    }
}

/// A new level on the board: the world shifts so its sheet lies at the origin (the camera along with it), and its
/// miniature hides.
fn follow_level(
    level: Res<Level>,
    mut shown: Local<Option<usize>>,
    mut root: Single<&mut Transform, With<OverworldRoot>>,
    mut miniatures: Query<(&Miniature, &mut Visibility)>,
    mut rig: ResMut<camera::CameraRig>,
    mut pawn: ResMut<plan::Pawn>,
) {
    if *shown == Some(level.index) {
        return;
    }
    *shown = Some(level.index);
    let at = SHEETS[level.index].at;
    let shifted = Vec3::new(-at.x, 0.0, -at.y);
    // The camera moves with the world, so the swap can't be seen.
    rig.shift(shifted - root.translation);
    root.translation = shifted;
    // Nève stands on the sheet being played, however it was loaded.
    pawn.stand(level.index);
    for (miniature, mut visibility) in &mut miniatures {
        *visibility = if miniature.0 == level.index { Visibility::Hidden } else { Visibility::Inherited };
    }
}

#[cfg(test)]
mod tests {
    use super::props::CLEAR;
    use super::*;

    /// Every sheet lies on flat ground, clear of props, and the road between them is dry.
    #[test]
    fn sheets_lie_flat_and_clear() {
        for sheet in SHEETS.iter() {
            let (min, max) = (sheet.at, sheet.at + sheet.size() - 1.0);
            for p in [min, max, Vec2::new(min.x, max.y), Vec2::new(max.x, min.y), sheet.center()] {
                assert!(height(p).abs() < 0.1, "a sheet's ground is flat: {} at {p}", height(p));
            }
        }
        for (_, transform) in props() {
            let p = transform.translation.xz();
            assert!(SHEETS.iter().all(|s| s.distance(p) > CLEAR), "a prop stands on a sheet at {p}");
        }
        for p in ROAD.iter().flatten() {
            assert!(height(*p) > WATER, "the road is under water at {p}");
        }
        // Each leg leaves and enters a bed square to its edge, so the ribbon's ends lie flush.
        for (k, leg) in ROAD.iter().enumerate() {
            let (a, b, n) = (&SHEETS[k], &SHEETS[k + 1], leg.len());
            assert_eq!(leg[0].x, a.at.x + a.size().x - 0.4, "leg {k} starts on the east edge");
            assert_eq!(leg[n - 1].x, b.at.x - 0.6, "leg {k} ends on the west edge");
            assert!((leg[1].y - leg[0].y).abs() < 0.05, "leg {k} leaves square: {} {}", leg[0], leg[1]);
            assert!((leg[n - 1].y - leg[n - 2].y).abs() < 0.05, "leg {k} arrives square");
        }
    }
}
