//! What the player points at and has picked: the cell under the cursor, the tool, the route shown,
//! le Calque.

use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::Step;
use crate::campaign::{LoadLevel, load_level};
use crate::level::Level;
use crate::look::WorldCamera;
use crate::schedule::Screen;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, init)
        .add_systems(Update, (hover, reset.after(load_level)).in_set(Step::Input))
        // No stale pencil line behind the menus.
        .add_systems(OnExit(Screen::Playing), |mut selection: ResMut<Selection>| {
            selection.hovered = None
        });
}

#[derive(Resource, Debug, Clone, PartialEq)]
pub struct Selection {
    /// Index in `TOOLS` of the tool in hand.
    pub tool: usize,
    /// The species whose route is shown (Tab).
    pub shown: usize,
    /// Le Calque (F1) is on.
    pub calque: bool,
    /// The cell under the cursor.
    pub hovered: Option<usize>,
}

impl Selection {
    /// What a level starts with: its first tool, its first species, nothing else.
    pub fn for_level(level: &Level) -> Selection {
        Selection {
            tool: level.tools[0],
            shown: level.present.first().copied().unwrap_or(0),
            calque: false,
            hovered: None,
        }
    }
}

fn init(mut commands: Commands, level: Res<Level>) {
    commands.insert_resource(Selection::for_level(&level));
}

/// A level was just loaded: back to its first tool and species.
pub(super) fn reset(
    mut loads: MessageReader<LoadLevel>,
    level: Res<Level>,
    mut selection: ResMut<Selection>,
) {
    if loads.read().last().is_some() {
        *selection = Selection::for_level(&level);
    }
}

/// Cell under the mouse cursor, unless the cursor is on the bar or a page.
pub(crate) fn hover(
    level: Res<Level>,
    mut selection: ResMut<Selection>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<WorldCamera>>,
    panels: Query<&Hovered>,
) {
    if panels.iter().any(Hovered::get) {
        // Written only when it changes: the verdict is judged again on a change.
        if selection.hovered.is_some() {
            selection.hovered = None;
        }
        return;
    }
    let (camera, camera_transform) = *camera;
    let grid = level.terrain.grid();
    let hit = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .and_then(|ray| ray.plane_intersection_point(Vec3::Y * 0.2, InfinitePlane3d::new(Vec3::Y)));
    let hovered = hit.and_then(|p| grid.cell_at(p.x, p.z));
    if selection.hovered != hovered {
        selection.hovered = hovered;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_bare_level_starts_on_the_first_tool_and_species() {
        let level = Level::parse("S...M");
        assert_eq!(
            Selection::for_level(&level),
            Selection { tool: 0, shown: 0, calque: false, hovered: None }
        );
    }
}
