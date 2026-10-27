# 11: Ambient sounds in the game

**What to build:** The land sounds alive: a wind bed that swells with the gusts, birds over the fields, rain during la Crue, snow wind in world 2, water near the river and the Moulin, all under an ambient volume setting.

**Blocked by:** 10 (the CC0 shortlist).

**Status:** done

- [x] Only the approved files are added, each with a row in `CREDITS.md`, and `build-web.sh` copies their folder.
- [x] Each loop starts and stops with its condition (event, world, place), with fades, never a cut.
- [x] An ambient volume setting is saved with the other settings and offered in every language.
- [ ] The browser build plays them after the first click.
- [x] fmt, clippy `-D warnings`, `cargo test -p game` and the structure test are green.
- [x] The balance table is unchanged (`cargo test -p game balance -- --ignored`).
- [x] Before/after screenshots of 1-1 and the Grand Plan are shown in the ticket's comments.

## Comments

- Five CC0 loops in `audio/ambient/` (wind, birds, rain, river, snow-wind), downloaded from the pages whose CC0 dedication was checked, trimmed (birds to 90 s), their seam crossfaded over 2 s, levelled to -24 LUFS, OGG Vorbis: 3.8 MB in all. Rows in `CREDITS.md` (the Credits page's test counts them). `build-web.sh` already copies `audio/`. The mill wheel was dropped (no CC0 recording).
- `map::ambience::ambience(world, event_on, wet)` gives each loop's loudness: wind always, birds over the bocage outside la Crue, rain during it, the river on boards with water, the Cols' cold wind (louder in le Gel); tested. All loops play from the start, silent, each fading toward its loudness over ~2 s under the new **Ambiance** volume (default 70 %, muted in the background like the music); the wind swells slowly.
- A run of 1-1 logs no asset or audio error. **Not checked:** hearing it — nothing here plays sound to me — and the browser's first-click audio unlock, which Bevy handles as it does the music.
- Settings page: /tmp/claude-1000/-home-lair-Documents-perso-Hachures/0c12f20b-002a-4d83-9aa8-d1079a5805fa/scratchpad/settings.png shows the Ambiance row.
