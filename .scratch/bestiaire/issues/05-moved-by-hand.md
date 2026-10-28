# 05: Moved by hand

**What to build:** The figurines walk. The step advances with the distance walked: `Pas` (hop and waddle), `Echasses` (slow tall sway), `Trot` (nod and pitch), `Roule` (wheels turn, the box rocks on rough tiles). They turn toward the next cell smoothly, and stand still while paused or pinned.

**Blocked by:** 03.

**Status:** done

Tests first:
- [x] `gait::pose`: never below the ground, periodic in phase, identity when not moving; stilts sway taller and slower than `Pas`.
- [x] ECS (`combat/tests.rs` style): a walker's stride phase grows with distance, and does not grow while pinned.
- [x] ECS: yaw eases toward the heading and never overshoots it.

Build:
- [x] `combat/figure/gait.rs`: `Gait` (from `Shape`), `pose`, the `Stride` component, the `animate` system.
- [x] `enemies.rs` `step` no longer writes the rotation; `animate` does.
- [x] Two shots of one wave (`HACHURES_WAVE=1 HACHURES_AT=t1,t2`) show steps, not slides.
- [x] fmt, clippy `-D warnings`, `cargo test -p game` and the structure test are green.
- [x] The balance table is unchanged (`cargo test -p game balance -- --ignored`).
- [x] Before/after parade shots (`HACHURES_PARADE=1`) are shown in the ticket's comments.

## Comments

- `figure/gait.rs`: `Gait` (from `Shape`), `cadence`, `pose`, `limb`, `Stride` (on walkers only, so the parade stands still), `Limb` (wheels and legs), `animate` (after `walk`, in `Step::Simulate`, so it stops when paused). `Body` keeps its rest transform. `step` no longer snaps the rotation; `animate` eases the yaw at 12/s. Review fix: `walked` is in world units, so the wheels now roll by `walked / scale` (they were turning 1.3 times too fast). Red run: `stilts_stride_slower_than_feet`, `legs_alternate` and `a_stride_follows_the_ground_walked` failed. The identity-pose tests and `turns_ease_without_overshoot` passed on the stubs, since `step` still snapped the rotation then.
- Harness: `HACHURES_WAVE=1` now works without `HACHURES_DEMO`; with the demo, its towers killed the first walkers at the spawn before a shot could catch them.
- Shots, 0.2 s apart: /tmp/claude-1000/-home-lair-Documents-perso-Hachures/b2d5565c-b945-4017-af1f-42e30343b0ac/scratchpad/shots/k05-wave-1.png and /tmp/claude-1000/-home-lair-Documents-perso-Hachures/b2d5565c-b945-4017-af1f-42e30343b0ac/scratchpad/shots/k05-wave-2.png (`HACHURES_WAVE=1 HACHURES_LEVEL=1 HACHURES_AT=5,5.2 HACHURES_CAM=2.5,3.2,5.2,2.5,0.3,1.3`). A file of Chaîneurs walks, each at its own height in the hop.
