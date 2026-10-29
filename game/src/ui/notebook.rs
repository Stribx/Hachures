//! The page of Nève's notebook at the end of a level: what held, the sceaux, the names scratched
//! out, and the choice, made by striking out the line not taken.

use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::ui_widgets::Activate;

use super::DuringPlay;
use super::hud::capitalized;
use super::widgets::{Panel, Rise, framed, primary};
use crate::campaign::save::Save;
use crate::i18n::{t, tf};
use crate::level::data::LEVELS;
use crate::level::{Level, LevelEntity};
use crate::menu::open;
use crate::overworld::plan::WalkTo;
use crate::overworld::travel::Travel;
use crate::schedule::Screen;
use crate::theme::{BODY_SIZE, Face, HEADING_SIZE, INK, PAPER, SMALL_SIZE, SUBTITLE_SIZE, font};

/// The page, written for the option picked and the sceaux at the time: it is written again when
/// they change.
#[derive(Component, PartialEq)]
pub struct Notebook {
    picked: Option<String>,
    sceaux: u32,
}

const WIDTH: f32 = 560.0;

/// Ink text, faded by `alpha`.
fn line(assets: &AssetServer, text: impl Into<String>, size: f32, alpha: f32) -> impl Bundle {
    (Text::new(text), font(assets, Face::Roman, size), TextColor(INK.with_alpha(alpha)))
}

/// Opens the page once the level is won or lost, and writes it again when the choice changes.
pub fn sync(
    mut commands: Commands,
    assets: Res<AssetServer>,
    level: Res<Level>,
    save: Res<Save>,
    page: Query<(Entity, &Notebook)>,
) {
    let over = level.finished || level.lives == 0;
    let picked = level.choice.as_ref().and_then(|c| level.choices.get(&c.id).cloned());
    let now = Notebook { picked, sceaux: save.sceaux() };
    match page.single() {
        Ok((page, written)) if !over || *written != now => commands.entity(page).despawn(),
        Err(_) if over => spawn(&mut commands, &assets, &level, now),
        _ => {}
    }
}

fn spawn(commands: &mut Commands, assets: &AssetServer, level: &Level, written: Notebook) {
    let (picked, sceaux) = (written.picked.clone(), written.sceaux);
    commands
        .spawn((
            written,
            LevelEntity,
            DuringPlay,
            // The map stays still under the page.
            Hovered::default(),
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|screen| {
            let mut page = screen.spawn((
                Panel,
                UiTransform::IDENTITY,
                BackgroundColor(PAPER),
                Node {
                    width: px(WIDTH),
                    flex_direction: FlexDirection::Column,
                    row_gap: px(10),
                    padding: UiRect::all(px(24)),
                    ..default()
                },
            ));
            // The page comes in once; written again for a choice, it stays put.
            if picked.is_none() {
                page.insert(Rise::default());
            }
            page.with_children(|p| {
                spawn_header(p, assets, level);
                spawn_erased(p, assets, level);
                spawn_choice(p, assets, level, picked.as_ref());
                spawn_actions(p, assets, level, sceaux);
            });
        });
}

/// The title, the level, and what the victory brought.
fn spawn_header(p: &mut ChildSpawnerCommands, assets: &AssetServer, level: &Level) {
    let defends = capitalized(t(&level.world().defends));
    let won = level.finished;
    let title =
        if won { tf("notebook.held", &[&defends]) } else { tf("notebook.fell", &[&defends]) };
    p.spawn(line(assets, title, HEADING_SIZE, 1.0));
    p.spawn(line(assets, t(&level.name), BODY_SIZE, 0.7));
    if won {
        p.spawn(line(assets, t("notebook.victory_seal"), BODY_SIZE, 1.0));
        if let Some(challenge) = &level.challenge {
            let met = if level.challenge_met() {
                t("notebook.challenge_met")
            } else {
                t("notebook.challenge_missed")
            };
            let said = tf("notebook.challenge", &[&t(&challenge.text), &met]);
            p.spawn(line(assets, said, BODY_SIZE, 1.0));
        }
    }
}

/// Le prix du nom, paid this level.
fn spawn_erased(p: &mut ChildSpawnerCommands, assets: &AssetServer, level: &Level) {
    if level.erased.is_empty() {
        p.spawn(line(assets, t("notebook.no_name_erased"), BODY_SIZE, 1.0));
    } else {
        p.spawn(line(assets, t("notebook.names_erased"), BODY_SIZE, 1.0));
        p.spawn(Node { flex_wrap: FlexWrap::Wrap, column_gap: px(16), ..default() }).with_children(
            |names| {
                for &k in &level.erased {
                    names.spawn((
                        line(assets, t(&level.places[k].name), BODY_SIZE, 0.6),
                        Strikethrough,
                    ));
                }
            },
        );
    }
}

/// The choice of a won level: the prompt, and one button per option, the ones not taken struck out.
fn spawn_choice(
    p: &mut ChildSpawnerCommands,
    assets: &AssetServer,
    level: &Level,
    picked: Option<&String>,
) {
    let Some(choice) = level.choice.as_ref().filter(|_| level.finished) else { return };
    p.spawn(line(assets, t(&choice.prompt), SUBTITLE_SIZE, 1.0));
    for (k, (option, label)) in choice.options.iter().enumerate() {
        let struck = picked.is_some_and(|o| o != option);
        let label = line(
            assets,
            format!("{} · {}", k + 1, t(label)),
            SMALL_SIZE,
            if struck { 0.4 } else { 1.0 },
        );
        let mut button = if struck {
            p.spawn(framed(WIDTH - 48.0, (label, Strikethrough)))
        } else {
            p.spawn(framed(WIDTH - 48.0, label))
        };
        button.observe(move |_: On<Activate>, mut level: ResMut<Level>, mut save: ResMut<Save>| {
            level.choose(k, &mut save);
            save.store();
        });
    }
}

/// The buttons at the foot of the page.
fn spawn_actions(p: &mut ChildSpawnerCommands, assets: &AssetServer, level: &Level, sceaux: u32) {
    let (won, index) = (level.finished, level.index);
    p.spawn(Node { column_gap: px(12), margin: UiRect::top(px(8)), ..default() }).with_children(
        |actions| {
            if !won {
                actions
                    .spawn(framed(0.0, line(assets, t("notebook.retry"), BODY_SIZE, 1.0)))
                    .observe(move |_: On<Activate>, mut travel: MessageWriter<Travel>| {
                        travel.write(Travel { to: index, again: true });
                    });
                return;
            }
            if level.chosen() && index + 1 < LEVELS.len() {
                actions
                    .spawn(primary(0.0, line(assets, t("notebook.next_level"), BODY_SIZE, 1.0)))
                    .observe(move |_: On<Activate>, mut walks: MessageWriter<WalkTo>| {
                        walks.write(WalkTo(index + 1));
                    });
            } else if level.chosen() {
                // The last level: the campaign so far, read back.
                actions
                    .spawn(framed(0.0, line(assets, t("notebook.read_carnet"), BODY_SIZE, 1.0)))
                    .observe(open(Screen::Carnet, Screen::Playing));
            }
            let sceaux = tf("notebook.sceaux", &[&sceaux]);
            actions
                .spawn(framed(0.0, line(assets, sceaux, BODY_SIZE, 1.0)))
                .observe(open(Screen::Sceaux, Screen::Playing));
        },
    );
}
