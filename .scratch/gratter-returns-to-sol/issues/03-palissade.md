# 03: Palissade, an imperial barricade Gratter returns to Sol

**What to build:** Level designers can draw a Palissade (`p`) in a level map or lay one with a scripted change. No species can cross it; the levels test makes sure no palissade, standing or laid later, ever leaves a species without a path, with or without the world event. Nève can't ink on it. La Crue spares it the way it spares a Barricade. Gratter turns it into plain Sol, like an Éboulis. The Calque names the tile, and the board shows a row of sharpened stakes in imperial colours that reads differently from Nève's barricade. See the spec: user stories 2, 16–20, 22–25, 28.

**Blocked by:** 02 (Éboulis, a slow tile Gratter returns to Sol): it brings the "scrape a tile to Sol" act.

**Status:** ready-for-agent

- [x] `p` parses to Palissade; a parse test covers it.
- [x] No species prices Palissade, and a data test checks it.
- [x] Palissade is not buildable: inking a tile or a tower on it is refused.
- [x] Gratter on a Palissade leaves Sol. A second scrape there is refused with "nothing to scrape".
- [x] World 1's Crue lists Palissade in its spares. A Palissade on low ground stays a Palissade under la Crue (a `Level` test).
- [x] The existing levels-are-playable test still passes and needs no change. No level uses the tile yet.
- [x] `tile.palissade` is in every i18n file, and `cargo test -p game i18n` is green.
- [x] A stake-row prop is built in code next to the other tile props, distinct from the barricade.
- [x] Screenshot harness on a scratch map holding a Palissade next to a Barricade: the two read differently at play distance. Attach the shot path.
- [x] `cargo test -p game`, clippy `-D warnings` and the structure test are green. The balance bot table is unchanged.

## Notes

- Tests through `Level`: `gratter_scrapes_a_palissade`, `a_palissade_is_not_buildable`, `a_palissade_is_impassable`, `la_crue_spares_a_palissade`. Data: `a_palissade_is_parsed_as_drawn`, `no_species_walks_a_palissade`.
- Shot: /tmp/claude-1000/-home-lair-Documents-perso-Hachures/8eb342b6-f526-492c-9f5c-0ebcbd57a420/scratchpad/palissade.png (0-0's two open rows replaced by a horizontal run, a vertical pair and two barricades, then restored).
- Polish left: in a horizontal run the rails stop 0.05 short of the cell edge, so neighbouring palissades show a thin gap.
