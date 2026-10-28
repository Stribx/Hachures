# 04: Every people dressed

**What to build:** Every species in `species.ron` gets its props and its people's coat, as on its card (DA phase 7): Jalonneur, Échassier, Fuyard, Gens du Col, Harde, Convoi.

**Blocked by:** 03.

**Status:** done

Tests first:
- [x] Every species in `species.ron` has a distinct silhouette signature (part count and bounding box).
- [x] Every coat contrasts in value (relative luminance) with pasture, meadow, wheat and snow (`theme` and `map::cell_color`) by a set margin.
- [x] No coat is in the seal's red family.

Build:
- [x] Props: the jalon, stilts and a straw hat, the bundle and the child, the hood, rope and alpenstock, the pack boxes, the canvas hood and the leader.
- [x] Coats retuned in `species.ron` to the palette per people.
- [x] Parade shot: each species is named from its silhouette (judged in colour; the greyscale check moves to 08 with the rest of the art review).
- [x] fmt, clippy `-D warnings`, `cargo test -p game` and the structure test are green.
- [x] The balance table is unchanged (`cargo test -p game balance -- --ignored`).
- [x] Before/after parade shots (`HACHURES_PARADE=1`) are shown in the ticket's comments.

## Comments

- `Carry` has eleven new props; `figure/props.rs` holds every prop and places it from the body's anchors (head, crown, chest). Red run: `no_coat_is_sealing_wax`, `props_of_the_people` and `every_coat_reads_on_the_fields` failed; `every_species_has_its_own_silhouette` already passed on the old part counts. The Échassier's burnt sienna from ticket 01 failed two tests (contrast 1.99 on the meadow, 0.19 from the seal), so it is now an olive reed-ochre, (0.46, 0.40, 0.14), and DA phase 7 says so. Lowest contrast per coat, all on the meadow: chaineur 3.1, jalonneur 4.5, fuyard 2.1, gens du Col 2.1, harde 2.6, convoi 2.1. The snow colour is `map::look(Tile::Glace)`.
- Shots: /tmp/claude-1000/-home-lair-Documents-perso-Hachures/b2d5565c-b945-4017-af1f-42e30343b0ac/scratchpad/shots/k04-left.png (the convoy's canvas hood and its leader, the Échassier's straw hat, the Fuyard's bundle and kerchief), /tmp/claude-1000/-home-lair-Documents-perso-Hachures/b2d5565c-b945-4017-af1f-42e30343b0ac/scratchpad/shots/k04-right.png (the Harde's pack boxes; the boss is still two boxes, for 07).
