//! Le Droit de Passage: what may be inked so that no map, now or to come, is closed.

use super::{Act, TOOLS};
use crate::i18n::n_;
use crate::level::Level;
use crate::map::flow::can_place;
use crate::map::grid::{Grid, Tile};
use crate::map::terrain::Edit;

impl Level {
    /// Why `tool` can't be inked at `cell`, if it can't.
    #[must_use]
    pub fn refusal(&self, cell: usize, tool: usize) -> Option<&'static str> {
        match TOOLS[tool].act {
            Act::Biffer => return self.biffer_refusal(cell),
            Act::Rename => return self.rename_refusal(cell, tool),
            _ => {}
        }
        let Some(tile) = TOOLS[tool].act.lays() else {
            // Scraping leaves plain Sol, which every species walks: it can lengthen a route, never
            // close one.
            return if !self.terrain.base().scrapable(cell) {
                Some(n_("refusal.nothing_to_scrape"))
            } else if self.sepia < self.tool_cost(tool) {
                Some(n_("refusal.sepia"))
            } else {
                None
            };
        };
        if !self.terrain.grid().tiles[cell].buildable() {
            Some(n_("refusal.firm_ground"))
        } else if self.sepia < self.tool_cost(tool) {
            Some(n_("refusal.sepia"))
        } else if self.walkers.iter().any(|w| w.cell == cell) {
            Some(n_("refusal.column"))
        } else {
            self.closes(cell, tile)
        }
    }

    /// Le Droit de Passage: why `tile` at `cell` would close the map, now or around the world
    /// event.
    pub(super) fn closes(&self, cell: usize, tile: Tile) -> Option<&'static str> {
        let event = self.world().event.as_ref();
        if self.changes.iter().any(|c| c.cells(self.terrain.base().width).any(|n| n == cell)) {
            Some(n_("refusal.relief_changes"))
        } else if !can_place(
            self.terrain.grid(),
            &self.species,
            &self.present,
            cell,
            tile,
            &self.walkers,
        ) {
            Some(n_("refusal.closed"))
        } else if let Some(event) = event
            && self
                .maps_ahead()
                .iter()
                .any(|map| !can_place(map, &self.species, &self.present, cell, tile, &[]))
        {
            Some(&event.refuse_next)
        } else if let Some(event) = event
            && self.event_on
            && !can_place(self.terrain.base(), &self.species, &self.present, cell, tile, &[])
        {
            Some(&event.refuse_after)
        } else if !self.changes.is_empty()
            && !can_place(&self.after_changes(), &self.species, &self.present, cell, tile, &[])
        {
            Some(n_("refusal.relief_closes"))
        } else {
            None
        }
    }

    /// The maps of the waves still to come that the world event floods, each with the changes
    /// fallen by then.
    fn maps_ahead(&self) -> Vec<Grid> {
        let Some(event) = &self.world().event else { return Vec::new() };
        let width = self.terrain.base().width;
        let mut seen = Vec::new();
        let mut maps = Vec::new();
        for w in self.wave.number + 1..=self.waves.len() as u32 {
            let fallen = self.changes.iter().filter(|c| c.wave <= w).count();
            // Same changes fallen, same map: build it once.
            if self.event_at(w) && !seen.contains(&fallen) {
                seen.push(fallen);
                let edits =
                    self.changes.iter().filter(|c| c.wave <= w).flat_map(|c| c.edits(width));
                maps.push(self.terrain.ahead(edits).under(event));
            }
        }
        maps
    }

    /// Ronces-mères: as a wave starts, one bramble spreads to a neighbouring cell, if le Droit de
    /// Passage allows.
    pub fn grow_ronces(&mut self) {
        let base = self.terrain.base();
        let spread = (0..base.tiles.len())
            .filter(|&i| base.tiles[i] == Tile::Ronces)
            .flat_map(|i| base.neighbours(i))
            .find(|&n| base.tiles[n].buildable() && self.closes(n, Tile::Ronces).is_none());
        if let Some(n) = spread {
            self.alter([(n, Edit::Ink(Tile::Ronces))]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ink::tool_index;
    use crate::level::Walker;
    use crate::level::data::{Change, Place};

    use crate::map::species::index_of;

    /// Two ways: the top one on plain ground, the bottom one on low ground.
    const TWO_WAYS: &str = "
        S...M
        .^^^.
        .,,,.
        ";

    #[test]
    fn a_flood_far_ahead_is_counted() {
        let mut level = Level::with_chaineur(TWO_WAYS);
        level.events = vec![3];
        let barricade = tool_index("barricade").unwrap();
        // Alone, the plain way is not enough once the low one drowns at wave 3.
        assert_eq!(level.refusal(2, barricade), Some("world.1.refuse_next"));
        level.events = vec![];
        assert_eq!(level.refusal(2, barricade), None, "no flood, no problem");
    }

    #[test]
    fn a_barricade_that_closes_nothing_is_accepted() {
        let mut level = Level::with_chaineur(TWO_WAYS);
        level.events = vec![3];
        // The dead-end middle of the low way: the plain way stays open dry or flooded.
        assert_eq!(level.refusal(12, tool_index("barricade").unwrap()), None);
    }

    /// Three ways A (top), B (low, floods), C (bottom). A rockfall closes A at wave 2, the flood
    /// comes at wave 4.
    #[test]
    fn a_rockfall_then_the_flood_is_counted() {
        let mut level = Level::with_chaineur(
            "
            S...M
            .^^^.
            .,,,.
            .^^^.
            .....
            ",
        );
        level.changes = vec![Change {
            wave: 2,
            at: vec![(2, 0)],
            becomes: Tile::Roc,
            announce: None,
            color: None,
        }];
        level.events = vec![4];
        let c = 22;
        assert_eq!(level.refusal(c, tool_index("barricade").unwrap()), Some("world.1.refuse_next"));
        level.events = vec![];
        assert_eq!(
            level.refusal(c, tool_index("barricade").unwrap()),
            None,
            "without the flood, B stays open"
        );
    }

    #[test]
    fn gratter_scrapes_only_a_borne() {
        let mut level = Level::with_chaineur("S...M");
        level.alter([(2, Edit::PlantBorne)]);
        let gratter = tool_index("gratter").unwrap();
        assert_eq!(level.refusal(1, gratter), Some("refusal.nothing_to_scrape"));
        assert_eq!(level.refusal(2, gratter), None);
        let sepia = level.sepia;
        level.ink(2, gratter);
        assert!(level.terrain.base().bornes.is_empty(), "the borne is gone");
        assert_eq!(level.sepia, sepia - 25);
        assert_eq!(level.refusal(2, gratter), Some("refusal.nothing_to_scrape"));
    }

    /// The pencil line for Gratter is the Jalonneurs' route once the borne is gone, on the tile it
    /// stood on.
    #[test]
    fn gratter_preview_takes_the_borne_off() {
        let mut level = Level::with_chaineur("S...B");
        let jalonneur = index_of(&level.species, "jalonneur").unwrap();
        level.alter([(1, Edit::Ink(Tile::Chaussee)), (1, Edit::PlantBorne)]);
        let gratter = tool_index("gratter").unwrap();
        let (grid, field) = level.what_if(1, gratter, jalonneur);
        assert_eq!(grid.tiles[1], Tile::Chaussee, "the tile under the borne stays");
        level.ink(1, gratter);
        assert_eq!(field.dist, level.terrain.fields()[jalonneur].dist);
    }

    /// Gratter on an Éboulis leaves plain Sol, in the tiles and in the map the world event reads.
    #[test]
    fn gratter_scrapes_an_eboulis() {
        let mut level = Level::with_chaineur("S.e.M");
        let gratter = tool_index("gratter").unwrap();
        assert_eq!(level.refusal(2, gratter), None);
        let sepia = level.sepia;
        level.ink(2, gratter);
        assert_eq!(
            (level.terrain.base().tiles[2], level.terrain.base().map[2]),
            (Tile::Sol, Tile::Sol)
        );
        assert_eq!(level.sepia, sepia - level.tool_cost(gratter));
        assert_eq!(level.refusal(2, gratter), Some("refusal.nothing_to_scrape"));
    }

    /// Gratter on a Palissade leaves plain Sol, as on an Éboulis. Two rows: the palissade alone
    /// doesn't close the way.
    #[test]
    fn gratter_scrapes_a_palissade() {
        let mut level = Level::with_chaineur("S.p.M\n.....");
        let gratter = tool_index("gratter").unwrap();
        assert_eq!(level.refusal(2, gratter), None);
        level.ink(2, gratter);
        assert_eq!(
            (level.terrain.base().tiles[2], level.terrain.base().map[2]),
            (Tile::Sol, Tile::Sol)
        );
        assert_eq!(level.refusal(2, gratter), Some("refusal.nothing_to_scrape"));
    }

    #[test]
    fn a_palissade_is_not_buildable() {
        let level = Level::with_chaineur("S.p.M\n.....");
        assert_eq!(level.refusal(2, tool_index("barricade").unwrap()), Some("refusal.firm_ground"));
    }

    #[test]
    fn gratter_refuses_what_is_not_decor() {
        let gratter = tool_index("gratter").unwrap();
        for tile in [
            Tile::Barricade,
            Tile::Boue,
            Tile::Ronces,
            Tile::Chaussee,
            Tile::Glace,
            Tile::Eau,
            Tile::Roc,
        ] {
            let mut level = Level::with_chaineur("S...M");
            level.alter([(2, Edit::Ink(tile))]);
            assert_eq!(level.refusal(2, gratter), Some("refusal.nothing_to_scrape"), "{tile:?}");
        }
    }

    #[test]
    fn an_eboulis_is_not_buildable() {
        let level = Level::with_chaineur("S.e.M");
        assert_eq!(level.refusal(2, tool_index("barricade").unwrap()), Some("refusal.firm_ground"));
    }

    #[test]
    fn scraping_an_eboulis_keeps_its_name() {
        let mut level = Level::with_chaineur("S.e.M");
        level.places = vec![Place { name: "place.pre_commun".into(), at: vec![(2, 0)] }];
        let gratter = tool_index("gratter").unwrap();
        let names = level.names;
        level.ink(2, gratter);
        assert_eq!(level.names, names);
        assert!(level.erased.is_empty());
        assert_eq!(level.place_at(2), Some(0), "the name is still written");
    }

    /// Scraping never closes a way, so a walker on the cell or a scripted change to come don't
    /// refuse it.
    #[test]
    fn gratter_ignores_walkers_and_changes() {
        let gratter = tool_index("gratter").unwrap();
        let mut level = Level::with_chaineur("S.e.M");
        level.walkers.push(Walker { species: level.present[0], cell: 2 });
        assert_eq!(level.refusal(2, gratter), None, "under a walker");
        level.walkers.clear();
        level.changes = vec![Change {
            wave: 2,
            at: vec![(2, 0)],
            becomes: Tile::Roc,
            announce: None,
            color: None,
        }];
        assert_eq!(level.refusal(2, gratter), None, "on a cell a change will hit");
    }

    #[test]
    fn gratter_preview_takes_the_eboulis_off() {
        let mut level = Level::with_chaineur("S.e.M");
        let k = level.present[0];
        let gratter = tool_index("gratter").unwrap();
        let (_, field) = level.what_if(2, gratter, k);
        level.ink(2, gratter);
        assert_eq!(field.dist, level.terrain.fields()[k].dist);
    }

    /// A scripted change leaves `map` as drawn; after a scrape La Crue must still find plain Sol
    /// there.
    #[test]
    fn la_crue_spares_a_scraped_eboulis() {
        let mut level = Level::with_chaineur("S.,.M");
        level.alter([(2, Edit::Ink(Tile::Eboulis))]);
        level.ink(2, tool_index("gratter").unwrap());
        assert_eq!(
            level.terrain.base().under(level.world().event.as_ref().unwrap()).tiles[2],
            Tile::Sol
        );
    }
}
