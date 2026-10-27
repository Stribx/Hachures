//! The names written on the sheet, struck out once inked over, and the price read at the cursor before the click.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::widgets::{self, follow_cursor};
use crate::i18n::{t, tf};
use crate::ink::Selection;
use crate::level::{Level, LevelEntity};
use crate::look::WorldCamera;
use crate::map::cell_pos;
use crate::schedule::{Screen, over_world};
use crate::theme::{BODY_SIZE, INK, font};

/// The name of `level.places[.0]`, written over its cells.
#[derive(Component)]
pub struct PlaceLabel(usize);

/// What inking at the cursor would cost or why it is refused, next to the cursor.
#[derive(Component)]
pub struct Tooltip;

const WIDTH: f32 = 240.0;

pub fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((Tooltip, widgets::tooltip(&assets)));
}

/// Writes the level's names over their cells (under the menus' paper), and strikes out the ones inked over.
/// A place's label: its node, colour and visibility, and whether it is struck out.
type Label<'a> = (Entity, &'a PlaceLabel, &'a mut Node, &'a mut TextColor, &'a mut Visibility, Has<Strikethrough>);

pub fn label_places(
    mut commands: Commands,
    assets: Res<AssetServer>,
    level: Res<Level>,
    camera: Single<(&Camera, &GlobalTransform), With<WorldCamera>>,
    screen: Res<State<Screen>>,
    mut labels: Query<Label>,
) {
    // A new level cleared the old labels (they are `LevelEntity`): write its own.
    if labels.is_empty() {
        for (k, place) in level.places.iter().enumerate() {
            commands.spawn((
                LevelEntity,
                PlaceLabel(k),
                Text::new(t(&place.name)),
                font(&assets, true, BODY_SIZE),
                TextColor(INK),
                TextLayout::justify(Justify::Center),
                Node { position_type: PositionType::Absolute, width: px(WIDTH), ..default() },
                // Under the title, pause and settings pages.
                GlobalZIndex(-1),
            ));
        }
        return;
    }
    let (camera, at) = *camera;
    let width = level.terrain.base().width;
    let shown = if over_world(screen) { Visibility::Hidden } else { Visibility::Inherited };
    for (entity, &PlaceLabel(k), mut node, mut color, mut visibility, struck) in &mut labels {
        *visibility = shown;
        let Some(place) = level.places.get(k) else { continue };
        let center = place.at.iter().map(|&(x, y)| cell_pos(level.terrain.base(), y * width + x)).sum::<Vec3>()
            / place.at.len() as f32;
        if let Ok(p) = camera.world_to_viewport(at, center) {
            node.left = px(p.x - WIDTH / 2.0);
            // Just under the cell centre, where the routes run: a route over a name must not read as struck out.
            node.top = px(p.y + 2.0);
        }
        if level.erased.contains(&k) && !struck {
            commands.entity(entity).insert(Strikethrough);
            color.0 = INK.with_alpha(0.45);
        }
    }
}

/// The price before the click: a refusal, or the name this ink would scratch out.
pub fn tooltip(
    level: Res<Level>,
    selection: Res<Selection>,
    window: Single<&Window, With<PrimaryWindow>>,
    tip: Single<(&mut Text, &mut Node, &mut Visibility), With<Tooltip>>,
) {
    let (mut text, mut node, mut visibility) = tip.into_inner();
    let said = selection.hovered.filter(|_| !level.finished).and_then(|i| match level.refusal(i, selection.tool) {
        Some(why) => Some(t(why).to_string()),
        None => level.place_at(i).map(|p| tf("places.erases", &[&t(&level.places[p].name)])),
    });
    let (Some(said), Some(cursor)) = (said, window.cursor_position()) else {
        *visibility = Visibility::Hidden;
        return;
    };
    text.0 = said;
    follow_cursor(&mut node, cursor);
    *visibility = Visibility::Inherited;
}
