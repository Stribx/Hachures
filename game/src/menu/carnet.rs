//! Le carnet de Nève at the end of the campaign: the Rôle des Noms, each level won with the names scratched out
//! there, the choices made (the line not taken struck out), and how each level closed.

use bevy::prelude::*;
use bevy::ui_widgets::Activate;

use super::{Screen, back, button, label, open, page, text};
use crate::campaign::save::Save;
use crate::i18n::{t, tf};
use crate::level::data::{LEVELS, LevelData};
use crate::level::story::closing_lines;
use crate::theme::{INK, SMALL_SIZE, SUBTITLE_SIZE, TITLE_SIZE, VEIL};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Carnet), spawn);
}

/// Button action: open the carnet, coming back to `from`.
pub fn open_carnet(from: Screen) -> impl Fn(On<Activate>, Commands, ResMut<NextState<Screen>>) {
    open(Screen::Carnet, from)
}

/// Whether the campaign's last level is won: the carnet can be read.
pub fn written(save: &Save) -> bool {
    save.levels.get(LEVELS.len() - 1).is_some_and(|r| r.won)
}

fn spawn(mut commands: Commands, assets: Res<AssetServer>, save: Res<Save>) {
    commands.spawn(page(Screen::Carnet, VEIL)).with_children(|p| {
        p.spawn(text(&assets, t("carnet.title"), TITLE_SIZE));
        p.spawn(text(&assets, tf("carnet.roll", &[&save.names()]), SUBTITLE_SIZE));
        p.spawn(Node {
            max_width: percent(94),
            flex_wrap: FlexWrap::Wrap,
            justify_content: JustifyContent::Center,
            column_gap: px(40),
            row_gap: px(16),
            ..default()
        })
        .with_children(|pages| {
            let won = save.levels.iter().enumerate().filter(|(_, r)| r.won);
            for (i, record) in won {
                let data = LevelData::of(i);
                pages
                    .spawn(Node {
                        width: px(420),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(2),
                        ..default()
                    })
                    .with_children(|level| {
                        level.spawn(text(&assets, t(&data.name), SUBTITLE_SIZE));
                        if record.erased.is_empty() {
                            level
                                .spawn(text(&assets, t("carnet.nothing_erased"), SMALL_SIZE))
                                .insert(TextColor(INK.with_alpha(0.6)));
                        }
                        for name in &record.erased {
                            level.spawn((text(&assets, t(name), SMALL_SIZE), Strikethrough));
                        }
                        if let Some(choice) = &data.choice
                            && let Some(picked) = save.choices.get(&choice.id)
                        {
                            level.spawn(text(&assets, t(&choice.prompt), SMALL_SIZE));
                            for (option, label) in &choice.options {
                                if option == picked {
                                    level.spawn(text(&assets, t(label), SMALL_SIZE));
                                } else {
                                    level
                                        .spawn((text(&assets, t(label), SMALL_SIZE), Strikethrough))
                                        .insert(TextColor(INK.with_alpha(0.5)));
                                }
                            }
                        }
                        for line in closing_lines(&data.notes, &save.choices) {
                            level.spawn(text(&assets, t(&line).to_string(), SMALL_SIZE));
                        }
                    });
            }
        });
        p.spawn(button(label(&assets, t("common.back")))).observe(back);
    });
}
