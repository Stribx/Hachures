//! The sheet's ground: as drawn, as inked, as walked under the world event, and the routes over it.
//! Every lasting change goes through `alter`, so the map, the towers' shadow and the flow fields never disagree.

use std::collections::HashMap;

use super::flow::{self, FlowField};
use super::grid::{Grid, Tile};
use super::species::Species;
use crate::campaign::skills::Skill;
use crate::combat::towers::{TowerStats, Weapon};
use crate::level::world::Event;

/// A lasting change to one cell.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Edit {
    /// Ink on the land (Nève's tiles, the Chaussée, Ronces-mères): la Crue still reads the ground under it.
    Ink(Tile),
    /// A tower stands up on the cell.
    Raise(Weapon),
    /// The land itself changes (a scripted change): la Crue reads the new ground.
    Land(Tile),
    /// Gratter: the borne comes off, else the cell returns to plain Sol. The caller has checked `Grid::scrapable`.
    Scrape,
    /// La Grande Chaîne plants a borne on the cell.
    PlantBorne,
    /// The borne on the cell falls, shot down.
    DropBorne,
}

/// What the towers' shadow reaches: each weapon's range, Équerre, Ombre portée.
pub struct Reach {
    pub ranges: HashMap<Weapon, f32>,
    /// Équerre: a tower with another beside it reaches one tile further.
    pub equerre: bool,
    /// Ombre portée: how much each covering tower weighs for the species reading l'Ombre.
    pub shadow: f32,
}

impl Reach {
    /// Each weapon's range from the tower table, shaped by the skills owned.
    pub fn new(stats: &HashMap<Weapon, TowerStats>, skills: &[Skill]) -> Reach {
        Reach {
            ranges: stats.iter().map(|(&weapon, s)| (weapon, s.range)).collect(),
            equerre: skills.contains(&Skill::Equerre),
            shadow: if skills.contains(&Skill::OmbrePortee) { 1.5 } else { 1.0 },
        }
    }
}

pub struct Terrain {
    /// The map as Nève inked it.
    base: Grid,
    /// What the species walk on now: `base`, or `base` under the world event while it lasts.
    grid: Grid,
    fields: Vec<FlowField>,
    /// (cell, weapon) of every tower inked.
    towers: Vec<(usize, Weapon)>,
    reach: Reach,
    /// The world event under way, if any.
    event: Option<&'static Event>,
    /// Terrain blocks must be rebuilt from `grid`.
    redraw: bool,
}

impl Terrain {
    pub fn new(base: Grid, reach: Reach, species: &[Species]) -> Terrain {
        let mut terrain = Terrain {
            grid: base.clone(),
            base,
            fields: Vec::new(),
            towers: Vec::new(),
            reach,
            event: None,
            redraw: true,
        };
        terrain.settle(species);
        terrain
    }

    /// Lets `edits` fall on the map, then rebuilds the shadow, the map as walked and the flow fields once.
    pub fn alter(&mut self, edits: impl IntoIterator<Item = (usize, Edit)>, species: &[Species]) {
        for (i, edit) in edits {
            apply(&mut self.base, &mut self.towers, i, edit);
        }
        self.settle(species);
    }

    /// The map and `shown`'s field if `edit` fell on `i`: the same change as `alter`, one field only.
    pub fn preview(&self, i: usize, edit: Edit, shown: &Species) -> (Grid, FlowField) {
        let (mut base, mut towers) = (self.base.clone(), self.towers.clone());
        apply(&mut base, &mut towers, i, edit);
        let grid = self.walked(base, &towers);
        let field = FlowField::new(&grid, shown);
        (grid, field)
    }

    /// The world event under way, or none: the map as walked follows.
    pub fn set_event(&mut self, event: Option<&'static Event>, species: &[Species]) {
        self.event = event;
        self.settle(species);
    }

    /// Reach of the tower at `cell`; Équerre adds 1 when another tower stands beside it.
    pub fn reach(&self, cell: usize, weapon: Weapon) -> f32 {
        tower_reach(&self.reach, &self.base, &self.towers, cell, weapon)
    }

    /// `base` once `edits` have fallen, for the maps to come: no shadow, no event, no fields.
    pub fn ahead(&self, edits: impl IntoIterator<Item = (usize, Edit)>) -> Grid {
        let (mut base, mut towers) = (self.base.clone(), self.towers.clone());
        for (i, edit) in edits {
            apply(&mut base, &mut towers, i, edit);
        }
        base
    }

    /// Whether the blocks must be rebuilt since the last call.
    pub fn take_redraw(&mut self) -> bool {
        std::mem::take(&mut self.redraw)
    }

    pub fn base(&self) -> &Grid {
        &self.base
    }

    pub fn grid(&self) -> &Grid {
        &self.grid
    }

    pub fn fields(&self) -> &[FlowField] {
        &self.fields
    }

    /// Rebuild `grid` and the flow fields after `base`, the towers or the event changed.
    fn settle(&mut self, species: &[Species]) {
        self.grid = self.walked(self.base.clone(), &self.towers);
        self.base.cover.clone_from(&self.grid.cover);
        self.fields = flow::fields(&self.grid, species);
        self.redraw = true;
    }

    /// `base` with the towers' shadow, under the world event while it lasts.
    fn walked(&self, mut base: Grid, towers: &[(usize, Weapon)]) -> Grid {
        base.cover = (0..base.tiles.len())
            .map(|i| {
                let covering = towers
                    .iter()
                    .filter(|&&(t, weapon)| base.within(i, t, tower_reach(&self.reach, &base, towers, t, weapon)));
                covering.count() as f32 * self.reach.shadow
            })
            .collect();
        match self.event {
            Some(event) => base.under(event),
            None => base,
        }
    }
}

/// Reach of the tower at `cell` among `towers`; Équerre adds 1 when another tower stands beside it.
fn tower_reach(reach: &Reach, grid: &Grid, towers: &[(usize, Weapon)], cell: usize, weapon: Weapon) -> f32 {
    let paired = reach.equerre && grid.neighbours(cell).any(|n| towers.iter().any(|t| t.0 == n));
    reach.ranges[&weapon] + if paired { 1.0 } else { 0.0 }
}

/// What `edit` does to `base` and the towers at `i`.
fn apply(base: &mut Grid, towers: &mut Vec<(usize, Weapon)>, i: usize, edit: Edit) {
    match edit {
        Edit::Ink(tile) => base.tiles[i] = tile,
        Edit::Raise(weapon) => {
            base.tiles[i] = Tile::Tour;
            towers.push((i, weapon));
        }
        Edit::Land(tile) => {
            base.tiles[i] = tile;
            base.map[i] = tile;
        }
        Edit::Scrape if base.bornes.contains(&i) => base.bornes.retain(|&b| b != i),
        Edit::Scrape => apply(base, towers, i, Edit::Land(Tile::Sol)),
        Edit::PlantBorne => base.bornes.push(i),
        Edit::DropBorne => base.bornes.retain(|&b| b != i),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combat::towers::load_stats;
    use crate::level::world::WORLDS;
    use crate::map::species::{self, index_of};

    fn reach() -> Reach {
        Reach::new(&load_stats(), &[])
    }

    /// The pencil line for a tower counts the shadow it will cast: the Harde, reading l'Ombre, goes round it.
    #[test]
    fn a_tower_preview_casts_its_shadow() {
        let species = species::load();
        let harde = index_of(&species, "harde").unwrap();
        let mut terrain = Terrain::new(Grid::parse("S.......M\n........."), reach(), &species);
        let before = terrain.fields()[harde].dist.clone();
        let (_, field) = terrain.preview(4, Edit::Raise(Weapon::Arbalete), &species[harde]);
        terrain.alter([(4, Edit::Raise(Weapon::Arbalete))], &species);
        assert_ne!(terrain.fields()[harde].dist, before, "the tower's shadow weighs on the Harde");
        assert_eq!(field.dist, terrain.fields()[harde].dist, "the preview is what the ink does");
    }

    /// La Crue reads the land, not the ink (docs, section 5): a rockfall on low ground stays dry, mud drowns.
    #[test]
    fn la_crue_reads_the_land_not_the_ink() {
        let species = species::load();
        let mut terrain = Terrain::new(Grid::parse("S,,,M"), reach(), &species);
        terrain.alter([(1, Edit::Land(Tile::Roc)), (2, Edit::Ink(Tile::Boue))], &species);
        terrain.set_event(Some(&WORLDS[0].event), &species);
        assert_eq!(&terrain.grid().tiles[1..4], [Tile::Roc, Tile::Eau, Tile::Eau]);
        terrain.set_event(None, &species);
        assert_eq!(&terrain.grid().tiles[1..4], [Tile::Roc, Tile::Boue, Tile::SolBas]);
    }

    /// Gratter takes a borne off and leaves the ground under it; an Éboulis or a Palissade goes back to plain Sol,
    /// in the map la Crue reads too.
    #[test]
    fn a_scrape_takes_off_a_borne_or_returns_the_land_to_sol() {
        let species = species::load();
        let mut terrain = Terrain::new(Grid::parse("S.e.p.M"), reach(), &species);
        terrain.alter([(1, Edit::Ink(Tile::Chaussee)), (1, Edit::PlantBorne)], &species);
        terrain.alter([(1, Edit::Scrape), (2, Edit::Scrape), (4, Edit::Scrape)], &species);
        let base = terrain.base();
        assert!(base.bornes.is_empty());
        assert_eq!((base.tiles[1], base.tiles[2], base.tiles[4]), (Tile::Chaussee, Tile::Sol, Tile::Sol));
        assert_eq!((base.map[2], base.map[4]), (Tile::Sol, Tile::Sol));
    }

    #[test]
    fn a_palissade_is_impassable() {
        let species = species::load();
        let chaineur = index_of(&species, "chaineur").unwrap();
        let terrain = Terrain::new(Grid::parse("S.p.M\n....."), reach(), &species);
        assert_eq!(terrain.fields()[chaineur].dist[2], f32::INFINITY);
    }

    #[test]
    fn la_crue_spares_a_palissade() {
        let species = species::load();
        let mut terrain = Terrain::new(Grid::parse("S.,.M\n....."), reach(), &species);
        terrain.alter([(2, Edit::Ink(Tile::Palissade))], &species);
        assert_eq!(terrain.base().under(&WORLDS[0].event).tiles[2], Tile::Palissade);
    }

    /// Équerre: a tower with another beside it reaches one tile further; alone, it doesn't.
    #[test]
    fn equerre_pairs_towers() {
        let species = species::load();
        let mut terrain = Terrain::new(Grid::parse("S.....M"), Reach { equerre: true, ..reach() }, &species);
        terrain.alter([(2, Edit::Raise(Weapon::Arbalete))], &species);
        assert_eq!(terrain.reach(2, Weapon::Arbalete), 3.0);
        terrain.alter([(3, Edit::Raise(Weapon::Arbalete))], &species);
        assert_eq!(terrain.reach(2, Weapon::Arbalete), 4.0);
    }

    /// Every tool's preview is what inking it does, for every species, with the world event on.
    #[test]
    fn every_preview_is_what_the_ink_does() {
        let species = species::load();
        let map = "S.,e..M\n.,,,,..";
        let edits = [Edit::Ink(Tile::Barricade), Edit::Ink(Tile::Boue), Edit::Raise(Weapon::Mortier), Edit::Scrape];
        for (i, edit) in [4, 2, 5, 3].into_iter().zip(edits) {
            let mut terrain = Terrain::new(Grid::parse(map), reach(), &species);
            terrain.set_event(Some(&WORLDS[0].event), &species);
            let previews: Vec<_> = species.iter().map(|s| terrain.preview(i, edit, s)).collect();
            terrain.alter([(i, edit)], &species);
            for (k, (grid, field)) in previews.iter().enumerate() {
                assert_eq!(grid.tiles, terrain.grid().tiles, "{edit:?}");
                assert_eq!(field.dist, terrain.fields()[k].dist, "{edit:?}, {}", species[k].name);
            }
        }
    }
}
