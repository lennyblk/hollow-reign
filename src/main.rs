mod catalog;
mod class;
mod combat;
mod combat_ui;
mod enemy;
mod enemy_catalog;
mod equipment;
mod grace;
mod grace_ui;
mod inventory_ui;
mod item;
mod map;
mod map_ui;
mod navigation;
mod npc_ui;
mod phrases;
mod player;
mod save;
mod stats;
mod status;
mod typing;
mod zone;

use class::Class;
use combat_ui::{CombatResult, run_combat};
use enemy_catalog::open_chest;
use equipment::Equipment;
use inventory_ui::open_inventory;
use map::World;
use navigation::{NavigationEvent, NavigationState, run_navigation};
use player::Player;

fn main() {
    let world = World::new();

    let has_save = save::save_exists();
    let (mut player, mut state) = match start_menu(has_save) {
        StartChoice::Continue => save::load(&world).unwrap_or_else(|| new_game(&world)),
        StartChoice::NewGame => new_game(&world),
    };

    loop {
        match run_navigation(&world, &mut state) {
            NavigationEvent::RestAtGrace(id) => {
                let grace_name = {
                    let zone = world.get(state.zone);
                    let loc = zone.get_location(state.location_id).unwrap();
                    loc.contents
                        .grace
                        .as_ref()
                        .map(|g| g.name)
                        .unwrap_or("Grace")
                };
                player.rest_at_grace(id);
                save::save(&player, &state);
                grace_ui::run_grace_menu(&mut player, grace_name);
                save::save(&player, &state);
            }
            NavigationEvent::TalkToNpc(npc) => {
                npc_ui::run_npc_dialogue(&mut player, npc);
                save::save(&player, &state);
            }
            NavigationEvent::EnterCombat => {
                let zone = state.zone;
                let zone_data = world.get(zone);
                let location = zone_data.get_location(state.location_id).unwrap();
                let spawns = &location.contents.enemies;
                match run_combat(&mut player, zone, spawns) {
                    CombatResult::Victory { items, souls } => {
                        player.souls += souls;
                        let item_count = items.len();
                        for item in items {
                            player.pick_up(item);
                        }
                        println!(
                            "\r\n  Victoire ! +{} ames, {} objet(s) recupere(s).\r\n",
                            souls, item_count
                        );
                        println!("  Appuie sur Entree...");
                        let mut buf = String::new();
                        std::io::stdin().read_line(&mut buf).ok();
                    }
                    CombatResult::Defeat => {
                        println!("\r\n  Vous etes mort. Retour a la derniere grace.\r\n");
                        player.respawn_at_grace();
                        println!("  Appuie sur Entree...");
                        let mut buf = String::new();
                        std::io::stdin().read_line(&mut buf).ok();
                    }
                    CombatResult::Fled => {
                        println!("\r\n  Vous avez fuis le combat.\r\n");
                        println!("  Appuie sur Entree...");
                        let mut buf = String::new();
                        std::io::stdin().read_line(&mut buf).ok();
                    }
                }
            }
            NavigationEvent::OpenChest(id) => {
                let items = open_chest(id);
                println!("\r\n  [Coffre] Ouvert !");
                for item in items {
                    let name = item.name();
                    println!("    + {}", name);
                    player.pick_up(item);
                }
                println!("\r\n  Appuie sur Entree...");
                let mut buf = String::new();
                std::io::stdin().read_line(&mut buf).ok();
            }
            NavigationEvent::Save => {
                save::save(&player, &state);
                println!("\r\n  [Save] Partie sauvegardee.\r\n");
                println!("  Appuie sur Entree...");
                let mut buf = String::new();
                std::io::stdin().read_line(&mut buf).ok();
            }
            NavigationEvent::OpenInventory => {
                let mut out = std::io::stdout();
                open_inventory(&mut out, &mut player, false);
            }
            NavigationEvent::Quit => {
                save::save(&player, &state);
                println!("\r\n  Au revoir.\r\n");
                break;
            }
        }
    }
}

fn new_game(world: &World) -> (Player, NavigationState) {
    let mut player = Player::new("Héros".to_string(), Class::Knight);
    player.hp = player.stats.max_hp();
    player.equipment.push(Equipment::empty());
    let state = NavigationState::new(world);
    (player, state)
}

enum StartChoice {
    Continue,
    NewGame,
}

fn start_menu(has_save: bool) -> StartChoice {
    use crossterm::{
        cursor::{Hide, MoveTo, Show},
        event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
        execute,
        style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
        terminal::{self, Clear, ClearType},
    };
    use std::io::{self, Write};

    const ART: &[&str] = &[
        r" _____                                                             _____ ",
        r"( ___ )-----------------------------------------------------------( ___ )",
        r" |   |                                                             |   | ",
        r" |   |  _    _       _ _                 _____      _              |   | ",
        r" |   | | |  | |     | | |               |  __ \    (_)             |   | ",
        r" |   | | |__| | ___ | | | _____      __ | |__) |___ _  __ _ _ __   |   | ",
        r" |   | |  __  |/ _ \| | |/ _ \ \ /\ / / |  _  // _ \ |/ _` | '_ \  |   | ",
        r" |   | | |  | | (_) | | | (_) \ V  V /  | | \ \  __/ | (_| | | | | |   | ",
        r" |   | |_|  |_|\___/|_|_|\___/ \_/\_/   |_|  \_\___|_|\__, |_| |_| |   | ",
        r" |   |                                                 __/ |       |   | ",
        r" |   |                                                |___/        |   | ",
        r" |___|                                                             |___| ",
        r"(_____)-----------------------------------------------------------(_____) ",
    ];

    let art_width = ART.iter().map(|l| l.len()).max().unwrap_or(73) as u16;
    // block: 13 art + 3 gap + 2 options + 2 hint = 20 rows
    const BLOCK_H: u16 = 20;

    let mut out = io::stdout();
    terminal::enable_raw_mode().ok();
    execute!(out, Hide).ok();

    // 0 = Continuer (only if has_save), 1 = Nouvelle partie
    let mut selected: usize = if has_save { 0 } else { 1 };

    let choice = loop {
        let (tw, th) = terminal::size().unwrap_or((120, 30));
        let col0 = (tw.saturating_sub(art_width)) / 2;
        let row0 = (th.saturating_sub(BLOCK_H)) / 2;

        execute!(out, Clear(ClearType::All)).ok();

        // ASCII art
        for (i, line) in ART.iter().enumerate() {
            execute!(
                out,
                MoveTo(col0, row0 + i as u16),
                SetForegroundColor(Color::DarkYellow),
                SetAttribute(Attribute::Bold),
                Print(line),
                ResetColor,
            )
            .ok();
        }

        // Menu (centré sous l'art)
        let menu_row = row0 + ART.len() as u16 + 3;
        let options: &[(&str, bool)] =
            &[("Continuer la partie", has_save), ("Nouvelle partie", true)];

        for (i, (label, enabled)) in options.iter().enumerate() {
            let prefix = if i == selected { "> " } else { "  " };
            let text = format!("{}{}", prefix, label);
            let mcol = (tw.saturating_sub(text.len() as u16 + 2)) / 2;
            if i == selected {
                execute!(
                    out,
                    MoveTo(mcol, menu_row + i as u16),
                    SetForegroundColor(Color::DarkYellow),
                    SetAttribute(Attribute::Bold),
                    Print(&text),
                    ResetColor,
                )
                .ok();
            } else if *enabled {
                execute!(
                    out,
                    MoveTo(mcol, menu_row + i as u16),
                    SetForegroundColor(Color::White),
                    Print(&text),
                    ResetColor,
                )
                .ok();
            } else {
                execute!(
                    out,
                    MoveTo(mcol, menu_row + i as u16),
                    SetForegroundColor(Color::DarkGrey),
                    Print(&text),
                    ResetColor,
                )
                .ok();
            }
        }

        // Hint
        let hint = "[↑↓] Naviguer   [Entrée] Confirmer";
        let hcol = (tw.saturating_sub(hint.len() as u16)) / 2;
        execute!(
            out,
            MoveTo(hcol, menu_row + options.len() as u16 + 1),
            SetForegroundColor(Color::DarkGrey),
            Print(hint),
            ResetColor,
        )
        .ok();

        out.flush().ok();

        if let Ok(Event::Key(KeyEvent {
            code,
            kind: KeyEventKind::Press,
            ..
        })) = event::read()
        {
            match code {
                KeyCode::Up => {
                    // monte vers Continuer seulement si has_save
                    if selected > 0 && has_save {
                        selected -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected < 1 {
                        selected += 1;
                    }
                }
                KeyCode::Enter | KeyCode::Char(' ') => {
                    break if selected == 0 {
                        StartChoice::Continue
                    } else {
                        StartChoice::NewGame
                    };
                }
                _ => {}
            }
        }
    };

    execute!(out, Show).ok();
    terminal::disable_raw_mode().ok();
    choice
}
