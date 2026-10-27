//! The widgets every screen draws with: ink text, framed and primary buttons, panels with their corner marks and
//! drawn skins, icon slots and rising pages, the systems that make them look and move, and the typed-letter check.

use bevy::input::keyboard::Key;
use bevy::input_focus::InputFocus;
use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::ui::{Pressed, VisualBox};
use bevy::ui_widgets::Button;

use crate::settings::Settings;
use crate::theme::{BODY_SIZE, INK, LIT, PAPER, PRESSED, SEAL, SEAL_DEEP, font};

/// Whether the letter `c` was just typed, wherever it sits on the keyboard (A is A on AZERTY too).
pub(crate) fn letter(keys: &ButtonInput<Key>, c: &str) -> bool {
    keys.just_pressed(Key::Character(c.into())) || keys.just_pressed(Key::Character(c.to_uppercase().into()))
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (mark_corners, skin, reveal_icons, primary_ink, button_look, rise).chain());
}

/// Ink text in the game's font.
pub(crate) fn text(assets: &AssetServer, text: impl Into<String>, size: f32) -> impl Bundle {
    (Text::new(text), font(assets, false, size), TextColor(INK))
}

/// A tooltip, hidden until a system writes its words and puts it next to the cursor.
pub(crate) fn tooltip(assets: &AssetServer) -> impl Bundle {
    (
        Text::default(),
        font(assets, true, BODY_SIZE),
        TextColor(INK),
        BackgroundColor(PAPER),
        Node { position_type: PositionType::Absolute, padding: UiRect::axes(px(8), px(2)), ..default() },
        Visibility::Hidden,
        // Over the panels.
        GlobalZIndex(50),
    )
}

/// Puts a tooltip just below and right of the cursor.
pub(crate) fn follow_cursor(node: &mut Node, cursor: Vec2) {
    node.left = px(cursor.x + 18.0);
    node.top = px(cursor.y + 18.0);
}

/// A button at least `min_width` wide.
pub(crate) fn framed(min_width: f32, label: impl Bundle) -> impl Bundle {
    frame(min_width, label, Color::NONE, INK)
}

fn frame(min_width: f32, label: impl Bundle, background: Color, border: Color) -> impl Bundle {
    (
        Button,
        Hovered::default(),
        Node {
            min_width: px(min_width),
            padding: UiRect::axes(px(16), px(6)),
            border: UiRect::all(px(1)),
            justify_content: JustifyContent::Center,
            ..default()
        },
        BorderColor::all(border),
        BackgroundColor(background),
        UiTransform::IDENTITY,
        children![label],
    )
}

/// The one action a page leads to, in the seal's red.
pub(crate) fn primary(min_width: f32, label: impl Bundle) -> impl Bundle {
    (frame(min_width, label, SEAL, SEAL), Primary)
}

#[derive(Component)]
pub(crate) struct Primary;

/// A sheet of paper for a panel, its corners marked like the neatline of a map.
#[derive(Component)]
pub(crate) struct Panel;

/// A drawn image (`docs/art-prompts.md`) that takes over a node's flat paper once it has loaded, 9-sliced `.1` px
/// in from its edges (zero: stretched whole). Until then, or if it never comes, the flat paper stays.
#[derive(Component)]
pub(crate) struct Skin(pub Handle<Image>, pub BorderRect);

/// An icon's place, kept empty (taking no room) until its drawn image has loaded.
#[derive(Component)]
pub(crate) struct IconSlot;

fn reveal_icons(mut slots: Query<(&mut Node, Has<ImageNode>), With<IconSlot>>) {
    for (mut node, drawn) in &mut slots {
        node.display = if drawn { Display::Flex } else { Display::None };
    }
}

/// A panel's four corner marks: short ink angles just inside the sheet, as on the edge of a survey sheet.
fn mark_corners(panels: Query<Entity, Added<Panel>>, mut commands: Commands) {
    const ARM: f32 = 12.0;
    const IN: f32 = 6.0;
    for panel in &panels {
        for (top, left) in [(true, true), (true, false), (false, true), (false, false)] {
            let (inset, rule) = (px(IN), px(1));
            commands.spawn((
                ChildOf(panel),
                Node {
                    position_type: PositionType::Absolute,
                    width: px(ARM),
                    height: px(ARM),
                    top: if top { inset } else { Val::Auto },
                    bottom: if top { Val::Auto } else { inset },
                    left: if left { inset } else { Val::Auto },
                    right: if left { Val::Auto } else { inset },
                    border: UiRect {
                        top: if top { rule } else { Val::ZERO },
                        bottom: if top { Val::ZERO } else { rule },
                        left: if left { rule } else { Val::ZERO },
                        right: if left { Val::ZERO } else { rule },
                    },
                    ..default()
                },
                BorderColor::all(INK),
                Pickable::IGNORE,
            ));
        }
    }
}

pub(crate) fn skin(
    assets: Res<AssetServer>,
    skins: Query<(Entity, &Skin), Without<ImageNode>>,
    mut commands: Commands,
) {
    for (entity, Skin(image, inset)) in &skins {
        if !assets.is_loaded(image) {
            continue;
        }
        let image_mode = if *inset != BorderRect::ZERO {
            NodeImageMode::Sliced(TextureSlicer { border: *inset, ..default() })
        } else {
            NodeImageMode::Stretch
        };
        commands.entity(entity).insert((
            // The frame covers the whole node, around its padding, not just the words inside.
            ImageNode { image: image.clone(), image_mode, visual_box: VisualBox::BorderBox, ..default() },
            BackgroundColor(Color::NONE),
            BorderColor::all(Color::NONE),
        ));
    }
}

/// A primary button's words are written in paper on the seal.
fn primary_ink(buttons: Query<&Children, Added<Primary>>, mut texts: Query<&mut TextColor>) {
    for children in &buttons {
        for &child in children {
            if let Ok(mut color) = texts.get_mut(child) {
                color.0 = PAPER;
            }
        }
    }
}

/// A rare page coming in: it rises 24 px and settles from 96 % (220 ms, ease-out).
#[derive(Component, Default)]
pub(crate) struct Rise(f32);

fn rise(
    time: Res<Time>,
    settings: Res<Settings>,
    mut pages: Query<(Entity, &mut Rise, &mut UiTransform)>,
    mut commands: Commands,
) {
    // With reduced motion, the page is simply there.
    let step = if settings.reduced_motion { 1.0 } else { time.delta_secs() / 0.22 };
    for (entity, mut rise, mut transform) in &mut pages {
        rise.0 = (rise.0 + step).min(1.0);
        let t = 1.0 - (1.0 - rise.0).powi(3);
        transform.translation = Val2::px(0.0, 24.0 * (1.0 - t));
        transform.scale = Vec2::splat(0.96 + 0.04 * t);
        if rise.0 >= 1.0 {
            commands.entity(entity).remove::<Rise>();
        }
    }
}

/// What `button_look` reads and moves on a button.
type ButtonLook<'a> = (Entity, &'a Hovered, Has<Pressed>, Has<Primary>, &'a mut BackgroundColor, &'a mut UiTransform);

/// Buttons darken under the mouse or the keyboard focus, and more while pressed. Under the mouse they also lift
/// 2 px, and pressed they give a little (0.97), unless motion is reduced; the focus from the keyboard only darkens,
/// at once.
fn button_look(
    time: Res<Time>,
    settings: Res<Settings>,
    focus: Res<InputFocus>,
    mut buttons: Query<ButtonLook, With<Button>>,
) {
    let still = settings.reduced_motion;
    // About 60 ms to get there.
    let ease = 1.0 - (-time.delta_secs() / 0.06).exp();
    for (entity, hovered, pressed, primary, mut background, mut transform) in &mut buttons {
        let lit = hovered.get() || focus.get() == Some(entity);
        let tint = match (primary, pressed, lit) {
            (true, true, _) => SEAL_DEEP.darker(0.05),
            (true, false, true) => SEAL_DEEP,
            (true, false, false) => SEAL,
            (false, true, _) => PRESSED,
            (false, false, true) => LIT,
            (false, false, false) => Color::NONE,
        };
        background.0 = tint;
        let scale = if pressed && !still { 0.97 } else { 1.0 };
        let lift = if hovered.get() && !pressed && !still { -2.0 } else { 0.0 };
        let y = match transform.translation.y {
            Val::Px(y) => y,
            _ => 0.0,
        };
        transform.scale = transform.scale.lerp(Vec2::splat(scale), ease);
        transform.translation = Val2::px(0.0, y + (lift - y) * ease);
    }
}
