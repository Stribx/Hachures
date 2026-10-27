# 02: The enluminure palette

**What to build:** The engraving is hand-coloured: watercolour washes under the ink, like a Cassini map. The land keeps much more of its own colour (the fade toward the paper is set per material: light on the land, kept on the ink lines), the palette gains the colours of a bocage (pasture, meadow, wheat, fallow, flax, hedge, sky), and the fog and the corner vignette warm up instead of greying. Each tile keeps its colour family, and Sol and Sol bas get theirs: dry pasture and wet meadow.

**Blocked by:** None (can start immediately).

**Status:** ready-for-agent

- [ ] The fade toward the paper is a per-material setting; the land and the tiles use a lighter one, the ink lines and the UI paper stay as they are.
- [ ] The bocage colours live in the shared palette, each with a one-line comment; no colour literal is repeated.
- [ ] Sol reads as dry pasture, Sol bas as wet meadow, and at play distance a player still tells Sol, Sol bas, Boue, Ronces, Eau, Gué, Roc, Chaussée and every marker apart (check each on a screenshot).
- [ ] The Grand Plan's land is greener and warmer, with no hard colour cut (its existing test still passes).
- [ ] The fog and the vignette tint warm, not grey.
- [ ] A taste-skill review round on the before/after shots is appended to `docs/direction-artistique.md`.
- [ ] fmt, clippy `-D warnings`, `cargo test -p game` and the structure test are green.
- [ ] The balance table is unchanged (`cargo test -p game balance -- --ignored`).
- [ ] Before/after screenshots of 1-1 and the Grand Plan are shown in the ticket's comments.
