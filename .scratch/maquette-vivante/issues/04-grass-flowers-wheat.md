# 04: Grass, flowers and wheat that sway

**What to build:** The fields grow: felt tufts on pasture, wildflower dots in the meadows, rows of wheat on the crop fields, each kind chosen by its field and scattered by the cell, the same on every load. They sway in the wind and gusts roll across them. They stay low, so a tile, a marker, the pencil line or an inked piece is never hidden, and an inked cell loses its grass.

**Blocked by:** 01 (wind in the ink), 03 (fields on the board).

**Status:** ready-for-agent

- [ ] A pure function says what grows on each cell (kind and spots) from the fields and the grid; nothing grows on markers, on inked or non-open tiles, or under a tower.
- [ ] Tests cover that function: deterministic, never on a marker, never on an inked cell.
- [ ] The growth is a few shared meshes and one material, so drawing cost stays within the budget.
- [ ] Density follows the graphics preset: Low is sparse, High is full.
- [ ] It sways with the shared wind; gusts are visible crossing a field.
- [ ] `HACHURES_FPS=1` at High: no more than about 2 ms more GPU time than before this feature.
- [ ] Two shots of the same view at different times (`HACHURES_AT=t1,t2`) show the motion, and nothing else differs.
- [ ] fmt, clippy `-D warnings`, `cargo test -p game` and the structure test are green.
- [ ] The balance table is unchanged (`cargo test -p game balance -- --ignored`).
- [ ] Before/after screenshots of 1-1 and the Grand Plan are shown in the ticket's comments.
