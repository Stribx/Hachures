//! The label down the left side: the level's state, or the sceaux tree.

use bevy::prelude::*;

use super::{Hud, TreeOpen};
use crate::campaign::save::Save;
use crate::campaign::skills::{BRANCHES, Skill};
use crate::ink::TOOLS;
use crate::level::Level;
use crate::level::data::LEVELS;

pub fn update_label(level: Res<Level>, save: Res<Save>, tree: Res<TreeOpen>, mut label: Single<&mut Text, With<Hud>>) {
    if tree.0 {
        label.0 = tree_text(&save, level.finished);
        return;
    }
    let tool = &TOOLS[level.tool];
    let tools: Vec<String> = level.tools.iter().map(|&t| format!("[{}] {}", t + 1, TOOLS[t].name)).collect();
    let mut text = format!(
        "{}\n\
         Sépia : {}    Rôle : {}    Noms : {}    Vague : {} / {}\n\
         Outil : {} ({})    Espèce : {}\n\
         {}\n\
         Espace : vague suivante    Tab : espèce    F1 : Calque    A : arbre des sceaux",
        level.name,
        level.sepia,
        level.lives,
        level.names,
        level.wave.number,
        level.waves.len(),
        tool.name,
        level.tool_cost(level.tool),
        level.species[level.shown].name,
        tools.join(" "),
    );
    if let Some(challenge) = &level.challenge {
        text += &format!("\nDéfi : {}", challenge.text);
    }
    if let Some(phase) = &level.boss {
        text += &format!("\n{} : {phase}", level.boss_file().name);
    }
    if level.finished {
        let challenge = if level.challenge_met() { "réussi, second sceau gagné" } else { "manqué" };
        text +=
            &format!("\nVictoire : {} tient. Sceau de la victoire gagné ; défi {challenge}.", level.world().defends);
        if let Some(choice) = &level.choice {
            text += &format!("\n{}", choice.prompt);
            for (k, (option, label)) in choice.options.iter().enumerate() {
                let picked = if level.choices.get(&choice.id) == Some(option) { "  ← choisi" } else { "" };
                text += &format!("\n[{}] {label}{picked}", k + 1);
            }
        }
        text += if !level.chosen() {
            ""
        } else if level.index + 1 < LEVELS.len() {
            "\nEntrée : niveau suivant."
        } else {
            "\nFin du chapitre jouable."
        };
    }
    let event = &level.world().event;
    if level.event_on {
        text += &format!("\n{}", event.during);
    } else if level.event_next {
        text += &format!("\n{}", event.announce);
    }
    if level.lives == 0 {
        text += &format!("\nLe Rôle est effacé : {} n'a pas tenu. Entrée : recommencer.", level.world().defends);
    }
    if let Some(i) = level.hovered.filter(|_| !level.finished) {
        match level.refusal(i) {
            Some(why) => text += &format!("\n{why}"),
            // Lecture fine: what the pencil line costs the shown species, before and after.
            None if level.has(Skill::LectureFine) => {
                let (what_if, field) = level.what_if(i);
                if let Some(&spawn) = what_if.spawns_for(&level.species[level.shown]).first() {
                    text += &format!("\nTrait : {} → {}", level.fields[level.shown].dist[spawn], field.dist[spawn]);
                }
            }
            None => {}
        }
        if level.calque {
            let tile = level.grid.tiles[i];
            let costs: Vec<String> = level
                .present
                .iter()
                .map(|&k| {
                    format!(
                        "{} {} (but à {})",
                        level.species[k].name,
                        level.species[k].cost(tile),
                        level.fields[k].dist[i]
                    )
                })
                .collect();
            text += &format!("\nCalque — {tile:?} : {}", costs.join(", "));
        }
    }
    label.0 = text;
}

fn tree_text(save: &Save, can_buy: bool) -> String {
    let mut text =
        format!("Arbre des sceaux : {} à dépenser    Rôle des Noms : {}    A : fermer\n", save.sceaux(), save.names());
    text += if can_buy {
        "[1] [2] [3] : acheter le nœud suivant de la branche\n"
    } else {
        "On achète entre deux niveaux.\n"
    };
    for (b, (branch, nodes)) in BRANCHES.iter().enumerate() {
        text += &format!("\n[{}] {branch}\n", b + 1);
        for node in nodes {
            let owned = if save.skills.contains(&node.skill) { "[x]" } else { "[ ]" };
            text += &format!("    {owned} {} ({}) : {}\n", node.name, node.cost, node.effect);
        }
    }
    text + "\nPaliers 4 et 5 : avec les pouvoirs des mondes suivants."
}
