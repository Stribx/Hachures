# HACHURES — Bible narrative (version unique)

*Hachures* est le titre de travail que je retiens. Une cartographe déserteuse y défend les Marches contre les légions d'un empire qui veut sceller le monde dans un dernier registre. Le joueur dévie les colonnes en redessinant le relief, mais chaque trait a un prix : un nom effacé de la terre. Le titre parle de cartographie sans tomber dans le générique : les hachures sont les petits traits qui figuraient le relief sur les cartes anciennes, et ce sont aussi les lignes de pente que suit chaque peuple. Mes recherches n'ont trouvé aucun jeu de ce nom sur Steam ni sur itch.io.

## TL;DR
- **Titre :** je retiens *Hachures*, avec « Qui trace la frontière ? » comme accroche. Je n'ai trouvé aucun jeu homonyme. *Ligne de partage* et *Isohypse* sont les alternatives (la seconde est en collision mineure avec un objet de *Battlefield 6*). *Le Dernier Cadastre* devient le nom du registre impérial dans l'histoire, car le titre est trop proche de *Cadastre: A Land Survey RPG*, un jeu de rôle sur table publié sur itch.io qui parle de frontières et d'arpenteurs.
- **Histoire :** Nève Achard, ancienne Première Arpenteuse de l'Empire, déserte avant que le Dernier Cadastre ne fige les Marches. Dans ce monde, une carte faite à l'encre de terre et vue par des témoins devient vraie. Chaque peuple lit le relief à sa façon : par la pente, par l'ombre (la peur) ou par le but. Retournement central : les crues, les gels et les effondrements viennent de cartes rivales qui tirent la même terre dans plusieurs sens. L'Empire a donc un argument solide, et la question morale reste ouverte jusqu'au bout.
- **Game design :** il ne change pas. Chaque mécanique se ramène à trois opérations moteur : [T] changer le type d'une case, [P] changer un profil de coûts, [S] ajouter une source de Dijkstra. Les seules exceptions sont la couche de combat (PV, dégâts), la jauge cachée « Tension de la feuille » et la règle de récompense des Fuyards ; je les signale toutes. La règle de pose devient le **Droit de Passage**, le mode F1 devient le **Calque**, et le public visé passe à 12 ans et plus.

---

## En-tête

| Paramètre | Valeur |
|---|---|
| Titre de travail | **Hachures** (accroche : « Qui trace la frontière ? ») |
| Genre | Tower defense 3D low-poly fondé sur le détournement de chemins : un flow field Dijkstra par espèce |
| Plateforme | Navigateur (Rust + Bevy compilé en WebAssembly), jouable à la souris seule |
| Chapitres | 6 mondes de 3 niveaux à 12 vagues, plus 1 niveau de boss chacun, soit 24 niveaux |
| Ton | Sombre et épique, retenu, sans gore ni cynisme : une tragédie politique à hauteur de personnages |
| Public | 12 ans et plus (ajusté à partir de 10 ans et plus, voir plus bas), joueurs occasionnels à confirmés, recruteurs |
| Difficulté | Normale et Héroïque. Tension dynamique cachée. Les règles de terrain simultanées passent de 1 (monde 1) à 3 (monde 6) |
| Durée | Campagne de 7 à 9 h, niveaux de 15 à 20 min, post-game de 5 h et plus |
| Inspirations d'esprit | *Kingdom Rush* (structure), *Bastion* (narration en direct), Metroidvania (retour dans les zones), *The Banner Saga* (systèmes qui portent l'émotion), *Pentiment* (choix sans bonne réponse), *Frostpunk* et *Darkest Dungeon* (coût moral des décisions), *Thronefall* et *Bad North* (lisibilité minimaliste) |
| Inspirations littéraires et historiques (esprit seulement) | Cadastre napoléonien, Ordnance Survey d'Irlande et la pièce *Translations* de Brian Friel, Great Trigonometrical Survey de l'Inde, Conférence de Berlin, arrêt *Mabo* ; Borges, *De la rigueur de la science* ; China Miéville, *The City & The City* (deux peuples qui ne voient pas la même ville) |

**Pourquoi 12 ans et plus ?** Le jeu parle de conquête, de conscription de peuples annexés, d'exil et de noms effacés. Les adversaires abattus sont parfois des voisins enrôlés de force. Aucun sang n'est montré : en low-poly, les unités vaincues se brisent en éclats d'encre. Mais la portée morale suppose qu'on sache lire un conflit où personne n'a entièrement tort. La mécanique reste lisible pour un joueur occasionnel. Il faut confirmer la classification officielle au moment de la publication ; ici, c'est une cible de conception.

### Vérification du titre et des alternatives

| Titre | Ce que la recherche a trouvé | Verdict |
|---|---|---|
| **Hachures** | Aucun jeu de ce nom sur Steam ou itch.io. Les seuls résultats proches sont des homonymes partiels sans rapport (*Hachishaku* sur Steam, 2025)\[1\] | **Retenu**, disponible a priori. Le mot existe aussi en anglais (*hachures*, terme de cartographie) |
| Ligne de partage | Aucun jeu trouvé, seulement des résultats génériques | Alternative n° 1. Le jeu de mots est fort (ligne de partage des eaux et frontière), mais le titre est long et ne se traduit pas |
| Isohypse | Aucun jeu sur Steam ni itch.io. En revanche, « Isohypse AR » est une arme cosmétique de *Battlefield 6* (lancement de la saison 4)\[2\] | Alternative n° 2. Collision mineure dans le secteur du jeu vidéo |
| Le Dernier Cadastre | Aucun jeu de ce nom. Mais *Cadastre: A Land Survey RPG* existe sur itch.io : un jeu de rôle sur table « about borders and the surveyors who draw them »\[3\] | Écarté comme titre, gardé dans l'histoire : c'est le registre impérial |
| Écartés | *Terra Nullius* : plusieurs jeux sur itch.io et une forte charge historique.\[4\]\[5\]\[6\]\[7\] *Carto* : jeu de Sunhead Games (2020) où une jeune cartographe réarrange la géographie. *Orographe* : nom d'un studio d'illustration qui travaille pour le jeu vidéo\[8\]\[9\]\[10\] | Pris ou trop proches |

Je n'ai interrogé aucune base de marques (INPI, EUIPO, USPTO). Il faut le faire avant toute page publique. Il faut aussi renommer le fichier de travail en « Hachures — Bible narrative ».

**Voisin à distinguer : *Carto*.** Dans *Carto*, une petite cartographe fait pivoter des tuiles de carte pour changer la géographie. Le ton est doux et le jeu est un puzzle.\[11\]\[12\] *Hachures* doit s'en écarter nettement : ici, on ne déplace jamais de tuiles, on ne fait pas tourner de pièces de carte, l'esthétique est celle de la gravure et non du livre d'images, et le pouvoir est toujours payé.

---

## 1. Pitch en 5 phrases

1. Dans les Marches, une carte tracée à l'encre de terre et vue par des témoins devient le paysage : la rivière se déplace, la haie pousse, le village s'efface.
2. Nève Achard a levé ces terres pour l'Empire du Registre. Quand les légions arrivent pour sceller le Dernier Cadastre, qui fixera chaque frontière pour toujours, elle déserte avec son carnet et sa plume.
3. Les légions sont faites de peuples annexés. Chacun lit le relief à sa manière : les Échassiers aiment la boue, les Hardes fuient l'ombre des tours, les Jalonneurs ne voient que la borne à voler. Nève ne les arrête donc pas, elle leur redessine le chemin.
4. Chaque trait coûte : pour changer une case, il faut lui ôter son nom, et ce que ce nom gardait meurt avec lui. Le premier nom que Nève a effacé était celui de son propre village.
5. Plus la guerre avance, plus l'argument de l'Empire pèse : les cartes rivales déchirent la terre, et une carte unique ferait cesser les crues. À la fin, Nève doit décider qui a le droit de tracer, et si elle-même l'a encore.

**Ouverture jouable (3 minutes, sert de tutoriel).** Une main gratte un nom sur un plan. Au loin, un village s'efface maison par maison, comme si le paysage oubliait qu'il avait un toit. C'est un souvenir : Nève, dix ans plus tôt, gagne son grade en « simplifiant » une vallée. Retour au présent. Les premières colonnes impériales franchissent le gué. Nève pose sa première barricade, le trait de crayon en pointillé montre le détour, et la colonne se tord. Dernière annotation dans la marge : « Je ne me souviens plus du nom. C'est ça, le prix. »

---

## 2. Univers, lore et règles du monde

### Le monde
Les **Marches** sont une bande de terres longue de plusieurs centaines de lieues. On y trouve des bocages inondables, des cols, des combes calcaires, des dunes de sel et une ville d'écluses. Elles s'étendent entre l'**Empire du Registre** (au nord-ouest) et des terres qu'aucune carte n'a jamais tenues. L'Empire n'a pas de dieu-roi. Il a une **Chancellerie des Limites**, une armée de **chaîneurs** (arpenteurs-soldats qui mesurent à la chaîne de fer) et une conviction : ce qui est inscrit au Registre existe, et le reste est à inscrire.

### Les règles du fantastique (ce que l'on peut, ce que cela coûte, ce que l'on ne peut pas)
1. **L'encre de terre.** On broie de la terre du lieu dans de l'huile de noix pour obtenir la *sépia*. Une carte tracée avec cette encre et **vue par un témoin** (le cartographe suffit) devient vraie, dans un rayon que le trait peut réellement couvrir. Ce n'est pas de la magie à distance : Nève doit être sur place, carte dépliée.
2. **Le prix du nom.** Pour changer la nature d'un lieu (faire d'un champ un marais), il faut lui retirer son nom. Ce nom disparaît des mémoires, y compris de celle du cartographe. L'Empire a tout bâti là-dessus : renommer, c'est conquérir.
3. **Le Droit de Passage.** Une carte qui ne laisse passer personne est une *carte close*. La terre la rejette : l'encre ne prend pas, le trait se craquelle. La légende dit qu'un premier chancelier a scellé la Vallée de Vaux sans issue et qu'il ne reste de cette vallée qu'une tache blanche. Même l'Empire respecte ce droit, et même Nève y est tenue.
4. **Les trois lectures.** Chaque peuple voit le relief selon sa nature. Certains lisent **la Pente** : ils jugent le terrain lui-même. D'autres lisent **l'Ombre** : ils fuient ce qui les menace, à commencer par les tours. D'autres encore lisent **le But** : seul compte l'endroit où ils vont. Une même vallée n'est donc pas la même vallée pour tous.
5. **La terre hésite.** Quand deux cartes vraies se contredisent sur un même lieu, la terre oscille entre elles : elle inonde, gèle, s'effondre ou migre. On sent l'hésitation venir un peu avant, quand les hachures du sol frissonnent. C'est le grand secret du monde, révélé au monde 3 : les catastrophes des Marches sont des querelles de cartes.
6. **Le sceau et la sépia.** Chaque légionnaire porte un sceau de cire et d'encre, sa part du Registre. Brisé, le sceau rend son encre. C'est la seule ressource en niveau.
7. **Limites.** Nève ne peut tracer que sur les cases que le trait peut atteindre, sur la feuille du niveau. Elle ne peut pas effacer une créature, seulement le chemin qui la mène. Elle ne peut pas rendre un nom effacé : le monde 6 le confirme, et seul le secret de fin le contourne en partie.

### Ce que l'histoire nous prête (ancrages réels, utilisés comme esprit et jamais cités en jeu)
- **Le cadastre comme code.** La loi du 15 septembre 1807 a institué en France le premier cadastre général parcellaire, base d'un impôt foncier.\[13\] Napoléon voulait en faire un instrument juridique et un outil fiscal, et l'instruction de 1811 en faisait « le grand livre terrier de France ». Le chantier a duré plus de quarante ans.\[14\]\[15\]\[16\] La Chancellerie des Limites en tire sa bureaucratie patiente, ses registres, et l'idée qu'un plan fait foi en justice.
- **Renommer pour posséder.** L'Ordnance Survey d'Irlande (1825-1841) a cartographié l'île avec des Royal Engineers.\[17\] Selon l'historien Nollaig Ó Muraíle, John O'Donovan y a sourcé quelque 140 000 formes de noms, surtout dans les bibliothèques de Trinity College et de la Royal Irish Academy plutôt qu'à pied à travers l'Irlande, et il en a proposé les graphies anglicisées. La pièce *Translations* de Brian Friel (1980) se passe en 1833 à Ballybeg, transcription de *Baile Beag*.\[18\] Le géographe J.H. Andrews, principale source de Friel, jugeait « dangerously ambiguous » de dire que la Survey avait « nommé » le pays, car des noms anglicisés étaient déjà en usage, et Friel a lui-même reconnu en 1983 de « tiny bruises inflicted on history ». C'est le cœur du « prix du nom », et la nuance nourrit l'ambiguïté.
- **Mesurer pour gouverner.** Le Great Trigonometrical Survey de l'Inde a commencé le 10 avril 1802 près de Madras. Son arc méridien, achevé en 1841 sous George Everest, s'étend sur 2 400 km, du cap Comorin à Banog, près de Mussoorie. L'historien Matthew Edney y voit une géographie qui servit en première ligne de la conquête territoriale de la Compagnie des Indes.\[19\] D'où la Grande Chaîne et le Théodolite, boss des mondes 1 et 2.
- **La ligne droite et son mythe.** On répète que 80 % des frontières africaines suivent des parallèles et des méridiens.\[20\] Une étude de 2025 (Paine, Qiu et Ricart-Huguet, *American Political Science Review*) montre que les lignes droites ne sont le trait principal que de 37 % des frontières bilatérales, dont 18 % de lignes astronomiques, et que des frontières politiques précoloniales en ont influencé 62 %.\[21\] Le Méridien (boss du monde 4) joue sur cette tension : la ligne droite existe, mais elle n'est jamais toute l'histoire. En 1890, Lord Salisbury avoua : « We have been giving away mountains and rivers and lakes to each other, only hindered by the small impediment that we never knew exactly where the mountains and rivers and lakes were. »
- **La fiction juridique.** Le 3 juin 1992, l'arrêt *Mabo* de la Haute Cour d'Australie a fait de la *terra nullius* une fiction juridique.\[4\]\[6\]\[22\] Pour le procès, Eddie Mabo avait dessiné une carte de l'île de Mer avec les noms des familles sur chaque parcelle.\[23\] Le Rôle des Noms d'Oudine (monde 1) en est l'écho : une contre-carte faite de mémoire.

### Tableau « règle du monde → traduction en jeu »

| Règle du monde | Traduction en jeu | Opération moteur |
|---|---|---|
| L'encre de terre fait le relief | Poser une barricade, de la boue, des ronces ou une tour | [T] changer le type de la case |
| Le trait de crayon précède l'encre | Aperçu du détour en pointillé avant la pose | Calcul de Dijkstra à blanc, rien n'est appliqué |
| Le Droit de Passage | Pose refusée si une espèce terrestre n'a plus aucun chemin. La vérification inclut l'état annoncé de l'événement suivant | Vérification de Dijkstra par espèce |
| Chaque peuple lit la Pente | Profil de coûts par espèce (1 = normal, plus haut = évité, ∞ = infranchissable) | [P] |
| Lire l'Ombre | Coût = 1 + nombre de tours couvrant la case | [P], recalculé à chaque pose |
| Lire le But | Fuyards, voleurs et alliés escortés suivent une autre source | [S] |
| La terre hésite | Événement de monde annoncé une vague à l'avance (frisson des hachures) | [T] ou [P] selon le monde |
| Le sceau rend son encre | Ressource unique, la sépia, gagnée sur les ennemis vaincus | Couche combat [X] |
| Le prix du nom | Changer le type d'une case nommée efface son nom, qui sort du Rôle des Noms | [T] et une donnée de progression |
| Le Calque | Mode F1 : flow fields et coûts de chaque espèce | Affichage, sans opération |
| La feuille tendue | Jauge cachée d'équilibrage dynamique | Exception [X] |
| Les feuillets manquants | Zones verrouillées des mondes précédents | [T] via les pouvoirs obtenus |

**Le mode F1, justifié.** Le *Calque* est une feuille de papier huilé que Nève pose sur le monde. C'est le premier exercice de tout apprenti arpenteur : « lire avant de tracer ». Sous le calque, chaque espèce apparaît comme un champ de hachures orientées (son flow field), et la légende affiche les coûts case par case. Le Calque est disponible pour tous, sans pénalité. C'est une aide d'accessibilité, et c'est le meilleur argument du projet devant un recruteur.

---

## 3. Personnages principaux et antagonistes

### Nève Achard, la cartographe (protagoniste)
- **Qui :** 34 ans, ancienne Première Arpenteuse des Marches, née dans un village dont elle a oublié le nom.
- **Désir :** arrêter le scellement des Marches sans devenir ce qu'elle combat.
- **Obstacle :** les légions, son ancienne maîtresse, la règle du prix du nom, et le fait que l'Empire n'a pas tort sur tout.
- **Faille intime :** elle croit qu'une bonne carte résout tout et préfère le plan aux gens. Elle a obtenu son grade en effaçant le village de sa mère, et ne s'en souvient que par le vide.
- **Voix :** phrases courtes, mesures exactes, ironie sèche. Elle ne dit jamais « je crois », elle dit « j'ai mesuré ».
- **Narration en direct :** son carnet de levé s'écrit dans la marge pendant la partie et réagit aux poses du joueur (voir section 9).
- Répliques :
  - « Trois cases de boue valent une barricade. Et la boue, ça ne brûle pas. »
  - « Je ne défends pas une frontière. Je défends le droit de ne pas en avoir une seule. »
  - « J'ai écrit ce bief. Je savais qu'ils passeraient par là. C'est pour ça que j'ai honte. »

### Ilse Damerval, Chancelière des Limites (antagoniste principale)
- **Qui :** 61 ans. Elle a formé Nève et l'aimait comme une fille, ce qu'elle ne dit jamais.
- **Désir :** sceller le Dernier Cadastre pour que plus aucune terre n'hésite.
- **Raison :** sa fille est morte dans la Déchirure de Vaux, quand deux cartes rivales ont ouvert le sol sous un village.
- **Faille :** elle confond l'immobilité et la paix.
- **Voix :** langue juridique, élégante, patiente. Elle cite des articles du Registre, ne crie jamais et ne ment jamais.
- Répliques :
  - « Une frontière tracée une fois pour toutes ne tue plus personne. Ce sont les frontières qui hésitent qui remplissent les cimetières. »
  - « Article premier : ce qui est inscrit existe. Article second : ce qui n'est pas inscrit attend. Rien n'est cruel là-dedans, Nève. C'est de l'ordre. »
  - « Même moi, je laisse une issue. Ne me fais pas l'injure de croire le contraire. »

### Corentin Vass, Légat-Chaîneur (rival)
- **Qui :** 26 ans, ancien apprenti de Nève. Il a appris d'elle à compter les pas.
- **Désir :** que Nève reconnaisse qu'il est devenu meilleur qu'elle.
- **Faille :** il obéit aux nombres parce qu'ils ne le jugent pas.
- **Arc :** du zèle au doute (monde 5), puis trois issues selon les choix.
- **Voix :** chiffres, unités, débit rapide. Il tutoie Nève comme un enfant qui en veut à sa mère.
- Répliques :
  - « Quarante-deux chaînes de long, trois de large. Je l'ai mesurée moi-même, ta vallée. C'est toi qui m'as appris. »
  - « Tu as toujours dit qu'une carte ne ment pas. Alors pourquoi tu en fais une autre ? »

### Les héros et cibles à défendre de chaque monde

| Monde | Personnage | Ce qu'on défend | Voix et réplique |
|---|---|---|---|
| 1 | **Oudine**, gardienne des Noms, 70 ans | Le Moulin des Noms, où le Rôle des Noms est gravé sur les meules | Terrienne, à proverbes : « Ce champ s'appelle Pain-Perdu parce qu'on l'a perdu deux fois. Mets-le sur ta carte, ou je te le répète jusqu'à l'hiver. » |
| 2 | **Hesse Corbière**, passeuse de cols | Son convoi (allié escorté) | Sardonique : « Je ne passe pas des gens. Je passe des histoires qui ont encore des jambes. » |
| 3 | **Pellan Fosse**, maître-étayeur aveugle | Le Puits d'Air, qui fait respirer les galeries | Lent, sensoriel : « Ici, on ne voit pas la pente. On l'entend. » |
| 4 | **Yaro Douze-Puits**, cheffe de caravane | La Saline et ses puits | Tranchante : « Trace-la droite, ta ligne. Comme ça, au moins, on saura qui l'a tracée. » |
| 5 | **Bérangère Cloche**, pontière | La Grande Vanne | Pragmatique : « L'eau n'a pas de camp. Elle a des horaires. » |
| 6 | Personne, ou tout le monde | La Case blanche : le village effacé de Nève | Ce qui manque parle par le silence |

### Les légats, commandants des boss
Ils ont une réplique de cri et une de chute chacun, ce qui reste dans le budget.
- **Gast**, sergent-arpenteur (monde 1) : « Borne quatre ! Plantez ! Le reste suivra. »
- **Ombeline Sarre**, légate de visée (monde 2) : « Trois points suffisent à tenir une montagne. »
- **Les frères Tarrière**, foreurs (monde 3), qui finissent les phrases l'un de l'autre.
- **Ascelin Droit**, le Méridien (monde 4) : « Je ne dévie pas. C'est mon seul mérite. »

---

## 4. Boucle de gameplay globale

| Échelle | Durée | Ce que fait le joueur | Ce que raconte la boucle |
|---|---|---|---|
| Courte | 10 à 40 s | Lire les hachures (Calque si besoin), choisir une case, voir le trait de crayon en pointillé, encrer (poser) | Nève redessine, le monde obéit, une colonne se tord |
| Moyenne | Une vague, environ 1 min 15 à 1 min 40 | Annonce de l'événement une vague à l'avance, ajustements, gain de sépia | La terre hésite, Nève anticipe |
| Session | Un niveau, 15 à 20 min (12 vagues) | Victoire, sceaux (points de compétence), choix éventuel, annotations | Le carnet s'écrit, un nom est gardé ou perdu |
| Méta | Un monde, environ 1 h 30 | Boss, nouveau pouvoir, retour aux zones verrouillées | Nève gagne des moyens et perd des souvenirs |

---

## 5. Tableau des 6 mondes

| # | Titre | Biome | Monstres principaux | Mécanique clé (règles simultanées) | Boss | Récompense |
|---|---|---|---|---|---|---|
| 1 | Les Marches de Saule | Bocage inondable, gués, moulins | Chaîneurs, Échassiers conscrits, Jalonneurs, Fuyards | **La Crue** (1) | La Grande Chaîne | Pouvoir **Gratter** et tour Harpon (au niveau 1-3) |
| 2 | Les Cols de Verre | Cols, névés, lacs gelés | Gens du Col, Hardes, Chaîneurs, convoi allié | **Le Gel** (1) | Le Théodolite de Givre | Pouvoir **Renommer** et tour Lanterne |
| 3 | Les Combes Creuses | Karst, ardoisières, galeries | Taupiers, Étayeurs, Jalonneurs | **Les Effondrements** et la lecture de l'Ombre en masse (2) | La Foreuse | Tour Sonde et le feuillet « Carte signée » |
| 4 | Le Partage des Sables | Dunes, salines, puits | Sabliers, Caravaniers rancuniers, Hardes | **Les Dunes migrantes** et les voleurs (2) | Le Méridien | Pouvoir **Dater** |
| 5 | Vannes-sur-Lieue | Polders, canaux, ville basse | Haleurs, Échassiers, Chaîneurs d'élite | **Les Écluses**, la Crue et les voleurs (3) | L'Éclusier (Corentin) | Tour Treuil et la clé de la Chancellerie |
| 6 | Le Plan-Relief | Une vallée entière sculptée en maquette dans la capitale | Greffiers, Légats, tous les autres | **La Mise au net**, les Écluses et le Gel (3) | Le Dernier Cadastre (Ilse) | Pouvoir **Signer** (éditeur) et fins |

Choix de valeur : la progression des règles simultanées est 1, 1, 2, 2, 3, 3.

---

## 6. Les mondes en détail

### Monde 1 : Les Marches de Saule (chapitre du MVP)

1. **Thème et ambiance.** Le premier sang de la guerre et le premier nom. C'est une terre douce que l'on voit perdre ses mots. Pluie fine, saules taillés en têtards, cloches de vaches au loin.
2. **Biome.** Prairies basses, haies vives, un bief de moulin, des gués. Case propre au monde : le **Gué**, une eau peu profonde qui coûte peu aux Échassiers et davantage aux autres.
3. **Carte mentale.**
   - Points d'intérêt : le Gué-aux-Mères, les Haies de Pain-Perdu, le Bief, le Moulin des Noms.
   - Zone verrouillée : le **Moulin noyé**, sous un éboulis de meules. Il faut le pouvoir Gratter (fin du monde 1) pour y entrer, et la zone n'offre son vrai contenu qu'avec Renommer (monde 2).
   - Faction : les gens du bocage, menés par Oudine.
4. **Monstres.**
   - **Chaîneur** : lit la Pente et suit la route la plus sèche. Faible contre l'arbalète. 2 sépia.
   - **Échassier conscrit** : lit la Pente, préfère la boue et le gué, déteste les ronces. Faible contre le mortier quand ils se regroupent dans la boue. 3 sépia.
   - **Jalonneur** : lit le But. Il vise la **Borne-Mère** (seconde cible) pour la voler. Rapide et fragile, faible contre le harpon. 2 sépia.
   - **Fuyards** : non hostiles. Ils lisent le But (le Refuge). Ils ne rapportent pas de sépia mais des **Noms** pour le Rôle.
5. **Mécanique unique : la Crue.** Une vague à l'avance, les hachures du sol frissonnent en bleu et le grondement monte. Pendant une vague, les cases basses (Gué et Sol bas) deviennent Eau [T], puis elles redeviennent ce qu'elles étaient. Le Droit de Passage tient compte de la crue annoncée : on ne peut pas encrer une barricade qui fermerait tout pendant la crue.
6. **Événements scriptés.**
   - 1-1, vague 6 : première crue, commentée dans la marge [T].
   - 1-2, vague 4 : les premiers Fuyards apparaissent [S Refuge].
   - 1-3, vague 9 : un Échassier fredonne la berceuse d'Oudine. C'est la graine du retournement du monde 2 (pur habillage sonore).
7. **Boss : la Grande Chaîne** (un chariot tiré par deux bœufs, qui traîne une chaîne d'arpentage de vingt maillons, commandé par Gast).
   - **Phase 1, le Levé :** elle suit son flow field. Toutes les trois cases, elle pose de la **Chaussée** derrière elle [T] (coût 0,5 pour les impériaux, donc les colonnes l'empruntent). Point faible : la lanterne de visée, à l'avant. Trois tirs d'arbalète l'étourdissent.
   - **Phase 2, le Bornage :** elle plante trois bornes. Chaque borne devient une source pour les Jalonneurs [S]. Abattre une borne ou la gratter [T] retire la source.
   - **Phase 3, la Chaîne rompue :** les bœufs paniquent et le boss passe à la lecture de l'Ombre [P]. Il fuit les tours, ce qui permet de le rabattre. Point faible : le joug, exposé dans les virages. Un harpon l'immobilise.
8. **Progression.**
   - 1-1 : barricade, boue, arbalète.
   - 1-2 : ronces, mortier.
   - 1-3 : harpon.
   - Boss : pouvoir **Gratter**.
   - Chaque niveau donne 2 sceaux (un pour la victoire, un pour le défi de lecture).
9. **Narration.**
   - **Objectif :** tenir le Moulin des Noms le temps qu'Oudine grave le Rôle.
   - **Rebondissement :** la Grande Chaîne suit exactement le levé que Nève avait fait dix ans plus tôt, et elle reconnaît son écriture sur les bornes.
   - **Choix (fin du 1-3) :** où placer le Refuge ?
     - *Au Moulin* : les Fuyards traversent la zone de tir. Le mortier peut les blesser, mais chacun qui arrive garde un Nom. Le Rôle s'enrichit et Oudine parlera plus tard.
     - *Dans le Bois* : chemin sûr, mais le Moulin est à découvert. Au boss, l'Empire grave sa propre borne sur la meule : un Nom est perdu et Oudine se tait jusqu'au monde 6.
   - Répliques :
     - Oudine : « Le gué ne s'appelle pas "gué". Il s'appelle comme celles qui l'ont passé avec un enfant sur la hanche. »
     - Gast : « Borne quatre ! Plantez ! Le reste suivra. »
     - Nève (marge) : « C'est ma main sur la borne. Dix ans, et l'encre n'a pas pâli. »
10. **Direction artistique, musique, SFX.**
    - Palette : vert-de-gris, ocre et blanc de pluie. Les cases impériales (Chaussée) sont plates, pâles et quadrillées comme un plan.
    - Musique : violoncelle seul et bourdon de vielle, avec un percussif de bois qui imite le ruisseau. Le thème de Nève est un motif de cinq notes, légèrement boiteux.
    - SFX : plume qui gratte (pose), tampon sourd à chaque apparition de légion, cliquetis de la chaîne, grondement de crue.
11. **Boucles, transition et ouverture.**
    - Boucle : apprendre une lecture par niveau (la Pente, puis le But avec les voleurs et les Fuyards).
    - Transition : la Grande Chaîne s'effondre dans le bief. Dans ses coffres, Nève trouve un ordre de marche vers les Cols, signé « C. Vass ».
    - Ouverture : « Il a gardé ma façon de faire les V. »

### Monde 2 : Les Cols de Verre

1. **Thème et ambiance.** La loyauté. Qui passe, qui livre, qui reste. Le silence blanc et les sonnailles désaccordées.
2. **Biome.** Cols, névés, lacs gelés, refuges de pierre sèche. Case propre au monde : la **Glace**, qui apparaît par l'événement.
3. **Carte mentale.**
   - Points d'intérêt : la Combe aux Sonnailles, le Refuge Sarre, la Corniche des Convois.
   - Zone verrouillée : le **Refuge enseveli**, pris dans la glace. Il faut Renommer (fin du monde 2) pour changer la Glace en Eau et ouvrir un passage par le lac. Il faut ensuite Dater (monde 4) pour suspendre le gel.
   - Faction : les passeurs, qui ne sont alliés de personne.
4. **Monstres.**
   - **Gens du Col** (conscrits) : lisent la Pente. Ils aiment la glace et grimpent le roc à coût 3. Faibles contre la Lanterne, qui les aveugle et les ralentit. 3 sépia.
   - **Hardes** (bêtes de bât) : lisent l'Ombre, avec un coût de 1 plus le nombre de tours qui couvrent la case. On les canalise en plaçant les tours, pas les murs. 4 sépia.
   - **Chaîneurs** : comme au monde 1.
   - **Convoi de Hesse** (allié escorté) : lit le But (la sortie du col, une source [S]). Le joueur doit lui garder un chemin court et sûr.
5. **Mécanique unique : le Gel.** Annoncé par un givre qui court sur les hachures. L'Eau devient Glace [T] pendant deux vagues. Pour les Gens du Col, le lac devient une autoroute ; pour le convoi, un raccourci.
6. **Événements scriptés.**
   - 2-2, vague 7 : une avalanche bloque un couloir. C'est une case qui passe en Roc [T], vérifiée par le Droit de Passage dans les données.
   - 2-3 : niveau d'escorte complet, avec la source du convoi [S].
7. **Boss : le Théodolite de Givre** (une tour sur patins, à grande lentille, commandée par Ombeline Sarre).
   - **Phase 1, Visée :** une ligne de cases d'Eau gèle dans l'axe de la lentille [T].
   - **Phase 2, Triangulation :** deux mâts de signal sont plantés et deviennent les sources des Jalonneurs [S]. Si trois points sont tenus pendant une vague, le col est scellé et la partie est perdue.
   - **Phase 3, Dégel forcé :** la lentille fêlée chauffe, toute la Glace redevient Eau [T] et la tour s'enlise. Point faible : la fêlure, que la Lanterne révèle.
8. **Progression.** Tour Lanterne au niveau 2-1. Pouvoir **Renommer** après le boss, avec son coût : un Nom, sauf si la case n'en a pas.
9. **Narration.**
   - **Objectif :** faire passer le convoi de Hesse.
   - **Rebondissement :** le convoi transporte des déserteurs échassiers, ceux-là mêmes qui marchaient contre le Moulin au monde 1. Les légions sont faites de voisins.
   - **Choix :**
     - *Escorter les déserteurs* : le niveau 2-3 devient plus dur (escorte et vagues pleines). Au monde 5, une partie des Échassiers fait défection en pleine bataille : leur profil passe en Fuyards [P].
     - *Les livrer à Corentin*, qui détourne en échange une légion : le 2-3 compte deux vagues de moins. Au monde 5, les Échassiers deviennent **Rancuniers** (profil plus agressif, ronces à coût 3 au lieu de 5) et Hesse refuse de reparaître.
   - Répliques :
     - Hesse : « Tu veux savoir qui est l'ennemi ? Regarde qui a froid aux pieds dans les deux colonnes. »
     - Corentin : « Donne-les-moi et je retire la 4e. C'est de l'arithmétique, Nève. »
     - Nève : « Tout est de l'arithmétique. C'est ça le problème. »
10. **Direction artistique, musique, SFX.** Bleu ardoise et blanc de craie, ciel bas. La musique ajoute des sonnailles accordées un quart de ton trop bas et un chœur grave sans paroles. SFX : craquement de glace, lentille qui chante, vent dans les cordes du convoi.
11. **Boucles, transition et ouverture.**
    - Boucle : apprendre l'Ombre. Les tours deviennent des murs que seules certaines espèces voient.
    - Transition : Ombeline, vaincue, livre un mot. « La Chancelière vous attend dans les Combes. Elle veut vous montrer quelque chose que vous avez signé. »
    - Ouverture sur le Calque : sous la glace fondue apparaissent des hachures que personne n'a tracées.

### Monde 3 : Les Combes Creuses

1. **Thème et ambiance.** La responsabilité. Ce qu'on a signé sans le voir. Obscurité, gouttes, et le grand silence des galeries.
2. **Biome.** Plateau karstique, ardoisières, galeries étayées. Cases propres au monde : l'**Éboulis** (lent) et le **Gouffre** (∞ pour tous les marcheurs).
3. **Carte mentale.**
   - Points d'intérêt : les Ardoisières, le Puits d'Air, la Galerie Signée.
   - Zone verrouillée : la **Salle des Premiers Plans**. Il faut Dater (monde 4) pour suspendre un effondrement et l'atteindre.
   - Faction : les étayeurs aveugles de Pellan, qui lisent le relief par le son.
4. **Monstres.**
   - **Taupier** : lit la Pente. Le roc lui coûte 2 et l'éboulis 1, il passe sous les barricades à coût 3. Faible contre la Sonde. 4 sépia.
   - **Étayeur impérial** : lit l'Ombre et il est blindé. Il répare les bornes. Faible contre le mortier. 4 sépia.
   - **Jalonneur des galeries** : lit le But (le Puits d'Air).
5. **Mécanique unique : les Effondrements.** Une vague à l'avance, des fissures rouges apparaissent sur les hachures. Les cases marquées deviennent des Gouffres [T], définitivement pour le reste du niveau. Le Droit de Passage est garanti par les données : aucun ensemble d'effondrements ne ferme toutes les issues avec les seules cases de base. Le joueur ne peut pas encrer ce qui fermerait tout après l'effondrement annoncé.
6. **Événements scriptés.**
   - 3-2 : la lumière tombe et seul le Calque montre les colonnes pendant deux vagues. C'est un événement de rendu pur, sans opération.
   - 3-3 : la Galerie Signée, voir le rebondissement.
7. **Boss : la Foreuse** (une machine de forage qui plante des bornes profondes, conduite par les frères Tarrière).
   - **Phase 1 :** sous terre. Sa trajectoire n'est visible qu'au Calque. Là où elle perce, la case devient Éboulis [T].
   - **Phase 2 :** elle provoque des effondrements en chaîne [T] et les Taupiers apprennent à longer les gouffres [P].
   - **Phase 3 :** elle sort à l'air. Point faible : la mèche, exposée à chaque virage. La Sonde fait double dégât.
8. **Progression.** Tour Sonde (frappe sous terre). Feuillet « Carte signée » : il révèle les zones verrouillées sur la carte du monde.
9. **Narration.**
   - **Objectif :** garder le Puits d'Air ouvert pour que les étayeurs respirent.
   - **Rebondissement :** dans la Galerie Signée, Ilse a laissé le plan originel des Marches, signé « N. Achard ». En marge, de la main de Nève jeune : « Village sans intérêt, à simplifier. » Ilse y a joint une note : « Tu vois bien que les crues commencent là où deux cartes se recouvrent. Aide-moi à n'en faire qu'une. » C'est la révélation centrale : la terre hésite à cause des cartes rivales. Elle était préparée depuis le monde 1, où la crue arrive toujours après la pose d'une borne impériale.
   - **Choix :**
     - *Garder le plan* : il révèle les galeries cachées (deux feuillets supplémentaires et un niveau 4-2 plus facile). Mais Corentin en a une copie, et au monde 4 les légions sortent aussi des galeries (une source d'apparition supplémentaire, simple donnée de niveau).
     - *Le brûler* : moins d'informations, rien pour l'ennemi. Nève perd la seule trace écrite de son village.
   - Répliques :
     - Pellan : « Ta carte sent la cire neuve. Les anciennes sentaient la sueur. »
     - Nève : « Je ne me souviens pas de l'avoir écrit. Je me souviens de la plume. »
     - Un frère Tarrière : « On ne creuse pas, madame. On… » L'autre : « …confirme. »
10. **Direction artistique, musique, SFX.** Noir d'ardoise et jaune de lampe, des hachures phosphorescentes au Calque. La musique se fait de percussions métalliques, de gouttes accordées et de contrebasse. Le thème de Nève est joué à l'envers. SFX : bois d'étai qui gémit, souffle du puits, roche qui cède.
11. **Boucles, transition et ouverture.**
    - Boucle : deux règles en même temps (effondrements et Ombre).
    - Transition : les étayeurs remontent au jour et Pellan dit : « Au sud, les sables bougent depuis que l'Empire y a planté sa ligne. »
    - Ouverture : Nève a désormais un argument contre elle-même.

### Monde 4 : Le Partage des Sables

1. **Thème et ambiance.** La tentation de trancher. Une ligne droite comme solution, et ce qu'elle coupe. Lumière dure et vent continu.
2. **Biome.** Erg, salines blanches, puits. Cases propres au monde : la **Dune** (coût variable selon les peuples) et la **Saline**.
3. **Carte mentale.**
   - Points d'intérêt : les Salines, le Puits Douze, l'Erg Partagé.
   - Zone verrouillée : la **Saline engloutie**, sous une dune. Il faut Gratter, puis Dater pour figer la dune.
   - Faction : deux clans caravaniers en querelle de puits, celui de Yaro et celui des Hautes-Tentes.
4. **Monstres.**
   - **Sablier** : lit la Pente, la dune lui coûte 1 et la boue 4. Faible contre l'Héliographe, une variante de la Lanterne. 3 sépia.
   - **Caravaniers rancuniers** : lisent le But et visent les puits. Leur nombre dépend du choix. 3 sépia.
   - **Hardes** : comme au monde 2.
5. **Mécanique unique : les Dunes migrantes.** Annoncées par un sifflement de sable sur les hachures. Toutes les deux vagues, une rangée de Dunes avance d'une case dans le sens du vent : la case quittée devient Sol et la case gagnée devient Dune [T].
6. **Événements scriptés.**
   - 4-2 : si le plan du monde 3 a été gardé, une source d'apparition supplémentaire s'ouvre dans les galeries.
   - 4-3 : vent tournant, la direction de migration s'inverse une fois.
7. **Boss : le Méridien** (une colonne qui marche en ligne droite derrière Ascelin Droit, porteur du gnomon).
   - **Phase 1, la Ligne :** la colonne lit le relief comme une règle. Profil uniforme : 1 partout, ∞ seulement sur le Roc et la Barricade [P]. Les dunes ne la dévient pas, seuls les murs le font.
   - **Phase 2, Midi :** l'Héliographe aveugle la colonne, qui passe à la lecture de l'Ombre [P]. C'est la seule phase où on peut la plier.
   - **Phase 3, le Tracé :** derrière elle, une file de cases devient **Cadastré** [T] et coupe la carte en deux. Les dunes ne franchissent plus la ligne. Point faible : le porteur du gnomon, exposé à chaque arrêt de visée.
8. **Progression.** Pouvoir **Dater** après le boss : une fois par niveau, suspendre l'événement annoncé ou le déclencher en avance.
9. **Narration.**
   - **Objectif :** tenir les puits.
   - **Rebondissement :** Yaro ne veut pas être sauvée de l'Empire. Elle veut que Nève trace la frontière que l'Empire n'a pas su tracer entre les deux clans.
   - **Choix :**
     - *Tracer la ligne droite* : au boss, les deux clans aident Nève, avec des tours alliées gratuites en phase 1. Au monde 6, le clan lésé des Hautes-Tentes rejoint les voleurs.
     - *Refuser* : le boss est plus dur et les deux clans restent neutres. Yaro accuse Nève de lâcheté, et elle n'a pas tort.
   - Répliques :
     - Yaro : « Vous, les gens de plume, vous aimez les questions. Nous, on a soif. »
     - Ascelin Droit : « Je ne dévie pas. C'est mon seul mérite. »
     - Nève (marge, si elle trace) : « Ligne droite, 31 lieues. La plus facile que j'aie jamais tracée. C'est mauvais signe. »
10. **Direction artistique, musique, SFX.** Blanc de sel, ombres bleues, ciel presque blanc ; le Cadastré est un beige net qui coupe l'image. La musique se fait de cordes frottées, de souffle et de rhombe, sans pastiche « oriental ». SFX : sable qui coule, miroir qui claque, pas cadencés du Méridien.
11. **Boucles, transition et ouverture.**
    - Boucle : la règle de terrain se déplace. On apprend à poser pour la vague d'après.
    - Transition : un pigeon de Bérangère arrive avec le message « Vannes ferme ses écluses demain. Corentin tient la Grande. »
    - Ouverture : Nève peut maintenant décider du moment où la terre hésite. C'est le pouvoir de l'Empire.

### Monde 5 : Vannes-sur-Lieue

1. **Thème et ambiance.** Le prix collectif. Noyer l'ennemi, c'est aussi noyer la ville basse. Brume de canal, cloches d'écluse, linge aux fenêtres.
2. **Biome.** Polders, digues, canaux, une ville sur deux niveaux. Case propre au monde : la **Vanne**, qui alterne entre Eau et Sol selon un horaire.
3. **Carte mentale.**
   - Points d'intérêt : le Quai des Haleurs, les Polders bas, la Ville d'en dessous, la Grande Vanne.
   - Zone verrouillée : la **Ville d'en dessous**, noyée. Il faut Dater pour vider le bassin.
   - Faction : les pontiers de Bérangère.
4. **Monstres.**
   - **Haleur** : lit la Pente et nage (Eau à 1, Vanne à 1). Faible contre le Treuil, qui le tire hors de l'eau. 3 sépia.
   - **Échassiers** : Fuyards ou Rancuniers, selon le choix du monde 2.
   - **Chaîneur d'élite** : plus de PV, Chaussée à 0,5.
   - **Jalonneur** : vise la Grande Vanne.
5. **Mécanique unique : les Écluses.** Un carillon sonne une vague à l'avance. Les Vannes s'ouvrent (Eau) ou se ferment (Sol) [T], selon un horaire donné en RON.
6. **Événements scriptés.**
   - 5-2 : la Crue du monde 1 revient (troisième règle simultanée).
   - 5-3 : Corentin ouvre une vanne hors horaire, pour une seule case [T].
7. **Boss : l'Éclusier** (Corentin, sur la barge-porte de la Grande Vanne).
   - **Phase 1 :** il ouvre et ferme les vannes hors horaire [T], avec une annonce raccourcie mais toujours présente.
   - **Phase 2 :** toutes les légions prennent la Grande Vanne comme destination [S] ; c'est un siège.
   - **Phase 3, le Doute :** si les déserteurs ont été escortés au monde 2, la moitié des Échassiers passent en profil Fuyards [P] et Corentin hésite à voix haute. Point faible : le rouage maître de la barge. Le Treuil l'expose.
8. **Progression.** Tour Treuil (tire une unité d'une case, ce qui change son chemin). Clé de la Chancellerie.
9. **Narration.**
   - **Objectif :** garder la Grande Vanne.
   - **Rebondissement :** Corentin ne veut pas noyer la ville ; c'est Bérangère qui le propose. Ouvrir la Grande Vanne en avance (Dater) noierait la légion, avec les maisons de la ville basse.
   - **Choix :**
     - *Noyer* : la phase 1 du boss est sautée et la ville basse est perdue. Bérangère survit et ne pardonne pas.
     - *Ne pas noyer* : combat complet. Corentin, vaincu et vivant, peut rejoindre Nève au monde 6.
   - Répliques :
     - Bérangère : « Je sais combien de seaux contient chaque rue. C'est mon métier. Ne me demande pas si c'est bien. »
     - Corentin : « Tu m'as appris que la carte a raison. Montre-moi qu'elle peut avoir tort, et je pose ma chaîne. »
     - Nève : « Elle peut avoir tort. J'en suis la preuve. »
10. **Direction artistique, musique, SFX.** Brique sombre, eau vert-bouteille, lanternes. La musique est un orgue de barbarie lent, avec des cloches d'écluse sur les temps faibles. SFX : crémaillères, eau qui se rue, cordes de halage.
11. **Boucles, transition et ouverture.**
    - Boucle : trois règles en même temps.
    - Transition : la clé de la Chancellerie est gravée du même motif que la borne du monde 1.
    - Ouverture : Ilse écrit : « Viens. Il reste une case blanche, et elle porte ton nom. »

### Monde 6 : Le Plan-Relief

1. **Thème et ambiance.** Le droit de tracer. Majesté froide, silence de bibliothèque et voix qui récitent des numéros de parcelle.
2. **Biome.** Une vallée entière sculptée en maquette géante sous une verrière : la carte des Marches, à l'échelle d'un pas pour une lieue. Case propre au monde : le **Cadastré** (coût 0,5 pour les impériaux, 3 ou plus pour les autres peuples).
3. **Carte mentale.**
   - Points d'intérêt : les Archives de plain-pied, la Salle des Échelles, le Plan-Relief, la Case blanche.
   - Zone verrouillée : la **Salle de Vaux**, où l'on comprend la mort de la fille d'Ilse. On n'y accède qu'en post-game, avec Signer.
   - Faction : les greffiers, et les alliés réunis selon les choix.
4. **Monstres.**
   - **Greffier** : lit la Pente et ne marche bien que sur le Cadastré. 2 sépia.
   - **Légat** : élite, lit le But (la Table de Nève). 6 sépia.
   - Toutes les espèces précédentes reviennent, en mélange.
5. **Mécanique unique : la Mise au net.** Le grincement d'une presse l'annonce. Pendant une vague, toutes les espèces prennent le profil impérial [P] : la « lecture unique ». C'est la thèse de l'Empire rendue jouable : quand tout le monde lit pareil, les détours de Nève ne servent plus à rien. Les Écluses et le Gel restent actifs : trois règles simultanées.
6. **Événements scriptés.**
   - 6-2 : au fil des vagues, les cases du bord deviennent Cadastré, anneau après anneau [T].
   - 6-3 : selon les choix, Corentin, Hesse ou Yaro apparaissent comme sources alliées ou hostiles [S].
7. **Boss : le Dernier Cadastre** (Ilse à la presse du Registre).
   - **Phase 1, Lecture unique :** Mise au net permanente [P]. Il faut tenir par la seule puissance des tours.
   - **Phase 2, le Registre :** le Cadastré se referme sur le centre [T]. Ilse respecte le Droit de Passage : « Même moi. »
   - **Phase 3, la Case blanche :** toutes les légions convergent vers une case vide au centre, le village effacé de Nève [S]. Point faible : la matrice de sceau qu'Ilse tient à la main, exposée seulement quand la source est active.
8. **Progression.** Pouvoir **Signer** : accès à l'éditeur de niveaux. Il est débloqué par toutes les fins, mais sa forme dépend de la fin obtenue.
9. **Narration.**
   - **Objectif :** atteindre la presse.
   - **Rebondissement :** sceller le Cadastre exige une signature, et le nom du signataire s'efface de toutes les mémoires. Ilse a toujours voulu que ce soit elle, pour que personne ne se souvienne de la cartographe qui a figé le monde. Elle offre à Nève de prendre sa place, ou de la regarder faire.
   - **Choix final :** voir section 10.
   - Répliques :
     - Ilse : « Je ne veux pas gagner, Nève. Je veux que plus personne n'ait à gagner. »
     - Oudine, si le Rôle est riche : « Il y a des noms ici que même toi tu ne m'as pas pris. »
     - Nève : « Qui a le droit de tracer ? Je ne sais pas. Je sais seulement que ce n'est plus moi seule. »
10. **Direction artistique, musique, SFX.** Bois ciré, verrière et lumière zénithale ; les modèles des mondes précédents sont réutilisés à petite échelle comme maquettes. La musique se fait au clavecin mécanique, avec un chœur qui chante des numéros de parcelles. Le thème boiteux de Nève y est « corrigé » en marche régulière, puis il se défait selon la fin. SFX : presse, tampons en rafale, papier qui se tend.
11. **Boucles, transition et ouverture.**
    - Boucle : synthèse des trois lectures et des trois règles.
    - Transition : la fin choisie.
    - Ouverture : le post-game et la Salle de Vaux.

---

## 7. Progression : les 5 pouvoirs et l'arbre de compétences

| Palier | Pouvoir (nom en jeu) | Effet | Opération | Obtenu | Coût | Rouvre |
|---|---|---|---|---|---|---|
| 1 | **Tracer** | Poser tours et aménagements | [T] | Prologue | Sépia | — |
| 2 | **Gratter** | Effacer un élément du décor (éboulis, borne, palissade) qui redevient du Sol | [T] | Boss du monde 1 | Sépia | Le Moulin noyé, la Saline engloutie |
| 3 | **Renommer** | Changer le type d'une case (Sol → Eau, etc.) | [T] | Boss du monde 2 | Sépia et un Nom si la case est nommée | Le Refuge enseveli, l'intérieur du Moulin noyé |
| 4 | **Dater** | Suspendre ou déclencher en avance l'événement de monde, une fois par niveau | [T] ou [P] (celle de l'événement) | Boss du monde 4 | Sépia | La Salle des Premiers Plans, la Ville d'en dessous |
| 5 | **Signer** | Accès à l'éditeur de niveaux (données RON) | Les trois | Fin du monde 6 | — | La Salle de Vaux et les feuillets partagés |

**Arbre de compétences.** Il compte 3 branches de 5 nœuds. Les nœuds coûtent 1, 2, 3, 4 et 6 sceaux, soit 16 par branche et **48 au total**. Chacun des 24 niveaux rapporte 2 sceaux : un pour la victoire, un pour le défi de lecture, par exemple « aucun Fuyard blessé » ou « finir sans Gratter ». Ce sont des valeurs choisies pour être cohérentes.

| Nœud | Le Trait (tours) | Le Relief (aménagements) | La Légende (lecture et économie) |
|---|---|---|---|
| 1 (1 sceau) | Mine de plomb : arbalète −10 % | Boue profonde : +1 au coût de la boue pour les espèces hostiles [P] | Lecture fine : le trait de crayon affiche les coûts |
| 2 (2) | Équerre : +1 de portée pour deux tours alignées | Ronces-mères : les ronces gagnent une case adjacente par vague, une fois [T] | Sceaux brisés : +15 % de sépia |
| 3 (3) | Contre-visée : mortier +25 % contre les boss | Fascines : barricade −30 % | Ombre portée : les tours comptent ×1,5 pour les lecteurs d'Ombre [P] |
| 4 (4) | Double trait : deux aménagements en un clic | Grattoir fin : Gratter coûte la moitié | Leurre : une fausse borne attire les voleurs [S] |
| 5 (6) | Encre fixe : une tour par niveau survit à l'événement | Relief vivant : les aménagements effacés par un événement reviennent après lui [T] | Nom gardé : une fois par niveau, Renommer ne coûte pas de Nom |

**Boucle Metroidvania.** Chaque monde cache un feuillet verrouillé, qui forme un niveau bonus de 12 vagues avec une règle inédite. Le monde 1 en a deux étapes. Les pouvoirs des paliers 2 à 4 les rouvrent. La carte du monde (le *Grand Plan*) montre les feuillets en blanc hachuré tant qu'ils restent fermés.

---

## 8. Bestiaire transversal (ligne de coûts par espèce)

Pour les lecteurs de la Pente, le chemin dépend du terrain. Pour les lecteurs de l'Ombre, chaque case praticable coûte 1 plus le nombre de tours qui la couvrent, et les cases infranchissables restent ∞. Pour les lecteurs du But, la source de Dijkstra est une cible secondaire. PV relatifs : 1,0 correspond au Chaîneur, avec 100 PV au monde 1, vague 1.

| Espèce | Lecture | Sol | Boue | Ronces | Eau | Barricade | Roc | Types spéciaux | PV rel. | Butin (sépia) | Faiblesse |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Chaîneur | Pente | 1 | 3 | 4 | ∞ | ∞ | ∞ | Gué 2 · Glace 2 · Éboulis 3 · Dune 2 · Vanne selon l'état · Chaussée 0,5 · Cadastré 0,5 | 1,0 | 2 | Arbalète |
| Jalonneur | But (borne ou cible) | 1 | 3 | 4 | ∞ | ∞ | ∞ | comme le Chaîneur | 0,7 | 2 | Harpon |
| Échassier conscrit | Pente | 1 | 1 | 5 | 2 | ∞ | ∞ | Gué 1 · Glace 4 · Dune 3 | 1,2 | 3 | Ronces, mortier |
| Échassier rancunier (monde 5) | Pente | 1 | 1 | 3 | 2 | ∞ | ∞ | Gué 1 · Vanne 1 | 1,4 | 3 | Mortier |
| Fuyard (non hostile) | But (Refuge) | 1 | 2 | 3 | ∞ | ∞ | ∞ | Gué 2 | — | des Noms | — |
| Harde | Ombre | 1 + tours | 1 + tours | 1 + tours | ∞ | ∞ | ∞ | Glace : 1 + tours | 2,0 | 4 | Placement des tours |
| Gens du Col | Pente | 1 | 2 | 3 | ∞ | ∞ | 3 | Glace 1 | 1,3 | 3 | Lanterne |
| Convoi de Hesse (allié) | But (sortie) | 1 | 2 | 3 | ∞ | ∞ | ∞ | Glace 2 | — | — | — |
| Taupier | Pente | 2 | 2 | 1 | ∞ | 3 | 2 | Éboulis 1 · Gouffre ∞ | 1,5 | 4 | Sonde |
| Étayeur | Ombre | 1 + tours | 1 + tours | 1 + tours | ∞ | ∞ | ∞ | Éboulis : 1 + tours | 1,8 | 4 | Mortier |
| Sablier | Pente | 2 | 4 | 3 | ∞ | ∞ | ∞ | Dune 1 · Saline 1 | 1,2 | 3 | Héliographe |
| Caravanier rancunier | But (puits) | 2 | 4 | 3 | ∞ | ∞ | ∞ | Dune 1 | 1,0 | 3 | Harpon |
| Haleur | Pente | 1 | 2 | 4 | 1 | ∞ | ∞ | Vanne 1 | 1,4 | 3 | Treuil |
| Greffier | Pente | 3 | 4 | 6 | ∞ | ∞ | ∞ | Cadastré 0,5 | 1,0 | 2 | Tout, sauf sur le Cadastré |
| Légat | But (Table de Nève) | 1 | 2 | 3 | ∞ | ∞ | ∞ | Cadastré 0,5 | 3,0 | 6 | Harpon puis mortier |

**Courbe de PV** (formule conservée : monde *n* = PV du monde 1 × 1,5^(n−1), puis ×1,12^(k−1) à la vague *k*). Choix de valeur : 100 PV pour le Chaîneur au monde 1.

| Monde | PV de base (vague 1) | PV à la vague 12 (×3,48) |
|---|---|---|
| 1 | 100 | ≈ 348 |
| 2 | 150 | ≈ 522 |
| 3 | 225 | ≈ 783 |
| 4 | 338 | ≈ 1 174 |
| 5 | 506 | ≈ 1 761 |
| 6 | 759 | ≈ 2 642 |

Économie de départ, à valider en test : 120 sépia au début de chaque niveau. Barricade 10, boue 15, ronces 20, arbalète 50, harpon 70, mortier 90.

---

## 9. Mécaniques transverses

| Mécanique | Habillage | Opération moteur |
|---|---|---|
| Pose d'un aménagement ou d'une tour | Encrer un trait | [T] |
| Aperçu du détour | Trait de crayon en pointillé | Calcul seul |
| Droit de Passage | La carte close est rejetée | Vérification de Dijkstra par espèce terrestre, état actuel et état annoncé |
| Lecture de la Pente | Profil de terrain | [P] |
| Lecture de l'Ombre | 1 + tours couvrant la case | [P], recalcul à chaque pose ou destruction |
| Lecture du But | Refuge, borne, puits, sortie | [S] |
| Événement de monde | La terre hésite (crue, gel, effondrements, dunes, écluses, mise au net) | [T], ou [P] pour la Mise au net |
| Annonce une vague à l'avance | Frisson des hachures et indice sonore | Lecture des données, aucune opération |
| Gratter | Effacer un élément du décor | [T] |
| Renommer et prix du nom | La case perd son nom, qui sort du Rôle | [T] et un compteur de progression |
| Dater | Suspendre ou avancer l'événement | [T] ou [P] |
| Défection en bataille | Les conscrits deviennent des Fuyards | [P] |
| Leurre | Fausse borne | [S] |
| Escorte | Convoi vers la sortie | [S] |
| Fuyards qui rapportent des Noms | Récompense des non-hostiles | Exception [X], donnée de progression hors terrain |
| Calque (F1) | Exercice d'apprenti, accessibilité | Affichage |
| Tension de la feuille | Jauge cachée : si le joueur domine, la feuille « se tend » et la Chancellerie envoie des vagues plus mixtes. S'il peine, la feuille « gondole » et les sceaux rendent +10 % de sépia | Exception [X], tables d'apparition et de butin |
| Combat (PV, dégâts, harpon, treuil) | Tours | Exception [X], couche de combat |
| Narration en marge | Le carnet de Nève réagit aux événements | Déclencheurs de texte, sans opération |

**Exceptions signalées.** Il y en a trois. La couche de combat (PV, dégâts, effets de tir) n'a jamais fait partie des trois opérations de terrain. La Tension de la feuille agit sur les tables d'apparition et de butin. La récompense des Fuyards en Noms est une donnée de progression. Toutes les mécaniques de **terrain et de chemin**, y compris les phases de boss et les événements scriptés, se ramènent bien à [T], [P] ou [S].

**Narration en direct et budget.** *Bastion* a montré la force d'un narrateur qui réagit à presque chaque action du joueur ; son auteur, Greg Kasavin, le voulait au rythme du joueur, sans jamais le sortir du jeu.\[24\]\[25\] Mais selon Supergiant Games, *Bastion* a enregistré environ 3 000 répliques de narration avec Logan Cunningham. *Hachures* garde l'idée et coupe le coût : le carnet de Nève s'écrit **à la main dans la marge**, en texte, sans voix pour le MVP. Budget estimé : environ 80 annotations réactives, 20 répliques de boss et 15 répliques de choix par monde. Avec le prologue et les fins, cela fait **environ 750 lignes**. Une version voix pourrait se limiter à une quarantaine de lignes clés.

**Réutilisation d'assets.** Toutes les espèces partagent un même gabarit humanoïde low-poly. On les distingue par la silhouette (échasses pour les Échassiers, paniers pour les Haleurs) et par une palette par peuple. Les boss sont des assemblages de props existants (chariot, roues, mâts, lentilles). Le monde 6 réutilise les décors des mondes 1 à 5 à petite échelle. Les hachures sont un seul décalque procédural, teinté selon le monde.

---

## 10. Fins multiples, post-game et rejouabilité

Deux variables cachées décident de la fin : le **Rôle des Noms** (de 0 à 6 Noms majeurs, un par monde, gagnés ou perdus par les choix et l'usage de Renommer) et la **Parole donnée** (escorte au monde 2, refus de noyer au monde 5). Suivant *Pentiment*, il n'y a pas de bonne réponse : Josh Sawyer a déclaré à PC Gamer « for this to be compelling in the way that I want it to be, there cannot be a right answer ». *The Banner Saga* montre comment des systèmes enchaînés (ressources, moral, morts) rendent les pertes personnelles.\[26\] Ici, chaque Nom perdu coûte une ligne de dialogue et un allié possible.

| Fin | Condition | Ce qui se passe | Conséquence en post-game |
|---|---|---|---|
| **Le Trait unique** | Nève signe à la place d'Ilse | Les Marches se figent : plus de crues, plus de détours. Personne ne se souvient de Nève, et le Rôle devient un plan | L'éditeur Signer est disponible, mais les événements de monde y sont désactivés. La paix est réelle et la terre est muette |
| **La Feuille brûlée** | Nève brûle le Cadastre | Plus aucune carte ne fait foi. L'Empire se défait en provinces et la terre hésite partout | Mode « Terre instable » : deux événements par niveau |
| **La Légende commune** | Au moins 4 Noms, et l'escorte ou le refus de noyer | Nève propose une carte en calques : chaque peuple trace sa propre couche de lecture. Ilse ne signe pas. Nève dépose sa plume et perd le pouvoir de tracer | L'éditeur complet, avec export et import des feuillets en RON pour la communauté |
| **Oubrenne** (secrète) | 6 Noms et victoire en Héroïque | La Case blanche n'est pas nommée par Nève mais par Oudine, qui l'avait gardée dans le Rôle depuis toujours | Dernière annotation : « Oubrenne. Je n'aurais pas su l'écrire. » |

Pour Corentin, trois issues : allié au monde 6, reddition silencieuse, ou mort sous la presse si la ville a été noyée.

**Post-game (5 h et plus, valeurs choisies) :**
- 7 feuillets verrouillés, environ 20 min chacun, soit environ 2 h 20 ;
- le mode Héroïque, où les PV sont multipliés par 1,25, l'annonce des événements arrive une demi-vague plus tard et la Tension est plus étroite ;
- 12 défis du Géomètre, définis entièrement en RON (par exemple « aucune barricade » ou « seulement l'Ombre ») ;
- la Salle de Vaux ;
- l'éditeur.

**Rejouabilité :** quatre fins, cinq choix aux conséquences jouables, 48 sceaux, et des niveaux partagés.

---

## 11. Trois pistes de DLC (chacune apporte une nouvelle opération moteur)

| DLC | Pitch | Nouvelle opération | Exemple |
|---|---|---|---|
| **Les Faussaires** | Des ateliers clandestins vendent de fausses cartes aux légions | **[V] Perception** : une espèce lit un type de case différent du type réel | Un marais « dessiné » en Sol attire une colonne qui s'y enlise |
| **Les Ponts de corde** | Des peuples des gorges relient des points éloignés | **[A] Ajouter une arête** : une connexion entre deux cases non adjacentes | Tunnel, téléphérique, passage secret, avec le Droit de Passage recalculé sur le graphe |
| **Les Vents** | Une côte où la pente dépend de la direction | **[D] Coût directionnel** : le coût d'une case dépend du sens d'entrée | Monter une falaise coûte 4, la descendre coûte 1 ; les courants de marée suivent la même logique |

---

## 12. Prompt de continuation réutilisable

> Tu es narrative designer sur **Hachures**, tower defense 3D low-poly (Rust + Bevy, WebAssembly, souris). Crée un **monde n° [N]** cohérent avec cette bible.
> **Contraintes fixes :** 3 niveaux de 12 vagues et 1 boss. Toute mécanique de terrain se ramène à [T] changer le type d'une case, [P] changer un profil de coûts, [S] ajouter une source de Dijkstra (signale toute exception). Le Droit de Passage interdit de fermer tous les chemins d'une espèce terrestre, y compris dans l'état annoncé de l'événement. Les trois lectures sont la Pente, l'Ombre (1 + tours) et le But. Une ressource : la sépia. Un événement de monde annoncé une vague avant par un frisson des hachures. PV = 100 × 1,5^(N−1) × 1,12^(vague−1). Le nombre de règles simultanées est [R].
> **Lore :** l'encre de terre et le témoin ; le prix du nom (Renommer efface un Nom du Rôle) ; la terre hésite entre des cartes rivales ; l'Empire du Registre, Ilse Damerval, Corentin Vass, Nève Achard (voix sèche, mesures, ironie).
> **Livrables :** les 11 points (thème, biome, carte mentale avec zone verrouillée et faction, 3 ou 4 espèces avec leur ligne de coûts complète sur Sol, Boue, Ronces, Eau, Barricade, Roc et un type nouveau, mécanique unique avec opération, événements scriptés, boss en 3 phases avec opération et point faible, progression, narration avec objectif, rebondissement préparé, choix non évident à double conséquence et 3 répliques de voix distinctes, DA, musique, SFX, boucles, transition et ouverture). Ajoute un **extrait RON** sur ce modèle :
> `Species(id: "haleur", lecture: Pente, couts: { Sol: 1.0, Boue: 2.0, Ronces: 4.0, Eau: 1.0, Barricade: Inf, Roc: Inf, Vanne: 1.0 }, pv_rel: 1.4, butin: 3)`
> **Interdits :** élu, prophétie, seigneur des ténèbres, orques, amnésie de commodité, mentor sacrifié pour faire pleurer, violence graphique, et peuples réels plaqués en exotisme. Ne réutilise pas la rotation de tuiles. Termine sur une ouverture.

---

## Plan du MVP (chapitre 1, 8 semaines, valeurs indicatives)

| Semaine | Livrable |
|---|---|
| 1 | Grille, types de cases et profils en RON, flow fields par espèce, Droit de Passage |
| 2 | Pose, trait de crayon, sépia, Calque F1 |
| 3 | Trois aménagements, arbalète, mortier, harpon ; Chaîneur, Échassier, Jalonneur, Fuyard |
| 4 | Crue et annonce, prise en compte de l'état annoncé |
| 5 | Niveaux 1-1 et 1-2 (données seules) |
| 6 | Niveau 1-3 et Grande Chaîne (trois phases, en données) |
| 7 | Annotations en marge, choix du Refuge, sauvegarde, sceaux |
| 8 | Équilibrage, Tension de la feuille, build WASM, page de présentation |

---

## Note d'auteur

**Choix narratifs.**
- J'ai remplacé « le méchant empire » par une **thèse défendable**. Les cartes rivales tuent vraiment, et Ilse a perdu sa fille à cause d'elles. Le joueur doit donc se battre contre une idée qui n'a pas tort, ce qui rend la question « qui a le droit de tracer ? » sérieuse plutôt que rhétorique.
- Le **prix du nom** relie la mécanique au thème. Le pouvoir le plus utile du jeu, Renommer, est celui qui ressemble le plus à l'Empire. Plus le joueur optimise, plus il colonise : c'est l'harmonie ludonarrative recherchée.
- Le **Droit de Passage** est la seule loi que respectent les deux camps. C'est le terrain moral commun, et Ilse le revendique : « Même moi. »
- Les ancrages historiques (cadastre de 1807, Ordnance Survey et anglicisation des noms, Great Trigonometrical Survey, mythe des 80 % de frontières droites corrigé par l'étude de 2025, arrêt *Mabo*) nourrissent les détails sans jamais être cités en jeu. Le jeu ne fait pas la leçon : il fait payer.

**Risques.**
1. Dire « monstres » pour des conscrits peut mettre mal à l'aise. C'est voulu, mais il faut le doser : les annotations nomment parfois un soldat, jamais tous.
2. La fin « Légende commune » pourrait passer pour la « bonne » fin. Elle coûte donc à Nève son pouvoir, et le post-game « Terre instable » de la fin brûlée doit rester désirable.
3. La proximité avec *Carto* : il faut bannir toute tuile qui pivote.
4. Le budget de texte : 750 lignes, c'est déjà beaucoup pour un portfolio. Si l'on doit couper, on coupe d'abord les répliques des légats.
5. La disponibilité du titre n'a été vérifiée que sur Steam, itch.io et le web, pas dans les bases de marques.

**Pistes.**
- Un mode « Carnet » qui exporte les annotations de la partie comme un récit.
- Un Rôle des Noms consultable, où chaque nom gardé a deux lignes d'histoire.
- La musique adaptative du thème « corrigé », qui peut devenir la signature sonore du projet.
- Une démo du chapitre 1 sur itch.io, jouable dans le navigateur.

**Clichés évités (liste de travail).** Élu et prophétie ; seigneur des ténèbres sans raisons ; races monolithiques (orcs) ; mentor qui meurt pour motiver ; amnésie de commodité (ici, l'oubli est une règle payée et visible) ; grimdark gratuit (viol, torture) ; noms à apostrophes ; « ancien mal qui s'éveille » ; empire purement maléfique ; peuples réels déguisés en exotisme ; magie sans coût.

---

## Checklist qualité

| Critère | Statut | Où on le vérifie |
|---|---|---|
| Accroche | ✅ | Prologue : un nom gratté, un village qui s'efface, « Je ne me souviens plus du nom. C'est ça, le prix. » |
| Personnages | ✅ | Nève, Ilse et Corentin ont chacun un désir, un obstacle et une faille ; voix distinctes pour Oudine, Hesse, Pellan, Yaro et Bérangère |
| Conflit | ✅ | Extérieur (les légions), moral (l'Empire a un argument), intime (Nève a signé) |
| Rythme | ✅ | Une lecture ou une règle nouvelle par monde (1, 1, 2, 2, 3, 3) ; boss en 3 phases ; un choix par monde |
| Cohérence | ✅ | Trois opérations, exceptions signalées, Droit de Passage valable pour tous, géographie nord-ouest → sud → capitale |
| Originalité | ✅ | La cartographie comme coût, les catastrophes comme querelles de cartes, la Mise au net (« lecture unique ») jouable |
| Émotion | ✅ | Oubrenne, Oudine et le Rôle, Ilse et sa fille, Corentin qui veut être vu |
| Thème | ✅ | Qui a le droit de tracer ? Aucune fin ne répond sans payer |
| Style | ✅ | Sensoriel, retenu, sans jargon inutile : le vocabulaire d'arpentage est toujours expliqué par l'usage |
| Pay-off | ✅ | Écriture sur la borne (monde 1) → Galerie Signée (monde 3) → Case blanche (monde 6). Berceuse (monde 1) → déserteurs (monde 2) → défection (monde 5). La crue suit les bornes (monde 1) → révélation (monde 3) |

## Sources

1. [Hachishaku on Steam](https://store.steampowered.com/app/3572580/Hachishaku/)
2. [Battlefield 6 Twitch Drops — 7 Rewards Available](https://twitchdrops.app/game/battlefield-6)
3. [V1.1 Released, Plus Markdown File and Playingcards Config File! - Cadastre: A Land Survey RPG by To Be Resolved](https://dadstep.itch.io/cadastre-a-land-survey-rpg/devlog/1640784/v11-released-plus-markdown-file-and-playingcards-config-file)
4. [Terra Nullius](https://www.taylorfrancis.com/chapters/edit/10.4324/9781003197805-17/terra-nullius-conor-leggott)
5. [Terra Nullius by SunCraft](https://reddteam.itch.io/terra-nullius)
6. [Dinkum and the erasure of First Nations People in Australian video games](https://www.gameshub.com/news/opinions-analysis/dinkum-and-erasure-of-first-nations-people-in-australian-video-games-25548/)
7. [terra nullius](https://gnitolo.itch.io/terra-nullius)
8. [Prestations](https://orographe-illustration.fr/projets.html)
9. [A quick review for Carto by Sunhead](https://medium.com/@cozyquarium/a-quick-review-for-carto-by-sunhead-6f577f20ffcc)
10. [Carto on Steam](https://store.steampowered.com/app/1172450/Carto/)
11. [Carto Review: Carto Review](https://gameinformer.com/review/carto/carto-review-putting-the-pieces-together)
12. [Carto — Game Raven Review](https://www.gameravenreview.com/carto)
13. [Aide à la recherche - Cadastre napoléonien](https://archives.finistere.fr/rechercher/aides-la-recherche/aide-la-recherche-cadastre-napoleonien)
14. [Le cadastre dit « napoléonien » - Fer et savoir-faire](https://www.feretsavoirfaire.org/le-cadastre-dit-napoleonien/)
15. [Le Cadastre Parcellaire de 1807](https://cadastre.pagesperso-orange.fr/napo2.htm)
16. [Cadastre Napoléonien - La Croix Comtesse](https://www.lacroixcomtesse.fr/cadastre-napoleonien/)
17. [Translations - 19th Century Ordnance Survey of Ireland](https://sites.google.com/a/cord.edu/translations/translations-home/contextual-considerations/19th-century-ordnance-survey-of-ireland)
18. [Putting Ireland on the map — Paul Clements on 200 years of the Ordnance Survey in Ireland](https://www.irishtimes.com/opinion/an-irish-diary/2024/01/01/putting-ireland-on-the-map-paul-clements-on-200-years-of-the-ordnance-survey-in-ireland/)
19. [Mapping an empire : the geographical construction of British India, 1765-1843 : Edney, Matthew H : Free Download, Borrow, and Streaming : Internet Archive](https://archive.org/details/mappingempiregeo0000edne)
20. [Berlin Conference](https://openborders.info/blog/tag/berlin-conference/)
21. [Endogenous Colonial Borders: Precolonial States and Geography in the Partition of Africa](https://www.cambridge.org/core/journals/american-political-science-review/article/endogenous-colonial-borders-precolonial-states-and-geography-in-the-partition-of-africa/132D6CBDE92946D14CCC64E59A94D3D2)
22. [Mabo decision](https://www.nma.gov.au/defining-moments/resources/mabo-decision)
23. [Mabo decision](https://digital-classroom.nma.gov.au/defining-moments/mabo-decision)
24. [Bastion Review - GameSpot](https://www.gamespot.com/reviews/bastion-review/1900-6416093/)
25. [Supergiant's Greg Kasavin explains the process behind Bastion's narration](https://www.pcgamer.com/supergiants-greg-kasavin-explains-the-process-behind-bastions-narration/)
26. [Unstitching the Banner Saga — Wireframe Magazine](https://wireframe.raspberrypi.com/articles/unstitching-the-banner-saga)
