# 03: Fields on the board

**What to build:** Each board becomes a little quilt: its open ground (Sol and Sol bas) is cut into a few fields, each with its own patch colour from its tile's family (a pasture, a fallow, a meadow…), and a felt stitch runs along the seam where two fields meet. The layout comes from the level itself, the same every time it loads, with nothing to write in the level files.

**Blocked by:** 02 (the enluminure palette).

**Status:** ready-for-agent

- [ ] A pure function gives the fields of a board from its grid and a seed: the same input always gives the same fields.
- [ ] Fields cover Sol and Sol bas only; no other tile and no marker cell (spawns, goals, Refuge, Sortie) belongs to one.
- [ ] A field never mixes Sol and Sol bas, so the colour families hold.
- [ ] Fields are a few cells to a dozen, not single cells, and neighbouring fields differ in colour.
- [ ] Tests cover all four rules above on small parsed maps.
- [ ] On the board, each field shows its patch colour and a thin stitch line follows every seam between two fields; inked pieces, la Crue and le Gel still redraw correctly over them.
- [ ] fmt, clippy `-D warnings`, `cargo test -p game` and the structure test are green.
- [ ] The balance table is unchanged (`cargo test -p game balance -- --ignored`).
- [ ] Before/after screenshots of 1-1 and the Grand Plan are shown in the ticket's comments.
