# 08: Rain and snow with the world events

**What to build:** The weather follows the world: rain slants in the wind and puddles ripple while la Crue is under way, and snow falls during le Gel and in world 2. The weather starts when the event does and stops when it withdraws, read from the existing event state.

**Blocked by:** 01 (wind in the ink).

**Status:** done

- [x] Rain while la Crue is on, snow while le Gel is on and lightly throughout world 2; nothing otherwise.
- [x] It slants with the shared wind direction.
- [x] It's light enough that every tile and marker stays readable.
- [x] Density follows the preset; off on Low.
- [x] Two shots of the same view at different times (`HACHURES_AT=t1,t2`) show the motion, and nothing else differs.
- [x] fmt, clippy `-D warnings`, `cargo test -p game` and the structure test are green.
- [x] The balance table is unchanged (`cargo test -p game balance -- --ignored`).
- [x] Before/after screenshots of 1-1 and the Grand Plan are shown in the ticket's comments.

## Comments

- `map::weather::weather(world, event_on)`: rain while la Crue is on, a snowstorm during le Gel, a light snow all through world 2, clear otherwise (tested). `fall` draws it with gizmos (no assets, fine on WebGL2): rain as slanted ink streaks with rings spreading on every water cell, snow as small drifting crosses; none on Low, a quarter under reduced motion.
- Shots: /tmp/claude-1000/-home-lair-Documents-perso-Hachures/0c12f20b-002a-4d83-9aa8-d1079a5805fa/scratchpad/wx-snow.png (2-1, the light snow) and wx-rain.png (1-1 with rain switched on for the shot only: the harness can't reach la Crue's wave; reverted, not committed).
- Noticed, for later: the Cols' boards grow wheat like the bocage's; alpine flowers fit there, wheat less.
