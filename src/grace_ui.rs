use std::io::{self, Write};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};

use crate::map::World;
use crate::player::Player;
use crate::stats::Stats;
use crate::ui;
use crate::zone::ZoneId;

// ─── ART ASCII GRÂCE ─────────────────────────────────────────────────────────

const GRACE_ASCII: &str =
"                  ░  ░░░
               ░ ░ ░░░░░  ░   ░░
              ░░░░░░░▒░░░░░░░ ░
               ░░░░░░▒░░░░░░░░
               ░░░░░░░▒▒░░░░░░░
               ░░░░▒▒▒░▒░▒░░░░░░
           ░░ ░░░▒▒▒▒░░▒▒▒░░░░░░
            ░░░▒▒▒▓▒▒░░▒▒▒░▒▒░░
             ░░░▒▓▓▒░░░▒▒▒▒░░░░
            ░░▒▒▒▓▓▓▒▒░▒▒▒▒░░░░
           ░░▒▒▒▓▓▓▓▒▒▒░░▒▒░░░░
           ░░░▒▒▒▓▓▓▒▒▒▒░▒▒░░▒▒░
           ░░░▒▒▒▓▓██▓▓▒░▓▓▒▒▒░░
        ░░░░░░▒▓▓▓▓▓▓█▓▓░▒▓▒▒▓▒▓░
        ░▒▒▓▒▓░▒▒▒▓▓▓▓█▓▒░▒▒▓██▓░░
          ░░▒▒░▒▒▓▓█▓▓▓▓▓▒▒█▓▓▒██▓░
        ░▒▒░░░░▓▓▓██▓▓▓▓▓▓▒███▓▓███▓░
      ░▒▓▓▓░▒▒░▒▓▓██▓░▒▓█▓▒▒▓████████▒░
   ░░▒▒██▓▓▒▓▓▒▓▓█▓█▓▓▓▓▓▓█▒▒▓▓███▓▓██▓▒░
░░░▒▒▒▓▓▒▓▓▓▓▓▓▓███▓▓▓▓▓▓▓▓▓▓▓▓▓███████▓▓▒░
░░▒▓█▓▓▓▓▓▓███▓██████▓██▓▒▓▒▓▓▓███████▓██▓▒
  ░░░▓██████▓▓█▓████▓▓▓▓▓▓▒▒▒▓████▓██▓▓▓▒
    ░░░░░░░▒▓▓▓▓████▓▓▓▓██▓▒▓▓▓██▓▓▒░░▒░░
            ░▓▓▓▓▒░░▒▒▒▓▒▒░▓█▓▒░░
                           ░░";

// ─── STATS LEVEL-UP ──────────────────────────────────────────────────────────

const STATS: &[(&str, &str, &str)] = &[
    ("vigor",        "Vigueur",       "Augmente les PV maximum"),
    ("strength",     "Force",         "Augmente les degats physiques"),
    ("dexterity",    "Dexterite",     "Augmente les degats physiques (x0.5)"),
    ("intelligence", "Intelligence",  "Augmente les degats elementaires"),
    ("faith",        "Foi",           "Augmente la resistance elementaire"),
    ("arcane",       "Arcane",        "Augmente la decouverte d'objets"),
    ("mind",         "Esprit",        "Ameliore la concentration de frappe"),
];

fn current_atk(stats: &Stats) -> u32 {
    5 + stats.strength + stats.dexterity / 2
}

fn stat_preview(player: &Player, key: &str) -> String {
    let s = &player.stats;
    match key {
        "vigor" => {
            let cur  = s.max_hp();
            let next = 100 + (s.vigor + 1) * 10;
            format!("PV Max : {} -> {} (+{})", cur, next, next - cur)
        }
        "strength" => {
            let cur  = current_atk(s);
            let next = 5 + (s.strength + 1) + s.dexterity / 2;
            format!("Attaque : {} -> {} (+{})", cur, next, next - cur)
        }
        "dexterity" => {
            let cur  = current_atk(s);
            let next = 5 + s.strength + (s.dexterity + 1) / 2;
            if next > cur {
                format!("Attaque : {} -> {} (+{})", cur, next, next - cur)
            } else {
                format!("Attaque : {} (prochain +1 au niveau pair de dex)", cur)
            }
        }
        "intelligence" => format!("Degats elem. : {} -> {} (+1)", s.intelligence, s.intelligence + 1),
        "faith"        => format!("Resistance elem. : {} -> {} (+1)", s.faith, s.faith + 1),
        "arcane"       => format!("Arcane : {} -> {} (+1)", s.arcane, s.arcane + 1),
        "mind"         => format!("Esprit : {} -> {} (+1 concentration)", s.mind, s.mind + 1),
        _              => String::new(),
    }
}

// ─── ÉCRAN ───────────────────────────────────────────────────────────────────

enum Screen {
    Main,
    LevelUp,
    FastTravel,
}

// ─── POINT D'ENTRÉE ──────────────────────────────────────────────────────────

/// Retourne Some(grace_id) si le joueur choisit un voyage rapide, None sinon.
pub fn run_grace_menu(
    player: &mut Player,
    grace_name: &str,
    current_grace_id: u32,
    world: &World,
) -> Option<u32> {
    // Liste des grâces accessibles (visitées, hors grâce courante), triées par id
    let fast_travel: Vec<(u32, &str, ZoneId)> = world
        .all_graces()
        .into_iter()
        .filter(|(id, _, _)| *id != current_grace_id && player.visited_graces.contains(id))
        .collect();

    let can_fast_travel = !fast_travel.is_empty();

    let mut out = io::stdout();
    terminal::enable_raw_mode().ok();
    execute!(out, Hide).ok();

    let mut screen    = Screen::Main;
    let mut main_sel  = 0usize; // 0 = level up, 1 = fast travel
    let mut stat_sel  = 0usize;
    let mut ft_sel    = 0usize;
    let mut flash: Option<(&str, bool)> = None;
    let mut result: Option<u32> = None;

    loop {
        match screen {
            Screen::Main => {
                draw_main(&mut out, player, grace_name, main_sel, can_fast_travel);
            }
            Screen::LevelUp => {
                draw_level_up(&mut out, player, grace_name, stat_sel, flash);
            }
            Screen::FastTravel => {
                draw_fast_travel(&mut out, grace_name, &fast_travel, ft_sel);
            }
        }

        if let Ok(Event::Key(KeyEvent { code, kind: KeyEventKind::Press, .. })) = event::read() {
            flash = None;
            match screen {
                Screen::Main => match code {
                    KeyCode::Up => {
                        if main_sel > 0 { main_sel -= 1; }
                    }
                    KeyCode::Down => {
                        if main_sel == 0 && can_fast_travel { main_sel = 1; }
                    }
                    KeyCode::Enter | KeyCode::Char(' ') => {
                        if main_sel == 0 {
                            screen = Screen::LevelUp;
                        } else if can_fast_travel {
                            screen = Screen::FastTravel;
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => break,
                    _ => {}
                },
                Screen::LevelUp => match code {
                    KeyCode::Up => {
                        if stat_sel > 0 { stat_sel -= 1; }
                    }
                    KeyCode::Down => {
                        if stat_sel < STATS.len() - 1 { stat_sel += 1; }
                    }
                    KeyCode::Enter | KeyCode::Char(' ') => {
                        let (key, _, _) = STATS[stat_sel];
                        if player.level_up(key) {
                            if key == "vigor" { player.hp = player.stats.max_hp(); }
                            flash = Some(("Amelioration reussie !", true));
                        } else {
                            flash = Some(("Ames insuffisantes.", false));
                        }
                    }
                    KeyCode::Esc => {
                        screen = Screen::Main;
                    }
                    _ => {}
                },
                Screen::FastTravel => match code {
                    KeyCode::Up => {
                        if ft_sel > 0 { ft_sel -= 1; }
                    }
                    KeyCode::Down => {
                        if ft_sel + 1 < fast_travel.len() { ft_sel += 1; }
                    }
                    KeyCode::Enter | KeyCode::Char(' ') => {
                        result = Some(fast_travel[ft_sel].0);
                        break;
                    }
                    KeyCode::Esc => {
                        screen = Screen::Main;
                    }
                    _ => {}
                },
            }
        }
    }

    execute!(out, Show).ok();
    terminal::disable_raw_mode().ok();
    result
}

// ─── RENDU : ÉCRAN PRINCIPAL ─────────────────────────────────────────────────

fn draw_main(
    out: &mut io::Stdout,
    player: &Player,
    grace_name: &str,
    selected: usize,
    can_fast_travel: bool,
) {
    let (tw, _th) = terminal::size().unwrap_or((120, 35));
    execute!(out, Clear(ClearType::All), MoveTo(0, 0)).ok();

    draw_header(out, grace_name, tw);
    let art_rows = draw_art_and_info(out, player, tw);

    let sc = 48u16;
    let right_width = (tw as usize).saturating_sub(sc as usize + 2);

    // Titre
    execute!(
        out,
        MoveTo(sc, 2),
        SetForegroundColor(Color::White),
        SetAttribute(Attribute::Bold),
        Print("GRACE"),
        ResetColor,
    ).ok();
    execute!(
        out,
        MoveTo(sc, 3),
        SetForegroundColor(ui::DIM),
        Print("─".repeat(right_width.min(30))),
        ResetColor,
    ).ok();

    let options = [
        ("Augmenter niveau",  true),
        ("Voyage rapide",     can_fast_travel),
    ];

    for (i, (label, enabled)) in options.iter().enumerate() {
        let row = 5 + i as u16;
        if i == selected {
            execute!(
                out,
                MoveTo(sc, row),
                SetForegroundColor(ui::ACCENT),
                SetAttribute(Attribute::Bold),
                Print(format!("▶ {}", label)),
                ResetColor,
            ).ok();
        } else if *enabled {
            execute!(
                out,
                MoveTo(sc, row),
                SetForegroundColor(Color::White),
                Print(format!("  {}", label)),
                ResetColor,
            ).ok();
        } else {
            execute!(
                out,
                MoveTo(sc, row),
                SetForegroundColor(ui::DIM),
                Print(format!("  {} (aucune grace visitee)", label)),
                ResetColor,
            ).ok();
        }
    }

    let _ = art_rows;
    draw_hint(out, tw, "[haut/bas] Choisir    [Entree] Confirmer    [Esc] Fermer");
    out.flush().ok();
}

// ─── RENDU : LEVEL UP ────────────────────────────────────────────────────────

fn draw_level_up(
    out: &mut io::Stdout,
    player: &Player,
    grace_name: &str,
    selected: usize,
    flash: Option<(&str, bool)>,
) {
    let (tw, _th) = terminal::size().unwrap_or((120, 35));
    execute!(out, Clear(ClearType::All), MoveTo(0, 0)).ok();

    draw_header(out, grace_name, tw);
    draw_art_and_info(out, player, tw);

    let sc = 48u16;
    let right_width = (tw as usize).saturating_sub(sc as usize + 2);

    execute!(
        out,
        MoveTo(sc, 2),
        SetForegroundColor(Color::White),
        SetAttribute(Attribute::Bold),
        Print("STATISTIQUES"),
        ResetColor,
    ).ok();

    // Âmes + coût niveau suivant
    let cost = player.soul_cost();
    execute!(out, MoveTo(sc + 16, 2),
        SetForegroundColor(ui::ACCENT), SetAttribute(Attribute::Bold),
        Print(format!("  ◈ {} ames", player.souls)), ResetColor).ok();
    execute!(out, MoveTo(sc + 16, 3),
        SetForegroundColor(if player.souls >= cost { Color::Green } else { Color::Red }),
        Print(format!("  Cout : {}", cost)), ResetColor).ok();

    execute!(
        out,
        MoveTo(sc, 3),
        SetForegroundColor(ui::DIM),
        Print("─".repeat(right_width)),
        ResetColor,
    ).ok();

    for (i, (key, label, _desc)) in STATS.iter().enumerate() {
        let val  = player.stats.get(key).unwrap_or(0);
        let srow = 4 + i as u16;

        if i == selected {
            execute!(
                out,
                MoveTo(sc, srow),
                SetForegroundColor(ui::ACCENT),
                SetAttribute(Attribute::Bold),
                Print(format!("▶ {:<14} {:>3}  ->  {}", label, val, val + 1)),
                ResetColor,
            ).ok();
        } else {
            execute!(
                out,
                MoveTo(sc, srow),
                SetForegroundColor(Color::White),
                Print(format!("  {:<14} {:>3}", label, val)),
                ResetColor,
            ).ok();
        }
    }

    let preview_row = 4 + STATS.len() as u16 + 1;
    execute!(
        out,
        MoveTo(sc, preview_row),
        SetForegroundColor(ui::DIM),
        Print("─".repeat(right_width)),
        ResetColor,
    ).ok();

    let (key, _, desc) = STATS[selected];
    execute!(
        out,
        MoveTo(sc, preview_row + 1),
        SetForegroundColor(ui::DIM),
        Print(desc),
        ResetColor,
    ).ok();
    execute!(
        out,
        MoveTo(sc, preview_row + 2),
        SetForegroundColor(Color::Cyan),
        SetAttribute(Attribute::Bold),
        Print(stat_preview(player, key)),
        ResetColor,
    ).ok();

    if let Some((msg, success)) = flash {
        execute!(
            out,
            MoveTo(sc, preview_row + 4),
            SetForegroundColor(if success { Color::Green } else { Color::Red }),
            SetAttribute(Attribute::Bold),
            Print(msg),
            ResetColor,
        ).ok();
    }

    draw_hint(out, tw, "[haut/bas] Choisir    [Entree] Ameliorer    [Esc] Retour");
    out.flush().ok();
}

// ─── RENDU : VOYAGE RAPIDE ───────────────────────────────────────────────────

fn draw_fast_travel(
    out: &mut io::Stdout,
    grace_name: &str,
    list: &[(u32, &str, ZoneId)],
    selected: usize,
) {
    let (tw, _th) = terminal::size().unwrap_or((120, 35));
    execute!(out, Clear(ClearType::All), MoveTo(0, 0)).ok();

    draw_header(out, grace_name, tw);

    let sc = 4u16;

    execute!(
        out,
        MoveTo(sc, 2),
        SetForegroundColor(Color::White),
        SetAttribute(Attribute::Bold),
        Print("VOYAGE RAPIDE — Choisir une grace :"),
        ResetColor,
    ).ok();
    execute!(
        out,
        MoveTo(sc, 3),
        SetForegroundColor(ui::DIM),
        Print("─".repeat(50)),
        ResetColor,
    ).ok();

    for (i, (_, name, zone)) in list.iter().enumerate() {
        let row = 5 + i as u16;
        let zone_name = zone.name();
        if i == selected {
            execute!(
                out,
                MoveTo(sc, row),
                SetForegroundColor(ui::ACCENT),
                SetAttribute(Attribute::Bold),
                Print(format!("▶ {:<30} {}", name, zone_name)),
                ResetColor,
            ).ok();
        } else {
            execute!(
                out,
                MoveTo(sc, row),
                SetForegroundColor(Color::White),
                Print(format!("  {:<30} {}", name, zone_name)),
                ResetColor,
            ).ok();
        }
    }

    draw_hint(out, tw, "[haut/bas] Choisir    [Entree] Voyager    [Esc] Retour");
    out.flush().ok();
}

// ─── UTILITAIRES DE RENDU ────────────────────────────────────────────────────

fn draw_header(out: &mut io::Stdout, grace_name: &str, tw: u16) {
    ui::top_header(out, "GRACE", grace_name, ui::ACCENT, tw as usize);
}

/// Dessine l'art ASCII + infos joueur en colonne gauche. Retourne le nombre de lignes de l'art.
fn draw_art_and_info(out: &mut io::Stdout, player: &Player, tw: u16) -> u16 {
    let ascii = GRACE_ASCII;
    let mut row = 2u16;

    for line in ascii.lines() {
        execute!(
            out,
            MoveTo(2, row),
            SetForegroundColor(ui::DIM),
            Print(line),
            ResetColor,
        ).ok();
        row += 1;
    }

    row += 1;
    execute!(out, MoveTo(2, row), SetForegroundColor(Color::White),
        SetAttribute(Attribute::Bold), Print(&player.name), ResetColor).ok();
    row += 1;

    execute!(out, MoveTo(2, row), SetForegroundColor(ui::ACCENT),
        Print(format!("Niveau  {}", player.level)), ResetColor).ok();
    row += 1;

    execute!(out, MoveTo(2, row), SetForegroundColor(Color::White),
        Print(format!("Ames    {}", player.souls)), ResetColor).ok();
    row += 1;

    let cost = player.soul_cost();
    execute!(
        out,
        MoveTo(2, row),
        SetForegroundColor(if player.souls >= cost { Color::Green } else { Color::Red }),
        Print(format!("Cout    {} ames", cost)),
        ResetColor,
    ).ok();
    row += 2;

    let max_hp = player.stats.max_hp();
    let hp_color = if player.hp * 100 / max_hp.max(1) > 60 { Color::Green }
        else if player.hp * 100 / max_hp.max(1) > 30 { Color::Yellow }
        else { Color::Red };
    execute!(out, MoveTo(2, row), SetForegroundColor(hp_color),
        Print(format!("PV      {} / {}", player.hp, max_hp)), ResetColor).ok();

    let _ = tw;
    row
}

fn draw_hint(out: &mut io::Stdout, tw: u16, hint: &str) {
    let (_, th) = terminal::size().unwrap_or((tw, 35));
    ui::hint_bar(out, tw as usize, th as usize, hint);
}
