//! The tooltip of a tool card: what the tool is and does, while the card is hovered or just picked by its key.

use bevy::ecs::system::SystemParam;
use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::bar::ToolButton;
use super::hud::capitalized;
use super::widgets::tooltip;
use crate::campaign::lexique::tool_lines;
use crate::campaign::save::Save;
use crate::campaign::skills::Skill;
use crate::i18n::t;
use crate::ink::TOOLS;
use crate::level::Level;

/// The words of the tool under the cursor, above its card.
#[derive(Component)]
pub struct ToolTip;

/// Seconds a tool picked by its key keeps its tip.
const KEYED: f32 = 2.0;

/// The widest the tip grows, in pixels.
const WIDTH: f32 = 360.0;

pub fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((ToolTip, tooltip(&assets)));
}

/// The tool picked by its key, and the seconds left on its tip.
#[derive(Default)]
pub struct Keyed(Option<(usize, f32)>);

/// What the tip reads and what it waits for.
#[derive(SystemParam)]
pub struct Reads<'w> {
    time: Res<'w, Time>,
    keys: Res<'w, ButtonInput<KeyCode>>,
    level: Res<'w, Level>,
    save: Res<'w, Save>,
}

/// Shows the tip of the hovered card, or for 2 s of the card just picked by its key: the hover wins.
pub fn tool_tip(
    reads: Reads,
    window: Single<&Window, With<PrimaryWindow>>,
    cards: Query<(&ToolButton, &Hovered, &ComputedNode, &UiGlobalTransform)>,
    mut keyed: Local<Keyed>,
    tip: Single<(&mut Text, &mut Node, &mut Visibility), With<ToolTip>>,
) {
    let (mut text, mut node, mut visibility) = tip.into_inner();
    let Reads { time, keys, level, save } = reads;
    if let Some(&tool) = level.tools.iter().find(|&&t| keys.just_pressed(TOOLS[t].key)) {
        keyed.0 = Some((tool, KEYED));
    }
    if let Some((_, left)) = &mut keyed.0 {
        *left -= time.delta_secs();
    }
    keyed.0 = keyed.0.filter(|&(_, left)| left > 0.0);
    let hovered = cards.iter().find(|(_, hovered, ..)| hovered.get());
    let shown = hovered.or_else(|| keyed.0.and_then(|(tool, _)| cards.iter().find(|(card, ..)| card.0 == tool)));
    let Some((&ToolButton(tool), _, size, at)) = shown else {
        *visibility = Visibility::Hidden;
        return;
    };
    let words = words(&level, &save, tool);
    if text.0 != words {
        text.0 = words;
    }
    // Just above the card, its left edge on the card's.
    let (left, top) = ((at.translation.x - size.size().x / 2.0), (at.translation.y - size.size().y / 2.0));
    node.left = px(left * size.inverse_scale_factor);
    node.top = Val::Auto;
    // In UI units, as the card's own size: the window in logical pixels would miss any UI scale.
    node.bottom = px((window.physical_height() as f32 - top) * size.inverse_scale_factor + 8.0);
    node.max_width = px(WIDTH);
    *visibility = Visibility::Inherited;
}

/// The name, what it is, and its numbers: the species not yet met are left out of them.
fn words(level: &Level, save: &Save, tool: usize) -> String {
    let met: Vec<_> = level.species.iter().filter(|s| save.seen.contains(&s.label)).cloned().collect();
    let lines = tool_lines(tool, level.tool_cost(tool), &level.tower_stats, &met, level.has(Skill::Equerre));
    let label = TOOLS[tool].label;
    let mut all = vec![capitalized(t(label)), t(&format!("{label}.text")).to_string()];
    all.extend(lines.iter().map(|l| l.shown()));
    all.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ink::tool_index;
    use crate::map::grid::Grid;
    use crate::map::species;

    #[test]
    fn a_hovered_card_shows_its_numbers() {
        let mut app = App::new();
        let mut level = Level::new(Grid::parse("S.M"), species::load());
        level.tools = vec![0, tool_index("arbalète").unwrap()];
        app.add_plugins(MinimalPlugins).insert_resource(level).init_resource::<Save>();
        app.init_resource::<ButtonInput<KeyCode>>();
        app.world_mut().spawn((Window::default(), PrimaryWindow));
        let tip = app.world_mut().spawn((ToolTip, Text::default(), Node::default(), Visibility::Hidden)).id();
        let card = (ComputedNode::default(), UiGlobalTransform::default());
        let arbalete = app.world_mut().spawn((ToolButton(tool_index("arbalète").unwrap()), Hovered(true), card)).id();
        app.add_systems(Update, tool_tip);

        app.update();
        assert_eq!(*app.world().get::<Visibility>(tip).unwrap(), Visibility::Inherited);
        let words = &app.world().get::<Text>(tip).unwrap().0;
        assert!(words.starts_with("Arbalète\n"), "{words}");
        assert!(words.contains("30"), "{words}");

        app.world_mut().entity_mut(arbalete).insert(Hovered(false));
        app.update();
        assert_eq!(*app.world().get::<Visibility>(tip).unwrap(), Visibility::Hidden);
    }

    #[test]
    fn a_key_shows_the_tip_for_two_seconds() {
        let mut app = App::new();
        let mut level = Level::new(Grid::parse("S.M"), species::load());
        let arbalete = tool_index("arbalète").unwrap();
        level.tools = vec![arbalete];
        app.add_plugins(MinimalPlugins).insert_resource(level).init_resource::<Save>();
        app.init_resource::<ButtonInput<KeyCode>>();
        app.world_mut().spawn((Window::default(), PrimaryWindow));
        let tip = app.world_mut().spawn((ToolTip, Text::default(), Node::default(), Visibility::Hidden)).id();
        let card = (ComputedNode::default(), UiGlobalTransform::default());
        app.world_mut().spawn((ToolButton(arbalete), Hovered(false), card));
        app.add_systems(Update, tool_tip);

        app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(TOOLS[arbalete].key);
        app.update();
        assert_eq!(*app.world().get::<Visibility>(tip).unwrap(), Visibility::Inherited);
        app.world_mut().resource_mut::<ButtonInput<KeyCode>>().clear_just_pressed(TOOLS[arbalete].key);
        app.update();
        assert_eq!(*app.world().get::<Visibility>(tip).unwrap(), Visibility::Inherited);
    }
}
