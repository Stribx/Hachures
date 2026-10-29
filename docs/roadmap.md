# Roadmap

What is done, what comes next, and how far the whole game is. The design is `docs/bible.md`; this
file only tracks it.

**Total: ≈ 40%** (last updated 2026-10-29, after 28a6517).

## How the % is counted

Each section below is a checklist with a weight. A section's % is its ticked items over its items;
the total is the weighted sum. Recount it rather than guess it.

| Section | Weight | Done | Counts |
|---|---|---|---|
| Engine and systems | 30% | 25 / 41 (61%) | 18.3 |
| Levels and bosses | 35% | 9 / 32 (28%) | 9.8 |
| Powers and skill tree | 15% | 13 / 21 (62%) | 9.3 |
| Endings and post-game | 10% | 1 / 13 (8%) | 0.8 |
| Narration, sound, art | 10% | 9 / 40 (22%) | 2.2 |
| **Total** | | | **≈ 40%** |

The first estimate (before this file) said ≈ 30%, but its own table summed to ≈ 42%, with the
engine guessed at 85%. Counted item by item, the engine is at 61%: worlds 4–6 still need most of
their mechanics.

**Upkeep.** Tick an item with the commit that lands it, `[x] Item (abc1234)`. Update "Next up", the
table and the total in the same change. When the bible adds work, add items; never drop an unticked
one without a word in the commit. Items done before this file are marked `(before 248d9c6)`.

## Next up

1. **World 3, milestone 2:** `worlds/3.ron`, the karst region of the Grand Plan (`EAST`, a karst
   ramp, `Biome::Karst`), levels 3-1, 3-2 (the darkness) and 3-3 (the Galerie Signée choice).
2. **World 3, milestone 3:** la Foreuse and level 3-4.
3. **World 4:** les Dunes migrantes, the Héliographe, the wells, le Méridien, then Dater.
4. **Bonus sheets:** the feuillet system on the Grand Plan, starting with the Moulin noyé (Gratter,
   then Renommer, both in hand).

## Bugs and polish

Not counted in the %: small debts found along the way.

- [ ] Run the web build in a browser: the wind shaders, the sound after the first click, the figures
- [ ] The Calque prints a raw "inf" for a cost or a distance (`ui/readout.rs`)
- [ ] A thin gap shows between neighbouring palissade rails
- [ ] Cloud shadows can't be turned off (a setting, or the Low preset)

## Open questions for the bible

Not counted in the %: the owner decides, then the bible changes first.

- Fuyards have no HP in section 8, yet the mortar "peut les blesser" (the code gives them HP 0.5).
- "Légat" names both the boss commanders and a world-6 enemy.
- The Échassiers' defection is "une partie" in world 2 and "la moitié" in world 5.
- The Chaîneur d'élite has no row in section 8, and its "Chaussée à 0,5" is already the base
  Chaîneur's.
- `docs/art-prompts.md` says "not an old engraving" and bans watercolour, while the bible and the
  art direction want an illuminated engraving with watercolour washes; it still mentions the corner
  marks the art direction removed.
- Answered: the Étayeur mends "bornes endommagées", and bornes now have HP (28a6517).

## Engine and systems (30%)

- [x] Grid, tiles and per-species cost profiles in RON (before 248d9c6)
- [x] Lecture de la Pente (before 248d9c6)
- [x] Lecture de l'Ombre (before 248d9c6)
- [x] Lecture du But: Borne-Mère, Refuge, Sortie, imperial bornes (before 248d9c6)
- [x] Le Droit de Passage, on the map now and on the maps announced (before 248d9c6)
- [x] Inking, the pencil preview, the sépia economy (before 248d9c6)
- [x] Biffer, le repentir (before 248d9c6)
- [x] Gratter (before 248d9c6)
- [x] La Crue, world 1's event (before 248d9c6)
- [x] Le Gel, world 2's event (before 248d9c6)
- [x] Scripted changes of the relief, the 2-2 avalanche (before 248d9c6)
- [x] La Tension de la feuille (before 248d9c6)
- [x] Combat: HP, damage, weaknesses; arbalète, mortier, harpon (before 248d9c6)
- [x] The Lanterne (before 248d9c6)
- [x] Boss phases from data: Chaussée, stun, bornes, pin (before 248d9c6)
- [x] The escort and the Fuyards who bring Names (before 248d9c6)
- [x] Margin notes, end-of-level choices and their consequences (before 248d9c6)
- [x] Saves, sceaux, the Lexique, the Calque (before 248d9c6)
- [x] The Grand Plan, menus, settings, credits (before 248d9c6)
- [x] Text in French and English (before 248d9c6)
- [x] Renommer and the price of a Name (248d9c6)
- [x] The Théodolite's rules: the freezing line, the seal, the thaw, the crack (248d9c6)
- [x] The Gouffre and les Effondrements, announced in red (28a6517)
- [x] The Sonde and what walks underground (28a6517)
- [x] Armour, mending bornes, passing under barricades (28a6517)
- [ ] The darkness of 3-2: only the Calque shows the columns
- [ ] The karst region of the Grand Plan and world 3's biome
- [ ] La Foreuse's rules: underground, chain collapses, Taupiers skirting the gouffres
- [ ] A choice that adds a spawn source later (keeping the plan, 3-3 → world 4)
- [ ] Les Dunes migrantes, the Dune and Saline tiles (world 4)
- [ ] The Héliographe (world 4)
- [ ] The wells as a goal for the Caravaniers rancuniers (world 4)
- [ ] Le Méridien's rules: the uniform profile, the Cadastré line (world 4)
- [ ] Dater: suspend or bring forward the event, once per level
- [ ] Les Écluses and the Vanne tile, on a RON timetable (world 5)
- [ ] The Treuil (world 5)
- [ ] Defection in battle: conscripts turn Fuyards [P] (world 5)
- [ ] L'Éclusier's rules: locks off schedule, the siege of the Grande Vanne (world 5)
- [ ] La Mise au net and the Cadastré tile (world 6)
- [ ] Le Dernier Cadastre's rules (world 6)
- [ ] Bonus sheets: locked feuillets on the Grand Plan, opened by powers

## Levels and bosses (35%)

Campaign: the prologue and 6 worlds × 4 levels (the 4th is the boss). Bonus sheets: 7.

- [x] 0-0, the prologue (before 248d9c6)
- [x] 1-1 (before 248d9c6)
- [x] 1-2 (before 248d9c6)
- [x] 1-3 (before 248d9c6)
- [x] 1-4, la Grande Chaîne (before 248d9c6)
- [x] 2-1 (before 248d9c6)
- [x] 2-2 (before 248d9c6)
- [x] 2-3, the escort (before 248d9c6)
- [x] 2-4, le Théodolite de Givre (248d9c6)
- [ ] 3-1
- [ ] 3-2, the darkness
- [ ] 3-3, la Galerie Signée
- [ ] 3-4, la Foreuse
- [ ] 4-1
- [ ] 4-2
- [ ] 4-3
- [ ] 4-4, le Méridien
- [ ] 5-1
- [ ] 5-2
- [ ] 5-3
- [ ] 5-4, l'Éclusier
- [ ] 6-1
- [ ] 6-2
- [ ] 6-3
- [ ] 6-4, le Dernier Cadastre
- [ ] Bonus: le Moulin noyé, first stage (Gratter)
- [ ] Bonus: le Moulin noyé, inside (Renommer)
- [ ] Bonus: le Refuge enseveli (Renommer, then Dater)
- [ ] Bonus: la Salle des Premiers Plans (Dater)
- [ ] Bonus: la Saline engloutie (Gratter, then Dater)
- [ ] Bonus: la Ville d'en dessous (Dater)
- [ ] Bonus: la Salle de Vaux (Signer, post-game)

## Powers and skill tree (15%)

Powers (docs, section 7):

- [x] Tracer (before 248d9c6)
- [x] Biffer (before 248d9c6)
- [x] Gratter (before 248d9c6)
- [x] Renommer (248d9c6)
- [ ] Dater
- [ ] Signer

Skill nodes, 3 branches × 5:

- [x] Mine de plomb (before 248d9c6)
- [x] Équerre (before 248d9c6)
- [x] Contre-visée (before 248d9c6)
- [ ] Double trait
- [ ] Encre fixe
- [x] Boue profonde (before 248d9c6)
- [x] Ronces-mères (before 248d9c6)
- [x] Fascines (before 248d9c6)
- [ ] Grattoir fin
- [ ] Relief vivant
- [x] Lecture fine (before 248d9c6)
- [x] Sceaux brisés (before 248d9c6)
- [x] Ombre portée (before 248d9c6)
- [ ] Leurre
- [ ] Nom gardé

## Endings and post-game (10%)

- [x] The end-of-campaign screen, le carnet (before 248d9c6)
- [ ] What decides the ending: the Rôle des Noms (6 major Names) and la Parole donnée
- [ ] Ending: le Trait unique
- [ ] Ending: la Feuille brûlée, and its "Terre instable" mode
- [ ] Ending: la Légende commune
- [ ] Ending: Oubrenne (secret)
- [ ] Corentin: ally in world 6
- [ ] Corentin: silent surrender
- [ ] Corentin: dead under the press
- [ ] Héroïque mode
- [ ] The 12 défis du Géomètre, in RON
- [ ] The level editor (Signer), with RON import and export
- [ ] La Carte signée: the locked zones shown on the Grand Plan (world 3's reward)

## Narration, sound, art (10%)

Narration (margin notes, boss lines, choices; drafts allowed where the bible is silent):

- [x] Prologue (before 248d9c6)
- [x] World 1 (before 248d9c6)
- [x] World 2 (248d9c6)
- [ ] World 3
- [ ] World 4
- [ ] World 5
- [ ] World 6

Sound (one placeholder track and five Kenney sound effects today):

- [ ] Nève's theme, five limping notes
- [ ] Score, world 1: cello and hurdy-gurdy drone
- [ ] Score, world 2: bells a quarter-tone low, wordless choir
- [ ] Score, world 3: metal percussion, tuned drops, double bass
- [ ] Score, world 4: bowed strings, breath, bullroarer
- [ ] Score, world 5: slow barrel organ, lock bells
- [ ] Score, world 6: mechanical harpsichord, choir of parcel numbers
- [ ] SFX, world 1: quill, stamp, chain, flood rumble
- [ ] SFX, world 2: ice cracking, the lens singing, wind in the ropes
- [ ] SFX, world 3: groaning props, the well's breath, giving rock
- [ ] SFX, world 4: running sand and the rest of section 6
- [ ] SFX, world 5: racks, rushing water, towing ropes
- [ ] SFX, world 6: the press
- [x] Ambience, world 1: birds, rain, river, wind (before 248d9c6)
- [x] Ambience, world 2: snow wind (before 248d9c6)
- [ ] Ambience, world 3
- [ ] Ambience, world 4
- [ ] Ambience, world 5
- [ ] Ambience, world 6

Art:

- [x] The figurine kit: one body, silhouettes and props per people (before 248d9c6)
- [x] World 1's land: the bocage (before 248d9c6)
- [x] World 2's land: the mountains (before 248d9c6)
- [ ] World 3's land: karst and slate
- [ ] World 4's land: dunes and salt
- [ ] World 5's land: polders and the low town
- [ ] World 6's land: the Plan-Relief
- [x] Boss: la Grande Chaîne, its cart, oxen and chain (before 248d9c6)
- [ ] Boss: le Théodolite, its runners and lens (it borrows the cart)
- [ ] Boss: la Foreuse
- [ ] Boss: le Méridien
- [ ] Boss: l'Éclusier's barge
- [ ] Boss: Ilse's press
- [ ] Tower models of our own (the four Kenney pieces, the Sonde's plain shapes)
