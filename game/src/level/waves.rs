//! Waves, the sépia they pay, and La Tension de la feuille.

use bevy::prelude::*;

use super::Level;
use super::story::Trigger;
use crate::campaign::skills::Skill;

/// Progress of the current wave.
#[derive(Default)]
pub struct Wave {
    /// 1-based; 0 before the first wave.
    pub number: u32,
    /// From Space until its last enemy is gone.
    pub running: bool,
    /// Species still to spawn, last one first.
    pub queue: Vec<usize>,
    pub spawned: usize,
    pub spawn_in: f32,
}

/// Spawn order of a wave, last one first: one of each (species, count) group in turn, so species come mixed.
pub fn spawn_order(mut groups: Vec<(usize, u32)>) -> Vec<usize> {
    let mut order = Vec::new();
    while groups.iter().any(|g| g.1 > 0) {
        for (species, count) in &mut groups {
            if *count > 0 {
                order.push(*species);
                *count -= 1;
            }
        }
    }
    order.reverse();
    order
}

impl Level {
    /// Sends the next wave, and the world event with it if it is due. Returns whether the wave after it
    /// brings the event, for the rumble.
    ///
    /// # Panics
    ///
    /// If every wave has been sent already: the caller checks that one is left.
    pub fn start_wave(&mut self) -> bool {
        let sent = self.wave.number as usize;
        let number = self.wave.number + 1;
        let mut queue = self.waves[sent].clone();
        if self.tension >= 2 {
            self.mix(&mut queue);
        }
        self.wave = Wave { number, running: true, queue, ..default() };
        self.lives_at_wave_start = self.lives;
        // Scripted changes due now fall on the empty map.
        let (due, rest): (Vec<_>, _) = std::mem::take(&mut self.changes).into_iter().partition(|c| c.wave == number);
        self.changes = rest;
        let width = self.terrain.base().width;
        self.alter(due.iter().flat_map(|c| c.edits(width)));
        let started = self.event_at(number) && !self.event_at(number - 1);
        self.event_on = self.event_at(number);
        self.event_next = self.event_at(number + 1);
        self.follow_event();
        if self.has(Skill::RoncesMeres) {
            self.grow_ronces();
        }
        self.note(Trigger::Wave(number));
        if started {
            self.note(Trigger::Event);
        }
        self.event_next
    }

    /// The wave's last enemy is gone: the world event withdraws until the next wave, and the sheet tightens or buckles.
    pub fn end_wave(&mut self) {
        self.wave.running = false;
        if self.event_on {
            self.event_on = false;
            self.follow_event();
        }
        let lost = self.lives_at_wave_start.saturating_sub(self.lives) as i32;
        self.tension = (self.tension + if lost == 0 { 1 } else { -lost.min(2) }).clamp(-2, 2);
        debug!("wave {} lost {lost} Rôle: tension {}", self.wave.number, self.tension);
    }

    /// The sheet is taut: the Chancellerie sends more mixed waves. Every third enemy of the wave's
    /// most common hostile species becomes another hostile species of the same wave. Species sent
    /// alone (the boss) are left as they are.
    fn mix(&self, queue: &mut [usize]) {
        let count = |k: usize| queue.iter().filter(|&&s| s == k).count();
        let mut kinds: Vec<usize> =
            queue.iter().copied().filter(|&s| self.species[s].hostile && count(s) > 1).collect();
        kinds.sort_unstable();
        kinds.dedup();
        let Some(&common) = kinds.iter().max_by_key(|&&k| count(k)) else { return };
        let others: Vec<usize> = kinds.into_iter().filter(|&k| k != common).collect();
        if others.is_empty() {
            return;
        }
        let swapped = queue.iter_mut().filter(|s| **s == common).skip(2).step_by(3);
        for (n, species) in swapped.enumerate() {
            *species = others[n % others.len()];
        }
    }

    /// Sépia for an enemy shot down.
    pub fn loot(&mut self, sepia: u32) {
        let skill = if self.has(Skill::SceauxBrises) { 1.15 } else { 1.0 };
        // The sheet buckles: the player struggles, the sceaux give back more ink.
        let buckled = if self.tension <= -1 { 1.10 } else { 1.0 };
        self.loot_carry += sepia as f32 * skill * buckled;
        let whole = self.loot_carry.floor();
        self.sepia += whole as u32;
        self.loot_carry -= whole;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::campaign::save::Save;
    use crate::map::species::index_of;

    #[test]
    fn groups_interleave() {
        assert_eq!(spawn_order(vec![(0, 2), (1, 1)]), vec![0, 1, 0]);
    }

    #[test]
    fn tension_de_la_feuille() {
        let mut level = Level::load(0, &Save::default());
        level.tension = 2;
        let k = |name| index_of(&level.species, name).unwrap();
        let (c, e, boss) = (k("chaineur"), k("echassier"), k("grande_chaine"));
        let mut queue = vec![c, c, c, e, c, c, c, e];
        level.mix(&mut queue);
        assert_eq!(queue, vec![c, c, e, e, c, c, e, e], "taut: every third of the commonest becomes another");
        let mut lone = vec![c, c, c, boss];
        level.mix(&mut lone);
        assert_eq!(lone, vec![c, c, c, boss], "nothing else to mix with: a species sent alone stays alone");

        level.tension = 0;
        level.start_wave();
        level.lives -= 2;
        level.end_wave();
        assert_eq!(level.tension, -2, "two Rôle lost: the sheet buckles");
        level.sepia = 0;
        level.loot(10);
        assert_eq!(level.sepia, 11, "a buckled sheet gives back 10 % more ink");
    }
}
