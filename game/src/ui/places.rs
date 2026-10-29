//! The names written on the sheet, struck out once inked over, and the price read at the cursor
//! before the click.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::widgets::{self, follow_cursor};
use crate::i18n::{t, tf};
use crate::ink::{Act, Selection, TOOLS, Verdict};
use crate::level::{Level, LevelEntity};
use crate::look::WorldCamera;
use crate::map;
use crate::schedule::{Screen, over_world};
use crate::theme::{BODY_SIZE, Face, INK, font};

/// The name of `level.places[.0]`, written over its cells.
#[derive(Component)]
pub struct PlaceLabel(usize);

/// What inking at the cursor would cost or why it is refused, next to the cursor.
#[derive(Component)]
pub struct PriceTip;

const WIDTH: f32 = 240.0;

pub fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((PriceTip, widgets::tooltip(&assets)));
}

/// A place's label: its node, colour and visibility, and whether it is struck out.
type Label<'a> = (
    Entity,
    &'a PlaceLabel,
    &'a mut Node,
    &'a mut TextColor,
    &'a mut Visibility,
    Has<Strikethrough>,
);

/// Writes the level's names over their cells (under the menus' paper), and strikes out the ones
/// inked over.
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
                font(&assets, Face::Italic, BODY_SIZE),
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
        visibility.set_if_neq(shown);
        // Hidden, a label needs no placing: a `Node` write lays the whole interface out again.
        if shown == Visibility::Hidden {
            continue;
        }
        let Some(place) = level.places.get(k) else { continue };
        let center = place
            .at
            .iter()
            .map(|&(x, y)| map::cell_pos(level.terrain.base(), y * width + x))
            .sum::<Vec3>()
            / place.at.len() as f32;
        if let Ok(p) = camera.world_to_viewport(at, center) {
            // Just under the cell centre, where the routes run: a route over a name must not read
            // as struck out.
            let (left, top) = (px(p.x - WIDTH / 2.0), px(p.y + 2.0));
            if (node.left, node.top) != (left, top) {
                (node.left, node.top) = (left, top);
            }
        }
        if level.erased.contains(&k) && !struck {
            commands.entity(entity).insert(Strikethrough);
            color.0 = INK.with_alpha(0.45);
        }
    }
}

/// The price before the click: a refusal, the name this ink would scratch out, or what biffing
/// gives back.
pub fn price_tip(
    level: Res<Level>,
    verdict: Res<Verdict>,
    selection: Res<Selection>,
    window: Single<&Window, With<PrimaryWindow>>,
    tip: Single<(&mut Text, &mut Node, &mut Visibility), With<PriceTip>>,
) {
    let (mut text, mut node, mut visibility) = tip.into_inner();
    let said = price(&level, &verdict, TOOLS[selection.tool].act);
    let (Some(said), Some(cursor)) = (said, window.cursor_position()) else {
        visibility.set_if_neq(Visibility::Hidden);
        return;
    };
    text.set_if_neq(Text(said));
    follow_cursor(&mut node, cursor);
    *visibility = Visibility::Inherited;
}

/// What the tooltip says for `act` at the verdict's cell, if anything.
fn price(level: &Level, verdict: &Verdict, act: Act) -> Option<String> {
    let i = verdict.at?;
    match verdict.refusal {
        Some(why) => Some(t(why).to_string()),
        None if act == Act::Biffer => level.refund(i).map(|r| tf("places.refund", &[&r])),
        None => act
            .names()
            .then(|| level.place_at(i))
            .flatten()
            .map(|p| tf("places.erases", &[&t(&level.places[p].name)])),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ink::tool_index;
    use crate::level::data::Place;
    use crate::map::grid::Grid;
    use crate::map::species;

    /// A level with "le Pré commun" on cell 2 and a barricade Nève inked on cell 8.
    fn level() -> Level {
        let mut level = Level::new(Grid::parse("S...M\n.....\n....."), species::load());
        level.places = vec![Place { name: "place.pre_commun".into(), at: vec![(2, 0)] }];
        level.ink(8, tool_index("barricade").unwrap());
        level
    }

    fn at(i: usize) -> Verdict {
        Verdict { at: Some(i), refusal: None, pencil: None }
    }

    fn act(name: &str) -> Act {
        TOOLS[tool_index(name).unwrap()].act
    }

    #[test]
    fn biffer_says_what_comes_back() {
        assert_eq!(price(&level(), &at(8), act("biffer")), Some(tf("places.refund", &[&10])));
    }

    #[test]
    fn a_tile_laid_on_a_name_warns_it_is_erased() {
        let erases = tf("places.erases", &[&t("place.pre_commun")]);
        assert_eq!(price(&level(), &at(2), act("barricade")), Some(erases));
    }

    /// Gratter lays nothing, so it never scratches a name out (docs, section 7).
    #[test]
    fn gratter_never_warns_of_a_name() {
        assert_eq!(price(&level(), &at(2), act("gratter")), None);
    }

    #[test]
    fn a_refusal_says_why() {
        let refused = Verdict { refusal: Some("refusal.nothing_to_biffer"), ..at(2) };
        assert_eq!(
            price(&level(), &refused, act("biffer")),
            Some(t("refusal.nothing_to_biffer").to_string())
        );
    }
}
