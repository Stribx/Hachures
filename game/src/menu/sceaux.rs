//! L'arbre des sceaux (docs, section 7): three branches, bought node by node with the sceaux the levels gave.
//! Open from the title, from the page at the end of a level, or with A during play.

use bevy::input::keyboard::Key;
use bevy::prelude::*;
use bevy::ui_widgets::Activate;

use super::{BackTo, Screen, back, button, framed, label, open, page, text};
use crate::campaign::LoadLevel;
use crate::campaign::save::Save;
use crate::campaign::skills::{BRANCHES, next_in};
use crate::i18n::{t, tp};
use crate::level::Level;
use crate::theme::{HEADING_SIZE, INK, SMALL_SIZE, SUBTITLE_SIZE, TITLE_SIZE, VEIL};
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

/// The line with the sceaux left to spend.
#[derive(Component)]
struct Purse;

fn spawn(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(page(Screen::Sceaux, VEIL)).with_children(|p| {
        p.spawn(text(&assets, t("sceaux.title"), TITLE_SIZE));
        p.spawn((text(&assets, "", SUBTITLE_SIZE), Purse));
        p.spawn(Node { column_gap: px(24), ..default() }).with_children(|row| {
            for (b, branch) in BRANCHES.iter().enumerate() {
                row.spawn(Node { flex_direction: FlexDirection::Column, row_gap: px(8), ..default() }).with_children(
                    |column| {
                        column.spawn(text(&assets, t(branch.name), HEADING_SIZE));
                        for k in 0..branch.nodes.len() {
                            column
                                .spawn(framed(300.0, (text(&assets, "", SMALL_SIZE), NodeLabel(b, k))))
                                .observe(buy(b, k));
                        }
                    },
                );
            }
        });
        p.spawn(text(&assets, t("sceaux.note"), SMALL_SIZE));
        p.spawn(button(label(&assets, t("common.back")))).observe(back);
    });
}

/// Button action: buy node `k` of `branch`, if it is the next one and the sceaux are there.
fn buy(branch: usize, k: usize) -> impl Fn(On<Activate>, ResMut<Save>, Res<Level>, MessageWriter<LoadLevel>) {
    move |_, mut save, level, mut load| {
        let node = &BRANCHES[branch].nodes[k];
        if next_in(branch, &save.skills).is_none_or(|n| n.skill != node.skill) || save.sceaux() < node.cost {
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
) {
    let n = save.sceaux();
    purse.0 = tp("sceaux.purse", n, &[&n, &save.names()]);
    for (&NodeLabel(b, k), mut text, mut color) in &mut nodes {
        let node = &BRANCHES[b].nodes[k];
        let owned = save.skills.contains(&node.skill);
        let next = next_in(b, &save.skills).is_some_and(|n| n.skill == node.skill) && save.sceaux() >= node.cost;
        let owned_said = if owned { t("sceaux.owned") } else { "" };
        text.0 = format!("{} ({}) : {}{owned_said}", t(node.name), node.cost, t(node.effect));
        color.0 = if owned || next { INK } else { INK.with_alpha(0.4) };
    }
}
