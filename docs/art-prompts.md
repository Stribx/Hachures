# Hachures: image prompts (MOSS-Image)

Every image the game loads from `game/assets/ui/`, in the order the player sees them most. Until a file exists, the game draws a plain fallback, so add them in any order.

## Art direction: illuminated editorial cartography

An illuminated engraving rather than a photorealistic old document: think of a modern Swiss topographic map
combined with hand-applied watercolor washes, museum wayfinding signs, and a restrained risograph poster. The 3D
world already looks like this (flat pastel planes, procedural hachures in the shadows); the interface has to
speak the same language.

- **Flat pictograms in two tones:** dark sepia ink on warm cream paper. One bold geometric silhouette, one even line weight.
- **Hachures are a print pattern,** not shading: crisp parallel bands filling a shape, like a screenprint.
- **One accent:** seal red, as a flat block, kept for the main action (the game draws it in code).
- **Frames, buttons and panels are drawn in code:** thin rules and corner marks like the neatline of a survey sheet. No image needed for them.
- **What to avoid:** fake aged-paper texture, scrolls, filigree, bevels, metal, realistic rendering, and the
  "fantasy RPG icon" look. Watercolor washes are allowed as restrained map-color accents, not as soft painterly
  rendering.

## Workflow settings (once)

| Node | Setting |
|---|---|
| Text to Image (MOSS-Image) | `enable_prompt_enhancer` true, `prompt_enhancer_seed` 0 (keep it fixed), steps 25, cfg 2.5, er_sde, simple |
| `seed` | change it to get another variant of the same prompt |
| `ref_image (for llm)` | plug in an image you already approved from the same family. The enhancer reads it and keeps the family consistent. Leave it empty for the very first one. |
| Sélecteur de résolution | ratio given on each card, mégapixels 1.0, multiple 16 |
| Remove background | on for every card below |

**Negative prompt** (the same for every image; paste it once into the second box):

```
aged paper, parchment, stains, ornate, filigree, scrollwork, flourish, bevel, emboss, metallic, chrome, glossy, 3d render, cgi, realistic, photograph, soft painterly, smooth gradient, airbrush, fantasy rpg icon, skeuomorphic, rainbow, grey, text, letters, words, numbers, writing, signature, watermark, logo, frame, border, checkerboard, drop shadow, blurry, lowres, worst quality, jpeg artifact, nsfw
```

**Fitting.** MOSS can't output exact sizes, so each card has a `magick` command that trims, scales and centres the image to the size the game expects. Save the MOSS output as `raw.png`, then run the command from the repository root. Or drop the raw file anywhere and ask me to fit it.

---

## 1. Tool icons: made in the game
No longer drawn: each tool's piece is filmed in a little studio off the board and the bar shows the picture (`game/src/ui/studio/`), so an icon is always its piece. See `docs/direction-artistique.md`, phase 3.

## 2. Counter icons: `game/assets/ui/icon-<name>.png`
Done: drawn as vector pictograms (sources `icon-<name>.svg` in `../Hachures-asset-library/ui/`) and rasterised with `magick -background none -density 384 icon-<name>.svg -resize 60x60 -gravity center -extent 64x64 icon-<name>.png`. The prompt below stays for a generated set.

- **Final:** 64 × 64, shown at **28 px** next to a number in the top bar. At that size only one shape reads, so leave the hatching out.
- **Ratio** 1:1 · **Remove background:** on · `ref_image`: one of your approved tool icons
- **Fit:** `magick raw.png -fuzz 5% -trim +repage -resize 60x60 -background none -gravity center -extent 64x64 game/assets/ui/icon-<name>.png`
- **Prompt** (replace `[OBJECT]`):

```
Contemporary flat pictogram of [OBJECT], like a symbol from a modern map legend or a museum wayfinding sign. A single solid shape in dark sepia ink with one small cream paper detail inside it, clean geometric edges, even line weight, extremely simple and readable at a tiny size. Front view, centred, filling most of the frame. Isolated on a plain flat white background.
```

| File | Counts | `[OBJECT]` |
|---|---|---|
| `icon-sepia.png` | ink (the currency) | a single drop of ink |
| `icon-cible.png` | lives of the defended place | a small house with a water-mill wheel on its side |
| `icon-noms.png` | the Rôle des Noms | a single feather quill, slanted |
| `icon-vague.png` | the wave | a swallow-tailed pennant on a short pole |

## 3. Transition clouds: `game/assets/ui/cloud-1.png` … `cloud-4.png` (low priority)
The current clouds work until you redo them. Redo them last, so they match the new icons.
- **Final:** 1024 × 640, with 24 px of empty margin on every side. A cloud cut by the image edge shows a straight line when it slides in.
- **Ratio** 3:2 · **Remove background:** on · `ref_image`: your first approved new cloud
- **Fit:** `magick raw.png -fuzz 5% -trim +repage -resize 976x592 -background none -gravity center -extent 1024x640 game/assets/ui/cloud-<n>.png`
- **Prompt** (replace `[SHAPE]`):

```
Contemporary flat illustration of a cloud, [SHAPE], cut from warm cream paper like a layered papercut or a modern Japanese woodblock print. A clean outline in dark sepia ink, a few bands of crisp parallel hatching lines under its belly as a graphic print pattern, the rest flat and solid. The whole cloud floats in the middle of the image with empty space all around it. Isolated on a plain flat white background.
```

| File | `[SHAPE]` |
|---|---|
| `cloud-1.png` | wide and flat, with three rounded lobes |
| `cloud-2.png` | tall and towering, with a flat rounded base |
| `cloud-3.png` | two big merged lobes |
| `cloud-4.png` | a long drifting wisp with tapered ends |

## Drawn in code: no image
Buttons, the seal-red main button and panels (`game/src/ui/widgets.rs`: `frame`, `primary`). They stay sharp at any resolution and follow the palette in code.

## Not used by the game yet: skip for now
Nothing loads these yet. Make them once a screen uses them:
- `panel-torn.png` (torn notebook page)
- `seal.png`, `seal-empty.png` (wax seals for level nodes)
- `title.png` (title key art)
- `paper-grain.png` (shader texture)
