# Gratter returns a cell to Sol

Status: ready-for-agent

## Problem Statement

Gratter, the power Nève earns by beating la Grande Chaîne, can only lift an imperial borne. The bible (sections 7 and 9) and the Glossary say Gratter returns a cell to plain Sol, and that it scrapes three things: a borne, an Éboulis, a Palissade. Neither Éboulis nor Palissade exists in the game, so world 1's locked sheet (the Moulin noyé, under an éboulis de meules), world 3 (the Foreuse lays Éboulis) and any level where the Empire lays a palissade can't be built.

## Solution

Two new tiles, Éboulis and Palissade, that level designers can draw on a map or lay with a scripted change. Gratter on either turns the cell into plain Sol for good, and the world event then treats it as Sol too. Gratter on a borne works as before. Gratter can only be refused when there is nothing to scrape or not enough sépia. It never costs a Name, and no level ever needs it to be won.

## User Stories

1. As Nève, I want to scrape an Éboulis back to Sol, so that a slow stretch of ground becomes plain again.
2. As Nève, I want to scrape a Palissade back to Sol, so that a road the Empire closed opens again.
3. As Nève, I want to keep scraping an imperial borne as before, so that the Jalonneurs lose a source.
4. As Nève, I want Gratter refused with "rien à gratter ici" on any cell that holds no borne, Éboulis or Palissade, so that I don't spend sépia for nothing.
5. As Nève, I want Gratter refused on my own barricades, mud and brambles, so that it is never an undo.
6. As Nève, I want Gratter refused on Chaussée, Glace, Eau, Gouffre and Roc, so that it only takes off what the bible names.
7. As Nève, I want Gratter refused when I lack the sépia, like every tool.
8. As Nève, I want a scrape to cost the same flat price whatever the target, so that the price is easy to read.
9. As Nève with Grattoir fin, I want every scrape at half price, whatever the target.
10. As Nève, I want a scrape on a named place to cost no Name, so that Gratter never rayes anything from the Rôle.
11. As Nève, I want to scrape a cell even while a column walks on it, so that I can open a road under pressure.
12. As Nève, I want to scrape a cell a scripted change will hit later, knowing the change will overwrite it.
13. As Nève, I want the pencil line before a scrape to show the route the shown species would take once the cell is Sol, so that I can see whether scraping opens a road or lengthens one (a Taupier, later, prefers Éboulis).
14. As Nève, I want a scraped cell to stay Sol: nothing un-scrapes it by itself.
15. As Nève, I want la Crue never to flood a cell I scraped, because Gratter leaves plain Sol with no memory of the ground underneath.
16. As Nève, I want la Crue to spare a Palissade the way it spares a Barricade, so that an imperial wall doesn't melt into water for a wave.
17. As Nève, I want the Calque to name Éboulis and Palissade, so that I can read every cell.
18. As Nève, I want an Éboulis and a Palissade to look like what they are on the board, so that I can read the map without the Calque.
19. As Nève, I want to be unable to ink a tower or a tile on an Éboulis or a Palissade, so that I scrape first and then build.
20. As a player chasing "finir sans Gratter", I want every level to be winnable without scraping anything.
21. As every land species, I want Éboulis to be walkable at a cost, so that it slows me down without walling me in.
22. As every land species, I want a Palissade to be impassable, like a Barricade.
23. As a level designer, I want to draw an Éboulis or a Palissade straight into a level map with its own character.
24. As a level designer, I want to lay an Éboulis or a Palissade with a scripted change, at a given wave.
25. As a level designer, I want the levels test to fail if a Palissade or an Éboulis, standing or laid later, leaves any species without a path, with and without the world event.
26. As a level designer, I want a test that fails if a species can't walk on Sol, because Gratter skips the Droit de Passage on that promise.
27. As la Grande Chaîne, I want to go wherever a Chaîneur can, Éboulis included, so that the boss never makes the Droit de Passage stricter.
28. As a translator, I want the two tile names as i18n keys in every language, so that no tile is shown without a name.

## Implementation Decisions

- **Tiles.** `Tile` gains `Eboulis` and `Palissade`. Neither is buildable: Nève inks only on open, dry ground, as today.
- **Map characters.** `e` is Éboulis and `p` is Palissade in `Grid::parse`. Both are free today; markers stay uppercase.
- **What can be scraped.** A cell can be scraped when it holds a borne, or when its tile is Éboulis or Palissade. Put this in one place, on the grid (a query that names the scrapable tiles), and make the "nothing to scrape" refusal and the act itself both use it.
- **What a scrape does.** `Act::Scrape` keeps its name. A scrape lifts the borne if there is one; otherwise it sets the cell to Sol in both the live tiles and the drawn map that the world event reads. Writing the drawn map is what gives the scrape "no memory": the event no longer sees the old ground. `Act::lays()` stays `None` for a scrape, so the prix du nom (which keys on a laid tile) still never applies.
- **Refusals for Gratter:** only "nothing to scrape" and "not enough sépia", as today. No column check, no scripted-change check, no Droit de Passage. Fix the comment that says scraping "only opens the map": it can lengthen a route, but it can never close one, because every species prices Sol.
- **The preview** (`what_if`) runs the same scrape on a copy, so the pencil line shows the route on plain Sol. Fix the ghost comment about "the route it opens" in the same way.
- **Price.** The Gratter tool keeps its single flat cost (25). Grattoir fin already halves a tool's cost and applies unchanged.
- **The world event.** World 1's Crue lists Palissade in `spares`, next to Barricade and Tour. World 2's Gel can't hit a Palissade (it hits Eau), so it needs nothing. An Éboulis drawn on the map isn't a tile any event hits.
- **Species costs** (a tile left out of `costs` is impassable). Éboulis: Chaîneur 3, Jalonneur 3, Échassier 3, Fuyard 3, Gens du Col 2, Harde 1 (an Ombre reader: 1 + towers), Convoi 4, Grande Chaîne 3, Chaîne rompue 2. Palissade appears in no species' costs. These values go into the bible's section 8 too.
- **i18n.** New keys `tile.eboulis` ("éboulis" / "rubble") and `tile.palissade` ("palissade" / "palisade") in every language file.
- **Look.** Both are built in code as tile props, like the barricade, mud and brambles (docs/3d-assets.md: tile-scale, repeated pieces are code, not assets). An Éboulis is a scatter of stone blocks; a Palissade is a row of sharpened stakes, in imperial colours, distinct from Nève's barricade. Add a footstep sound for Éboulis wherever the land picks a sound per tile.
- **Layers.** Everything stays in `map` (tiles, parse, props, the scrapable query) and `ink` (act, refusal, preview). `level` only reads the new tiles through the data, and no new message is needed.

## Testing Decisions

- A good test drives the public behaviour of `Level` and checks the outcome a player would see: is it refused and why, what the cell is now, what it cost, how many Names are left, which route is shown. It doesn't check the private helpers.
- **The seam is `Level`:** build one from `Grid::parse` with `species::load()`, then call `refusal`, `ink` and `what_if`. Prior art: the Gratter tests in the Droit de Passage module (`gratter_scrapes_only_a_borne`, `gratter_preview_takes_the_borne_off`) and la Crue's test in the ink module. Cases:
  - Éboulis and Palissade become Sol, and a second scrape is refused with "nothing to scrape".
  - A borne is still lifted.
  - Barricade, Boue, Ronces, Chaussée, Glace, Eau and Roc are refused.
  - The sépia goes down by the flat cost, halved with Grattoir fin.
  - Scraping a named place keeps every Name and adds no erased place.
  - Scraping works under a walker and on a cell a scripted change will hit.
  - The preview's field matches the field after the scrape.
  - After scraping a cell drawn as low ground, the map under la Crue keeps it Sol.
  - A Palissade on low ground stays a Palissade under la Crue.
- **Data checks:**
  - The existing `levels_are_playable` test already covers the base map, the map after changes, and both under the event, so palissades and éboulis in level data are checked as they appear.
  - New test next to the species loader: every species prices Sol, and no species prices Palissade.
  - Existing parse tests: add the two characters.
- **i18n:** `cargo test -p game i18n` catches a missing tile key.
- **The look is not unit-tested.** Check it with the screenshot harness on a scratch level map that holds both tiles.

## Out of Scope

- The locked sheets (the Moulin noyé, the Saline engloutie): how they open on the Grand Plan, and what is inside.
- Any level or world data that uses the new tiles. No current level gets an Éboulis or a Palissade in this change, apart from a test map.
- A boss phase that lays Éboulis or Palissades (the Foreuse, world 3) and the Étayeur's repairs.
- Taupier, Étayeur and the other later species, with their Éboulis prices.
- Per-target prices, and refunds.
- Balance tuning beyond the default costs. Run the balance bot and diff its table, which should be unchanged, since no level uses the tiles yet.

## Further Notes

- Decisions come from the Gratter grilling session; the Glossary (CLAUDE.md) and bible sections 7 to 9 hold the rules. Where code and bible disagree, the bible wins.
- The Taupier (world 3) will price Sol 2 and Éboulis 1, so scraping lengthens its route. That is why the preview is phrased as "the route once the cell is Sol", not "the route it opens".
- Keep each file under 200 lines of code and each function under 70 lines. The tile props probably belong in their own small module, as the Gratter's piece does.
