//! The clouds that close over the world while a level is swapped for another.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::travel::Trip;
use crate::theme::{FRAME, SHEET};
use crate::ui::widgets::Skin;

/// The clouds' layer, over everything but the cursor.
#[derive(Component)]
pub(super) struct Clouds;

/// A cloud: its middle in screen fractions when open (off screen) and closed (over the screen).
#[derive(Component)]
pub(super) struct Cloud {
    open: Vec2,
    closed: Vec2,
}

/// The paper behind the clouds once they meet, so no gap shows the swap.
#[derive(Component)]
pub(super) struct Fill;

/// A cloud's width on a 16:9 screen, in screen fractions. The drawings are 1.6:1, kept as drawn.
const CLOUD_WIDTH: f32 = 0.75;
const CLOUD_RATIO: f32 = 1.6;

/// Width and height of a cloud, in screen fractions, on a screen `aspect` wide for 1 high. On a narrower screen
/// (a portrait window, a tall browser tab) the clouds widen past its sides, so they stand as tall as on 16:9 and
/// the rows still overlap.
fn cloud_size(aspect: f32) -> Vec2 {
    let width = CLOUD_WIDTH * (16.0 / 9.0 / aspect).max(1.0);
    Vec2::new(width, width * aspect / CLOUD_RATIO)
}

pub(super) fn spawn_clouds(assets: Res<AssetServer>, mut commands: Commands) {
    let middle = Vec2::splat(0.5);
    // Closed, they overlap in rows over the whole screen.
    let closed = [
        (0.1, 0.12),
        (0.5, 0.05),
        (0.9, 0.15),
        (0.2, 0.45),
        (0.75, 0.5),
        (0.45, 0.6),
        (0.05, 0.85),
        (0.5, 0.95),
        (0.95, 0.88),
    ];
    commands
        .spawn((
            Clouds,
            Node {
                width: percent(100),
                height: percent(100),
                position_type: PositionType::Absolute,
                ..default()
            },
            GlobalZIndex(100),
            Pickable::IGNORE,
            Visibility::Hidden,
        ))
        .with_children(|layer| {
            layer.spawn((
                Fill,
                Node {
                    width: percent(100),
                    height: percent(100),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                BackgroundColor(SHEET.with_alpha(0.0)),
            ));
            for (i, &(x, y)) in closed.iter().enumerate() {
                let closed = Vec2::new(x, y);
                let away = (closed - middle).try_normalize().unwrap_or(Vec2::Y);
                layer.spawn((
                    Cloud { open: closed + away * 1.1, closed },
                    Skin {
                        image: assets.load(format!("ui/cloud-{}.png", i % 4 + 1)),
                        slice: BorderRect::ZERO,
                    },
                    Node {
                        width: percent(CLOUD_WIDTH * 100.0),
                        aspect_ratio: Some(CLOUD_RATIO),
                        position_type: PositionType::Absolute,
                        border: UiRect::all(px(2)),
                        border_radius: BorderRadius::all(percent(50)),
                        ..default()
                    },
                    // Until the drawn cloud is there: a round of paper, inked around.
                    BackgroundColor(Color::srgb(0.97, 0.95, 0.9)),
                    BorderColor::all(FRAME),
                ));
            }
        });
}

/// Clouds whose drawing has just come in.
type NewlyDrawn = (With<Cloud>, Added<ImageNode>);

/// A drawn cloud: its white is the paper's cream, and its round fallback shape no longer clips it.
pub(super) fn ink_clouds(mut clouds: Query<(&mut ImageNode, &mut Node), NewlyDrawn>) {
    for (mut image, mut node) in &mut clouds {
        image.color = SHEET.lighter(0.04);
        node.border_radius = BorderRadius::ZERO;
    }
}

pub(super) fn place_clouds(
    trip: Option<Res<Trip>>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut layer: Single<&mut Visibility, With<Clouds>>,
    mut fill: Single<&mut BackgroundColor, With<Fill>>,
    mut clouds: Query<(&Cloud, &mut Node)>,
) {
    let t = trip.map_or(0.0, |trip| trip.cover);
    **layer = if t > 0.0 { Visibility::Inherited } else { Visibility::Hidden };
    fill.0 = SHEET.with_alpha(((t - 0.8) / 0.2).clamp(0.0, 1.0));
    let size = cloud_size(window.width() / window.height().max(1.0));
    for (cloud, mut node) in &mut clouds {
        let at = cloud.open.lerp(cloud.closed, t) - size / 2.0;
        node.width = percent(size.x * 100.0);
        node.left = percent(at.x * 100.0);
        node.top = percent(at.y * 100.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// As drawn on 16:9; as tall on a portrait screen, and never narrower on a wide one.
    #[test]
    fn clouds_keep_their_height_on_any_screen() {
        let wide = cloud_size(16.0 / 9.0);
        assert_eq!(wide, Vec2::new(0.75, 0.75 * 16.0 / 9.0 / 1.6));
        let tall = cloud_size(9.0 / 16.0);
        assert!((tall.y - wide.y).abs() < 1e-6 && tall.x > 1.0, "{tall}");
        let cinema = cloud_size(21.0 / 9.0);
        assert!(cinema.x == 0.75 && cinema.y > wide.y, "{cinema}");
    }
}
