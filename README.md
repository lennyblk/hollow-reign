# Hollow Reign

Hollow Reign est un jeu de role en terminal, entierement en Rust, inspire des soulslike. Vous incarnez un errant qui traverse six zones maudites pour atteindre et tuer Le Roi Creux, dont le regme maintient le monde dans un cycle de mort et de corruption.

Le jeu tourne dans un terminal Windows et utilise le clavier pour naviguer, combattre et dialoguer. Il n'y a pas d'interface graphique : tout est rendu en ASCII avec de la couleur via crossterm.

---

## Fonctionnalites

- Trois classes jouables avec des statistiques distinctes (Chevalier, Mage, Voleur)
- Six zones avec leurs propres ennemis, elements et boss
- Systeme de combat au tour par tour avec defi de frappe (typing challenge)
- Points de Grace pour se soigner, gerer ses statistiques et voyager rapidement
- Systeme d'inventaire avec armes, armures et consommables
- Quatre PNJ avec des quetes, dialogues et recompenses
- Sauvegarde automatique a chaque point de Grace
- Musique de fond sur l'ecran titre
- Art ASCII unique pour chaque ennemi et chaque PNJ

---

## Zones

Le jeu se deroule dans six zones traversees dans l'ordre. Chaque zone a un element dominant qui influe sur les ennemis rencontres.

| Zone | Element(s) | Ennemis notables | Boss |
|---|---|---|---|
| Ashfeld | Saignement | Creux Errant, Chef des Creux | - |
| Gravemoor | Saignement, Poison | Mort-Rampant, Garde de Crypte | Gardien du Tombeau |
| Rotwood | Poison, Rot | Spore-Marcheur, Champignon Ambulant | L'Excroissance |
| The Cinders | Feu | Garde Igne, Capitaine des Braises | Seigneur du Feu |
| Frostveil | Glace, Foudre | Sentinelle Gelee, Chasseur des Glaces | La Reine Tonnerre |
| The Void | Rot, Foudre | Vestige Corrompu, Ombre Electrique | Le Roi Creux |

Les zones 1 et 2 (Ashfeld et Gravemoor) peuvent faire apparaitre jusqu'a trois ennemis en meme temps. A partir de la zone 3, les combats sont en un contre un.

---

## Classes

Au lancement d'une nouvelle partie, vous choisissez parmi trois classes. Chaque classe a des statistiques de base differentes qui influencent vos points de vie, vos degats et vos capacites.

**Chevalier**
- Statistiques equilibrees avec un fort Vigueur et Force
- Vigor 14 / Strength 14 / Dexterity 13 / Intelligence 9 / Mind 9 / Faith 9 / Arcane 7
- Bon pour les debutants

**Mage**
- Intelligence et Mind eleves, Force faible
- Vigor 9 / Intelligence 20 / Mind 14 / Dexterity 12 / Arcane 9 / Faith 7 / Strength 5
- Oriente vers les degats magiques et le Faith scaling

**Voleur**
- Dexterity maximale, stats polyvalentes
- Vigor 10 / Dexterity 20 / Arcane 11 / Mind 11 / Intelligence 11 / Faith 8 / Strength 10
- Favorise les armes a mise a l'echelle sur la dexterite

---

## Systeme de combat

Le combat est en tour par tour. A chaque tour vous choisissez une action parmi :

- **Attaquer** : un defi de frappe apparait (une serie de lettres a taper). La vitesse et la precision determinent les degats. Une frappe parfaite multiplie les degats.
- **Utiliser un item** : utiliser un consommable (soin, antidote, etc.) ou declencher la capacite d'une arme.
- **Fuir** : tenter de quitter le combat (pas toujours possible).

Les ennemis ont des elements associes (Feu, Glace, Poison, Rot, Saignement, Foudre). Certains elements appliquent des effets de statut sur la duree.

Les boss sont des combats uniques sans possibilite de respawn des ennemis.

---

## Points de Grace

Les Points de Grace sont des sanctuaires repandus dans chaque zone. En vous y reposant :

- Vos points de vie sont entierement restaures
- Vos Fioles d'Estus (soins) sont rechargees
- La partie est sauvegardee automatiquement
- Vous pouvez depenser vos Ames pour ameliorer vos statistiques
- Vous pouvez voyager vers d'autres Points de Grace deja visites

---

## PNJ et quetes

Quatre PNJ sont repartis dans les zones. Chacun propose une quete avec une recompense.

| PNJ | Zone | Quete |
|---|---|---|
| Edric le Creux | Ashfeld | Recuperer une vieille lame dans la tour de guet |
| Soeur Vael | Gravemoor | Tuer le Gardien du Tombeau et rapporter sa lame |
| Osryn le Chevalier Braise | The Cinders | Tuer le Seigneur du Feu et rapporter l'Ashfall Katana |
| Le Pelerin Pale | Frostveil | Partage d'informations sur le Roi Creux (pas de quete) |

---

## Controles

### Navigation
| Touche | Action |
|---|---|
| 1 2 3 | Se deplacer entre les lieux |
| Entree | Confirmer / Interagir |
| I | Ouvrir l'inventaire |
| Echap | Retour / Annuler |

### Combat
| Touche | Action |
|---|---|
| 1 / 2 / 3 | Choisir une action |
| Lettres | Taper le defi de frappe |
| Echap | Fuir le combat |

### Menus
| Touche | Action |
|---|---|
| Haut / Bas | Naviguer |
| Entree | Confirmer |
| Echap | Fermer / Refuser |

---

## Prerequis pour compiler

- [Rust](https://rustup.rs/) 1.85 ou superieur (edition 2024)
- Un terminal avec support des couleurs ANSI

Les prerequis varient selon le systeme d'exploitation.

---

## Compilation par plateforme

### Windows

Aucune dependance systeme supplementaire requise. Rust suffit.

```
cargo build --release
```

L'executable se trouve dans :

```
target\release\hollow-reign.exe
```

Lancer depuis PowerShell ou Windows Terminal :

```
.\target\release\hollow-reign.exe
```

Windows Terminal est recommande pour un meilleur rendu des couleurs et des caracteres Unicode.

---

### macOS

Aucune dependance supplementaire requise. CoreAudio est utilise automatiquement par rodio.

```
cargo build --release
```

L'executable se trouve dans :

```
target/release/hollow-reign
```

Lancer depuis le Terminal :

```
./target/release/hollow-reign
```

---

### Linux

rodio utilise ALSA pour l'audio. Il faut installer les headers de developpement ALSA avant de compiler.

Sur Debian / Ubuntu :

```
sudo apt install libasound2-dev
```

Sur Fedora / RHEL :

```
sudo dnf install alsa-lib-devel
```

Sur Arch :

```
sudo pacman -S alsa-lib
```

Ensuite :

```
cargo build --release
./target/release/hollow-reign
```

Si vous ne voulez pas de son, vous pouvez desactiver la feature audio en modifiant `Cargo.toml` :

```toml
rodio = { version = "0.19", default-features = false }
```

Dans ce cas la musique de titre ne jouera pas mais le jeu fonctionnera normalement.

---

## Lancer le jeu

**Note** : montez le volume avant de lancer le jeu, une musique joue sur l'ecran titre.

**Note** : une resolution de terminal d'au moins 120x35 est recommandee pour un affichage correct.

---

## Structure du code

```
src/
  main.rs          - Point d'entree, menu titre, boucle principale
  class.rs         - Classes jouables et statistiques de base
  player.rs        - Structure du joueur, inventaire, niveau
  stats.rs         - Calcul des statistiques et scaling
  combat.rs        - Logique du combat (tour par tour, effets)
  combat_ui.rs     - Rendu du combat en terminal
  enemy.rs         - Structure des ennemis et effets de statut
  enemy_catalog.rs - Catalogue de tous les ennemis avec leur art ASCII
  zone.rs          - Definition des zones et des lieux
  map.rs           - Structure du monde et navigation
  map_ui.rs        - Rendu de la carte et navigation
  grace.rs         - Logique des Points de Grace
  grace_ui.rs      - Menu des Points de Grace
  npc_ui.rs        - Dialogues et quetes des PNJ
  inventory_ui.rs  - Affichage et gestion de l'inventaire
  item.rs          - Types d'items et elements
  equipment.rs     - Gestion de l'equipement
  catalog.rs       - Catalogue de tous les items
  status.rs        - Effets de statut (poison, rot, saignement...)
  save.rs          - Sauvegarde et chargement (JSON)
  intro_ui.rs      - Selection de classe et lore d'intro
  navigation.rs    - Evenements de navigation entre les lieux
  typing.rs        - Systeme de defi de frappe en combat
  phrases.rs       - Textes de combat selon difficulte
  merchant_ui.rs   - Interface du marchand
```

---

## Dependances

| Crate | Usage |
|---|---|
| crossterm 0.27 | Rendu terminal, couleurs, saisie clavier |
| rodio 0.19 | Lecture audio (musique titre en WAV) |
| serde / serde_json | Sauvegarde et chargement en JSON |
| rand 0.8 | Aleatoire pour les spawns et le combat |
