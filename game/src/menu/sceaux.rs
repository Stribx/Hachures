//! L'arbre des sceaux (docs, section 7): three branches, bought node by node with the sceaux the
//! levels gave. Open from the title, from the page at the end of a level, or with A during play.

use bevy::input::keyboard::Key;
use bevy::prelude::*;
use bevy::ui_widgets::Activate;

use super::{BackTo, Screen, back, button, framed, label, open, page, text};
use crate::campaign::LoadLevel;
use crate::campaign::save::Save;
use crate::campaign::skills::{BRANCHES, next_in};
use crate::i18n::{t, tf, tp};
use crate::level::Level;
use crate::theme::{Face, HEADING_SIZE, INK, SMALL_SIZE, SUBTITLE_SIZE, TITLE_SIZE, VEIL, font};
use crate::ui::widgets::letter;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Sceaux), spawn)
        .add_systems(Update, (show_nodes.run_if(in_state(Screen::Sceaux)), key_a));
}

/// Button action: open the tree, coming back to `from`.
pub fn open_sceaux(from: Screen) -> impl Fn(On<Activate>, Commands, ResMut<NextState<Screen>>) {
    open(Screen::Sceaux, from)
}

/// A during play opens the tree; A again closes it.
fn key_a(
    keys: Res<ButtonInput<Key>>,
    screen: Res<State<Screen>>,
    from: Option<Res<BackTo>>,
    mut commands: Commands,
    mut next: ResMut<NextState<Screen>>,
) {
    if !letter(&keys, "a") {
        return;
    }
    match screen.get() {
        Screen::Playing => {
            commands.insert_resource(BackTo(Screen::Playing));
            next.set(Screen::Sceaux);
        }
        Screen::Sceaux => next.set(from.map_or(Screen::Title, |f| f.0)),
        _ => {}
    }
}

/// The label of node `.1` of branch `.0`.
#[derive(Component, Clone, Copy)]
struct NodeLabel(usize, usize);

/// The ", acquis" after the words of node `.1` of branch `.0`: always there, so owning the node
/// never wraps its words again, and seen only once owned. (The languages put it last.)
#[derive(Component, Clone, Copy)]
struct Owned(usize, usize);

/// Width of a node, and of a branch's column.
const NODE_WIDTH: f32 = 300.0;

/// The line with the sceaux left to spend.
#[derive(Component)]
struct Purse;

fn spawn(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(page(Screen::Sceaux, VEIL)).with_children(|p| {
        p.spawn(text(&assets, t("sceaux.title"), TITLE_SIZE));
        p.spawn((text(&assets, "", SUBTITLE_SIZE), Purse));
        // A grid, filled row by row: the branches' names, then their first nodes, and so on. The
        // nodes of a row share its height, so the tree lines up whatever each node says.
        let grid = Node {
            display: Display::Grid,
            grid_template_columns: RepeatedGridTrack::px(BRANCHES.len() as u16, NODE_WIDTH),
            column_gap: px(24),
            row_gap: px(8),
            ..default()
        };
        p.spawn(grid).with_children(|grid| {
            for branch in &BRANCHES {
                grid.spawn(text(&assets, t(branch.name), HEADING_SIZE));
            }
            for k in 0..BRANCHES[0].nodes.len() {
                for b in 0..BRANCHES.len() {
                    let mark = (
                        TextSpan::new(t("sceaux.owned")),
                        font(&assets, Face::Roman, SMALL_SIZE),
                        TextColor(Color::NONE),
                        Owned(b, k),
                    );
                    let words = (text(&assets, "", SMALL_SIZE), NodeLabel(b, k), children![mark]);
                    grid.spawn(framed(NODE_WIDTH, words)).observe(buy(b, k));
                }
            }
        });
        p.spawn(text(&assets, t("sceaux.note"), SMALL_SIZE));
        p.spawn(button(label(&assets, t("common.back")))).observe(back);
    });
}

/// Button action: buy node `k` of `branch`, if it is the next one and the sceaux are there.
fn buy(
    branch: usize,
    k: usize,
) -> impl Fn(On<Activate>, ResMut<Save>, Res<Level>, MessageWriter<LoadLevel>) {
    move |_, mut save, level, mut load| {
        let node = &BRANCHES[branch].nodes[k];
        if next_in(branch, &save.skills).is_none_or(|n| n.skill != node.skill)
            || save.sceaux() < node.cost
        {
            return;
        }
        save.skills.push(node.skill);
        save.store();
        // Nothing inked or sent yet: the level takes the skill right away.
        if level.used.is_empty() && level.wave.number == 0 {
            load.write(LoadLevel(level.index));
        }
    }
}

/// Owned nodes say so; the ones out of reach fade.
fn show_nodes(
    save: Res<Save>,
    mut purse: Single<&mut Text, With<Purse>>,
    mut nodes: Query<(&NodeLabel, &mut Text, &mut TextColor), Without<Purse>>,
    mut marks: Query<(&Owned, &mut TextColor), Without<NodeLabel>>,
) {
    let owned = |b: usize, k: usize| save.skills.contains(&BRANCHES[b].nodes[k].skill);
    for (&Owned(b, k), mut color) in &mut marks {
        let ink = if owned(b, k) { INK } else { Color::NONE };
        if color.0 != ink {
            color.0 = ink;
        }
    }
    let n = save.sceaux();
    purse.set_if_neq(Text(tp("sceaux.purse", n, &[&n, &save.names()])));
    for (&NodeLabel(b, k), mut text, mut color) in &mut nodes {
        let node = &BRANCHES[b].nodes[k];
        let next = next_in(b, &save.skills).is_some_and(|n| n.skill == node.skill)
            && save.sceaux() >= node.cost;
        // The last slot is the "owned" mark, a span of its own (`Owned`).
        let said = tf("sceaux.node", &[&t(node.name), &node.cost, &t(node.effect), &""]);
        text.set_if_neq(Text(said));
        color.0 = if owned(b, k) || next { INK } else { INK.with_alpha(0.4) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The words of node 0 of branch 0 and the colour of its "owned" mark, after a frame.
    fn first_node(save: Save) -> (String, Color) {
        let mut app = App::new();
        app.insert_resource(save).add_systems(Update, show_nodes);
        app.world_mut().spawn((Text::default(), Purse));
        let node = app.world_mut().spawn((Text::default(), TextColor(INK), NodeLabel(0, 0))).id();
        let mark = app
            .world_mut()
            .spawn((TextSpan::default(), TextColor(INK), Owned(0, 0), ChildOf(node)))
            .id();
        app.update();
        let world = app.world();
        (world.get::<Text>(node).unwrap().0.clone(), world.get::<TextColor>(mark).unwrap().0)
    }

    #[test]
    fn owning_a_node_shows_its_mark_without_moving_its_words() {
        let (words, mark) = first_node(Save::default());
        assert_eq!(mark.alpha(), 0.0, "not owned: the mark keeps its room, unseen");
        let owned = Save { skills: vec![BRANCHES[0].nodes[0].skill], ..default() };
        let (owned_words, owned_mark) = first_node(owned);
        assert_eq!(owned_words, words);
        assert_eq!(owned_mark, INK);
    }
}
