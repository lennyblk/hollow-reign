use std::io::{self, Write};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};

use crate::player::Player;
use crate::stats::Stats;

// ─── DONNÉES STATS ────────────────────────────────────────────────────────────

/// (clé interne, label affiché, description courte)
const STATS: &[(&str, &str, &str)] = &[
    ("vigor",        "Vigueur",       "Augmente les PV maximum"),
    ("strength",     "Force",         "Augmente les degats physiques"),
    ("dexterity",    "Dexterite",     "Augmente les degats physiques (x0.5)"),
    ("intelligence", "Intelligence",  "Augmente les degats elementaires"),
    ("faith",        "Foi",           "Augmente la resistance elementaire"),
    ("arcane",       "Arcane",        "Augmente la decouverte d'objets"),
    ("mind",         "Esprit",        "Ameliore la concentration de frappe"),
];

// ─── CALCULS PREVIEW ──────────────────────────────────────────────────────────

fn current_atk(stats: &Stats) -> u32 {
    5 + stats.strength + stats.dexterity / 2
}

fn stat_preview(player: &Player, key: &str) -> String {
    let s = &player.stats;
    match key {
        "vigor" => {
            let cur  = s.max_hp();
            let next = 300 + (s.vigor + 1) * 10;
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

// ─── BOUCLE PRINCIPALE ───────────────────────────────────────────────────────

pub fn run_grace_menu(player: &mut Player, grace_name: &str) {
    let mut out = io::stdout();
    terminal::enable_raw_mode().ok();
    execute!(out, Hide).ok();

    let mut selected = 0usize;
    let mut flash: Option<(&str, bool)> = None; // persiste jusqu'au prochain input

    loop {
        draw(&mut out, player, grace_name, selected, flash);

        if let Ok(Event::Key(KeyEvent { code, kind: KeyEventKind::Press, .. })) = event::read() {
            flash = None; // effacer sur n'importe quel input
            match code {
                KeyCode::Up => {
                    if selected > 0 { selected -= 1; }
                }
                KeyCode::Down => {
                    if selected < STATS.len() - 1 { selected += 1; }
                }
                KeyCode::Enter | KeyCode::Char(' ') => {
                    let (key, _, _) = STATS[selected];
                    if player.level_up(key) {
                        if key == "vigor" { player.hp = player.stats.max_hp(); }
                        flash = Some(("Amelioration reussie !", true));
                    } else {
                        flash = Some(("Ames insuffisantes.", false));
                    }
                }
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => break,
                _ => {}
            }
        }
    }

    execute!(out, Show).ok();
    terminal::disable_raw_mode().ok();
}

// ─── RENDU ────────────────────────────────────────────────────────────────────

fn draw(
    out:        &mut io::Stdout,
    player:     &Player,
    grace_name: &str,
    selected:   usize,
    flash:      Option<(&str, bool)>,
) {
    let (tw, th) = terminal::size().unwrap_or((120, 35));
    execute!(out, Clear(ClearType::All), MoveTo(0, 0)).ok();

    // ── En-tête ───────────────────────────────────────────────────────────────
    let title = format!("  Grace — {}  ", grace_name);
    let bar   = "═".repeat((tw as usize).saturating_sub(title.len() + 4));
    execute!(
        out,
        SetForegroundColor(Color::DarkYellow),
        SetAttribute(Attribute::Bold),
        Print(format!("══{}{}══\r\n", title, bar)),
        ResetColor,
    ).ok();

    // ── Colonne gauche : ASCII + infos joueur ─────────────────────────────────
    // art Knight ~43 chars large depuis x=2 → stats à droite à x=48
    let sc_info = 48u16;
    let ascii   = player.class.ascii();
    let mut row = 2u16;

    for line in ascii.lines() {
        execute!(
            out,
            MoveTo(2, row),
            SetForegroundColor(Color::DarkGrey),
            Print(line),
            ResetColor,
        ).ok();
        row += 1;
    }

    row += 1;
    execute!(
        out,
        MoveTo(2, row),
        SetForegroundColor(Color::White),
        SetAttribute(Attribute::Bold),
        Print(&player.name),
        ResetColor,
    ).ok();
    row += 1;

    execute!(
        out,
        MoveTo(2, row),
        SetForegroundColor(Color::DarkYellow),
        Print(format!("Niveau  {}", player.level)),
        ResetColor,
    ).ok();
    row += 1;

    execute!(
        out,
        MoveTo(2, row),
        SetForegroundColor(Color::White),
        Print(format!("Ames    {}", player.souls)),
        ResetColor,
    ).ok();
    row += 1;

    let cost       = player.soul_cost();
    let can_afford = player.souls >= cost;
    execute!(
        out,
        MoveTo(2, row),
        SetForegroundColor(if can_afford { Color::Green } else { Color::Red }),
        Print(format!("Cout    {} ames", cost)),
        ResetColor,
    ).ok();
    row += 2;

    // PV actuel / max
    let hp_color = if player.hp * 100 / player.stats.max_hp().max(1) > 60 {
        Color::Green
    } else if player.hp * 100 / player.stats.max_hp().max(1) > 30 {
        Color::Yellow
    } else {
        Color::Red
    };
    execute!(
        out,
        MoveTo(2, row),
        SetForegroundColor(hp_color),
        Print(format!("PV      {} / {}", player.hp, player.stats.max_hp())),
        ResetColor,
    ).ok();

    // ── Colonne droite : liste des stats ──────────────────────────────────────
    let sc = sc_info;

    execute!(
        out,
        MoveTo(sc, 2),
        SetForegroundColor(Color::White),
        SetAttribute(Attribute::Bold),
        Print("STATISTIQUES"),
        ResetColor,
    ).ok();

    let right_width = (tw as usize).saturating_sub(sc as usize + 2);
    execute!(
        out,
        MoveTo(sc, 3),
        SetForegroundColor(Color::DarkGrey),
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
                SetForegroundColor(Color::DarkYellow),
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

    // ── Preview stat sélectionnée ─────────────────────────────────────────────
    let preview_row = 4 + STATS.len() as u16 + 1;

    execute!(
        out,
        MoveTo(sc, preview_row),
        SetForegroundColor(Color::DarkGrey),
        Print("─".repeat(right_width)),
        ResetColor,
    ).ok();

    let (key, _, desc) = STATS[selected];
    execute!(
        out,
        MoveTo(sc, preview_row + 1),
        SetForegroundColor(Color::DarkGrey),
        Print(desc),
        ResetColor,
    ).ok();

    let preview = stat_preview(player, key);
    execute!(
        out,
        MoveTo(sc, preview_row + 2),
        SetForegroundColor(Color::Cyan),
        SetAttribute(Attribute::Bold),
        Print(&preview),
        ResetColor,
    ).ok();

    // ── Flash message (succès / erreur) ───────────────────────────────────────
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

    // ── Barre d'actions en bas ────────────────────────────────────────────────
    let bar_row = th.saturating_sub(2);
    execute!(
        out,
        MoveTo(0, bar_row),
        SetForegroundColor(Color::DarkGrey),
        Print(format!("  {}  ", "─".repeat((tw as usize).saturating_sub(4)))),
        ResetColor,
    ).ok();

    let hint    = "[haut/bas] Choisir    [Entree] Ameliorer    [Esc] Fermer";
    let hint_x  = (tw.saturating_sub(hint.len() as u16)) / 2;
    execute!(
        out,
        MoveTo(hint_x, bar_row + 1),
        SetForegroundColor(Color::DarkGrey),
        Print(hint),
        ResetColor,
    ).ok();

    out.flush().ok();
}
