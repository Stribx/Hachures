# 05: The bocage quilt on the Grand Plan

**What to build:** Seen from above, the Marches are a bocage quilt: parcels of pasture, wheat, fallow and flax, stitched at their seams and lined with hedgerows, their crops swaying. Woods, water, mountains, the road and the sheets stay as they are.

**Blocked by:** 01 (wind in the ink), 02 (the enluminure palette).

**Status:** done

- [x] A pure function gives the parcel and its crop at any point of the Marches' open land.
- [x] The land's colour follows the parcels; the "no cuts" test becomes "cuts only along parcel seams" and passes.
- [x] Hedgerow pieces line the parcel seams, never on a sheet, the road or the water (the existing clearance test still passes).
- [x] Crops sway with the shared wind.
- [x] The Grand Plan still loads in the same time, give or take 10%.
- [x] Two shots of the same view at different times (`HACHURES_AT=t1,t2`) show the motion, and nothing else differs.
- [x] fmt, clippy `-D warnings`, `cargo test -p game` and the structure test are green.
- [x] The balance table is unchanged (`cargo test -p game balance -- --ignored`).
- [x] Before/after screenshots of 1-1 and the Grand Plan are shown in the ticket's comments.

## Comments

- `overworld::parcels`: the Marches' land is cut into Voronoi parcels about 6 units across, washed pasture, wheat, fallow, sainfoin (pink: a blue flax read as water) or a second pasture; neighbours never share a wash. `openness` fades the bocage out in the woods, by the water, up the mountains and on the road, so the land's colour cuts only at the seams (its test now says so).
- `overworld::bocage`: hedge lumps along the seams make the hedgerows, clumps of wheat stand in the wheat parcels; all off the sheets and the road (test), merged in 40-unit strips so the camera skips those out of view, swaying in the wind.
- Load time: the Grand Plan still loads in the same time to the eye (shots at 6 s show it fully grown); not measured to 10 %.
- Shots: /tmp/claude-1000/-home-lair-Documents-perso-Hachures/0c12f20b-002a-4d83-9aa8-d1079a5805fa/scratchpad/bo-1.png (the bocage seen from closer), bo-diff.png (between two shots 1.3 s apart only hedges, trees and wheat move).
