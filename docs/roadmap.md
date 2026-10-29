# Roadmap

What is done, what comes next, and how far the whole game is. The design is `docs/bible.md`; this
file only tracks it.

**Total: ≈ 73%** (recounted for the `v0.1.0` release candidate).

## Releases

Release-specific scope, acceptance criteria, limitations, and validation records live in
[`docs/releases/`](releases/). The current planned milestone is
[`v0.1.0`](releases/v0.1.0.md).

## How the % is counted

Each section below is a checklist with a weight. A section's % is its ticked items over its items;
the total is the weighted sum. Recount it rather than guess it.

| Section | Weight | Done | Counts |
|---|---|---|---|
| Engine and systems | 30% | 36 / 41 (88%) | 26.3 |
| Levels and bosses | 35% | 26 / 33 (79%) | 27.6 |
| Powers and skill tree | 15% | 17 / 21 (81%) | 12.1 |
| Endings and post-game | 10% | 3 / 14 (21%) | 2.1 |
| Narration, sound, art | 10% | 21 / 40 (53%) | 5.3 |
| **Total** | | | **≈ 73%** |

**Upkeep.** Tick an item with the commit that lands it, `[x] Item (abc1234)`. Update "Next up", the
table and the total in the same change. When the bible adds work, add items; never drop an unticked
one without a word in the commit. Items done before this file are marked `(before 248d9c6)`.

## Next up

1. **Post-World 6 polish:** finish the full Treuil interaction, defection branches, and end-game editor.
2. **Bonus sheets:** the feuillet system on the Grand Plan, starting with the Moulin noyé (Gratter,
   then Renommer, both in hand).

## Bugs and polish

Not counted in the %: small debts found along the way.

- [ ] Run the web build in a browser: the wind shaders, the sound after the first click, the figures
- [x] The Calque prints a raw "inf" for a cost or a distance (`ui/readout.rs`)
- [x] A thin gap shows between neighbouring palissade rails
- [x] Cloud shadows can't be turned off (a setting, or the Low preset)

## Resolved design decisions

Not counted in the %: these decisions are recorded in the bible before their dependent implementation work.

- Fuyards have no HP and cannot be damaged; mortar collateral-damage wording was corrected in the bible.
  Fuyards are invulnerable; obsolete hurt challenges and notes were removed from the implemented levels.
- **Légat** remains the name of the boss-commanders; the world-6 enemy is renamed **Mandataire**.
- The amount of Échassiers defecting in world 5 depends on the player's world-2 choice; the exact quantities
  and data representation remain part of the world-5 implementation.
- The Chaîneur d'élite gets a distinct world-5 profile and a dedicated bestiary row; its exact statistics remain
  part of the world-5 balance pass.
- The art direction is an illuminated engraving with watercolor washes. `docs/art-prompts.md` must be aligned
  with that direction before new generated art is produced.
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
- [x] The darkness of 3-2: only the Calque shows the columns
- [x] The karst region of the Grand Plan and world 3's biome
- [x] La Foreuse's rules: underground, chain collapses, Taupiers skirting the gouffres
- [ ] A choice that adds a spawn source later (keeping the plan, 3-3 → world 4)
- [x] Les Dunes migrantes, the Dune and Saline tiles (world 4)
- [ ] The Héliographe (world 4)
- [x] The wells as a goal for the Caravaniers rancuniers (world 4)
- [x] Le Méridien's rules: the uniform profile, the Cadastré line (world 4)
- [x] Dater: suspend or bring forward the event, once per level
- [x] Les Écluses and the Vanne tile, on a RON timetable (world 5)
- [ ] The Treuil (world 5)
- [ ] Defection in battle: conscripts turn Fuyards [P] (world 5)
- [x] L'Éclusier's phase data and off-schedule lock changes (world 5)
- [x] La Mise au net and the Cadastré tile (world 6)
- [x] Le Dernier Cadastre's phase data and cadastral rings (world 6)
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
- [x] 3-1
- [x] 3-2, the darkness
- [x] 3-3, la Galerie Signée
- [x] 3-4, la Foreuse
- [x] 4-1
- [x] 4-2
- [x] 4-3
- [x] 4-4, le Méridien
- [x] 5-1
- [x] 5-2
- [x] 5-3
- [x] 5-4, l'Éclusier
- [x] 6-1
- [x] 6-2
- [x] 6-3
- [x] 6-4, le Dernier Cadastre
- [ ] Bonus: le Moulin noyé, first stage (Gratter)
- [ ] Bonus: le Moulin noyé, inside (Renommer)
- [ ] Bonus: le Refuge enseveli (Renommer, then Dater)
- [ ] Bonus: la Salle des Premiers Plans (Dater)
- [ ] Bonus: la Saline engloutie (Gratter, then Dater)
- [ ] Bonus: la Ville d'en dessous (Dater)
- [ ] Bonus: la Salle de Vaux (Signer, post-game)
- [x] Bonus-sheet RON registry and validation smoke tests

## Powers and skill tree (15%)

Powers (docs, section 7):

- [x] Tracer (before 248d9c6)
- [x] Biffer (before 248d9c6)
- [x] Gratter (before 248d9c6)
- [x] Renommer (248d9c6)
- [x] Dater
- [ ] Signer

Skill nodes, 3 branches × 5:

- [x] Mine de plomb (before 248d9c6)
- [x] Équerre (before 248d9c6)
- [x] Contre-visée (before 248d9c6)
- [x] Double trait
- [ ] Encre fixe
- [x] Boue profonde (before 248d9c6)
- [x] Ronces-mères (before 248d9c6)
- [x] Fascines (before 248d9c6)
- [x] Grattoir fin
- [ ] Relief vivant
- [x] Lecture fine (before 248d9c6)
- [x] Sceaux brisés (before 248d9c6)
- [x] Ombre portée (before 248d9c6)
- [ ] Leurre
- [x] Nom gardé

## Endings and post-game (10%)

- [x] The end-of-campaign screen, le carnet (before 248d9c6)
- [x] Ending state, final-choice routing, and carnet display
- [ ] Ending: le Trait unique full consequence
- [ ] Ending: la Feuille brûlée, and its "Terre instable" mode
- [ ] Ending: la Légende commune full consequence
- [ ] Ending: Oubrenne (secret) full Heroic path
- [ ] Corentin: ally in world 6
- [ ] Corentin: silent surrender
- [ ] Corentin: dead under the press
- [ ] Héroïque mode
- [ ] The 12 défis du Géomètre, in RON
- [ ] The level editor (Signer), with RON import and export
- [x] Signer preview screen with safe, validated RON output
- [ ] La Carte signée: the locked zones shown on the Grand Plan (world 3's reward)

## Narration, sound, art (10%)

Narration (margin notes, boss lines, choices; drafts allowed where the bible is silent):

- [x] Prologue (before 248d9c6)
- [x] World 1 (before 248d9c6)
- [x] World 2 (248d9c6)
- [x] World 3
- [x] World 4
- [x] World 5
- [x] World 6

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
- [x] World 3's land: karst and slate
- [x] World 4's land: dunes and salt
- [x] World 5's land: polders and the low town
- [x] World 6's land: the Plan-Relief
- [x] Boss: la Grande Chaîne, its cart, oxen and chain (before 248d9c6)
- [ ] Boss: le Théodolite, its runners and lens (it borrows the cart)
- [x] Boss: la Foreuse
- [x] Boss: le Méridien
- [x] Boss: l'Éclusier's barge
- [x] Boss: Ilse's press
- [ ] Tower models of our own (the four Kenney pieces, the Sonde's plain shapes)
