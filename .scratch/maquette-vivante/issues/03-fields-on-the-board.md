# 03: Fields on the board

**What to build:** Each board becomes a little quilt: its open ground (Sol and Sol bas) is cut into a few fields, each with its own patch colour from its tile's family (a pasture, a fallow, a meadow…), and a felt stitch runs along the seam where two fields meet. The layout comes from the level itself, the same every time it loads, with nothing to write in the level files.

**Blocked by:** 02 (the enluminure palette).

**Status:** done

- [x] A pure function gives the fields of a board from its grid and a seed: the same input always gives the same fields.
- [x] Fields cover Sol and Sol bas only; no other tile and no marker cell (spawns, goals, Refuge, Sortie) belongs to one.
- [x] A field never mixes Sol and Sol bas, so the colour families hold.
- [x] Fields are a few cells to a dozen, not single cells, and neighbouring fields differ in colour.
- [x] Tests cover all four rules above on small parsed maps.
- [x] On the board, each field shows its patch colour and a thin stitch line follows every seam between two fields; inked pieces, la Crue and le Gel still redraw correctly over them.
- [x] fmt, clippy `-D warnings`, `cargo test -p game` and the structure test are green.
- [x] The balance table is unchanged (`cargo test -p game balance -- --ignored`).
- [x] Before/after screenshots of 1-1 and the Grand Plan are shown in the ticket's comments.

## Comments

- `map::fields`: `fields(grid)` cuts the drawn open ground (Sol, Sol bas, and the pasture under drawn brambles) into fields of 3 to 12 cells, from a hash of the map itself, so inking never reshuffles them; `ground(grid)` gives each cell its field's wash (4 pastures, 3 meadows, neighbours always different) and is used by the board and by the sheet miniatures on the Grand Plan alike. Under la Crue a drowned meadow shows the water's colour.
- `map::seams`: a cream running stitch along every seam between two fields of the same open ground, one merged mesh per board.
- Tests: the same map gives the same fields; fields lie on open ground only and never mix pasture and meadow; 3 to 12 cells, mostly no crumbs; neighbours differ; washes stay in their family; each seam parts two fields.
- Shots: /tmp/claude-1000/-home-lair-Documents-perso-Hachures/0c12f20b-002a-4d83-9aa8-d1079a5805fa/scratchpad/fields2-playing.png (1-1 with the demo pieces) and fields2-plan.png.
