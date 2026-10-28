//! The click that inks: number keys pick a tool, and `Level` pays for it and lays it on the sheet.

use bevy::prelude::*;

use super::{Act, Selection, TOOLS, TowerBiffed, TowerInked};
use crate::campaign::skills::Skill;
use crate::combat::towers::Weapon;
use crate::i18n::n_;
use crate::level::story::MarginNote;
use crate::level::{Inked, Level};
use crate::map::flow::FlowField;
use crate::map::grid::{Grid, Tile};
use crate::map::terrain::Edit;

/// Number keys pick a tool; a click inks it on the hovered cell.
pub(crate) fn inking(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut level: ResMut<Level>,
    mut selection: ResMut<Selection>,
    mut inked: MessageWriter<TowerInked>,
    mut biffed: MessageWriter<TowerBiffed>,
) {
    if level.finished {
        return;
    }
    if let Some(&tool) = level.tools.iter().find(|&&t| keys.just_pressed(TOOLS[t].key)) {
        selection.tool = tool;
    }
    let Some(i) = selection.hovered else { return };
    if !mouse.just_pressed(MouseButton::Left) || level.refusal(i, selection.tool).is_some() {
        return;
    }
    let tower = level.terrain.base().tiles[i] == Tile::Tour;
    level.ink(i, selection.tool);
    let act = TOOLS[selection.tool].act;
    if let Some(weapon) = act.weapon() {
        inked.write(TowerInked { cell: i, weapon });
    } else if act == Act::Biffer && tower {
        biffed.write(TowerBiffed { cell: i });
    }
}

impl Level {
    /// Sépia cost of tool `tool`, skills included.
    pub fn tool_cost(&self, tool: usize) -> u32 {
        let spec = &TOOLS[tool];
        let factor = if spec.act == Act::Raise(Weapon::Arbalete) && self.has(Skill::MineDePlomb) {
            0.9
        } else if spec.act == Act::Lay(Tile::Barricade) && self.has(Skill::Fascines) {
            0.7
        } else {
            1.0
        };
        (spec.cost as f32 * factor).round() as u32
    }

    /// Inks `tool` at `cell`, paid in sépia, and in a Name if a place's name is written there. Biffer goes through here
    /// too: free, it costs no Name and gives sépia back. The caller has checked `refusal`.
    ///
    /// # Panics
    ///
    /// If `refusal` would have refused: the sépia is a `u32`, and paying more than is left underflows.
    pub fn ink(&mut self, cell: usize, tool: usize) {
        let act = TOOLS[tool].act;
        if act == Act::Biffer {
            return self.biffer(cell);
        }
        let paid = self.tool_cost(tool);
        self.sepia -= paid;
        if !self.used.contains(&tool) {
            self.used.push(tool);
        }
        // Gratter never costs a Name (docs, section 7).
        if let (Some(_), Some(p)) = (act.lays(), self.place_at(cell)) {
            self.erased.push(p);
            self.names -= 1;
            let line = n_("note.erased").to_string();
            self.margin.push(MarginNote { line, name: Some(self.places[p].name.clone()) });
        }
        if act.lays().is_some() {
            let (under, laid_at) =
                (self.terrain.base().tiles[cell], (!self.wave.running).then_some(self.wave.number));
            self.inked.push(Inked { cell, tool, under, paid, laid_at });
        }
        let edit = self.edit(cell, tool);
        self.alter([(cell, edit)]);
    }

    /// What inking `tool` at `cell` does to the cell.
    fn edit(&self, cell: usize, tool: usize) -> Edit {
        match TOOLS[tool].act {
            Act::Lay(tile) => Edit::Ink(tile),
            Act::Raise(weapon) => Edit::Raise(weapon),
            Act::Scrape => Edit::Scrape,
            Act::Biffer => Edit::Unink(
                self.stroke(cell).map_or(self.terrain.base().tiles[cell], |s| self.inked[s].under),
            ),
        }
    }

    /// The map and the shown species' field if `tool` were inked at `cell`.
    pub fn what_if(&self, cell: usize, tool: usize, shown: usize) -> (Grid, FlowField) {
        self.terrain.preview(cell, self.edit(cell, tool), &self.species[shown])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::campaign::save::Save;
    use crate::ink::tool_index;
    use crate::map::species::{self, index_of};

    #[test]
    fn la_crue_is_checked_before_inking() {
        // Dry road on top, the other way around is low ground.
        let mut level = Level::new(
            Grid::parse(
                "
                S...M
                ,^^^,
                ,,,,,
                ",
            ),
            species::load(),
        );
        let road = 2;
        assert_eq!(level.refusal(road, 0), None, "the low way around is open");
        level.waves = vec![vec![]; 3];
        level.events = vec![2];
        assert_eq!(level.refusal(road, 0), Some("world.1.refuse_next"));

        level.alter([(5, Edit::Raise(Weapon::Arbalete)), (11, Edit::Ink(Tile::Boue))]);
        let flooded = level.terrain.base().under(&level.world().event);
        assert_eq!(flooded.tiles[5], Tile::Tour, "towers stand through the flood");
        assert_eq!(flooded.tiles[11], Tile::Eau, "mud inked on low ground still drowns");
        assert_eq!(flooded.tiles[10], Tile::Eau);
    }

    /// A scripted avalanche: le Droit de Passage counts it before it falls, then it falls on time.
    #[test]
    fn avalanche_is_counted_before_it_falls() {
        // Two roads; the avalanche will close the top one at (2, 0).
        let mut level = Level::new(
            Grid::parse(
                "
                S...M
                .^^^.
                .....
                ",
            ),
            species::load(),
        );
        level.waves = vec![vec![]];
        level.changes =
            vec![crate::level::data::Change { wave: 1, at: vec![(2, 0)], becomes: Tile::Roc }];
        assert_eq!(level.refusal(2, 0), Some(n_("refusal.relief_changes")));
        assert_eq!(
            level.refusal(12, 0),
            Some("refusal.relief_closes"),
            "the bottom road will be the only one"
        );
        level.start_wave();
        assert_eq!(level.terrain.base().tiles[2], Tile::Roc);
        assert!(level.changes.is_empty(), "{} changes left", level.changes.len());
    }

    /// Le Gel (world 2): the lake freezes for two waves, then thaws.
    #[test]
    fn le_gel_freezes_the_lake() {
        let mut level = Level::new(
            Grid::parse(
                "
                S.~.M
                ",
            ),
            species::load(),
        );
        level.world = 1;
        level.events = vec![2];
        let ice = level.terrain.base().under(&level.world().event);
        assert_eq!(ice.tiles[2], Tile::Glace);
        assert_eq!(
            (1..=4).map(|w| level.event_at(w)).collect::<Vec<_>>(),
            [false, true, true, false],
            "two waves"
        );
    }

    /// Le prix du nom: inking over a named place costs one Name, once, however many of its cells are inked.
    #[test]
    fn inking_a_place_scratches_its_name() {
        let mut level = Level::new(Grid::parse("S.....M"), species::load());
        level.places = vec![crate::level::data::Place {
            name: "place.pre_commun".into(),
            at: vec![(2, 0), (3, 0)],
        }];
        assert_eq!(level.place_at(2), Some(0));
        let boue = tool_index("boue").unwrap();
        level.ink(2, boue);
        level.ink(3, boue);
        assert_eq!(level.names, -1);
        assert_eq!(level.place_at(3), None, "the name is gone");
        assert!(level.margin.last().unwrap().shown().contains("le Pré-Commun"));

        let mut save = Save::default();
        level.finish(&mut save);
        assert_eq!(
            save.levels[0].erased,
            ["place.pre_commun"],
            "the carnet and the Grand Plan remember it"
        );
    }

    #[test]
    fn skills_change_costs() {
        let save = Save {
            skills: vec![
                Skill::MineDePlomb,
                Skill::BoueProfonde,
                Skill::RoncesMeres,
                Skill::Fascines,
            ],
            ..Default::default()
        };
        // 1-1, after the prologue: it has brambles to spread.
        let level = Level::load(1, &save);
        let cost = |name: &str| level.tool_cost(tool_index(name).unwrap());
        assert_eq!((cost("arbalète"), cost("barricade"), cost("boue")), (45, 7, 15));
        let chaineur = &level.species[index_of(&level.species, "chaineur").unwrap()];
        assert_eq!(chaineur.cost(Tile::Boue), 4.0, "Boue profonde: 3 + 1");

        let mut level = level;
        let ronces = |level: &Level| {
            level.terrain.base().tiles.iter().filter(|&&t| t == Tile::Ronces).count()
        };
        let before = ronces(&level);
        level.grow_ronces();
        assert_eq!(ronces(&level), before + 1, "Ronces-mères: one more bramble per wave");
    }

    /// The click on Biffer takes the tower down through combat's message, and only when a tower was taken.
    #[test]
    fn biffing_a_tower_tells_combat() {
        let biffer = tool_index("biffer").unwrap();
        let mut level = Level::new(Grid::parse("S...M\n.....\n....."), species::load());
        level.ink(7, tool_index("arbalète").unwrap());
        level.ink(2, tool_index("barricade").unwrap());
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_message::<TowerInked>()
            .add_message::<TowerBiffed>()
            .init_resource::<ButtonInput<KeyCode>>()
            .insert_resource(Selection { tool: biffer, ..Selection::for_level(&level) })
            .insert_resource(level)
            .add_systems(Update, inking);
        for cell in [7, 2] {
            app.world_mut().resource_mut::<Selection>().hovered = Some(cell);
            let mut mouse = ButtonInput::<MouseButton>::default();
            mouse.press(MouseButton::Left);
            app.insert_resource(mouse);
            app.update();
        }
        let messages = app.world().resource::<Messages<TowerBiffed>>();
        let biffed: Vec<usize> = messages.get_cursor().read(messages).map(|m| m.cell).collect();
        assert_eq!(biffed, vec![7], "the tower, not the barricade");
        assert_eq!(app.world().resource::<Level>().terrain.base().tiles[7], Tile::Sol);
    }
}
