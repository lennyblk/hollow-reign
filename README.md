# Hollow Reign

Hollow Reign is a terminal-based role-playing game written entirely in Rust, inspired by soulslike games. You play as a wanderer crossing six cursed zones to reach and kill the Hollow King, whose reign keeps the world locked in a cycle of death and corruption.

The game runs in a terminal and uses the keyboard to navigate, fight and talk to NPCs. There is no graphical interface: everything is rendered in ASCII with colors via crossterm.

---

## Features

- Three playable classes with distinct stats (Knight, Mage, Rogue)
- Six zones each with their own enemies, elements and boss
- Turn-based combat with a typing challenge system
- Grace Points to heal, level up stats and fast travel
- Inventory system with weapons, armor and consumables
- Four NPCs with quests, dialogue and rewards
- Auto-save at every Grace Point
- Background music on the title screen
- Unique ASCII art for every enemy and NPC

---

## Zones

The game takes place across six zones traversed in order. Each zone has a dominant element that affects the enemies encountered.

| Zone | Element(s) | Notable Enemies | Boss |
|---|---|---|---|
| Ashfeld | Bleed | Creux Errant, Chef des Creux | - |
| Gravemoor | Bleed, Poison | Mort-Rampant, Garde de Crypte | Gardien du Tombeau |
| Rotwood | Poison, Rot | Spore-Marcheur, Champignon Ambulant | L'Excroissance |
| The Cinders | Fire | Garde Igne, Capitaine des Braises | Seigneur du Feu |
| Frostveil | Ice, Lightning | Sentinelle Gelee, Chasseur des Glaces | La Reine Tonnerre |
| The Void | Rot, Lightning | Vestige Corrompu, Ombre Electrique | Le Roi Creux |

Zones 1 and 2 (Ashfeld and Gravemoor) can spawn up to three enemies at once. From zone 3 onward, all fights are one on one.

---

## Classes

At the start of a new game you choose one of three classes. Each class has different base stats that affect your HP, damage output and scaling.

**Knight**
- Balanced stats with high Vigor and Strength
- Vigor 14 / Strength 14 / Dexterity 13 / Intelligence 9 / Mind 9 / Faith 9 / Arcane 7
- Recommended for beginners

**Mage**
- High Intelligence and Mind, low Strength
- Vigor 9 / Intelligence 20 / Mind 14 / Dexterity 12 / Arcane 9 / Faith 7 / Strength 5
- Oriented toward magic damage and Faith scaling

**Rogue**
- Maximum Dexterity, versatile stats
- Vigor 10 / Dexterity 20 / Arcane 11 / Mind 11 / Intelligence 11 / Faith 8 / Strength 10
- Favors weapons that scale with Dexterity

---

## Combat System

Combat is turn-based. Each turn you choose one action:

- **Attack**: a typing challenge appears (a sequence of letters to type). Speed and accuracy determine damage output. A perfect run multiplies damage.
- **Use item**: use a consumable (heal, antidote, etc.) or trigger a weapon ability.
- **Flee**: attempt to escape the fight (not always possible).

Enemies have associated elements (Fire, Ice, Poison, Rot, Bleed, Lightning). Some elements apply status effects over time.

Bosses are unique fights with no enemy respawn.

---

## Grace Points

Grace Points are sanctuaries spread across each zone. Resting at one:

- Fully restores your HP
- Recharges your Estus Flasks
- Auto-saves the game
- Lets you spend Souls to level up your stats
- Lets you fast travel to other Grace Points you have already visited

---

## NPCs and Quests

Four NPCs are spread across the zones. Each one offers a quest with a reward.

| NPC | Zone | Quest |
|---|---|---|
| Edric le Creux | Ashfeld | Retrieve an old blade from the watchtower |
| Soeur Vael | Gravemoor | Kill the Gardien du Tombeau and bring back his blade |
| Osryn le Chevalier Braise | The Cinders | Kill the Seigneur du Feu and bring back the Ashfall Katana |
| Le Pelerin Pale | Frostveil | Shares information about the Hollow King (no quest) |

---

## Controls

### Navigation
| Key | Action |
|---|---|
| 1 2 3 | Move between locations |
| Enter | Confirm / Interact |
| I | Open inventory |
| Escape | Back / Cancel |

### Combat
| Key | Action |
|---|---|
| 1 / 2 / 3 | Choose an action |
| Letters | Type the typing challenge |
| Escape | Flee the fight |

### Menus
| Key | Action |
|---|---|
| Up / Down | Navigate |
| Enter | Confirm |
| Escape | Close / Decline |

---

## Download (Windows)

A prebuilt Windows executable is available on the releases page:

https://github.com/lennyblk/hollow-reign/releases/tag/v1

Download `hollow-reign.exe`, place it anywhere and run it from a terminal. No installation required.

---

## Build Requirements

- [Rust](https://rustup.rs/) 1.85 or higher (edition 2024)
- A terminal with ANSI color support

Requirements vary by operating system.

---

## Building by Platform

### Windows

No additional system dependencies required. Rust is enough.

```
cargo build --release
```

The executable will be at:

```
target\release\hollow-reign.exe
```

Run from PowerShell or Windows Terminal:

```
.\target\release\hollow-reign.exe
```

Windows Terminal is recommended for best color and Unicode rendering.

---

### macOS

No additional dependencies required. CoreAudio is used automatically by rodio.

```
cargo build --release
```

The executable will be at:

```
target/release/hollow-reign
```

Run from Terminal:

```
./target/release/hollow-reign
```

---

### Linux

rodio uses ALSA for audio. You need to install the ALSA development headers before compiling.

On Debian / Ubuntu:

```
sudo apt install libasound2-dev
```

On Fedora / RHEL:

```
sudo dnf install alsa-lib-devel
```

On Arch:

```
sudo pacman -S alsa-lib
```

Then:

```
cargo build --release
./target/release/hollow-reign
```

If you do not want audio, you can disable it in `Cargo.toml`:

```toml
rodio = { version = "0.19", default-features = false }
```

The title screen music will not play but the game will run normally.

---

## Notes

- Turn up your volume before launching, music plays on the title screen.
- A terminal resolution of at least 120x35 is recommended for correct display.

---

## Code Structure

```
src/
  main.rs          - Entry point, title menu, main loop
  class.rs         - Playable classes and base stats
  player.rs        - Player struct, inventory, leveling
  stats.rs         - Stat calculations and scaling
  combat.rs        - Combat logic (turn-based, status effects)
  combat_ui.rs     - Combat rendering in terminal
  enemy.rs         - Enemy struct and status effects
  enemy_catalog.rs - Catalog of all enemies with ASCII art
  zone.rs          - Zone and location definitions
  map.rs           - World structure and navigation
  map_ui.rs        - Map rendering and navigation
  grace.rs         - Grace Point logic
  grace_ui.rs      - Grace Point menu
  npc_ui.rs        - NPC dialogue and quests
  inventory_ui.rs  - Inventory display and management
  item.rs          - Item types and elements
  equipment.rs     - Equipment management
  catalog.rs       - Catalog of all items
  status.rs        - Status effects (poison, rot, bleed...)
  save.rs          - Save and load (JSON)
  intro_ui.rs      - Class selection and intro lore
  navigation.rs    - Navigation events between locations
  typing.rs        - Typing challenge system for combat
  phrases.rs       - Combat text by difficulty
  merchant_ui.rs   - Merchant interface
```

---

## Dependencies

| Crate | Usage |
|---|---|
| crossterm 0.27 | Terminal rendering, colors, keyboard input |
| rodio 0.19 | Audio playback (title WAV music) |
| serde / serde_json | Save and load in JSON |
| rand 0.8 | RNG for spawns and combat |
