# Hachures : modèles 3D

Comment décider si un objet du plateau se fait **en code** ou en **asset** (prompt → image MOSS → modèle image-to-3D), et comment réussir les assets.

## La règle

| Si l'objet est… | Alors | Exemples |
|---|---|---|
| répété, posé en ligne, à l'échelle d'une case | **code** (primitives Bevy dans `game/src/map/mod.rs`, `prop()`) | barricade, boue, ronces, et plus tard éboulis, chaussée, vannes |
| unique, vu de près, organique | **asset** | tours, lieux à défendre, Nève, repères du Grand Plan |
| figurine d'espèce, nombreuse sur le plateau | **code** (`combat/figure/`), un seul jeu de pièces | Chaîneur, Fuyard, Harde, Convoi : voir les fiches du bestiaire (`direction-artistique.md`, phase 7) |
| boss | **assemblage en code** des mêmes pièces (bible §9) | la Grande Chaîne : chariot, bœufs, joug, lanterne, chaîne |

Pourquoi : une pièce répétée doit se **raccorder** à ses voisines, rester lisible à 60 px et changer de couleur selon le monde. Le code fait tout ça ; un modèle généré ne sait pas faire de jonctions exactes, et son maillage dense se transforme en gribouillis sous le shader de hachures.

La barricade montre ce que le code apporte :
- elle tend un bras vers chaque barricade voisine, donc une ligne forme un seul rempart, avec ses angles et ses T ;
- la compétence Fascines ajoute un fagot sous les poutres.

Pour voir les décors : `HACHURES_SHOT=out.png HACHURES_LEVEL=2 HACHURES_DEMO=1 [HACHURES_FASCINES=1] cargo run -p game --features dev`.

---

## Le pipeline asset

### 1. Image (MOSS)
Mêmes réglages que dans `docs/art/prompts.md` (activateur de prompt actif, cfg 2,5, 1,0 MP), avec :
- **Ratio** 1:1 ;
- **retrait du fond : désactivé.** L'outil 3D préfère un fond uni à un fond transparent ;
- `ref_image` : vide pour le premier modèle, puis le rendu d'un modèle déjà validé, pour que la famille reste cohérente.

L'outil image → 3D reconstruit ce qu'il voit, donc l'image doit montrer **un seul objet entier, en volume, sans ombre peinte**. Les icônes demandent l'inverse (une silhouette plate) : n'utilise pas leur gabarit ici.

**Prompt négatif 3D** (à coller dans la deuxième case) :

```
cropped, cut off, multiple objects, scene, environment, landscape, ground plane, cast shadow, dramatic lighting, rim light, reflections, glass, transparent, thin wires, ropes hanging loose, chains, fur, particles, smoke, motion blur, text, letters, logo, watermark, photograph, realistic, noisy texture, grunge, worst quality, lowres, jpeg artifact, nsfw
```

**Gabarit du prompt positif** (remplacer `[OBJET]` et `[DÉTAIL]`) :

```
Stylized low-poly 3D game asset of [OBJET], [DÉTAIL]. Chunky simplified proportions, thick sturdy parts, clean flat-shaded planes, matte solid colours from a limited palette: dark walnut wood, warm sepia brown, cream limestone, muted ochre, verdigris green, a single touch of seal red. Whole object visible and centred, three-quarter front view from slightly above. Soft even studio lighting, no cast shadow. Plain flat light grey background.
```

Pourquoi ces mots :
- « thick sturdy parts » : les pièces fines (cordes, pointes en aiguille) cassent à la reconstruction ;
- « matte solid colours » : le shader d'encre fait lui-même l'ombre et les hachures ;
- « three-quarter from slightly above » : c'est l'angle de la caméra de jeu, donc la face que le joueur voit est la mieux reconstruite.

### 2. Modèle 3D (ton outil image-to-3D)
Donne l'image telle quelle. Si l'outil propose des réglages, choisis faible densité et pas de PBR (rugosité/métal), juste la couleur.

### 3. Nettoyage (Blender), pour que le modèle entre dans le jeu sans retouche de code
- **Échelle :** 1 unité = 1 case. Emprise au sol ≤ 0,9 × 0,9.
- **Origine :** au centre de la base, posée au sol. Y vers le haut. Applique les transformations (Ctrl+A → All Transforms).
- **Avant :** l'objet regarde vers **+Z**. Les têtes de tour pivotent pour viser (`combat/towers.rs`, `look_at`).
- **Triangles :** environ 3 000 au plus (modificateur Decimate, ratio jusqu'à ce que la silhouette bouge).
- **Matériaux : remplace la texture générée par des couleurs unies,** une par pièce (bois, pierre, métal, accent). Une texture générée a l'éclairage peint dedans, et le shader le dédoublerait. Garde 3 à 5 matériaux.
- **Export :** glTF binaire (`.glb`), sans caméra ni lumière, dans `game/assets/models/`.
- Puis dis-moi le nom du fichier : je le branche dans le jeu et je fais la capture qui servira de `ref_image` pour son icône.

---

## Fiches

### Tête de tour : Harpon → `models/tower-harpon.glb`
Rôle en jeu : il **cloue** une cible sur place (`pin` 1,5 s). La silhouette doit dire « crochet », pas « catapulte » comme aujourd'hui.
- Taille : tête ≤ 0,6 de haut, emprise ≤ 0,7, posée sur le socle à y = 0,8.
- `[OBJET]` : `a heavy harpoon launcher mounted on a squat wooden swivel carriage`
- `[DÉTAIL]` : `one oversized barbed iron harpoon resting in its groove, pointing forward, a thick coiled rope drum on the side, iron bands on the wood`

### Tête de tour : Lanterne → `models/tower-lanterne.glb`
Rôle en jeu : elle **aveugle et ralentit** (monde 2, les Gens du Col). La silhouette doit dire « faisceau ».
- Taille : comme le Harpon.
- `[OBJET]` : `a surveyor's signal lantern on a short sturdy tripod`
- `[DÉTAIL]` : `a large round opaque pale-yellow lens disc on its front, a boxy brass-and-wood body with a small chimney cap, a thick hinged visor above the lens`

Garde la lentille **opaque**. Le verre transparent ne se reconstruit pas.

### Socle commun des tours → `models/tower-socle.glb`
Il remplacera `tower-round-bottom-a.glb`, le socle Kenney rouge et gris qui jure avec la palette.
- Taille : 0,6 de haut, emprise ≤ 0,85. Le dessus est plat et accueille la tête à y = 0,6 au-dessus de sa base.
- `[OBJET]` : `a squat survey marker pedestal of cut cream limestone`
- `[DÉTAIL]` : `a slightly tapered octagonal block with a flat top, a carved ochre band around its middle, a small engraved arrow on its front face`

### Lieu à défendre : le Moulin (monde 1) → `models/moulin.glb`
Aujourd'hui, la cible est une simple case brune. Un vrai bâtiment dit au joueur ce qu'il protège.
- Taille : ≤ 1,2 de haut, emprise ≤ 0,9.
- **Demande du code :** l'afficher sur la case cible. Je l'ajoute quand le modèle arrive.
- `[OBJET]` : `a small stout water mill house`
- `[DÉTAIL]` : `cream stone walls, a steep dark walnut roof, one large wooden paddle wheel on its side, a tiny seal-red door`

---

## Ensuite
Les icônes de ces tours se font **après** leur modèle, à partir d'une capture du modèle en jeu, en suivant `docs/art/prompts.md` §1.
