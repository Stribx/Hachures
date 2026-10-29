//! Biffer, le repentir: which of Nève's strokes may be taken back, and the sépia it returns (docs,
//! section 7).

use super::TOOLS;
use crate::i18n::n_;
use crate::level::Level;
use crate::map::terrain::Edit;

impl Level {
    /// Index in `inked` of Nève's last stroke on `cell`, if it still lies there as she inked it.
    pub(super) fn stroke(&self, cell: usize) -> Option<usize> {
        let last = self.inked.iter().rposition(|stroke| stroke.cell == cell)?;
        (TOOLS[self.inked[last].tool].act.lays() == Some(self.terrain.base().tiles[cell]))
            .then_some(last)
    }

    /// Sépia that biffing `cell` returns: all it cost while the ink is wet (inked between waves,
    /// none sent since), half once dry.
    pub fn refund(&self, cell: usize) -> Option<u32> {
        let stroke = &self.inked[self.stroke(cell)?];
        let wet = stroke.laid_at == Some(self.wave.number) && !self.wave.running;
        Some(if wet { stroke.paid } else { stroke.paid / 2 })
    }

    /// Why Biffer can't be used at `cell`. Returning the cell to what it was opens ways, never
    /// closes one, so le Droit de Passage doesn't apply; it costs nothing.
    pub(super) fn biffer_refusal(&self, cell: usize) -> Option<&'static str> {
        self.stroke(cell).is_none().then_some(n_("refusal.nothing_to_biffer"))
    }

    /// Takes back the stroke at `cell` and its sépia; the name it cost stays scratched out. The
    /// caller has checked `refusal`.
    pub(super) fn biffer(&mut self, cell: usize) {
        let refund = self.refund(cell).expect("a stroke to biffer");
        let stroke = self.inked.remove(self.stroke(cell).expect("a stroke to biffer"));
        self.sepia += refund;
        self.alter([(cell, Edit::Unink(stroke.under))]);
    }
}

#[cfg(test)]
mod tests {
    use crate::campaign::skills::Skill;
    use crate::ink::tool_index;
    use crate::level::data::Place;
    use crate::level::{Level, Walker};
    use crate::map::grid::Tile;

    use crate::map::terrain::Edit;

    fn tool(name: &str) -> usize {
        tool_index(name).unwrap()
    }

    /// Inks `name` at `i` the way the click does, then hands back the level.
    fn inked(mut level: Level, i: usize, name: &str) -> Level {
        assert_eq!(level.refusal(i, tool(name)), None, "{name} at {i}");
        level.ink(i, tool(name));
        level
    }

    const OPEN: &str = "S...M\n.....\n.....";

    #[test]
    fn wet_ink_returns_all_it_cost_and_the_cell() {
        let mut level = inked(Level::with_chaineur(OPEN), 2, "barricade");
        let sepia = level.sepia;
        assert_eq!(level.refund(2), Some(10));
        assert_eq!(level.refusal(2, tool("biffer")), None);
        level.ink(2, tool("biffer"));
        assert_eq!(level.sepia, sepia + 10);
        assert_eq!(level.terrain.base().tiles[2], Tile::Sol);
        assert_eq!(
            level.refusal(2, tool("biffer")),
            Some("refusal.nothing_to_biffer"),
            "nothing is left to take"
        );
    }

    /// What comes back is what was paid, skills included: a barricade with Fascines cost 7.
    #[test]
    fn the_refund_is_what_was_paid() {
        let mut level = Level::with_chaineur(OPEN);
        level.skills.push(Skill::Fascines);
        let level = inked(level, 2, "barricade");
        assert_eq!(level.refund(2), Some(7));
    }

    /// Once a wave is sent, the ink is dry: half comes back, rounded down.
    #[test]
    fn dry_ink_returns_half() {
        let mut level = Level::with_chaineur(OPEN);
        level.skills.push(Skill::Fascines);
        let mut level = inked(inked(level, 2, "barricade"), 8, "arbalète");
        level.wave.number = 1;
        assert_eq!((level.refund(2), level.refund(8)), (Some(3), Some(25)));
    }

    /// Ink laid while a wave runs is dry at once: no free tower picked up and moved ahead of the
    /// column.
    #[test]
    fn ink_laid_during_a_wave_is_dry() {
        let mut level = Level::with_chaineur(OPEN);
        level.wave.number = 1;
        level.wave.running = true;
        let mut level = inked(level, 7, "arbalète");
        assert_eq!(level.refund(7), Some(25));
        level.wave.running = false;
        assert_eq!(level.refund(7), Some(25), "still dry once the wave is over");
    }

    /// Ink laid between waves stays wet until the next wave is sent.
    #[test]
    fn ink_stays_wet_until_the_next_wave() {
        let mut level = Level::with_chaineur(OPEN);
        level.wave.number = 2;
        let mut level = inked(level, 7, "arbalète");
        assert_eq!(level.refund(7), Some(50));
        level.wave.number = 3;
        level.wave.running = true;
        assert_eq!(level.refund(7), Some(25));
    }

    #[test]
    fn a_tower_comes_down() {
        let mut level = inked(Level::with_chaineur(OPEN), 7, "mortier");
        level.ink(7, tool("biffer"));
        assert_eq!(level.terrain.base().tiles[7], Tile::Sol);
        let cover = &level.terrain.base().cover;
        assert!(cover.iter().all(|&c| c == 0.0), "{cover:?}");
    }

    /// Only what Nève inked herself: not the sheet's own barricade, the Ronces-mères' growth, a
    /// Palissade, a borne, or bare ground.
    #[test]
    fn biffer_takes_back_only_neves_strokes() {
        let mut level = Level::with_chaineur("S.#.p.M\n.......");
        level.alter([(8, Edit::Ink(Tile::Ronces)), (9, Edit::PlantBorne)]);
        for i in [1, 2, 4, 8, 9] {
            assert_eq!(
                level.refusal(i, tool("biffer")),
                Some("refusal.nothing_to_biffer"),
                "cell {i}"
            );
            assert_eq!(level.refund(i), None);
        }
    }

    /// Ink the land has since covered (the Chaussée laid over mud) is no longer Nève's to take
    /// back.
    #[test]
    fn ink_covered_since_is_not_hers() {
        let mut level = inked(Level::with_chaineur(OPEN), 2, "boue");
        level.alter([(2, Edit::Ink(Tile::Chaussee))]);
        assert_eq!(level.refusal(2, tool("biffer")), Some("refusal.nothing_to_biffer"));
    }

    /// The cell is what it was: a barricade inked on the Chaussée gives back the Chaussée.
    #[test]
    fn the_cell_is_what_it_was() {
        let mut level = Level::with_chaineur(OPEN);
        level.alter([(2, Edit::Ink(Tile::Chaussee))]);
        let mut level = inked(level, 2, "barricade");
        level.ink(2, tool("biffer"));
        assert_eq!(level.terrain.base().tiles[2], Tile::Chaussee);
    }

    /// A scratched-out name never comes back (docs, section 2, rule 7).
    #[test]
    fn the_name_stays_scratched_out() {
        let mut level = Level::with_chaineur(OPEN);
        level.places = vec![Place { name: "place.pre_commun".into(), at: vec![(2, 0)] }];
        let names = level.names;
        let mut level = inked(level, 2, "barricade");
        level.ink(2, tool("biffer"));
        assert_eq!((level.erased.clone(), level.names), (vec![0], names - 1));
        assert_eq!(level.margin.len(), 1, "biffing writes no second note");
    }

    /// Taking ink back never closes a way: a walker on the mud doesn't refuse it.
    #[test]
    fn biffer_ignores_le_droit_de_passage() {
        let mut level = inked(Level::with_chaineur(OPEN), 2, "boue");
        level.walkers.push(Walker { species: level.present[0], cell: 2 });
        assert_eq!(level.refusal(2, tool("biffer")), None);
    }

    /// The pencil line for Biffer is the route once the stroke is gone.
    #[test]
    fn biffer_preview_is_what_it_does() {
        let mut level = inked(Level::with_chaineur(OPEN), 2, "barricade");
        let k = level.present[0];
        let (grid, field) = level.what_if(2, tool("biffer"), k);
        level.ink(2, tool("biffer"));
        assert_eq!(grid.tiles, level.terrain.grid().tiles);
        assert_eq!(field.dist, level.terrain.fields()[k].dist);
    }

    /// Biffer is free, and using it doesn't count as using a tool for a challenge.
    #[test]
    fn biffer_is_free() {
        let mut level = inked(Level::with_chaineur(OPEN), 2, "barricade");
        level.sepia = 0;
        assert_eq!(level.refusal(2, tool("biffer")), None);
        level.ink(2, tool("biffer"));
        assert_eq!(level.used, vec![tool("barricade")]);
    }
}
