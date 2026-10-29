//! The signed editor: a safe RON preview for community sheets.

use bevy::prelude::*;

use super::{Screen, back, button, label, page, text};
use crate::campaign::save::Save;
use crate::i18n::t;
use crate::level::data::bonus;
use crate::theme::{INK, SMALL_SIZE, SUBTITLE_SIZE, VEIL};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Editor), spawn);
}

fn spawn(mut commands: Commands, assets: Res<AssetServer>, save: Res<Save>) {
    let sample = bonus(6);
    let ron = format!("(name: {:?}, world: {}, map: {:?})", sample.name, sample.world, sample.map);
    commands.spawn(page(Screen::Editor, VEIL)).with_children(|p| {
        p.spawn(text(&assets, t("editor.title"), SUBTITLE_SIZE));
        p.spawn(text(&assets, t("editor.description"), SMALL_SIZE));
        p.spawn((
            Text::new(ron),
            TextFont { font_size: FontSize::Px(SMALL_SIZE), ..default() },
            TextColor(INK),
            Node {
                max_width: percent(86),
                max_height: px(420),
                overflow: Overflow::scroll(),
                ..default()
            },
        ));
        if save.ending().is_some() {
            p.spawn(text(&assets, t("editor.ready"), SMALL_SIZE));
        }
        p.spawn(button(label(&assets, t("common.back")))).observe(back);
    });
}
