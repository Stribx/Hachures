# Le bestiaire

**Status:** ready-for-agent (the owner approved ticket 01 on 2026-10-27).

## Problem Statement

The board is a hand-coloured relief model: felt fields, wind, birds, inked pieces that fall into place. The enemies were never part of that work.
- Each species is two to four grey primitives in one flat colour, under the coarse hachure.
- They slide to the next cell without a step and snap their facing.
- A hit shows nothing, and they vanish at the Cible.
- They die as eight dark cubes.
- La Grande Chaîne is a dark box on four wheels.

They are the least "maquette" thing on the board, and they are what the player watches most.

## Solution

Every species becomes a **painted wooden figurine of its people**, moved across the model by an invisible hand.
- **The figurine:** a turned peg-doll on a felt base, with a hat per people and the props of its species: the Chaîneur's chain, the Jalonneur's pole, stilts, the Fuyard's bundle, a hood and an alpenstock, pack boxes.
- **The seal:** every hostile carries a seal-red wax seal on its chest; allies carry none.
- **The motion:** a step driven by the distance walked, eased turns, a jolt on a hit, a stagger when pinned, a seal that bursts into sépia on death, a figure that sinks into the board at the Cible.
- **La Grande Chaîne:** a cart with two oxen, Gast, a sighting lantern, a yoke, and a 20-link chain dragging along its path.

The art direction is in `docs/direction-artistique.md`, "Phase 7 : le bestiaire", with a card for every species and boss in the bible.

## Decisions (agreed with the owner)

- **Built in code**, from one kit of primitives (`docs/3d-assets.md` rule table). No generated models.
- **Procedural motion, "moved by hand".** No rigs, no glTF clips.
- **Bosses are assembled in code** from the same kit (bible §9). `3d-assets.md` no longer says "boss → asset".
- **Scope:** the 9 species in `species.ron` are rebuilt. Every other bible species and boss gets a card only.
- **Readability:**
  - value before hue: coats are darker than the fields;
  - red belongs only to the seal;
  - each species can be named from its silhouette in greyscale.
- **Visual only:** the balance table (`game/tests/balance.txt`) stays identical through every ticket. Enemies despawn on the same frame as before; death and arrival effects are separate visual-only entities.

## Tickets (one file each under `issues/`)

| # | Ticket | Blocked by | Delivers |
|---|---|---|---|
| 01 | Le bestiaire on paper | — | `ready-for-human`: DA phase 7 (figurine, seal, palette per people, motion, cards); the `3d-assets.md` rule table fixed. The owner approves. |
| 02 | The parade | — | `HACHURES_PARADE=1`: one still figure of each species in a row; baseline shots of today's enemies. |
| 03 | The figurine kit | 01 | `Carry` in the species data, a pure `kit::parts`, `EnemyLook` built per species, fine hachure. The Chaîneur is dressed first. |
| 04 | Every people dressed | 03 | Props and coats for every species, following the cards. |
| 05 | Moved by hand | 03 | Gaits, the step driven by distance, eased facing, still while paused. |
| 06 | Hit, pin, death, arrival | 05 | The jolt, the stagger, the seal bursting into sépia with a blot, the exit ghosts. |
| 07 | La Grande Chaîne and the bornes | 04, 05 | The boss assembled, the chain `trail`, the panic of phase 3, the new borne markers. |
| 08 | Final pass | 01–07 | GPU timings with a full wave, the web build, art review round 10, every "fini quand" checked. |

Every code ticket also requires:
- **TDD:** the ticket's tests are written first and seen failing, then made to pass;
- fmt, clippy `-D warnings`, the tests and the structure test pass;
- the balance table is unchanged;
- before/after shots: the parade, plus a wave with `HACHURES_AT=t1,t2` for anything that moves.

## Testing Decisions

- **The seams are pure functions:**
  - `kit::parts(shape, carries, hostile)`: seal rule, footprint, distinct silhouettes, value contrast against the fields;
  - `gait::pose(gait, phase, moving)`: never under the ground, periodic, identity when still;
  - `jolt(t)`: decays to zero;
  - `chain::trail(history, links, spacing)`: spacing, first link, short history.
- **ECS behaviour** is tested with the `combat/tests.rs` helpers (`app_with`, `walker`, `run`):
  - the stride grows with distance and freezes while pinned;
  - yaw eases toward the heading;
  - the enemy despawns on the same frame as before while a ghost plays;
  - phase 3 changes the oxen's gait.
- **Shape, colour and motion are judged on timed screenshots**, not unit tests.

## Out of Scope

- Rigged or skeletal animation; generated enemy models.
- New species in code (the Taupier and the others have cards only).
- The Kenney towers.
- Any rule, number or i18n text.

## Further Notes

Contradictions the review found, left for the owner (not decided here):
- **`docs/art-prompts.md`** says "not an old engraving" and bans watercolour in its negative prompt, while the bible and the DA say "gravure enluminée, lavis d'aquarelle". It still mentions corner marks the DA has removed.
- **Bible open questions:**
  - Fuyards have no HP in §8, yet the mortar "peut les blesser".
  - The Étayeur repairs "bornes endommagées", but bornes have no HP in the docs.
  - "Légat" names both the boss commanders and a world 6 enemy.
  - The Échassiers' defection is "une partie" in world 2 and "la moitié" in world 5.
  - The Chaîneur d'élite has no row in §8, and its "Chaussée à 0,5" is already the base Chaîneur's.
- **`boss_phases`** switches the boss's species without rebuilding its look. Ticket 07 makes phase 3 visible through the gait.
