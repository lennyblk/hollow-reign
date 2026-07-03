mod catalog;
mod class;
mod combat;
mod combat_ui;
mod enemy;
mod enemy_catalog;
mod equipment;
mod grace;
mod grace_ui;
mod intro_ui;
mod inventory_ui;
mod item;
mod map;
mod map_ui;
mod merchant_ui;
mod navigation;
mod npc_ui;
mod phrases;
mod player;
mod save;
mod stats;
mod status;
mod typing;
mod ui;
mod updater;
mod zone;

use class::Class;
use combat_ui::{CombatResult, run_combat};
use enemy_catalog::open_chest;
use equipment::Equipment;
use inventory_ui::open_inventory;
use map::World;
use navigation::{NavigationEvent, NavigationState, run_navigation};
use player::Player;
use zone::ZoneId;

fn main() {
    updater::check_and_update();
    let world = World::new();

    const TITLE_WAV: &[u8] = include_bytes!("assets/title.wav");
    let _title_music = {
        use rodio::{Decoder, OutputStream, Sink, Source};
        use std::io::Cursor;
        OutputStream::try_default()
            .ok()
            .and_then(|(stream, handle)| {
                let sink = Sink::try_new(&handle).ok()?;
                let source = Decoder::new(Cursor::new(TITLE_WAV)).ok()?.repeat_infinite();
                sink.append(source);
                sink.set_volume(0.6);
                Some((stream, sink))
            })
    };

    let has_save = save::save_exists();
    let (mut player, mut state) = match start_menu(has_save) {
        StartChoice::Continue => save::load(&world).unwrap_or_else(|| {
            let class = intro_ui::class_select();
            intro_ui::show_intro();
            new_game(&world, class)
        }),
        StartChoice::NewGame => {
            let class = intro_ui::class_select();
            intro_ui::show_intro();
            new_game(&world, class)
        }
    };

    drop(_title_music);

    loop {
        match run_navigation(&world, &mut state, &mut player) {
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
                state.defeated_locations.clear();
                save::save(&player, &state);
                let travel = grace_ui::run_grace_menu(&mut player, grace_name, id, &world);
                if let Some(target_id) = travel {
                    if let Some((zone, loc_id)) = world.location_of_grace(target_id) {
                        state.zone = zone;
                        state.location_id = loc_id;
                        state.last_location.insert(zone, loc_id);
                    }
                }
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
                    CombatResult::Victory {
                        items,
                        souls,
                        boss_killed,
                    } => {
                        player.souls += souls;
                        let item_count = items.len();
                        for item in items {
                            player.pick_up(item);
                        }
                        state.defeated_locations.insert(state.location_id);
                        if boss_killed {
                            state.killed_bosses.insert(state.zone);
                            if state.zone == ZoneId::TheVoid {
                                save::save(&player, &state);
                                show_victory();
                                break;
                            }
                        }
                        let boss_msg = if boss_killed {
                            "\r\n  [BOSS VAINCU] Cette zone est desormais sous votre controle.\r\n"
                        } else {
                            ""
                        };
                        println!(
                            "\r\n  Victoire ! +{} ames, {} objet(s) recupere(s).{}\r\n",
                            souls, item_count, boss_msg
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
                for item in items {
                    show_chest_item(&item);
                    player.pick_up(item);
                }
            }
            NavigationEvent::Save => {
                save::save(&player, &state);
                println!("\r\n  [Save] Partie sauvegardee.\r\n");
                println!("  Appuie sur Entree...");
                let mut buf = String::new();
                std::io::stdin().read_line(&mut buf).ok();
            }
            NavigationEvent::OpenMerchant(zone) => {
                let mut out = std::io::stdout();
                merchant_ui::open_merchant(&mut out, &mut player, zone);
                save::save(&player, &state);
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

fn show_chest_item(item: &item::Item) {
    use crossterm::{
        cursor::{Hide, Show},
        event::{self, Event, KeyEvent, KeyEventKind},
        execute,
        style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
        terminal::{self, Clear, ClearType},
    };
    use std::io::{self, Write};

    let mut out = io::stdout();
    terminal::enable_raw_mode().ok();
    execute!(out, Hide, Clear(ClearType::All)).ok();

    let (tw, th) = terminal::size().unwrap_or((120, 30));

    // En-tête
    let header = "  Coffre ouvert  ";
    let bar = "═".repeat((tw as usize).saturating_sub(header.len() + 4));
    execute!(
        out,
        SetForegroundColor(Color::DarkYellow),
        SetAttribute(Attribute::Bold),
        crossterm::cursor::MoveTo(0, 0),
        Print(format!("══{}{}══\r\n", header, bar)),
        ResetColor,
    )
    .ok();

    let ascii = item.ascii();
    let ascii_lines: Vec<&str> = ascii.lines().collect();

    // Strip indent commun
    let min_indent = ascii_lines
        .iter()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.len() - l.trim_start().len())
        .min()
        .unwrap_or(0);
    let stripped: Vec<&str> = ascii_lines
        .iter()
        .map(|l| {
            if l.len() >= min_indent {
                &l[min_indent..]
            } else {
                l
            }
        })
        .collect();

    let art_w = stripped
        .iter()
        .map(|l| l.chars().count())
        .max()
        .unwrap_or(0);
    let art_h = stripped.len() as u16;
    let col = (tw.saturating_sub(art_w as u16)) / 2;
    let row0 = (th.saturating_sub(art_h + 5)) / 2;

    // ASCII art centré
    for (i, line) in stripped.iter().enumerate() {
        execute!(
            out,
            crossterm::cursor::MoveTo(col, row0 + i as u16),
            SetForegroundColor(Color::DarkYellow),
            Print(line),
            ResetColor,
        )
        .ok();
    }

    // Nom de l'item
    let name = item.name();
    let ncol = (tw.saturating_sub(name.len() as u16)) / 2;
    execute!(
        out,
        crossterm::cursor::MoveTo(ncol, row0 + art_h + 1),
        SetForegroundColor(Color::White),
        SetAttribute(Attribute::Bold),
        Print(name),
        ResetColor,
    )
    .ok();

    // Hint
    let hint = "[ Appuyez sur une touche ]";
    let hcol = (tw.saturating_sub(hint.len() as u16)) / 2;
    execute!(
        out,
        crossterm::cursor::MoveTo(hcol, row0 + art_h + 3),
        SetForegroundColor(Color::DarkGrey),
        Print(hint),
        ResetColor,
    )
    .ok();
    out.flush().ok();

    loop {
        if let Ok(Event::Key(KeyEvent {
            kind: KeyEventKind::Press,
            ..
        })) = event::read()
        {
            break;
        }
    }

    execute!(out, Show).ok();
    terminal::disable_raw_mode().ok();
}

fn show_victory() {
    use crossterm::{
        cursor::{Hide, Show},
        event::{self, Event, KeyEvent, KeyEventKind},
        execute,
        style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
        terminal::{self, Clear, ClearType},
    };
    use std::io::{self, Write};

    const ART: &[&str] = &[
        "                                                                    ",
        "            Le Roi Creux est mort. Hollow Reign est libre.          ",
        "                                                                    ",
        "                     Vous etes le nouveau Roi.                      ",
        "                                                                    ",
    ];

    let mut out = io::stdout();
    terminal::enable_raw_mode().ok();
    execute!(out, Hide, Clear(ClearType::All)).ok();

    let (tw, th) = terminal::size().unwrap_or((120, 30));
    let row0 = (th.saturating_sub(ART.len() as u16 + 3)) / 2;

    for (i, line) in ART.iter().enumerate() {
        let col = (tw.saturating_sub(line.len() as u16)) / 2;
        execute!(
            out,
            crossterm::cursor::MoveTo(col, row0 + i as u16),
            SetForegroundColor(Color::DarkYellow),
            SetAttribute(Attribute::Bold),
            Print(line),
            ResetColor,
        )
        .ok();
    }

    let hint = "[ Appuyez sur une touche pour quitter ]";
    let hcol = (tw.saturating_sub(hint.len() as u16)) / 2;
    execute!(
        out,
        crossterm::cursor::MoveTo(hcol, row0 + ART.len() as u16 + 2),
        SetForegroundColor(Color::DarkGrey),
        Print(hint),
        ResetColor,
    )
    .ok();
    out.flush().ok();

    loop {
        if let Ok(Event::Key(KeyEvent {
            kind: KeyEventKind::Press,
            ..
        })) = event::read()
        {
            break;
        }
    }

    execute!(out, Show).ok();
    terminal::disable_raw_mode().ok();
}

fn new_game(world: &World, class: Class) -> (Player, NavigationState) {
    let mut player = Player::new("Héros".to_string(), class);
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
                SetForegroundColor(Color::Rgb {
                    r: 95,
                    g: 81,
                    b: 194
                }),
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

        // Volume reminder
        let vol = "♪ Volume up";
        let vcol = (tw.saturating_sub(vol.len() as u16)) / 2;
        execute!(
            out,
            MoveTo(vcol, menu_row + options.len() as u16 + 3),
            SetForegroundColor(Color::DarkGrey),
            Print(vol),
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
