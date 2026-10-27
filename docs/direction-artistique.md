# Hachures : direction artistique

## L'idée
> Dans Hachures, **le plan-relief** (les maquettes de territoire en bois, plâtre, soie et sable que Vauban faisait faire pour le roi, et le monde 6 de la bible) devient **le plateau lui-même** : une maquette posée sur la table de la cartographe, filmée comme une miniature.

Ce qu'on retient de LittleBigPlanet et Sackboy : tout est **un vrai objet fait main**, filmé comme une maquette, avec une lumière douce, un flou de profondeur et un retour sur chaque geste. Le rendu « triple A » vient d'une idée de matière tenue partout, pas du nombre d'assets.

**Test du concurrent :** *Carto* est un livre d'images en tuiles de papier. Un plan-relief sculpté et encré à la main ne lui ressemble pas. Ça tient la promesse de la bible : « l'esthétique est celle de la gravure et non du livre d'images ».

## Tri : ce qu'on fait, ce qui attend

| Élément | Verdict |
|---|---|
| Le plateau-maquette | **L'élément unique**, voir la feuille de route ci-dessous |
| Décors en code (barricade, boue, ronces) | gardés ; leurs matières changent à la phase 3 |
| Icônes d'outil | faites en jeu : filmées sur les pièces (`ui/studio.rs`). Les icônes de compteur restent à faire avec MOSS |
| Cadres d'interface à filet de 1 px et repères de coin | à retirer après la feuille de route : des « tics » (voir la revue) |
| Nuages, Moulin, têtes de tour, socle (`3d-assets.md`) | en pause jusqu'à la fin de la feuille de route |

## Feuille de route du plateau-maquette

On ne passe à la phase suivante que quand le critère « fini quand » tient sur une capture.

### Phase 1 : lumière et objectif (faite en partie)
- ✓ **Flou de profondeur** (`DepthOfField` en mode Gaussian, compatible WebGL2) sur la caméra (`look.rs`).
  - La mise au point suit ce que regarde la caméra (`overworld/travel.rs`, `drive`).
  - L'ouverture augmente quand la caméra monte (`PLAY_F_STOPS`) : le Grand Plan s'adoucit sur ses bords comme une miniature, et le plateau reste net d'une rangée à l'autre.
- ✓ **Ombres douces** (`ShadowFilteringMethod::Gaussian`).
- ✗ **Lumière chaude et ambiance froide :** sans effet. Le shader d'encre ne lit que l'*intensité* de la lumière (`hachure.wesl`, `luminance`), pas sa couleur. C'est reporté à la phase 2, dans le shader.
- **Fini quand :** en une seconde on voit « une maquette posée sur une table ». C'est atteint sur le Grand Plan, pas encore sur le plateau de jeu. La caméra de jeu est haute, donc il y a peu de profondeur à flouter, et c'est la grille uniforme qui domine. C'est le travail de la phase 2.

### Phase 2 : matières de maquette (faite)
- ✓ **Relief continu :** les blocs vont de bord à bord (`map/mod.rs`, `Cuboid::new(1.0, …)`). Les seuls traits qui restent sont de vraies marches de relief.
- ✓ **Grille sous la main :** quand le curseur est sur le plateau, la grille apparaît sur 2 cases autour de lui, de plus en plus pâle (`ink/overlay.rs`). Non vue en capture, car l'outil de capture n'a pas de curseur : à vérifier en jouant.
- ✓ **Lumière chaude et ombre froide** dans le shader (`hachure.wesl`), puisqu'il ignore la couleur des lampes.
- ✓ **Résine et bois ciré :** `look::varnished(color, gloss)`.
  - L'eau (Eau, Gué, Glace) et le socle en noyer du plateau prennent des reflets du soleil sur une surface ondulée comme de la résine coulée.
  - Le shader les calcule lui-même, depuis la direction du soleil de `look::setup`. Pour Bevy, la surface reste mate, car son ambiance pâlissait toute l'eau (essayé).
- Le sol garde le grain de papier qu'il avait déjà ; un vrai grain de plâtre reste possible plus tard.
- **Fini quand :** en plissant les yeux, le plateau se lit comme 3 ou 4 grandes masses (terre, eau, relief) et non comme 180 cases. ✓ C'est atteint : les deux crêtes, la plaine, la rivière, les fourrés.

### Phase 3 : pièces de maquette (faite)
- ✓ **Matières des pièces** (`map/mod.rs`, `prop`) :
  - la barricade en bâtonnets de bois clair, ligaturés de ficelle crème ;
  - les fascines en brindilles sombres ;
  - les ronces en fil de fer rouge sombre sur un tertre de feutre ;
  - la boue, une flaque de vernis brun à reflets.
- ✓ **Tours sculptées :** `look::Carved`.
  - Le socle est en tilleul uni : la texture Kenney est retirée.
  - L'arme est teintée noyer par-dessus sa texture, pour que ses pièces se distinguent encore.

  Ce sont toujours les modèles Kenney : de vraies figurines viendront par `docs/3d-assets.md`.
- ✓ **Icônes filmées** (`ui/studio.rs`) :
  - chaque outil est posé sur un carré de terrain, dans un studio hors du plateau, sur sa propre couche de rendu ;
  - une caméra le filme en 128 px vu de la caméra de jeu, et la barre affiche cette image. L'icône est donc toujours la pièce elle-même.

  Les caméras du studio filment en continu : les arrêter après quelques images faisait perdre la photo, prise avant que les shaders soient prêts.
- **Effet de bord corrigé :** `look::WorldCamera` désigne la caméra du plateau. Avec les caméras du studio, le survol, les noms de lieux et `HACHURES_CAM` ne trouvaient plus « la » caméra et ne s'exécutaient plus.
- **Fini quand :** chaque pièce se reconnaît à sa silhouette *et* à sa matière, et l'icône est l'image de la pièce. ✓ C'est atteint pour les outils de terrain. Pour les tours, ça reste limité par les modèles Kenney.

### Phase 4 : le geste (faite)
- ✓ **La pose :** une pièce encrée à la main tombe dans sa case comme une épingle enfoncée dans la maquette (0,16 s, de plus en plus vite), s'écrase un peu à l'impact puis se redresse (0,14 s). C'est `map::Land`. Les tours aussi (socle et arme).
- ✓ **L'impact :** une gerbe de 7 fibres couleur terre qui volent, retombent et s'effacent, et le son de la matière, sur une hauteur jamais tout à fait la même :
  - bois léger pour la barricade ;
  - mou pour la boue ;
  - herbe pour les ronces ;
  - planche pour les tours.

  Les sons sont du pack Kenney Impact Sounds, déjà crédité.
- ✓ **Seule la main anime :** un redessin qui change plus de 3 cases (nouveau niveau, Crue) ne fait rien tomber.
- ✓ **Le fantôme :** sous le curseur, la pièce de l'outil de terrain tenu, pâle (alpha 0,45), là où elle irait, avec les raccords de barricade déjà calculés. Seulement là où l'encre est permise.
- Vérifié en capture avec `HACHURES_INK=t` (nouveau, `shot.rs`) : chute, écrasement, fibres. **Pas vérifiés en capture :** le fantôme (pas de curseur) et le son. À essayer en jouant.
- **Fini quand :** poser une pièce fait plaisir en boucle, même sans vague. C'est à juger manette en main, ou plutôt souris en main.

### Phase 5 : lumière et eau (faite)
- ✓ **L'eau gravée :** les lignes d'eau des vieilles cartes, parallèles à la rive, serrées près du bord et de plus en plus espacées vers le large, puis effacées (`hachure.wesl`, `look::water`).
  - Elles suivent une carte de distance à la rive (`look::shore_image`) : sur le plateau, refaite à chaque redessin (la Crue, le Gel) ; sur le Grand Plan, calculée une fois depuis `height`.
  - La carte est légèrement floutée pour que les lignes arrondissent les coins, comme la main du graveur. Elles s'effacent là où deux rives se rejoignent, au lieu d'y gonfler.
  - Elles passent sous les ponts (une bande de terre d'une case entre deux eaux, `map::crossing`) et sortent du plateau comme d'une feuille coupée, au lieu de boucler en U au bout de chaque bief.
  - L'eau qui coule respire : les lignes ondulent lentement, les reflets dérivent. La glace (`flow` 0) reste figée.
  - Le ciel se reflète dans la résine quand le regard rase la surface ; les hauts-fonds sont plus pâles que le large.
- ✓ **Le vrai soleil :** le shader lit la direction et la couleur du soleil, et la couleur du ciel, depuis Bevy (`look::SUNLIGHT`, chaud ; l'ambiance, froide). Déplacer le soleil déplace les reflets.
- ✓ **Ombre douce et froide :** dans l'ombre, l'encre bleuit et s'allège un peu. Une ombre est un lavis avec des traits, pas un trou noir.
- ✓ **Les plis (bureau seulement) :** occlusion ambiante et ombres de contact (`look::creases`), avec SMAA puisque l'occlusion veut le MSAA éteint. Le shader change l'occlusion en traits : là où une pièce touche le sol, le graveur hachure. Le réglage *Ombres* les coupe avec les ombres. WebGL2 n'en a pas, le web garde le reste.
- ✓ **Le papier :** une passe plein écran (`assets/shaders/paper.wesl`, `look::Paper`) pose un grain de papier fixe et assombrit les coins vers le sépia, comme une photo de maquette. L'interface est dessinée après, intacte.
  - Elle passe avant le flou de profondeur : laissées sans ordre, les deux passes lisaient l'image de l'autre à l'image précédente, et le grain s'empilait.
- ✓ **Correctif :** `Tonemapping::None` désactivait en silence le tramage (`DebandDither`) que les commentaires promettaient. `Tonemapping::Linear` garde les couleurs et trame quand même.
- **Fini quand :** l'eau se lit comme une carte gravée et l'œil sent d'où vient la lumière. ✓ Atteint sur le plateau et le Grand Plan. Les plis restent discrets sur les pièces Kenney.

### Phase 6 : la maquette vivante (à faire)
La maquette reste une gravure, mais **enluminée** : des lavis d'aquarelle sous les hachures, comme une carte de Cassini coloriée à la main. Le plateau devient un **bocage en patchwork** : de petits champs en feutre et en tissu, verts, blonds et ocre, cousus entre eux, bordés de haies, et qui bougent avec le vent. Spécification et tickets : `.scratch/maquette-vivante/`.

- **Couleur.** La fonte vers le papier se règle par matière : légère sur la terre, gardée sur les traits d'encre. La palette gagne les couleurs du bocage (pâture, pré, blé, jachère, lin, haie, ciel), et le brouillard et le vignettage se réchauffent.
  - **Fini quand :** on dit « une carte coloriée à la main », pas « une maquette beige ».
- **Lisibilité.** Chaque case garde sa famille de couleur. Les champs ne varient qu'à l'intérieur du Sol (pâture sèche) et du Sol bas (pré humide). Boue, Ronces, Eau, Roc et Chaussée restent immédiatement reconnaissables, et l'herbe ne cache jamais une case ni un repère.
  - **Fini quand :** sur une capture de 1-1, chaque type de case et chaque repère se nomme sans hésiter.
- **Le patchwork.** Les champs sont tirés de chaque niveau, toujours les mêmes, sans donnée à écrire, avec une couture en feutre là où deux champs se touchent. Sur le Grand Plan, des parcelles bordées de haies.
  - **Fini quand :** en plissant les yeux, le plateau se lit comme un quilt de champs, et non plus comme une plaine unie.
- **Le vent.** Les arbres, l'herbe, les fleurs et le blé se balancent d'un même vent, et l'on voit les rafales traverser les champs. Leurs ombres suivent.
  - **Fini quand :** deux captures à deux instants différents ne diffèrent que par ce que le vent fait bouger.
- **La vie.** Des vols d'oiseaux en papier passent de temps en temps, des papillons volent au-dessus des fleurs, et les ombres des nuages glissent. La pluie vient avec la Crue et la neige avec le Gel, et la lumière se réchauffe de vague en vague. Des sons d'ambiance CC0 portent le tout.
  - **Fini quand :** le plateau vit même sans vague.
- **Budget.** Tout tourne aussi dans le navigateur, en suivant le préréglage graphique (Bas : peu d'herbe, pas d'oiseaux). Le préréglage Haut prend au plus 2 ms de GPU de plus qu'avant cette phase.

## Revues (grille taste-skill, `references/critique.md`)

La skill est faite pour des pages web, et son inspecteur ne mesure pas un rendu Bevy : ces revues sont faites à l'œil sur les captures, avec sa grille.

```
ROUND 1 (niveau 1-2, avant la phase 1)
Vu en une seconde : une grille beige.
Idée : aucune visible.
Scores : design 5,5 · utilisabilité 7 · créativité 5,5 · contenu 7 → 6,1
Plafonds : filets décoratifs visibles (boutons 1 px, repères de coin, tirets de grille) → 6,5 au plus
Trois problèmes :
1. Le plateau est 180 masses identiques, rien ne mène l'œil → phase 2.
2. Palette crème + rouge brique, socles Kenney rouge et gris → phase 3.
3. Filets de 1 px et repères de coin dans l'interface → après la feuille de route.

ROUND 2 (après la phase 1)
Vu en une seconde : le Grand Plan, une maquette ; le plateau de jeu, toujours une grille.
Idée : le plan-relief, lisible seulement sur le Grand Plan.
Scores : design 6 · utilisabilité 7 · créativité 6 · contenu 7 → 6,4
Plafonds : mêmes filets → 6,5 au plus
Trois problèmes :
1. Grille uniforme sur le plateau de jeu → phase 2 (relief continu).
2. Le flou reste discret en jeu, à cause de la caméra haute. Ne pas le pousser plus : les rangées proches deviennent floues et la lecture en pâtit (essayé à f/0,05).
3. Lumière sans couleur dans le shader → phase 2.
```

```
ROUND 3 (après la phase 2)
Vu en une seconde : une plaine coupée par une rivière, entre deux crêtes ; une maquette, plus un tableur.
Idée : le plan-relief se lit sur le plateau (masses continues, eau vernie, socle en bois).
Scores : design 6,5 · utilisabilité 7 · créativité 6,5 · contenu 7 → 6,8
Plafonds : filets de l'interface (boutons 1 px, repères de coin) → 6,5 au plus tant qu'ils restent
Trois problèmes :
1. Les pièces (tours Kenney rouge et gris, ronces trop noires) ne sont pas encore « de la maquette » → phase 3.
2. La plaine est grande et vide de détail : un grain de plâtre ou des touffes ici et là → phase 3.
3. Les filets de l'interface plafonnent toujours la note → à traiter après la feuille de route.
```

```
ROUND 4 (après la phase 3)
Vu en une seconde : une maquette de vallée, avec ses pièces en bois, ficelle et feutre.
Idée : le plan-relief, jusque dans la barre (les icônes sont les pièces filmées).
Scores : design 7 · utilisabilité 7,5 · créativité 7 · contenu 7 → 7,1
Plafonds : filets de l'interface (boutons 1 px, repères de coin) → 6,5 au plus tant qu'ils restent
Trois problèmes :
1. Les têtes de tour Kenney (rouge tirant sur le brun) restent les pièces les moins « maquette » → 3d-assets.md.
2. Poser une pièce n'a ni geste ni son → phase 4.
3. Les filets de l'interface plafonnent toujours la note.
```

```
ROUND 5 (après la phase 4)
Vu en une seconde : une maquette de vallée ; poser une pièce la fait tomber, taper et soulever un peu de terre.
Idée : le plan-relief, tenu du plateau à la barre et jusqu'au geste.
Scores : design 7 · utilisabilité 8 · créativité 7,5 · contenu 7 → 7,4
Plafonds : filets de l'interface (boutons 1 px, repères de coin) → 6,5 au plus tant qu'ils restent
Trois problèmes :
1. Les filets de l'interface plafonnent la note : c'est maintenant le premier chantier.
2. Les têtes de tour Kenney restent les pièces les moins « maquette » → 3d-assets.md.
3. Le fantôme et le son restent à juger en jouant.
```

```
ROUND 6 (après la phase 5)
Vu en une seconde : une maquette de vallée gravée ; l'eau a ses lignes de rive, comme sur une carte ancienne.
Idée : le plan-relief, et maintenant la gravure jusque dans l'eau et la lumière.
Scores : design 7,5 · utilisabilité 8 · créativité 8 · contenu 7 → 7,7
Plafonds : filets de l'interface (boutons 1 px, repères de coin) → 6,5 au plus tant qu'ils restent
Trois problèmes :
1. Les filets de l'interface plafonnent toujours la note : premier chantier.
2. ✓ Les lignes d'eau d'une rivière de deux cases faisaient des boucles en U au bout de chaque bief, un peu « piste » : corrigé (ponts et bords du plateau).
3. Le web n'a ni occlusion ni ombres de contact : à vérifier dans le navigateur que le reste y tient.
```

```
ROUND 7 (phase 6, après l'enluminure : ticket 02)
Vu en une seconde : une vallée verte coupée par une rivière bleue, entre deux crêtes de pierre chaude ; une carte coloriée à la main.
Idée : la gravure enluminée. Le plateau a enfin une couleur à lui, et chaque case garde la sienne.
Scores : design 7,5 · utilisabilité 8 · créativité 8 · contenu 7 → 7,6
Plafonds : filets de l'interface (boutons 1 px, repères de coin) → 6,5 au plus tant qu'ils restent
Trois problèmes :
1. La plaine est verte mais unie : rien ne la découpe encore en champs → ticket 03 (les champs du plateau).
2. Rien ne bouge en dehors de l'eau → tickets 01 et 04 (le vent, l'herbe).
3. Le blé derrière le plateau et les crêtes de pierre sont proches en valeur : à surveiller quand les champs du Grand Plan arrivent (ticket 05).
```

**Cible :** environ 7,5. Atteinte à l'arrondi près, hors plafond. La feuille de route du plateau est finie ; le prochain chantier est l'interface.
