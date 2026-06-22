use std::io::{self, Write};
use std::time::Duration;

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};

use crate::combat::Combat;
use crate::enemy::{Enemy, EnemyType};
use crate::enemy_catalog::{drops, spawn};
use crate::inventory_ui::{open_inventory, InventoryResult};
use crate::item::{Element, Item};
use crate::phrases::{Difficulty, PhrasePool};
use crate::player::Player;
use crate::typing::{perfect_threshold, time_limit_ms, typing_challenge};
use crate::zone::{EnemySpawn, ZoneId};

// ─── RÉSULTAT ────────────────────────────────────────────────────────────────

pub enum CombatResult {
    Victory { items: Vec<Item>, souls: u32, boss_killed: bool },
    Defeat,
    Fled,
}

// ─── POINT D'ENTRÉE ──────────────────────────────────────────────────────────

pub fn run_combat(player: &mut Player, zone: ZoneId, spawns: &[EnemySpawn]) -> CombatResult {
    // Zones 1-2 : groupes jusqu'à 3. Zones suivantes : 1 seul ennemi.
    let max_enemies = match zone {
        ZoneId::Ashfeld | ZoneId::Gravemoor => 3,
        _ => 1,
    };
    let mut enemies = Vec::new();
    'build: for s in spawns {
        for _ in 0..s.count {
            if enemies.len() >= max_enemies { break 'build; }
            enemies.push(spawn(zone, s.enemy_type, s.element));
        }
    }
    if enemies.is_empty() {
        return CombatResult::Fled;
    }

    let mut combat = Combat::new(enemies);
    let mut phrases = PhrasePool::new();
    let mut log: Vec<String> = Vec::new();
    let mut out = io::stdout();

    terminal::enable_raw_mode().ok();
    execute!(out, Hide).ok();

    let result = 'main: loop {
        combat.next_turn();

        // Ticks élémentaires en début de tour
        let tick = Combat::tick_player_effects(player);
        if tick > 0 {
            push_log(&mut log, format!("Effets sur toi : -{} PV", tick));
        }
        for i in 0..combat.enemies.len() {
            let dmg = combat.tick_enemy_effects(i);
            if dmg > 0 {
                let name = combat.enemies[i].name.clone();
                push_log(&mut log, format!("{} : -{} PV (effets)", name, dmg));
            }
        }

        if combat.is_over(player) {
            break 'main end_result(player, &combat, zone);
        }

        // ── Tour du joueur ────────────────────────────────────────────────────
        draw_combat(&mut out, player, &combat, &log, combat.turn, zone);

        if !player.status.can_act() {
            push_log(&mut log, "Tu es immobilise — tour passe.".to_string());
            draw_combat(&mut out, player, &combat, &log, combat.turn, zone);
        } else { 'input: loop {
            if let Ok(Event::Key(KeyEvent { code, kind: KeyEventKind::Press, .. })) = event::read() {
                match code {
                    // Attaque normale
                    KeyCode::Char('a') | KeyCode::Char('A') => {
                        if let Some(target) = select_target(&mut out, &combat) {
                            let base = base_damage(player);
                            let dmg = combat.player_attack(target, base);
                            let name = combat.enemies[target].name.clone();
                            push_log(&mut log, format!("Tu attaques {} : {} degats.", name, dmg));
                        }
                        break 'input;
                    }
                    // Attaque spéciale (typing challenge)
                    KeyCode::Char('s') | KeyCode::Char('S')
                        if combat.player_elemental_cooldown == 0 =>
                    {
                        if let Some(target) = select_target(&mut out, &combat) {
                            let et = combat.enemies[target].enemy_type;
                            let diff = enemy_difficulty(et);
                            let limit = time_limit_ms(&diff);
                            let pct = perfect_threshold(&diff);
                            let phrase = phrases.next(diff);

                            // typing_challenge gère enable/disable raw mode lui-même
                            let parry = typing_challenge(phrase, limit, pct);
                            // Il a désactivé raw mode — on réactive
                            terminal::enable_raw_mode().ok();
                            execute!(out, Hide).ok();

                            let base = base_damage(player);
                            let (dmg, burst) = combat.player_elemental_attack(player, target, base, parry);
                            let name = combat.enemies[target].name.clone();
                            if burst > 0 {
                                push_log(&mut log, format!("Attaque speciale sur {} : {} dmg + {} burst!", name, dmg, burst));
                            } else {
                                push_log(&mut log, format!("Attaque speciale sur {} : {} degats.", name, dmg));
                            }
                        }
                        break 'input;
                    }
                    // Estus
                    KeyCode::Char('h') | KeyCode::Char('H')
                        if player.estus_charges > 0 =>
                    {
                        player.use_estus();
                        push_log(&mut log, "Tu bois une fiole d'estus.".to_string());
                        break 'input;
                    }
                    // Inventaire plein écran — ne consomme le tour QUE si consommable utilisé
                    KeyCode::Char('i') | KeyCode::Char('I') => {
                        // open_inventory gère son propre raw mode (enable + disable)
                        let inv_result = open_inventory(&mut out, player, false);
                        // Il a désactivé raw mode — on réactive pour le combat
                        terminal::enable_raw_mode().ok();
                        execute!(out, Hide).ok();
                        // Redessine le combat par-dessus
                        draw_combat(&mut out, player, &combat, &log, combat.turn, zone);
                        if let InventoryResult::ConsumedItem = inv_result {
                            break 'input;
                        }
                    }
                    // Fuite
                    KeyCode::Char('f') | KeyCode::Char('F') => {
                        break 'main CombatResult::Fled;
                    }
                    _ => {}
                }
            }
        } } // end if can_act / 'input

        if combat.is_over(player) {
            break 'main end_result(player, &combat, zone);
        }

        // ── Tours des ennemis ─────────────────────────────────────────────────
        for i in 0..combat.enemies.len() {
            if !combat.enemies[i].is_alive() || !combat.enemies[i].can_act() {
                continue;
            }
            let name = combat.enemies[i].name.clone();
            let et = combat.enemies[i].enemy_type;

            if combat.turn % 3 == 0 {
                // Attaque élémentaire → typing challenge pour le joueur
                let diff = enemy_difficulty(et);
                let limit = time_limit_ms(&diff);
                let pct = perfect_threshold(&diff);
                let phrase = phrases.next(diff);

                let parry = typing_challenge(phrase, limit, pct);
                terminal::enable_raw_mode().ok();
                execute!(out, Hide).ok();

                let dmg = combat.enemy_elemental_attack(player, i, parry);
                push_log(&mut log, format!("{} attaque elementaire : -{} PV.", name, dmg));
            } else {
                let dmg = combat.enemy_attack(player, i);
                if dmg > 0 {
                    push_log(&mut log, format!("{} attaque : -{} PV.", name, dmg));
                } else {
                    push_log(&mut log, format!("{} attaque mais rate.", name));
                }
            }

            if player.hp == 0 || !player.status.can_act() {
                break;
            }
        }

        if combat.is_over(player) {
            break 'main end_result(player, &combat, zone);
        }
    };

    // Affiche l'état final 1.5s
    draw_combat(&mut out, player, &combat, &log, combat.turn, zone);
    std::thread::sleep(Duration::from_millis(1_500));

    execute!(out, Show).ok();
    terminal::disable_raw_mode().ok();

    result
}

// ─── HELPERS ─────────────────────────────────────────────────────────────────

fn end_result(player: &Player, combat: &Combat, zone: ZoneId) -> CombatResult {
    if player.hp > 0 {
        let boss_killed = combat.enemies.iter().any(|e| {
            e.enemy_type == EnemyType::Boss && !e.is_alive()
        });
        CombatResult::Victory {
            souls: combat.collect_souls(),
            items: collect_drops(combat, zone),
            boss_killed,
        }
    } else {
        CombatResult::Defeat
    }
}

fn collect_drops(combat: &Combat, zone: ZoneId) -> Vec<Item> {
    let mut items = Vec::new();
    for e in &combat.enemies {
        if !e.is_alive() {
            if let Some(&element) = e.elements.first() {
                let mut d = drops(zone, e.enemy_type, element);
                items.append(&mut d);
            }
        }
    }
    items
}

/// Sélection de cible interactive.
/// 1 ennemi vivant → retourne directement son index.
/// 2-3 ennemis vivants → affiche un prompt [1][2][3] et attend une touche.
/// [Esc] → annule (retourne None).
fn select_target(out: &mut io::Stdout, combat: &Combat) -> Option<usize> {
    let alive: Vec<(usize, &str)> = combat.enemies.iter()
        .enumerate()
        .filter(|(_, e)| e.is_alive())
        .map(|(i, e)| (i, e.name.as_str()))
        .collect();

    if alive.is_empty() {
        return None;
    }
    if alive.len() == 1 {
        return Some(alive[0].0);
    }

    // Affiche le prompt de sélection sous les actions
    execute!(
        out,
        Print("\r\n  "),
        SetForegroundColor(Color::White),
        SetAttribute(Attribute::Bold),
        Print("Cible : "),
        ResetColor,
    ).ok();
    for (n, (_, name)) in alive.iter().enumerate() {
        execute!(
            out,
            SetForegroundColor(Color::DarkYellow),
            SetAttribute(Attribute::Bold),
            Print(format!("[{}]", n + 1)),
            ResetColor,
            Print(format!(" {}  ", name)),
        ).ok();
    }
    execute!(
        out,
        SetForegroundColor(Color::DarkGrey),
        Print("[Esc] Annuler\r\n"),
        ResetColor,
    ).ok();
    out.flush().ok();

    loop {
        if let Ok(Event::Key(KeyEvent { code, kind: KeyEventKind::Press, .. })) = event::read() {
            match code {
                KeyCode::Char(c) if c.is_ascii_digit() => {
                    let n = (c as usize).saturating_sub('1' as usize);
                    if n < alive.len() {
                        return Some(alive[n].0);
                    }
                }
                KeyCode::Esc => return None,
                _ => {}
            }
        }
    }
}

fn base_damage(player: &Player) -> u32 {
    5 + player.stats.strength + player.stats.dexterity / 2
}

fn zone_enemy_color(zone: ZoneId) -> Color {
    match zone {
        ZoneId::Ashfeld    => Color::Rgb { r: 180, g: 40,  b: 40  }, // saignement
        ZoneId::Gravemoor  => Color::Rgb { r: 60,  g: 180, b: 60  }, // poison
        ZoneId::Rotwood    => Color::Rgb { r: 100, g: 140, b: 30  }, // pourriture
        ZoneId::TheCinders => Color::Rgb { r: 220, g: 100, b: 20  }, // feu
        ZoneId::Frostveil  => Color::Rgb { r: 60,  g: 200, b: 220 }, // glace
        ZoneId::TheVoid    => Color::Rgb { r: 150, g: 40,  b: 200 }, // void
    }
}

fn enemy_difficulty(et: EnemyType) -> Difficulty {
    match et {
        EnemyType::Mob => Difficulty::Short,
        EnemyType::MobLeader | EnemyType::MiniBoss => Difficulty::Medium,
        EnemyType::Boss => Difficulty::Long,
    }
}

fn push_log(log: &mut Vec<String>, msg: String) {
    log.push(msg);
    if log.len() > 6 {
        log.remove(0);
    }
}

fn hp_color(current: u32, max: u32) -> Color {
    if max == 0 { return Color::DarkGrey; }
    let pct = current * 100 / max;
    if pct > 60 { Color::Green }
    else if pct > 30 { Color::Yellow }
    else { Color::Red }
}

/// Imprime une barre de vie colorée directement dans stdout.
fn print_hp_bar(out: &mut io::Stdout, current: u32, max: u32, width: usize) {
    let filled = if max == 0 { 0 } else { (current as usize * width / max as usize).min(width) };
    let empty = width - filled;
    let color = hp_color(current, max);
    execute!(
        out,
        Print("["),
        SetForegroundColor(color),
        Print("█".repeat(filled)),
        ResetColor,
        SetForegroundColor(Color::DarkGrey),
        Print("░".repeat(empty)),
        ResetColor,
        Print("]"),
    ).ok();
}

fn element_label(e: &Element) -> &'static str {
    match e {
        Element::Fire      => "Feu",
        Element::Ice       => "Glace",
        Element::Lightning => "Foudre",
        Element::Bleed     => "Saignement",
        Element::Poison    => "Poison",
        Element::Rot       => "Pourriture",
    }
}

// ─── RENDU ───────────────────────────────────────────────────────────────────

fn draw_combat(out: &mut io::Stdout, player: &Player, combat: &Combat, log: &[String], turn: u32, zone: ZoneId) {
    let (tw, _) = terminal::size().unwrap_or((80, 24));
    let w = tw as usize;
    execute!(out, Clear(ClearType::All), MoveTo(0, 0)).ok();

    let ec = zone_enemy_color(zone);

    // ── En-tête ───────────────────────────────────────────────────────────────
    let header = format!("  Combat — Tour {}  ", turn);
    let bar_len = w.saturating_sub(header.len() + 4);
    execute!(
        out,
        SetForegroundColor(ec),
        SetAttribute(Attribute::Bold),
        Print(format!("══{}{}══\r\n\r\n", header, "═".repeat(bar_len))),
        ResetColor,
    ).ok();

    // ── Affichage combattants ─────────────────────────────────────────────────
    let next_row = if combat.enemies.len() == 1 {
        draw_1v1_side_by_side(out, player, &combat.enemies[0], combat, 2, w, ec)
    } else {
        let nr = draw_enemies_columns(out, combat, 2, w, ec);
        execute!(out, MoveTo(0, nr)).ok();
        separator(out, w);
        draw_player_block(out, player, combat);
        nr
    };
    execute!(out, MoveTo(0, next_row)).ok();

    // ── Séparateur ────────────────────────────────────────────────────────────
    separator(out, w);

    // ── Log ───────────────────────────────────────────────────────────────────
    if !log.is_empty() {
        execute!(out, Print("\r\n")).ok();
        for line in log {
            execute!(
                out,
                SetForegroundColor(Color::Grey),
                Print(format!("  > {}\r\n", line)),
                ResetColor,
            ).ok();
        }
        execute!(out, Print("\r\n")).ok();
    }

    // ── Séparateur ────────────────────────────────────────────────────────────
    separator(out, w);

    // ── Actions ───────────────────────────────────────────────────────────────
    execute!(out, Print("  ")).ok();
    print_action(out, "A", "Attaquer", true);
    print_action(out, "S", "Special", combat.player_elemental_cooldown == 0);
    print_action(out, "H", &format!("Estus ({})", player.estus_charges), player.estus_charges > 0);
    print_action(out, "I", "Inventaire", true);
    print_action(out, "F", "Fuir", true);
    execute!(out, Print("\r\n")).ok();

    // ── Cycle élémentaire ─────────────────────────────────────────────────────
    execute!(
        out,
        SetForegroundColor(Color::DarkGrey),
        Print("  Cycle : "),
        ResetColor,
    ).ok();
    let cycle = [
        ("Feu",         Color::Red),
        ("Glace",       Color::Cyan),
        ("Foudre",      Color::Yellow),
        ("Saignement",  Color::DarkRed),
        ("Poison",      Color::Green),
        ("Pourriture",  Color::DarkGreen),
    ];
    for (i, (name, color)) in cycle.iter().enumerate() {
        execute!(out, SetForegroundColor(*color), Print(name), ResetColor).ok();
        if i < cycle.len() - 1 {
            execute!(out, SetForegroundColor(Color::DarkGrey), Print(" > "), ResetColor).ok();
        }
    }
    execute!(
        out,
        SetForegroundColor(Color::DarkGrey),
        Print(" > Feu   (+50% / -25%)\r\n"),
        ResetColor,
    ).ok();

    out.flush().ok();
}

/// Affiche tous les ennemis côte à côte en colonnes égales.
/// Retourne la prochaine ligne libre après le bloc ennemis.
fn draw_enemies_columns(out: &mut io::Stdout, combat: &Combat, start_row: u16, w: usize, ec: Color) -> u16 {
    let n = combat.enemies.len();
    if n == 0 { return start_row; }

    let col_w = w / n;

    let max_h: u16 = combat.enemies.iter().map(|e| {
        if !e.is_alive() { 1u16 }
        else { e.ascii.lines().count().max(3) as u16 }
    }).max().unwrap_or(1);

    for row in 0..max_h {
        for (ci, e) in combat.enemies.iter().enumerate() {
            let x = (ci * col_w + 2) as u16;
            let y = start_row + row;
            execute!(out, MoveTo(x, y)).ok();

            if !e.is_alive() {
                if row == 0 {
                    execute!(
                        out,
                        SetForegroundColor(Color::DarkGrey),
                        Print(format!("{} [mort]", e.name)),
                        ResetColor,
                    ).ok();
                }
                continue;
            }

            let ascii_lines: Vec<&str> = e.ascii.lines().collect();
            // Strip padding commun pour éviter overflow
            let min_indent = ascii_lines.iter()
                .filter(|l| !l.trim().is_empty())
                .map(|l| l.len() - l.trim_start().len())
                .min()
                .unwrap_or(0);
            let max_art_w = col_w.saturating_sub(4); // marge stats
            let ascii_w = ascii_lines.iter()
                .map(|l| if l.len() >= min_indent { l[min_indent..].chars().count() } else { 0 })
                .max()
                .unwrap_or(0)
                .min(max_art_w);
            let elems: String = e.elements.iter().map(|el| element_label(el)).collect::<Vec<_>>().join("/");

            // Colonne ASCII
            if let Some(line) = ascii_lines.get(row as usize) {
                let stripped = if line.len() >= min_indent { &line[min_indent..] } else { line };
                let clipped: String = stripped.chars().take(ascii_w).collect();
                execute!(
                    out,
                    SetForegroundColor(ec),
                    Print(format!("{:<ascii_w$}", clipped)),
                    ResetColor,
                ).ok();
            } else if ascii_w > 0 {
                execute!(out, Print(" ".repeat(ascii_w))).ok();
            }

            if ascii_w > 0 {
                execute!(out, Print("  ")).ok();
            }

            // Stats (3 premières lignes)
            match row as usize {
                0 => { execute!(
                    out,
                    SetForegroundColor(ec),
                    SetAttribute(Attribute::Bold),
                    Print(&e.name),
                    ResetColor,
                ).ok(); }
                1 => {
                    print_hp_bar(out, e.hp, e.max_hp, 14);
                    execute!(
                        out,
                        SetForegroundColor(hp_color(e.hp, e.max_hp)),
                        Print(format!(" {:>3}/{:<3}", e.hp, e.max_hp)),
                        ResetColor,
                    ).ok();
                }
                2 => { execute!(
                    out,
                    SetForegroundColor(Color::DarkYellow),
                    Print(&elems),
                    ResetColor,
                ).ok(); }
                _ => {}
            }
        }
    }

    start_row + max_h + 1
}

fn draw_1v1_side_by_side(
    out: &mut io::Stdout,
    player: &Player,
    enemy: &Enemy,
    combat: &Combat,
    start_row: u16,
    tw: usize,
    ec: Color,
) -> u16 {
    let mid = (tw / 2) as u16;
    let max_hp = player.stats.max_hp();

    let p_ascii: Vec<&str> = player.class.ascii().lines().collect();
    let e_ascii: Vec<&str> = enemy.ascii.lines().collect();

    // strip common indent from enemy art
    let e_indent = e_ascii.iter()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.len() - l.trim_start().len())
        .min().unwrap_or(0);

    let p_art_w = p_ascii.iter().map(|l| l.chars().count()).max().unwrap_or(0)
        .min(mid as usize - 6);
    let e_art_w = e_ascii.iter()
        .map(|l| if l.len() >= e_indent { l[e_indent..].chars().count() } else { 0 })
        .max().unwrap_or(0)
        .min(mid as usize - 6);

    let elems: String = enemy.elements.iter()
        .map(|el| element_label(el)).collect::<Vec<_>>().join("/");

    let rows = p_ascii.len().max(e_ascii.len()).max(4) as u16;

    for i in 0..rows {
        let idx = i as usize;

        // ── Joueur (gauche) ──────────────────────────────────────────────────
        let pc = player.class.color();
        execute!(out, MoveTo(2, start_row + i)).ok();
        let p_line = p_ascii.get(idx).copied().unwrap_or("");
        let p_clipped: String = p_line.chars().take(p_art_w).collect();
        execute!(out,
            SetForegroundColor(pc), SetAttribute(Attribute::Bold),
            Print(format!("{:<p_art_w$}", p_clipped)), ResetColor,
            Print("  ")).ok();
        match idx {
            0 => { execute!(out, SetForegroundColor(pc), SetAttribute(Attribute::Bold), Print(&player.name), ResetColor).ok(); }
            1 => {
                print_hp_bar(out, player.hp, max_hp, 16);
                execute!(out, SetForegroundColor(hp_color(player.hp, max_hp)),
                    Print(format!(" {:>3}/{:<3}", player.hp, max_hp)), ResetColor).ok();
            }
            2 => { execute!(out, SetForegroundColor(Color::Yellow),
                Print(format!("Estus {}/{}  Ames: {}", player.estus_charges, player.max_estus(), player.souls)),
                ResetColor).ok(); }
            3 => {
                if combat.player_elemental_cooldown > 0 {
                    execute!(out, SetForegroundColor(Color::DarkGrey),
                        Print(format!("Special: {} tour(s)", combat.player_elemental_cooldown)), ResetColor).ok();
                } else {
                    execute!(out, SetForegroundColor(Color::Green), SetAttribute(Attribute::Bold),
                        Print("Special: PRET !"), ResetColor).ok();
                }
            }
            _ => {}
        }

        // ── Ennemi (droite) ──────────────────────────────────────────────────
        execute!(out, MoveTo(mid + 2, start_row + i)).ok();
        if enemy.is_alive() {
            let e_line = e_ascii.get(idx).copied().unwrap_or("");
            let stripped = if e_line.len() >= e_indent { &e_line[e_indent..] } else { e_line };
            let e_clipped: String = stripped.chars().take(e_art_w).collect();
            execute!(out,
                SetForegroundColor(ec),
                Print(format!("{:<e_art_w$}", e_clipped)), ResetColor,
                Print("  ")).ok();
            match idx {
                0 => { execute!(out, SetForegroundColor(ec), SetAttribute(Attribute::Bold), Print(&enemy.name), ResetColor).ok(); }
                1 => {
                    print_hp_bar(out, enemy.hp, enemy.max_hp, 14);
                    execute!(out, SetForegroundColor(hp_color(enemy.hp, enemy.max_hp)),
                        Print(format!(" {:>3}/{:<3}", enemy.hp, enemy.max_hp)), ResetColor).ok();
                }
                2 => { execute!(out, SetForegroundColor(Color::DarkYellow), Print(&elems), ResetColor).ok(); }
                _ => {}
            }
        } else if idx == 0 {
            execute!(out, SetForegroundColor(Color::DarkGrey),
                Print(format!("{} [mort]", enemy.name)), ResetColor).ok();
        }
    }

    start_row + rows + 1
}

fn draw_player_block(out: &mut io::Stdout, player: &Player, combat: &Combat) {
    let pc = player.class.color();
    let ascii_lines: Vec<&str> = player.class.ascii().lines().collect();
    let ascii_col = ascii_lines.iter().map(|l| l.chars().count()).max().unwrap_or(0) + 3;
    let max_hp = player.stats.max_hp();

    let row_count = ascii_lines.len().max(4);

    for i in 0..row_count {
        execute!(out, Print("  ")).ok();

        let line = ascii_lines.get(i).copied().unwrap_or("");
        execute!(
            out,
            SetForegroundColor(pc),
            SetAttribute(Attribute::Bold),
            Print(format!("{:<ascii_col$}", line)),
            ResetColor,
        ).ok();

        match i {
            0 => execute!(
                out,
                SetForegroundColor(pc),
                SetAttribute(Attribute::Bold),
                Print(&player.name),
                ResetColor,
            ).ok(),
            1 => {
                print_hp_bar(out, player.hp, max_hp, 16);
                execute!(
                    out,
                    SetForegroundColor(hp_color(player.hp, max_hp)),
                    Print(format!(" {:>3}/{:<3}", player.hp, max_hp)),
                    ResetColor,
                ).ok()
            }
            2 => execute!(
                out,
                SetForegroundColor(Color::Yellow),
                Print(format!("Estus {}/{}  ", player.estus_charges, player.max_estus())),
                ResetColor,
                Print(format!("Ames : {}", player.souls)),
            ).ok(),
            3 => {
                if combat.player_elemental_cooldown > 0 {
                    execute!(
                        out,
                        SetForegroundColor(Color::DarkGrey),
                        Print(format!("Special : {} tour(s)", combat.player_elemental_cooldown)),
                        ResetColor,
                    ).ok()
                } else {
                    execute!(
                        out,
                        SetForegroundColor(Color::Green),
                        SetAttribute(Attribute::Bold),
                        Print("Special : PRET !"),
                        ResetColor,
                    ).ok()
                }
            }
            _ => execute!(out, Print("")).ok(),
        };

        execute!(out, Print("\r\n")).ok();
    }
}

fn separator(out: &mut io::Stdout, w: usize) {
    execute!(
        out,
        Print("\r\n"),
        SetForegroundColor(Color::DarkGrey),
        Print(format!("  {}\r\n\r\n", "─".repeat(w.saturating_sub(4)))),
        ResetColor,
    ).ok();
}

fn print_action(out: &mut io::Stdout, key: &str, label: &str, enabled: bool) {
    if enabled {
        execute!(
            out,
            SetForegroundColor(Color::DarkYellow),
            SetAttribute(Attribute::Bold),
            Print(format!("[{}]", key)),
            ResetColor,
            Print(format!(" {}   ", label)),
        ).ok();
    } else {
        execute!(
            out,
            SetForegroundColor(Color::DarkGrey),
            Print(format!("[{}] {}   ", key, label)),
            ResetColor,
        ).ok();
    }
}
