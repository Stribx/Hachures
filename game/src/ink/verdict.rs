//! The verdict on the cell under the hand, made once a frame: may the tool in hand be inked there,
//! and the route if it were.

use bevy::prelude::*;

use super::Selection;
use crate::level::Level;
use crate::map::flow::FlowField;
use crate::map::grid::Grid;

#[derive(Resource, Default)]
pub struct Verdict {
    /// The cell under the hand, while the level is played.
    pub at: Option<usize>,
    /// Why the tool in hand can't be inked there.
    pub refusal: Option<&'static str>,
    /// The map and the shown species' field if it were: le trait de crayon.
    pub pencil: Option<(Grid, FlowField)>,
}

impl Verdict {
    pub fn of(level: &Level, selection: &Selection) -> Verdict {
        let Some(i) = selection.hovered.filter(|_| !level.finished) else {
            return Verdict::default();
        };
        let refusal = level.refusal(i, selection.tool);
        let pencil = refusal.is_none().then(|| level.what_if(i, selection.tool, selection.shown));
        Verdict { at: Some(i), refusal, pencil }
    }
}

/// Judges the cell under the hand once, after any click, for everything drawn this frame.
pub(super) fn judge(level: Res<Level>, selection: Res<Selection>, mut verdict: ResMut<Verdict>) {
    *verdict = Verdict::of(&level, &selection);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hovering(i: usize) -> (Level, Selection) {
        let level = Level::parse("S.....M\n.......");
        let selection = Selection { hovered: Some(i), ..Selection::for_level(&level) };
        (level, selection)
    }

    /// Once the level is won, the hand reads nothing: no frame, no pencil line, no price.
    #[test]
    fn a_finished_level_has_no_verdict() {
        let (mut level, selection) = hovering(3);
        level.finished = true;
        let verdict = Verdict::of(&level, &selection);
        assert_eq!((verdict.at, verdict.refusal, verdict.pencil.is_none()), (None, None, true));
    }

    /// Without the sépia, the hand is refused, says why, and draws no pencil line.
    #[test]
    fn a_refused_cell_says_why() {
        let (mut level, selection) = hovering(3);
        level.sepia = 0;
        let verdict = Verdict::of(&level, &selection);
        assert_eq!(
            (verdict.at, verdict.refusal, verdict.pencil.is_none()),
            (Some(3), Some("refusal.sepia"), true)
        );
    }

    /// Where the tool may go, the pencil line is the route the ink will leave.
    #[test]
    fn the_pencil_line_is_the_route_once_inked() {
        let (level, selection) = hovering(3);
        let verdict = Verdict::of(&level, &selection);
        let (mut inked, _) = hovering(3);
        inked.ink(3, selection.tool);
        let (_, field) = verdict.pencil.expect("a barricade off the only way is allowed");
        assert_eq!(field.dist, inked.terrain.fields()[selection.shown].dist);
        assert_ne!(
            field.dist,
            level.terrain.fields()[selection.shown].dist,
            "the barricade moves the route"
        );
    }
}
