//! L'arbre des sceaux (docs, section 7): three branches, each bought node by node.

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
    DoubleTrait,
    GrattoirFin,
    Leurre,
    EncreFixe,
    ReliefVivant,
    NomGarde,
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

/// A branch of the tree: its name (an i18n key) and its nodes in buying order.
pub struct Branch {
    pub name: &'static str,
    pub nodes: [Node; 5],
}

pub const BRANCHES: [Branch; 3] = [
    Branch {
        name: n_("skill.trait"),
        nodes: [
            node(
                Skill::MineDePlomb,
                n_("skill.mine_de_plomb.name"),
                1,
                n_("skill.mine_de_plomb.effect"),
            ),
            node(Skill::Equerre, n_("skill.equerre.name"), 2, n_("skill.equerre.effect")),
            node(
                Skill::ContreVisee,
                n_("skill.contre_visee.name"),
                3,
                n_("skill.contre_visee.effect"),
            ),
            node(
                Skill::DoubleTrait,
                n_("skill.double_trait.name"),
                4,
                n_("skill.double_trait.effect"),
            ),
            node(Skill::EncreFixe, n_("skill.encre_fixe.name"), 6, n_("skill.encre_fixe.effect")),
        ],
    },
    Branch {
        name: n_("skill.relief"),
        nodes: [
            node(
                Skill::BoueProfonde,
                n_("skill.boue_profonde.name"),
                1,
                n_("skill.boue_profonde.effect"),
            ),
            node(
                Skill::RoncesMeres,
                n_("skill.ronces_meres.name"),
                2,
                n_("skill.ronces_meres.effect"),
            ),
            node(Skill::Fascines, n_("skill.fascines.name"), 3, n_("skill.fascines.effect")),
            node(
                Skill::GrattoirFin,
                n_("skill.grattoir_fin.name"),
                4,
                n_("skill.grattoir_fin.effect"),
            ),
            node(
                Skill::ReliefVivant,
                n_("skill.relief_vivant.name"),
                6,
                n_("skill.relief_vivant.effect"),
            ),
        ],
    },
    Branch {
        name: n_("skill.legende"),
        nodes: [
            node(
                Skill::LectureFine,
                n_("skill.lecture_fine.name"),
                1,
                n_("skill.lecture_fine.effect"),
            ),
            node(
                Skill::SceauxBrises,
                n_("skill.sceaux_brises.name"),
                2,
                n_("skill.sceaux_brises.effect"),
            ),
            node(
                Skill::OmbrePortee,
                n_("skill.ombre_portee.name"),
                3,
                n_("skill.ombre_portee.effect"),
            ),
            node(Skill::Leurre, n_("skill.leurre.name"), 4, n_("skill.leurre.effect")),
            node(Skill::NomGarde, n_("skill.nom_garde.name"), 6, n_("skill.nom_garde.effect")),
        ],
    },
];

impl Skill {
    pub fn cost(self) -> u32 {
        BRANCHES
            .iter()
            .flat_map(|b| &b.nodes)
            .find(|n| n.skill == self)
            .expect("every skill has a node in BRANCHES")
            .cost
    }
}

/// The next node to buy in `branch`, if any is left.
pub fn next_in(branch: usize, owned: &[Skill]) -> Option<&'static Node> {
    BRANCHES[branch].nodes.iter().find(|n| !owned.contains(&n.skill))
}
