//! Le Droit de Passage: what may be inked so that no map, now or to come, is closed.

use super::TOOLS;
use crate::i18n::n_;
use crate::level::Level;
use crate::map::flow::can_place;
use crate::map::grid::{Grid, Tile};

impl Level {
    /// Why `tool` can't be inked at `i`, if it can't.
    pub fn refusal(&self, i: usize, tool: usize) -> Option<&'static str> {
        let Some(tile) = TOOLS[tool].act.lays() else {
            // Scraping only opens the map: no need to check that it stays open.
            return if !self.base.bornes.contains(&i) {
                Some(n_("refusal.nothing_to_scrape"))
            } else if self.sepia < self.tool_cost(tool) {
                Some(n_("refusal.sepia"))
            } else {
                None
            };
        };
        if !self.grid.tiles[i].buildable() {
            Some(n_("refusal.firm_ground"))
        } else if self.sepia < self.tool_cost(tool) {
            Some(n_("refusal.sepia"))
        } else if self.walkers.iter().any(|w| w.1 == i) {
            Some(n_("refusal.column"))
        } else {
            self.closes(i, tile)
        }
    }

    /// Le Droit de Passage: why `tile` at `i` would close the map, now or around the world event.
    fn closes(&self, i: usize, tile: Tile) -> Option<&'static str> {
        let event = &self.world().event;
        if self.changes.iter().any(|c| c.cells(self.base.width).any(|n| n == i)) {
            Some(n_("refusal.relief_changes"))
        } else if !can_place(&self.grid, &self.species, &self.present, i, tile, &self.walkers) {
            Some(n_("refusal.closed"))
        } else if self.maps_ahead().iter().any(|map| !can_place(map, &self.species, &self.present, i, tile, &[])) {
            Some(&event.refuse_next)
        } else if self.event_on && !can_place(&self.base, &self.species, &self.present, i, tile, &[]) {
            Some(&event.refuse_after)
        } else if !self.changes.is_empty()
            && !can_place(&self.after_changes(), &self.species, &self.present, i, tile, &[])
        {
            Some(n_("refusal.relief_closes"))
        } else {
            None
        }
    }

    /// The maps of the waves still to come that the world event floods, each with the changes fallen by then.
    fn maps_ahead(&self) -> Vec<Grid> {
        let event = &self.world().event;
        let mut seen = Vec::new();
        let mut maps = Vec::new();
        for w in self.wave.number + 1..=self.waves.len() as u32 {
            let fallen = self.changes.iter().filter(|c| c.wave <= w).count();
            // Same changes fallen, same map: build it once.
            if self.event_at(w) && !seen.contains(&fallen) {
                seen.push(fallen);
                let mut grid = self.base.clone();
                self.changes.iter().filter(|c| c.wave <= w).for_each(|c| c.apply(&mut grid));
                maps.push(grid.under(event));
            }
        }
        maps
    }

    /// Ronces-mères: as a wave starts, one bramble spreads to a neighbouring cell, if le Droit de Passage allows.
    pub fn grow_ronces(&mut self) {
        let base = &self.base;
        let spread = (0..base.tiles.len())
            .filter(|&i| base.tiles[i] == Tile::Ronces)
            .flat_map(|i| base.neighbours(i))
            .find(|&n| base.tiles[n].buildable() && self.closes(n, Tile::Ronces).is_none());
        if let Some(n) = spread {
            self.base.tiles[n] = Tile::Ronces;
            self.refresh();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ink::tool_index;
    use crate::level::data::Change;
    use crate::map::grid::Grid;
    use crate::map::species::{self, index_of};

    /// A world-1 level where only the Chaîneur walks (it can't cross Eau), with six waves.
    fn level(map: &str) -> Level {
        let species = species::load();
        let mut level = Level::new(Grid::parse(map), species);
        level.present = vec![index_of(&level.species, "chaineur").unwrap()];
        level.waves = vec![vec![]; 6];
        level
    }

    /// Two ways: the top one on plain ground, the bottom one on low ground.
    const TWO_WAYS: &str = "
        S...M
        .^^^.
        .,,,.
        ";

    #[test]
    fn a_flood_far_ahead_is_counted() {
        let mut level = level(TWO_WAYS);
        level.events = vec![3];
        let barricade = tool_index("barricade").unwrap();
        // Alone, the plain way is not enough once the low one drowns at wave 3.
        assert_eq!(level.refusal(2, barricade), Some("world.1.refuse_next"));
        level.events = vec![];
        assert_eq!(level.refusal(2, barricade), None, "no flood, no problem");
    }

    #[test]
    fn a_barricade_that_closes_nothing_is_accepted() {
        let mut level = level(TWO_WAYS);
        level.events = vec![3];
        // The dead-end middle of the low way: the plain way stays open dry or flooded.
        assert_eq!(level.refusal(12, tool_index("barricade").unwrap()), None);
    }

    /// Three ways A (top), B (low, floods), C (bottom). A rockfall closes A at wave 2, the flood comes at wave 4.
    #[test]
    fn a_rockfall_then_the_flood_is_counted() {
        let mut level = level(
            "
            S...M
            .^^^.
            .,,,.
            .^^^.
            .....
            ",
        );
        level.changes = vec![Change { wave: 2, at: vec![(2, 0)], becomes: Tile::Roc }];
        level.events = vec![4];
        let c = 22;
        assert_eq!(level.refusal(c, tool_index("barricade").unwrap()), Some("world.1.refuse_next"));
        level.events = vec![];
        assert_eq!(level.refusal(c, tool_index("barricade").unwrap()), None, "without the flood, B stays open");
    }

    #[test]
    fn gratter_scrapes_only_a_borne() {
        let mut level = level("S...M");
        level.base.bornes.push(2);
        level.refresh();
        let gratter = tool_index("gratter").unwrap();
        assert_eq!(level.refusal(1, gratter), Some("refusal.nothing_to_scrape"));
        assert_eq!(level.refusal(2, gratter), None);
        let sepia = level.sepia;
        level.ink(2, gratter);
        assert!(level.base.bornes.is_empty(), "the borne is gone");
        assert_eq!(level.sepia, sepia - 25);
        assert_eq!(level.refusal(2, gratter), Some("refusal.nothing_to_scrape"));
    }

    /// The pencil line for Gratter is the Jalonneurs' route once the borne is gone, on the tile it stood on.
    #[test]
    fn gratter_preview_takes_the_borne_off() {
        let mut level = level("S...B");
        let jalonneur = index_of(&level.species, "jalonneur").unwrap();
        level.base.tiles[1] = Tile::Chaussee;
        level.base.bornes.push(1);
        level.refresh();
        let gratter = tool_index("gratter").unwrap();
        let (grid, field) = level.what_if(1, gratter, jalonneur);
        assert_eq!(grid.tiles[1], Tile::Chaussee, "the tile under the borne stays");
        level.ink(1, gratter);
        assert_eq!(field.dist, level.fields[jalonneur].dist);
    }
}
