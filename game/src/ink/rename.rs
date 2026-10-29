//! Renommer: when Nève may change a cell's kind, land to water or water to land (docs, section 7).

use super::Act;
use crate::i18n::n_;
use crate::level::Level;
use crate::map::grid::Tile;
use crate::map::terrain::Edit;

impl Level {
    /// Why Renommer (tool `tool`) can't change `cell`, if it can't. Turning water to Sol closes
    /// nothing, every terrestrial species walks on Sol; turning anything to water goes through le
    /// Droit de Passage.
    pub(super) fn rename_refusal(&self, cell: usize, tool: usize) -> Option<&'static str> {
        let Some(edit) = self.terrain.grid().renamed(cell) else {
            return Some(n_("refusal.nothing_to_rename"));
        };
        if self.sepia < self.tool_cost(tool) {
            Some(n_("refusal.sepia"))
        } else if edit == Edit::Land(Tile::Sol) {
            None
        } else if self.walkers.iter().any(|w| w.cell == cell) {
            Some(n_("refusal.column"))
        } else {
            // ponytail: the maps ahead get Eau where the event would freeze the new water; too
            // strict, never too lax. Price them under the event if it refuses a fair rename.
            self.closes(cell, Tile::Eau)
        }
    }

    /// What Renommer does at `cell`; nothing at all where it has nothing to change.
    pub(super) fn rename_edit(&self, cell: usize) -> Edit {
        debug_assert!(Act::Rename.lays().is_none());
        self.terrain.grid().renamed(cell).unwrap_or(Edit::Ink(self.terrain.base().tiles[cell]))
    }
}

#[cfg(test)]
mod tests {
    use crate::ink::{Act, tool_index};
    use crate::level::data::Place;
    use crate::level::{Level, Walker};
    use crate::map::grid::{Grid, Tile};
    use crate::map::species::{self, index_of};
    use crate::map::terrain::Edit;

    fn renommer() -> usize {
        tool_index("renommer").unwrap()
    }

    /// A world-2 level where the Chaîneur and the Gens du Col walk, with six waves.
    fn sheet(map: &str) -> Level {
        let mut level = Level::new(Grid::parse(map), species::load());
        level.world = 1;
        level.present = ["chaineur", "gens_du_col"]
            .iter()
            .map(|s| index_of(&level.species, s).unwrap())
            .collect();
        level.waves = vec![vec![]; 6];
        level.sepia = 500;
        level
    }

    /// Le Gel under way, as `start_wave` sets it.
    fn freeze(level: &mut Level) {
        level.event_on = true;
        let event = level.world().event.as_ref();
        level.terrain.set_event(event, &level.species);
    }

    fn land(level: &Level, i: usize) -> (Tile, Tile) {
        (level.terrain.base().tiles[i], level.terrain.base().map[i])
    }

    #[test]
    fn renommer_turns_ground_to_water() {
        let mut level = sheet("S...M\n.....");
        assert_eq!(level.refusal(2, renommer()), None);
        let sepia = level.sepia;
        level.ink(2, renommer());
        assert_eq!(land(&level, 2), (Tile::Eau, Tile::Eau), "the ground itself changes");
        assert_eq!(level.sepia, sepia - level.tool_cost(renommer()));
        let mut level = sheet("S,bgM\n.....");
        for cell in [1, 2] {
            level.ink(cell, renommer());
            assert_eq!(land(&level, cell), (Tile::Eau, Tile::Eau));
        }
    }

    #[test]
    fn renommer_turns_water_to_sol() {
        let mut level = sheet("S.~gM\n.....");
        for cell in [2, 3] {
            assert_eq!(level.refusal(cell, renommer()), None);
            level.ink(cell, renommer());
            assert_eq!(land(&level, cell), (Tile::Sol, Tile::Sol));
        }
    }

    /// Ice the Gel froze thaws back to water, and stays water until the Gel ends.
    #[test]
    fn renommer_thaws_glace_until_the_gel_ends() {
        let mut level = sheet("S...M\n~~~~~");
        freeze(&mut level);
        assert_eq!(level.terrain.grid().tiles[7], Tile::Glace);
        assert_eq!(level.refusal(7, renommer()), None);
        level.ink(7, renommer());
        assert_eq!(level.terrain.grid().tiles[7], Tile::Eau, "thawed under the Gel");
        assert_eq!(level.terrain.grid().tiles[8], Tile::Glace, "its neighbours stay frozen");
        level.event_on = false;
        level.terrain.set_event(None, &level.species);
        freeze(&mut level);
        assert_eq!(level.terrain.grid().tiles[7], Tile::Glace, "the next Gel freezes it again");
    }

    #[test]
    fn renommer_refuses_a_closing_route() {
        let level = sheet("S...M");
        assert_eq!(level.refusal(2, renommer()), Some("refusal.closed"));
        let mut level = sheet("S...M\n~~~~~");
        freeze(&mut level);
        level.walkers.push(Walker { species: level.present[0], cell: 2 });
        assert_eq!(level.refusal(2, renommer()), Some("refusal.column"));
    }

    #[test]
    fn renommer_refuses_ink_and_markers() {
        let mut level = sheet("S...MX\n......");
        level.alter([
            (1, Edit::Ink(Tile::Boue)),
            (2, Edit::Raise(crate::combat::towers::Weapon::Arbalete)),
            (3, Edit::Ink(Tile::Chaussee)),
            (7, Edit::PlantBorne),
        ]);
        for cell in [0, 1, 2, 3, 4, 5, 7] {
            assert_eq!(
                level.refusal(cell, renommer()),
                Some("refusal.nothing_to_rename"),
                "{cell}"
            );
        }
        for tile in [Tile::Roc, Tile::Ronces, Tile::Barricade, Tile::Eboulis, Tile::Palissade] {
            let mut level = sheet("S...M\n.....");
            level.alter([(8, Edit::Land(tile))]);
            assert_eq!(level.refusal(8, renommer()), Some("refusal.nothing_to_rename"), "{tile:?}");
        }
    }

    #[test]
    fn renaming_a_place_costs_a_name() {
        let mut level = sheet("S...M\n.....");
        level.places = vec![Place { name: "place.pre_commun".into(), at: vec![(2, 1)] }];
        let names = level.names;
        level.ink(7, renommer());
        assert_eq!(level.names, names - 1);
        assert_eq!(level.erased, vec![0]);
        assert_eq!(level.place_at(7), None, "the name is out of the Rôle");
    }

    #[test]
    fn renaming_bare_ground_costs_no_name() {
        let mut level = sheet("S...M\n.....");
        let names = level.names;
        level.ink(7, renommer());
        assert_eq!(level.names, names);
        assert!(level.erased.is_empty());
    }

    #[test]
    fn biffer_does_not_undo_renommer() {
        let mut level = sheet("S...M\n.....");
        level.ink(7, renommer());
        assert!(level.inked.is_empty(), "not one of Nève's strokes");
        assert!(level.refusal(7, tool_index("biffer").unwrap()).is_some());
        assert!(Act::Rename.names());
    }

    #[test]
    fn renommer_preview_is_what_it_does() {
        let mut level = sheet("S...M\n.....");
        let k = level.present[0];
        let (grid, field) = level.what_if(7, renommer(), k);
        level.ink(7, renommer());
        assert_eq!(grid.tiles, level.terrain.grid().tiles);
        assert_eq!(field.dist, level.terrain.fields()[k].dist);
    }
}
