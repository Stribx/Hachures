//! L'arbre des sceaux (docs, section 7): three branches, each bought node by node.
//! Only tiers 1 to 3 are in: world 1 gives 8 sceaux, and tiers 4 and 5 need powers not in yet.

use serde::{Deserialize, Serialize};

use crate::i18n::n_;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Skill {
    MineDePlomb,
    Equerre,
    ContreVisee,
    BoueProfonde,
    RoncesMeres,
    Fascines,
    LectureFine,
    SceauxBrises,
    OmbrePortee,
}

pub struct Node {
    pub skill: Skill,
    pub name: &'static str,
    pub cost: u32,
    pub effect: &'static str,
}

const fn node(skill: Skill, name: &'static str, cost: u32, effect: &'static str) -> Node {
    Node { skill, name, cost, effect }
}

/// (branch, its nodes in buying order).
pub const BRANCHES: [(&str, [Node; 3]); 3] = [
    (
        n_("skill.trait"),
        [
            node(Skill::MineDePlomb, n_("skill.mine_de_plomb.name"), 1, n_("skill.mine_de_plomb.effect")),
            node(Skill::Equerre, n_("skill.equerre.name"), 2, n_("skill.equerre.effect")),
            node(Skill::ContreVisee, n_("skill.contre_visee.name"), 3, n_("skill.contre_visee.effect")),
        ],
    ),
    (
        n_("skill.relief"),
        [
            node(Skill::BoueProfonde, n_("skill.boue_profonde.name"), 1, n_("skill.boue_profonde.effect")),
            node(Skill::RoncesMeres, n_("skill.ronces_meres.name"), 2, n_("skill.ronces_meres.effect")),
            node(Skill::Fascines, n_("skill.fascines.name"), 3, n_("skill.fascines.effect")),
        ],
    ),
    (
        n_("skill.legende"),
        [
            node(Skill::LectureFine, n_("skill.lecture_fine.name"), 1, n_("skill.lecture_fine.effect")),
            node(Skill::SceauxBrises, n_("skill.sceaux_brises.name"), 2, n_("skill.sceaux_brises.effect")),
            node(Skill::OmbrePortee, n_("skill.ombre_portee.name"), 3, n_("skill.ombre_portee.effect")),
        ],
    ),
];

impl Skill {
    pub fn cost(self) -> u32 {
        BRANCHES.iter().flat_map(|b| &b.1).find(|n| n.skill == self).expect("every skill has a node in BRANCHES").cost
    }
}

/// The next node to buy in `branch`, if any is left.
pub fn next_in(branch: usize, owned: &[Skill]) -> Option<&'static Node> {
    BRANCHES[branch].1.iter().find(|n| !owned.contains(&n.skill))
}
