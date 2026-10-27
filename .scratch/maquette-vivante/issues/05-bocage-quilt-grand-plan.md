# 05: The bocage quilt on the Grand Plan

**What to build:** Seen from above, the Marches are a bocage quilt: parcels of pasture, wheat, fallow and flax, stitched at their seams and lined with hedgerows, their crops swaying. Woods, water, mountains, the road and the sheets stay as they are.

**Blocked by:** 01 (wind in the ink), 02 (the enluminure palette).

**Status:** ready-for-agent

- [ ] A pure function gives the parcel and its crop at any point of the Marches' open land.
- [ ] The land's colour follows the parcels; the "no cuts" test becomes "cuts only along parcel seams" and passes.
- [ ] Hedgerow pieces line the parcel seams, never on a sheet, the road or the water (the existing clearance test still passes).
- [ ] Crops sway with the shared wind.
- [ ] The Grand Plan still loads in the same time, give or take 10%.
- [ ] Two shots of the same view at different times (`HACHURES_AT=t1,t2`) show the motion, and nothing else differs.
- [ ] fmt, clippy `-D warnings`, `cargo test -p game` and the structure test are green.
- [ ] The balance table is unchanged (`cargo test -p game balance -- --ignored`).
- [ ] Before/after screenshots of 1-1 and the Grand Plan are shown in the ticket's comments.
