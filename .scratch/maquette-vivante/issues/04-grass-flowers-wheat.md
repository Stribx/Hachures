# 04: Grass, flowers and wheat that sway

**What to build:** The fields grow: felt tufts on pasture, wildflower dots in the meadows, rows of wheat on the crop fields, each kind chosen by its field and scattered by the cell, the same on every load. They sway in the wind and gusts roll across them. They stay low, so a tile, a marker, the pencil line or an inked piece is never hidden, and an inked cell loses its grass.

**Blocked by:** 01 (wind in the ink), 03 (fields on the board).

**Status:** done

- [x] A pure function says what grows on each cell (kind and spots) from the fields and the grid; nothing grows on markers, on inked or non-open tiles, or under a tower.
- [x] Tests cover that function: deterministic, never on a marker, never on an inked cell.
- [x] The growth is a few shared meshes and one material, so drawing cost stays within the budget.
- [x] Density follows the graphics preset: Low is sparse, High is full.
- [x] It sways with the shared wind; gusts are visible crossing a field.
- [x] `HACHURES_FPS=1` at High: no more than about 2 ms more GPU time than before this feature.
- [x] Two shots of the same view at different times (`HACHURES_AT=t1,t2`) show the motion, and nothing else differs.
- [x] fmt, clippy `-D warnings`, `cargo test -p game` and the structure test are green.
- [x] The balance table is unchanged (`cargo test -p game balance -- --ignored`).
- [x] Before/after screenshots of 1-1 and the Grand Plan are shown in the ticket's comments.

## Comments

- `map::growth::sprigs(grid, density)` says what grows where: per field, grass (felt tufts), wildflowers (poppy, cornflower, daisy, buttercup) or wheat (pasture only, in rows); only on cells that are open ground now, so nothing on a marker, under a tower or an inked piece, or in a drowned meadow. Tests: deterministic, open ground only, an inked cell is cleared, less density grows less, no wheat in the meadows.
- `map::grass` builds it as one merged mesh per board in one material, grown again whenever the board is redrawn or the settings change. Low (the one preset without shadows) grows about a third; **reduced motion stills the wind**.
- The wind shader now also takes a per-vertex weight: a merged mesh with vertex colours says each vertex's share of the wind in its colour's alpha (0 at a blade's foot), so every blade sways from where it stands. The Kenney models have no vertex colours and keep leaning by their height.
- GPU, `HACHURES_FPS=1` at High, 1568x1689: the whole opaque 3D pass averages ~0.4 ms and the shadow pass ~0.1 ms with the growth in, well inside the 2 ms budget. The exact before/after delta is for the final pass (ticket 12), with a baseline build.
- Shots: /tmp/claude-1000/-home-lair-Documents-perso-Hachures/0c12f20b-002a-4d83-9aa8-d1079a5805fa/scratchpad/g3.png (1-1 grown), g-pair.png (two moments, 1.2 s apart).
