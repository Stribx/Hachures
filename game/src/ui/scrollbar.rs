//! A thin ink scrollbar beside a scroll area, shown only when the area holds more than it shows.

use bevy::prelude::*;
use bevy::ui::ComputedNode;
use bevy::ui_widgets::{ControlOrientation, Scrollbar, ScrollbarThumb};

use crate::theme::{INK, INK_SOFT};

/// Width of the bar, in pixels.
const WIDTH: f32 = 6.0;

/// The vertical bar of the scroll area `target`: a faint track and a thumb the pointer can drag.
/// It stands beside the area, hidden until `show_when_needed` finds the area overflowing.
pub(crate) fn scrollbar(target: Entity) -> impl Bundle {
    (
        Node { width: px(WIDTH), border_radius: BorderRadius::all(px(WIDTH / 2.0)), ..default() },
        BackgroundColor(INK.with_alpha(0.08)),
        Scrollbar::new(target, ControlOrientation::Vertical, 24.0),
        Visibility::Hidden,
        children![(
            ScrollbarThumb { border_radius: BorderRadius::all(px(WIDTH / 2.0)), ..default() },
            BackgroundColor(INK_SOFT.with_alpha(0.6)),
        )],
    )
}

/// Shows a bar while its area holds more than it shows, hides it (and its thumb, which would fill
/// the whole track) otherwise.
pub(crate) fn show_when_needed(
    mut bars: Query<(&Scrollbar, &mut Visibility)>,
    areas: Query<&ComputedNode>,
) {
    for (bar, mut visibility) in &mut bars {
        let Ok(area) = areas.get(bar.target) else { continue };
        // Half a pixel of rounding is not something to scroll.
        let overflows = area.content_size().y > area.size().y + 0.5;
        let shown = if overflows { Visibility::Inherited } else { Visibility::Hidden };
        if *visibility != shown {
            *visibility = shown;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An area `content` high showing 100, its bar, and the bar's visibility after a frame.
    fn bar_after(content: f32) -> Visibility {
        let mut app = App::new();
        app.add_systems(Update, show_when_needed);
        let area = app
            .world_mut()
            .spawn(ComputedNode {
                size: Vec2::splat(100.0),
                content_size: Vec2::new(100.0, content),
                ..default()
            })
            .id();
        let bar = app
            .world_mut()
            .spawn((Scrollbar::new(area, ControlOrientation::Vertical, 8.0), Visibility::Hidden))
            .id();
        app.update();
        *app.world().get::<Visibility>(bar).unwrap()
    }

    #[test]
    fn a_bar_shows_only_when_its_area_overflows() {
        assert_eq!(bar_after(300.0), Visibility::Inherited);
        assert_eq!(bar_after(100.0), Visibility::Hidden);
        assert_eq!(bar_after(40.0), Visibility::Hidden);
    }
}
