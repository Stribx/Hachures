# 02: Éboulis, a slow tile Gratter returns to Sol

**What to build:** Level designers can draw an Éboulis (`e`) in a level map or lay one with a scripted change. Every species walks it, at the Éboulis cost in the spec. Nève can't ink on it. Gratter turns it into plain Sol for good: the drawn map that the world event reads becomes Sol too, so la Crue never floods a scraped cell. The pencil line before a scrape shows the shown species' route once the cell is Sol. The Calque names the tile, the board shows a scatter of stone blocks, and walking on it has its own footstep. See the spec: user stories 1, 4–15, 17–21, 23–24, 26–28 for Éboulis.

**Blocked by:** 01 (Gratter asks the grid what can be scraped).

**Status:** ready-for-agent

- [x] `e` parses to Éboulis; a parse test covers it.
- [x] Every species has an Éboulis cost: Chaîneur 3, Jalonneur 3, Échassier 3, Fuyard 3, Gens du Col 2, Harde 1, Convoi 4, Grande Chaîne 3, Chaîne rompue 2.
- [x] Éboulis is not buildable: inking a tile or a tower on it is refused.
- [x] Gratter on an Éboulis leaves Sol. A second scrape there is refused with "nothing to scrape".
- [x] A scrape costs the flat Gratter price. (Grattoir fin: dropped, see Notes.)
- [x] Scraping on a named place keeps every Name.
- [x] Scraping works under a walker and on a cell a scripted change will hit later.
- [x] A scraped cell that was drawn as low ground stays Sol under la Crue.
- [x] The preview's field equals the field after the scrape.
- [x] New data test: every species prices Sol.
- [x] The comments that say scraping "only opens the map" or shows "the route it opens" now say it never closes a path (every species walks Sol).
- [x] `tile.eboulis` is in every i18n file, and `cargo test -p game i18n` is green.
- [x] A stone-block prop is built in code next to the other tile props. (Footstep sound: dropped, see Notes.)
- [x] Screenshot harness on a scratch map holding Éboulis: the tile reads as stone at play distance. Shot: /tmp/claude-1000/-home-lair-Documents-perso-Hachures/8eb342b6-f526-492c-9f5c-0ebcbd57a420/scratchpad/eboulis.png (0-0 map filled with `e`, restored after).
- [x] `cargo test -p game`, clippy `-D warnings` and the structure test are green. The balance bot table is unchanged.

## Notes

- **Grattoir fin dropped:** that node is tier 4 of l'arbre des sceaux, and only tiers 1–3 exist in the code. The price stays flat (25). The halving belongs to the ticket that adds tier 4.
- **Footstep sound dropped:** the only per-tile sound (`landing_sound`) plays when Nève inks a piece and it lands. Nobody inks an Éboulis, and walkers have no per-tile footsteps.
- Tests through `Level`: `gratter_scrapes_an_eboulis`, `gratter_refuses_what_is_not_decor`, `an_eboulis_is_not_buildable`, `scraping_an_eboulis_keeps_its_name`, `gratter_ignores_walkers_and_changes`, `gratter_preview_takes_the_eboulis_off`, `la_crue_spares_a_scraped_eboulis`. Data: `an_eboulis_is_parsed_as_drawn`, `every_species_walks_plain_sol`.
