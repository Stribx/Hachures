# La maquette vivante

**Status:** ready-for-agent

## Problem Statement

The player sees a colourless board: pale beige blocks, beige fog, a sepia vignette, and nothing that moves but the water. The relief model reads as a model, but not as a living land worth defending.

## Solution

The board and the Grand Plan become a hand-coloured bocage quilt. The ink hachures and the paper stay. Watercolour washes go under them, like the Cassini maps, so it is still an engraving, now *enluminée*. Small fields in greens, golds and ochres, made of felt and fabric, are stitched together and edged with hedgerows. Grass, flowers and wheat sway in the wind, and gusts roll across them. Birds, butterflies and cloud shadows move over the land; rain and snow come with the world events; the light grows warmer with each wave. Ambient sound carries it all.

## Decisions (agreed with the owner)

- **Patchwork:** a bocage quilt in felt and fabric. Stitch lines run where two patches meet, and hedgerows run between fields on the Grand Plan.
- **Colour:** the hand-coloured map. Each surface's fade toward the paper colour is set per material, lighter on the land, while the ink lines are kept.
- **Readability:** each tile keeps its own colour family. Patches vary only inside Sol (dry pasture: greens, gold, fallow ochre) and Sol bas (wet meadow: deeper blue-greens). Boue, Ronces, Eau, Roc and Chaussée stay unmistakable. Grass never hides a tile or a marker.
- **Patches:** generated per level from a seed, the same every time the level loads, with no level data to write.
- **Web:** everything runs in the browser too. Grass density and moving life follow the graphics preset (Low is sparse and has no birds). The High preset may take about 2 ms more GPU time than today.
- **Sound:** CC0 only. A shortlist goes to the owner for approval, and every file gets a row in `CREDITS.md`.
- **Visual only:** the balance table (`game/tests/balance.txt`) must stay identical through every ticket.

## Tickets (one file each under `issues/` to come)

| # | Ticket | Blocked by | Delivers |
|---|---|---|---|
| 01 | Wind in the ink | — | The ink material gets a vertex stage, in the main, prepass and shadow passes, with a per-material wind amount and a shared wind direction. Trees on the Grand Plan sway. |
| 02 | The enluminure palette | — | Fade toward the paper set per material; new theme colours (pasture, meadow, wheat, fallow, flax, hedge, sky); warmer fog and vignette; colour families for Sol and Sol bas; a brighter `land_color`. |
| 03 | Fields on the board | 02 | A pure function gives the parcels of a level, deterministic and on Sol and Sol bas only. Each parcel has a patch colour, with felt stitches along its seams. |
| 04 | Grass, flowers and wheat that sway | 01, 03 | Felt tufts, flower dots and wheat rows, chosen by patch and placed by the cell hash. Gusts roll across them, and they never cover a marker. Density follows the preset. |
| 05 | The bocage quilt on the Grand Plan | 01, 02 | Parcels, hedgerows, stitches and swaying crops on the Marches. The test `land_colour_has_no_cuts` allows cuts only along parcel seams. Sheets and the road stay clear. |
| 06 | Cloud shadows | 02 | Soft cloud shadows drift with the wind over the board and the Grand Plan. |
| 07 | Birds and butterflies | 04 | Paper-and-ink flocks and butterflies, built in code. None on the Low preset. |
| 08 | Rain and snow with the world events | 01 | Slanted rain and puddle ripples during la Crue; snow during le Gel and in world 2. |
| 09 | Daylight follows the waves | 02 | The sun drifts later into the afternoon with each wave. Shadows stay readable. |
| 10 | Ambient sounds: CC0 shortlist | — | `ready-for-human`: wind, gusts, birds, rain, water and mill sounds, with sources and licences. |
| 11 | Ambient sounds in the game | 10 | Loops that follow the wind, the events and the place, under a volume setting, with CREDITS rows. |
| 12 | Final pass | 01–11 | GPU timings per preset (`HACHURES_FPS`), a web build check, art review round 7, and every "fini quand" checked. |

Every ticket also requires:
- fmt, clippy `-D warnings`, the tests and the structure test pass;
- the balance table is unchanged;
- before/after screenshots of 1-1 and the Grand Plan, plus `HACHURES_AT=t1,t2` shots for anything that moves.

## Testing Decisions

- The seams are pure functions: the parcels of a level (03), what goes where for tufts and flowers (04), and the parcels on the Grand Plan (05). Tests check behaviour, such as determinism, tiles kept, and markers left clear, the way `map/terrain.rs` and `overworld/terrain.rs` already do.
- Shader motion and colour are checked by timed screenshots, not unit tests.

## Out of Scope

New gameplay tiles or rules. Changing the balance. Replacing the Kenney tower models (`docs/3d-assets.md`). The interface frames named in the art direction's reviews.

## Further Notes

The direction still needs writing into `docs/direction-artistique.md` as "Phase 6: la maquette vivante", plus one clause in the bible's "Voisin à distinguer" paragraph (the engraving is *enluminée*). Both have the owner's approval.
