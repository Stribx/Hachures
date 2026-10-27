# 02: The enluminure palette

**What to build:** The engraving is hand-coloured: watercolour washes under the ink, like a Cassini map. The land keeps much more of its own colour (the fade toward the paper is set per material: light on the land, kept on the ink lines), the palette gains the colours of a bocage (pasture, meadow, wheat, fallow, flax, hedge, sky), and the fog and the corner vignette warm up instead of greying. Each tile keeps its colour family, and Sol and Sol bas get theirs: dry pasture and wet meadow.

**Blocked by:** None (can start immediately).

**Status:** done

- [x] The fade toward the paper is a per-material setting; the land and the tiles use a lighter one, the ink lines and the UI paper stay as they are.
- [x] The bocage colours live in the shared palette, each with a one-line comment; no colour literal is repeated.
- [x] Sol reads as dry pasture, Sol bas as wet meadow, and at play distance a player still tells Sol, Sol bas, Boue, Ronces, Eau, Gué, Roc, Chaussée and every marker apart (check each on a screenshot).
- [x] The Grand Plan's land is greener and warmer, with no hard colour cut (its existing test still passes).
- [x] The fog and the vignette tint warm, not grey.
- [x] A taste-skill review round on the before/after shots is appended to `docs/direction-artistique.md`.
- [x] fmt, clippy `-D warnings`, `cargo test -p game` and the structure test are green.
- [x] The balance table is unchanged (`cargo test -p game balance -- --ignored`).
- [x] Before/after screenshots of 1-1 and the Grand Plan are shown in the ticket's comments.

## Comments

- The fade toward the paper went from 0.35 to 0.15 (0.45 to 0.25 for the fine strokes of small pieces): the main desaturator. Sol is pasture green, Sol bas a bluer wet meadow, the water a bluer resin, the Roc ridges warm limestone, the vignette warm ochre. Fallow and flax wait for tickets 03 and 05, which will use them.
- Shots: before, /tmp/claude-1000/-home-lair-Documents-perso-Hachures/0c12f20b-002a-4d83-9aa8-d1079a5805fa/scratchpad/before-playing.png and before-plan.png; after, after2-playing.png (1-1 with the demo pieces) and after-plan.png.
