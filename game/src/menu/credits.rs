//! Les crédits: who made what the game is drawn, played and set with, read from `assets/CREDITS.md`
//! so the list has one source.

use bevy::prelude::*;
use bevy::ui_widgets::Activate;

use super::{Screen, back, label, open, page, primary, sheet, text};
use crate::i18n::t;
use crate::menu::BUTTON_WIDTH;
use crate::theme::{BODY_SIZE, INK_SOFT, SMALL_SIZE, SUBTITLE_SIZE, TITLE_SIZE, VEIL};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Credits), spawn);
}

/// Button action: open the credits, coming back to `from`.
pub fn open_credits(from: Screen) -> impl Fn(On<Activate>, Commands, ResMut<NextState<Screen>>) {
    open(Screen::Credits, from)
}

/// (asset, author, licence) of each row of the table in `CREDITS.md`.
fn rows(markdown: &str) -> Vec<(&str, &str, &str)> {
    markdown
        .lines()
        .filter(|line| line.starts_with("| `"))
        .filter_map(|line| {
            let cells: Vec<&str> = line.trim_matches('|').split('|').map(str::trim).collect();
            // | folder | asset | author | licence | source |
            Some((*cells.get(1)?, *cells.get(2)?, *cells.get(3)?))
        })
        .collect()
}

fn spawn(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(page(Screen::Credits, VEIL)).with_children(|page| {
        page.spawn(sheet()).with_children(|p| {
            p.spawn(text(&assets, t("menu.credits"), TITLE_SIZE));
            p.spawn(text(&assets, t("credits.subtitle"), SUBTITLE_SIZE));
            for (asset, author, licence) in rows(include_str!("../../assets/CREDITS.md")) {
                p.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|row| {
                    row.spawn(text(&assets, asset, BODY_SIZE));
                    // The licence's file is named in `CREDITS.md`, shipped beside the fonts.
                    let licence = licence.split(" (").next().unwrap_or(licence);
                    row.spawn(text(&assets, format!("{author} · {licence}"), SMALL_SIZE))
                        .insert(TextColor(INK_SOFT));
                });
            }
            p.spawn(primary(BUTTON_WIDTH, label(&assets, t("common.back")))).observe(back);
        });
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_credited_asset_is_read() {
        let rows = rows(include_str!("../../assets/CREDITS.md"));
        assert_eq!(rows.len(), 13, "update when CREDITS.md gains a row");
        assert_eq!(rows[3], ("Lonelyhood", "yd", "CC0"));
        assert_eq!(rows[4].1, "dhallcomposer", "the ambient sounds follow the music");
        assert_eq!(rows[11], ("The travel clouds", "AI-generated for Hachures", "CC0"));
    }
}
