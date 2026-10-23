//! L'arbre des sceaux (docs, section 7): three branches, each bought node by node.
//! Only tiers 1 to 3 are in: world 1 gives 8 sceaux, and tiers 4 and 5 need powers not in yet.

use serde::{Deserialize, Serialize};

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
    ("Le Trait", [
        node(Skill::MineDePlomb, "Mine de plomb", 1, "arbalète −10 %"),
        node(Skill::Equerre, "Équerre", 2, "+1 de portée pour deux tours côte à côte"),
        node(Skill::ContreVisee, "Contre-visée", 3, "mortier +25 % contre les boss"),
    ]),
    ("Le Relief", [
        node(Skill::BoueProfonde, "Boue profonde", 1, "+1 au coût de la boue pour les hostiles"),
        node(Skill::RoncesMeres, "Ronces-mères", 2, "les ronces gagnent une case par vague"),
        node(Skill::Fascines, "Fascines", 3, "barricade −30 %"),
    ]),
    ("La Légende", [
        node(Skill::LectureFine, "Lecture fine", 1, "le trait de crayon affiche les coûts"),
        node(Skill::SceauxBrises, "Sceaux brisés", 2, "+15 % de sépia"),
        node(Skill::OmbrePortee, "Ombre portée", 3, "les tours comptent ×1,5 pour les lecteurs d'Ombre"),
    ]),
];

impl Skill {
    pub fn cost(self) -> u32 {
        BRANCHES.iter().flat_map(|b| &b.1).find(|n| n.skill == self).unwrap().cost
    }
}

/// The next node to buy in `branch`, if any is left.
pub fn next_in(branch: usize, owned: &[Skill]) -> Option<&'static Node> {
    BRANCHES[branch].1.iter().find(|n| !owned.contains(&n.skill))
}
