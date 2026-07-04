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
use crate::inventory_ui::{InventoryResult, open_inventory};
use crate::item::Item;
use crate::phrases::{Difficulty, PhrasePool};
use crate::player::Player;
use crate::typing::{combat_challenge, perfect_threshold, time_limit_ms};
use crate::ui::{self, LogKind};
use crate::zone::{EnemySpawn, ZoneId};

// ─── RÉSULTAT ────────────────────────────────────────────────────────────────

pub enum CombatResult {
    Victory {
        items: Vec<Item>,
        souls: u32,
        boss_killed: bool,
    },
    Defeat,
    Fled,
}

// ─── ÉTAT AFFICHÉ (animations) ───────────────────────────────────────────────

/// Valeurs de PV actuellement affichées à l'écran. Elles convergent vers les
/// vraies valeurs par petits pas → les barres de vie s'animent à chaque coup.
struct Shown {
    player: u32,
    enemies: Vec<u32>,
}

fn step_toward(shown: &mut u32, actual: u32) -> bool {
    if *shown == actual {
        return false;
    }
    let diff = shown.abs_diff(actual);
    let step = (diff / 4).max(1);
    if *shown > actual {
        *shown -= step;
    } else {
        *shown += step;
    }
    true
}

/// Redessine l'écran en animant les barres de vie vers leur valeur réelle.
fn draw_animated(
    out: &mut io::Stdout,
    player: &Player,
    combat: &Combat,
    log: &[(LogKind, String)],
    turn: u32,
    zone: ZoneId,
    shown: &mut Shown,
) {
    loop {
        let mut moving = step_toward(&mut shown.player, player.hp);
        for (i, e) in combat.enemies.iter().enumerate() {
            moving |= step_toward(&mut shown.enemies[i], e.hp);
        }
        draw_combat(out, player, combat, log, turn, zone, shown, None);
        if !moving {
            break;
        }
        std::thread::sleep(Duration::from_millis(30));
    }
}

// ─── EFFETS D'IMPACT (juice) ──────────────────────────────────────────────────

const FX_FRAMES: u8 = 6;
const FX_HITSTOP_MS: u64 = 90;
const FX_FRAME_MS: u64 = 45;

#[derive(Clone, Copy)]
enum FxTarget {
    Player,
    Enemy(usize),
}

/// Animation d'impact transitoire : flash + tremblement + nombre de dégâts.
struct Fx {
    target: FxTarget,
    frame: u8,
    dmg: u32,
    crit: bool,
}

impl Fx {
    fn hits_enemy(&self, i: usize) -> bool {
        matches!(self.target, FxTarget::Enemy(x) if x == i)
    }
    fn hits_player(&self) -> bool {
        matches!(self.target, FxTarget::Player)
    }
    /// Couleur clignotante de l'ASCII pendant l'impact (None = couleur normale).
    fn flash(&self) -> Option<Color> {
        match self.frame {
            0 | 1 => Some(Color::White),
            2 | 3 => Some(Color::Rgb {
                r: 255,
                g: 140,
                b: 140,
            }),
            _ => None,
        }
    }
    /// Décalage horizontal (tremblement) appliqué au personnage frappé.
    fn shake(&self) -> i16 {
        match self.frame {
            0 => 1,
            1 => -1,
            2 => 1,
            3 => -1,
            _ => 0,
        }
    }
}

/// Nombre de dégâts qui « splash » à la place de la barre de vie pendant l'impact.
fn draw_splat(out: &mut io::Stdout, frame: u8, dmg: u32, crit: bool) {
    let color = match (crit, frame) {
        (_, 0) | (_, 1) => Color::White,
        (true, _) => Color::Rgb {
            r: 120,
            g: 230,
            b: 240,
        },
        (false, _) => Color::Rgb {
            r: 255,
            g: 90,
            b: 90,
        },
    };
    let text = if crit {
        format!("✦ -{} ✦", dmg)
    } else {
        format!("-{}", dmg)
    };
    execute!(
        out,
        SetForegroundColor(color),
        SetAttribute(Attribute::Bold),
        Print(text),
        ResetColor,
    )
    .ok();
}

/// Joue l'animation d'impact sur une cible, puis laisse la barre de vie se vider.
/// `shown` conserve encore l'ancienne valeur de PV → le nombre de dégâts s'affiche
/// pendant que le personnage clignote, avant que la barre ne descende réellement.
#[allow(clippy::too_many_arguments)]
fn play_fx(
    out: &mut io::Stdout,
    player: &Player,
    combat: &Combat,
    log: &[(LogKind, String)],
    turn: u32,
    zone: ZoneId,
    shown: &mut Shown,
    target: FxTarget,
    dmg: u32,
    crit: bool,
) {
    if dmg > 0 {
        for frame in 0..FX_FRAMES {
            let fx = Fx {
                target,
                frame,
                dmg,
                crit,
            };
            draw_combat(out, player, combat, log, turn, zone, shown, Some(&fx));
            let ms = if frame == 0 { FX_HITSTOP_MS } else { FX_FRAME_MS };
            std::thread::sleep(Duration::from_millis(ms));
        }
    }
    draw_animated(out, player, combat, log, turn, zone, shown);
}

// ─── POINT D'ENTRÉE ──────────────────────────────────────────────────────────

pub fn run_combat(player: &mut Player, zone: ZoneId, spawns: &[EnemySpawn]) -> CombatResult {
    // Toujours 1 seul ennemi : on affronte le plus important de la salle
    // (boss > miniboss > chef > mob) pour ne jamais rater le boss ni son butin.
    let prio = |t: EnemyType| match t {
        EnemyType::Boss => 3u8,
        EnemyType::MiniBoss => 2,
        EnemyType::MobLeader => 1,
        EnemyType::Mob => 0,
    };
    let mut enemies = Vec::new();
    if let Some(s) = spawns.iter().max_by_key(|s| prio(s.enemy_type)) {
        enemies.push(spawn(zone, s.enemy_type, s.element));
    }
    if enemies.is_empty() {
        return CombatResult::Fled;
    }

    let mut combat = Combat::new(enemies);
    let mut phrases = PhrasePool::new();
    let mut log: Vec<(LogKind, String)> = Vec::new();
    let mut out = io::stdout();

    let mut shown = Shown {
        player: player.hp,
        enemies: combat.enemies.iter().map(|e| e.hp).collect(),
    };

    terminal::enable_raw_mode().ok();
    execute!(out, Hide).ok();

    let result = 'main: loop {
        combat.next_turn();

        // Ticks élémentaires en début de tour
        let tick = Combat::tick_player_effects(player);
        if tick > 0 {
            push_log(
                &mut log,
                LogKind::Effect,
                format!("Effets sur toi : -{} PV", tick),
            );
        }
        for i in 0..combat.enemies.len() {
            let dmg = combat.tick_enemy_effects(i);
            if dmg > 0 {
                let name = combat.enemies[i].name.clone();
                push_log(
                    &mut log,
                    LogKind::Effect,
                    format!("{} : -{} PV (effets)", name, dmg),
                );
            }
        }

        if combat.is_over(player) {
            break 'main end_result(player, &combat, zone);
        }

        // ── Tour du joueur ────────────────────────────────────────────────────
        draw_animated(
            &mut out,
            player,
            &combat,
            &log,
            combat.turn,
            zone,
            &mut shown,
        );

        if !player.status.can_act() {
            push_log(
                &mut log,
                LogKind::Info,
                "Tu es immobilise — tour passe.".to_string(),
            );
            draw_animated(
                &mut out,
                player,
                &combat,
                &log,
                combat.turn,
                zone,
                &mut shown,
            );
        } else {
            // Vide les events résiduels (typing/parry du tour ennemi)
            while event::poll(Duration::from_millis(0)).unwrap_or(false) {
                let _ = event::read();
            }
            'input: loop {
                if let Ok(Event::Key(KeyEvent {
                    code,
                    kind: KeyEventKind::Press,
                    ..
                })) = event::read()
                {
                    match code {
                        // Attaque normale
                        KeyCode::Char('a') | KeyCode::Char('A') => {
                            if let Some(target) = select_target(&mut out, &combat) {
                                let base = base_damage(player);
                                let dmg = combat.player_attack(target, base);
                                let name = combat.enemies[target].name.clone();
                                push_log(
                                    &mut log,
                                    LogKind::PlayerHit,
                                    format!("Tu attaques {} : {} degats.", name, dmg),
                                );
                                play_fx(
                                    &mut out,
                                    player,
                                    &combat,
                                    &log,
                                    combat.turn,
                                    zone,
                                    &mut shown,
                                    FxTarget::Enemy(target),
                                    dmg,
                                    false,
                                );
                            }
                            break 'input;
                        }
                        // Attaque spéciale (mini-jeu)
                        KeyCode::Char('s') | KeyCode::Char('S')
                            if combat.player_elemental_cooldown == 0 =>
                        {
                            if let Some(target) = select_target(&mut out, &combat) {
                                let et = combat.enemies[target].enemy_type;
                                let diff = enemy_difficulty(et);
                                let limit = time_limit_ms(&diff);
                                let pct = perfect_threshold(&diff);
                                let phrase = phrases.next(diff);

                                // combat_challenge gère enable/disable raw mode lui-même
                                let parry = combat_challenge(phrase, limit, pct, diff);
                                // Il a désactivé raw mode — on réactive
                                terminal::enable_raw_mode().ok();
                                execute!(out, Hide).ok();

                                let base = base_damage(player);
                                let (dmg, burst) =
                                    combat.player_elemental_attack(player, target, base, parry);
                                let name = combat.enemies[target].name.clone();
                                if burst > 0 {
                                    push_log(
                                        &mut log,
                                        LogKind::PlayerHit,
                                        format!(
                                            "Attaque speciale sur {} : {} dmg + {} burst!",
                                            name, dmg, burst
                                        ),
                                    );
                                } else {
                                    push_log(
                                        &mut log,
                                        LogKind::PlayerHit,
                                        format!("Attaque speciale sur {} : {} degats.", name, dmg),
                                    );
                                }
                                // Le spécial « crit » (cyan + étincelles) et cumule burst.
                                play_fx(
                                    &mut out,
                                    player,
                                    &combat,
                                    &log,
                                    combat.turn,
                                    zone,
                                    &mut shown,
                                    FxTarget::Enemy(target),
                                    dmg + burst,
                                    true,
                                );
                            }
                            break 'input;
                        }
                        // Estus
                        KeyCode::Char('h') | KeyCode::Char('H') if player.estus_charges > 0 => {
                            player.use_estus();
                            push_log(
                                &mut log,
                                LogKind::Heal,
                                "Tu bois une fiole d'estus.".to_string(),
                            );
                            draw_animated(
                                &mut out,
                                player,
                                &combat,
                                &log,
                                combat.turn,
                                zone,
                                &mut shown,
                            );
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
                            draw_animated(
                                &mut out,
                                player,
                                &combat,
                                &log,
                                combat.turn,
                                zone,
                                &mut shown,
                            );
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
            }
        } // end if can_act / 'input

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
                // Attaque élémentaire → mini-jeu pour le joueur
                let diff = enemy_difficulty(et);
                let limit = time_limit_ms(&diff);
                let pct = perfect_threshold(&diff);
                let phrase = phrases.next(diff);

                let parry = combat_challenge(phrase, limit, pct, diff);
                terminal::enable_raw_mode().ok();
                execute!(out, Hide).ok();

                let dmg = combat.enemy_elemental_attack(player, i, parry);
                push_log(
                    &mut log,
                    LogKind::EnemyHit,
                    format!("{} attaque elementaire : -{} PV.", name, dmg),
                );
                play_fx(
                    &mut out,
                    player,
                    &combat,
                    &log,
                    combat.turn,
                    zone,
                    &mut shown,
                    FxTarget::Player,
                    dmg,
                    false,
                );
            } else {
                let dmg = combat.enemy_attack(player, i);
                if dmg > 0 {
                    push_log(
                        &mut log,
                        LogKind::EnemyHit,
                        format!("{} attaque : -{} PV.", name, dmg),
                    );
                } else {
                    push_log(
                        &mut log,
                        LogKind::Info,
                        format!("{} attaque mais rate.", name),
                    );
                }
                // Coup encaissé → flash rouge + tremblement sur le joueur.
                play_fx(
                    &mut out,
                    player,
                    &combat,
                    &log,
                    combat.turn,
                    zone,
                    &mut shown,
                    FxTarget::Player,
                    dmg,
                    false,
                );
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
    draw_animated(
        &mut out,
        player,
        &combat,
        &log,
        combat.turn,
        zone,
        &mut shown,
    );
    std::thread::sleep(Duration::from_millis(1_500));

    execute!(out, Show).ok();
    terminal::disable_raw_mode().ok();

    result
}

// ─── HELPERS ─────────────────────────────────────────────────────────────────

fn end_result(player: &Player, combat: &Combat, zone: ZoneId) -> CombatResult {
    if player.hp > 0 {
        let boss_killed = combat
            .enemies
            .iter()
            .any(|e| e.enemy_type == EnemyType::Boss && !e.is_alive());
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
    let alive: Vec<(usize, &str)> = combat
        .enemies
        .iter()
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
        Print("Cible :  "),
        ResetColor,
    )
    .ok();
    for (n, (_, name)) in alive.iter().enumerate() {
        ui::keycap(out, &format!("{}", n + 1), name, true);
    }
    execute!(
        out,
        SetForegroundColor(ui::DIM),
        Print("[Esc] Annuler\r\n"),
        ResetColor,
    )
    .ok();
    out.flush().ok();

    loop {
        if let Ok(Event::Key(KeyEvent {
            code,
            kind: KeyEventKind::Press,
            ..
        })) = event::read()
        {
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
        ZoneId::Ashfeld => Color::Rgb {
            r: 180,
            g: 40,
            b: 40,
        }, // saignement
        ZoneId::Gravemoor => Color::Rgb {
            r: 60,
            g: 180,
            b: 60,
        }, // poison
        ZoneId::Rotwood => Color::Rgb {
            r: 100,
            g: 140,
            b: 30,
        }, // pourriture
        ZoneId::TheCinders => Color::Rgb {
            r: 220,
            g: 100,
            b: 20,
        }, // feu
        ZoneId::Frostveil => Color::Rgb {
            r: 60,
            g: 200,
            b: 220,
        }, // glace
        ZoneId::TheVoid => Color::Rgb {
            r: 150,
            g: 40,
            b: 200,
        }, // void
    }
}

fn enemy_difficulty(et: EnemyType) -> Difficulty {
    match et {
        EnemyType::Mob => Difficulty::Short,
        EnemyType::MobLeader | EnemyType::MiniBoss => Difficulty::Medium,
        EnemyType::Boss => Difficulty::Long,
    }
}

fn push_log(log: &mut Vec<(LogKind, String)>, kind: LogKind, msg: String) {
    log.push((kind, msg));
    if log.len() > 6 {
        log.remove(0);
    }
}

/// Badges des éléments d'un ennemi, séparés par un espace.
fn print_element_badges(out: &mut io::Stdout, enemy: &Enemy) {
    for (i, el) in enemy.elements.iter().enumerate() {
        if i > 0 {
            execute!(out, Print(" ")).ok();
        }
        ui::badge(out, ui::element_label(el), ui::element_color(el));
    }
}

/// Charges d'estus en pips : ◆◆◇ + âmes.
fn print_estus_souls(out: &mut io::Stdout, player: &Player) {
    execute!(
        out,
        SetForegroundColor(ui::DIM),
        Print("Estus "),
        ResetColor
    )
    .ok();
    let max = player.max_estus();
    for i in 0..max.min(10) {
        if i < player.estus_charges {
            execute!(
                out,
                SetForegroundColor(Color::Rgb {
                    r: 240,
                    g: 180,
                    b: 60
                }),
                Print("◆"),
                ResetColor
            )
            .ok();
        } else {
            execute!(out, SetForegroundColor(ui::TRACK), Print("◇"), ResetColor).ok();
        }
    }
    execute!(
        out,
        SetForegroundColor(ui::DIM),
        Print("   Ames "),
        ResetColor,
        SetForegroundColor(Color::Rgb {
            r: 240,
            g: 180,
            b: 60
        }),
        SetAttribute(Attribute::Bold),
        Print(format!("{}", player.souls)),
        ResetColor,
    )
    .ok();
}

/// Ligne d'état du spécial : ● PRET / ◌ n tour(s).
fn print_special_status(out: &mut io::Stdout, cooldown: u32) {
    if cooldown > 0 {
        execute!(
            out,
            SetForegroundColor(ui::DIM),
            Print(format!("◌ Special dans {} tour(s)", cooldown)),
            ResetColor,
        )
        .ok();
    } else {
        execute!(
            out,
            SetForegroundColor(Color::Rgb {
                r: 95,
                g: 210,
                b: 120
            }),
            SetAttribute(Attribute::Bold),
            Print("● Special PRET"),
            ResetColor,
        )
        .ok();
    }
}

// ─── RENDU ───────────────────────────────────────────────────────────────────

#[allow(clippy::too_many_arguments)]
fn draw_combat(
    out: &mut io::Stdout,
    player: &Player,
    combat: &Combat,
    log: &[(LogKind, String)],
    turn: u32,
    zone: ZoneId,
    shown: &Shown,
    fx: Option<&Fx>,
) {
    let (tw, _) = terminal::size().unwrap_or((80, 24));
    let w = tw as usize;
    execute!(out, Clear(ClearType::All), MoveTo(0, 0)).ok();

    let ec = zone_enemy_color(zone);

    // ── En-tête (occupe les lignes 0-2) ──────────────────────────────────────
    ui::screen_header(out, "COMBAT", &format!("Tour {}", turn), ec, w);
    execute!(out, Print("\r\n")).ok();

    // ── Affichage combattants (à partir de la ligne 4) ────────────────────────
    let next_row = if combat.enemies.len() == 1 {
        draw_1v1_side_by_side(out, player, &combat.enemies[0], combat, 4, w, ec, shown, fx)
    } else {
        let nr = draw_enemies_columns(out, combat, 4, w, ec, shown, fx);
        execute!(out, MoveTo(0, nr)).ok();
        separator(out, w); // 3 lignes
        let player_rows = draw_player_block(out, player, combat, shown, fx);
        nr + 3 + player_rows
    };
    execute!(out, MoveTo(0, next_row)).ok();

    // ── Séparateur ────────────────────────────────────────────────────────────
    separator(out, w);

    // ── Journal (dernière ligne en clair, anciennes atténuées) ────────────────
    if !log.is_empty() {
        execute!(out, Print("\r\n")).ok();
        for (idx, (kind, line)) in log.iter().enumerate() {
            let is_last = idx == log.len() - 1;
            execute!(
                out,
                SetForegroundColor(kind.color()),
                Print(format!("  {} ", kind.icon())),
                ResetColor,
            )
            .ok();
            if is_last {
                execute!(
                    out,
                    SetForegroundColor(Color::White),
                    SetAttribute(Attribute::Bold),
                    Print(format!("{}\r\n", line)),
                    ResetColor,
                )
                .ok();
            } else {
                execute!(
                    out,
                    SetForegroundColor(ui::DIM),
                    Print(format!("{}\r\n", line)),
                    ResetColor,
                )
                .ok();
            }
        }
        execute!(out, Print("\r\n")).ok();
    }

    // ── Séparateur ────────────────────────────────────────────────────────────
    separator(out, w);

    // ── Actions (keycaps) ─────────────────────────────────────────────────────
    execute!(out, Print("  ")).ok();
    ui::keycap(out, "A", "Attaquer", true);
    ui::keycap(out, "S", "Special", combat.player_elemental_cooldown == 0);
    ui::keycap(
        out,
        "H",
        &format!("Estus ({})", player.estus_charges),
        player.estus_charges > 0,
    );
    ui::keycap(out, "I", "Inventaire", true);
    ui::keycap(out, "F", "Fuir", true);
    execute!(out, Print("\r\n\r\n")).ok();

    // ── Cycle élémentaire ─────────────────────────────────────────────────────
    execute!(
        out,
        SetForegroundColor(ui::DIM),
        Print("  Cycle  "),
        ResetColor,
    )
    .ok();
    let cycle = [
        (
            "Feu",
            Color::Rgb {
                r: 235,
                g: 110,
                b: 50,
            },
        ),
        (
            "Glace",
            Color::Rgb {
                r: 90,
                g: 200,
                b: 230,
            },
        ),
        (
            "Foudre",
            Color::Rgb {
                r: 240,
                g: 220,
                b: 90,
            },
        ),
        (
            "Saignement",
            Color::Rgb {
                r: 205,
                g: 65,
                b: 75,
            },
        ),
        (
            "Poison",
            Color::Rgb {
                r: 110,
                g: 200,
                b: 90,
            },
        ),
        (
            "Pourriture",
            Color::Rgb {
                r: 145,
                g: 165,
                b: 60,
            },
        ),
    ];
    for (i, (name, color)) in cycle.iter().enumerate() {
        execute!(out, SetForegroundColor(*color), Print(*name), ResetColor).ok();
        if i < cycle.len() - 1 {
            execute!(
                out,
                SetForegroundColor(ui::HAIRLINE),
                Print(" › "),
                ResetColor
            )
            .ok();
        }
    }
    execute!(
        out,
        SetForegroundColor(ui::DIM),
        Print(" › Feu   (+50% / -25%)\r\n"),
        ResetColor,
    )
    .ok();

    out.flush().ok();
}

/// Affiche tous les ennemis côte à côte en colonnes égales.
/// Retourne la prochaine ligne libre après le bloc ennemis.
fn draw_enemies_columns(
    out: &mut io::Stdout,
    combat: &Combat,
    start_row: u16,
    w: usize,
    ec: Color,
    shown: &Shown,
    fx: Option<&Fx>,
) -> u16 {
    let n = combat.enemies.len();
    if n == 0 {
        return start_row;
    }

    let col_w = w / n;

    let max_h: u16 = combat
        .enemies
        .iter()
        .enumerate()
        .map(|(i, e)| {
            // Un ennemi mort reste affiché tant que sa barre n'a pas fini de descendre
            if !e.is_alive() && shown.enemies[i] == 0 {
                1u16
            } else {
                e.ascii.lines().count().max(3) as u16
            }
        })
        .max()
        .unwrap_or(1);

    for row in 0..max_h {
        for (ci, e) in combat.enemies.iter().enumerate() {
            // Effet d'impact ciblant cet ennemi (flash / tremblement / splat).
            let efx = fx.filter(|f| f.hits_enemy(ci));
            let dx = efx.map(|f| f.shake()).unwrap_or(0);
            let art_col = efx.and_then(|f| f.flash()).unwrap_or(ec);
            let x = (((ci * col_w + 2) as i16) + dx).max(0) as u16;
            let y = start_row + row;
            execute!(out, MoveTo(x, y)).ok();

            let visually_alive = e.is_alive() || shown.enemies[ci] > 0;
            if !visually_alive {
                if row == 0 {
                    execute!(
                        out,
                        SetForegroundColor(ui::DIM),
                        Print(format!("✝ {}", e.name)),
                        ResetColor,
                    )
                    .ok();
                }
                continue;
            }

            let ascii_lines: Vec<&str> = e.ascii.lines().collect();
            // Strip padding commun pour éviter overflow
            let min_indent = ascii_lines
                .iter()
                .filter(|l| !l.trim().is_empty())
                .map(|l| l.len() - l.trim_start().len())
                .min()
                .unwrap_or(0);
            let max_art_w = col_w.saturating_sub(4); // marge stats
            let ascii_w = ascii_lines
                .iter()
                .map(|l| {
                    if l.len() >= min_indent {
                        l[min_indent..].chars().count()
                    } else {
                        0
                    }
                })
                .max()
                .unwrap_or(0)
                .min(max_art_w);

            // Colonne ASCII
            if let Some(line) = ascii_lines.get(row as usize) {
                let stripped = if line.len() >= min_indent {
                    &line[min_indent..]
                } else {
                    line
                };
                let clipped: String = stripped.chars().take(ascii_w).collect();
                execute!(
                    out,
                    SetForegroundColor(art_col),
                    Print(format!("{:<ascii_w$}", clipped)),
                    ResetColor,
                )
                .ok();
            } else if ascii_w > 0 {
                execute!(out, Print(" ".repeat(ascii_w))).ok();
            }

            if ascii_w > 0 {
                execute!(out, Print("  ")).ok();
            }

            // Stats (3 premières lignes)
            match row as usize {
                0 => {
                    execute!(
                        out,
                        SetForegroundColor(ec),
                        SetAttribute(Attribute::Bold),
                        Print(&e.name),
                        ResetColor,
                    )
                    .ok();
                }
                1 => {
                    // Pendant l'impact : nombre de dégâts à la place de la barre.
                    if let Some(f) = efx {
                        draw_splat(out, f.frame, f.dmg, f.crit);
                    } else {
                        ui::hp_bar(out, shown.enemies[ci], e.max_hp, 14);
                    }
                }
                2 => {
                    print_element_badges(out, e);
                }
                _ => {}
            }
        }
    }

    start_row + max_h + 1
}

#[allow(clippy::too_many_arguments)]
fn draw_1v1_side_by_side(
    out: &mut io::Stdout,
    player: &Player,
    enemy: &Enemy,
    combat: &Combat,
    start_row: u16,
    tw: usize,
    ec: Color,
    shown: &Shown,
    fx: Option<&Fx>,
) -> u16 {
    let mid = (tw / 2) as u16;
    let max_hp = player.stats.max_hp();

    let pfx = fx.filter(|f| f.hits_player());
    let efx = fx.filter(|f| f.hits_enemy(0));

    let p_ascii: Vec<&str> = player.class.ascii().lines().collect();
    let e_ascii: Vec<&str> = enemy.ascii.lines().collect();

    // strip common indent from enemy art
    let e_indent = e_ascii
        .iter()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.len() - l.trim_start().len())
        .min()
        .unwrap_or(0);

    let p_art_w = p_ascii
        .iter()
        .map(|l| l.chars().count())
        .max()
        .unwrap_or(0)
        .min(mid as usize - 6);
    let e_art_w = e_ascii
        .iter()
        .map(|l| {
            if l.len() >= e_indent {
                l[e_indent..].chars().count()
            } else {
                0
            }
        })
        .max()
        .unwrap_or(0)
        .min(mid as usize - 6);

    let rows = p_ascii.len().max(e_ascii.len()).max(4) as u16;

    let enemy_visually_alive = enemy.is_alive() || shown.enemies[0] > 0;

    for i in 0..rows {
        let idx = i as usize;

        // ── Joueur (gauche) ──────────────────────────────────────────────────
        let pc = player.class.color();
        let p_col = pfx.and_then(|f| f.flash()).unwrap_or(pc);
        let p_dx = pfx.map(|f| f.shake()).unwrap_or(0);
        let px = (2i16 + p_dx).max(0) as u16;
        execute!(out, MoveTo(px, start_row + i)).ok();
        let p_line = p_ascii.get(idx).copied().unwrap_or("");
        let p_clipped: String = p_line.chars().take(p_art_w).collect();
        execute!(
            out,
            SetForegroundColor(p_col),
            SetAttribute(Attribute::Bold),
            Print(format!("{:<p_art_w$}", p_clipped)),
            ResetColor,
            Print("  ")
        )
        .ok();
        match idx {
            0 => {
                execute!(
                    out,
                    SetForegroundColor(pc),
                    SetAttribute(Attribute::Bold),
                    Print(&player.name),
                    ResetColor
                )
                .ok();
            }
            1 => {
                if let Some(f) = pfx {
                    draw_splat(out, f.frame, f.dmg, f.crit);
                } else {
                    ui::hp_bar(out, shown.player, max_hp, 16);
                }
            }
            2 => {
                print_estus_souls(out, player);
            }
            3 => {
                print_special_status(out, combat.player_elemental_cooldown);
            }
            _ => {}
        }

        // ── Ennemi (droite) ──────────────────────────────────────────────────
        let e_col = efx.and_then(|f| f.flash()).unwrap_or(ec);
        let e_dx = efx.map(|f| f.shake()).unwrap_or(0);
        let ex = ((mid as i16) + 2 + e_dx).max(0) as u16;
        execute!(out, MoveTo(ex, start_row + i)).ok();
        if enemy_visually_alive {
            let e_line = e_ascii.get(idx).copied().unwrap_or("");
            let stripped = if e_line.len() >= e_indent {
                &e_line[e_indent..]
            } else {
                e_line
            };
            let e_clipped: String = stripped.chars().take(e_art_w).collect();
            execute!(
                out,
                SetForegroundColor(e_col),
                Print(format!("{:<e_art_w$}", e_clipped)),
                ResetColor,
                Print("  ")
            )
            .ok();
            match idx {
                0 => {
                    execute!(
                        out,
                        SetForegroundColor(ec),
                        SetAttribute(Attribute::Bold),
                        Print(&enemy.name),
                        ResetColor
                    )
                    .ok();
                }
                1 => {
                    if let Some(f) = efx {
                        draw_splat(out, f.frame, f.dmg, f.crit);
                    } else {
                        ui::hp_bar(out, shown.enemies[0], enemy.max_hp, 14);
                    }
                }
                2 => {
                    print_element_badges(out, enemy);
                }
                _ => {}
            }
        } else if idx == 0 {
            execute!(
                out,
                SetForegroundColor(ui::DIM),
                Print(format!("✝ {}", enemy.name)),
                ResetColor
            )
            .ok();
        }
    }

    start_row + rows + 1
}

fn draw_player_block(
    out: &mut io::Stdout,
    player: &Player,
    combat: &Combat,
    shown: &Shown,
    fx: Option<&Fx>,
) -> u16 {
    let pc = player.class.color();
    let ascii_lines: Vec<&str> = player.class.ascii().lines().collect();
    let ascii_col = ascii_lines
        .iter()
        .map(|l| l.chars().count())
        .max()
        .unwrap_or(0)
        + 3;
    let max_hp = player.stats.max_hp();

    let pfx = fx.filter(|f| f.hits_player());
    let p_col = pfx.and_then(|f| f.flash()).unwrap_or(pc);
    // Tremblement via l'indentation de gauche (pas de MoveTo ici).
    let lead = (2i16 + pfx.map(|f| f.shake()).unwrap_or(0)).max(0) as usize;

    let row_count = ascii_lines.len().max(4);
    let rows_u16 = row_count as u16;

    for i in 0..row_count {
        execute!(out, Print(" ".repeat(lead))).ok();

        let line = ascii_lines.get(i).copied().unwrap_or("");
        execute!(
            out,
            SetForegroundColor(p_col),
            SetAttribute(Attribute::Bold),
            Print(format!("{:<ascii_col$}", line)),
            ResetColor,
        )
        .ok();

        match i {
            0 => {
                execute!(
                    out,
                    SetForegroundColor(pc),
                    SetAttribute(Attribute::Bold),
                    Print(&player.name),
                    ResetColor,
                )
                .ok();
            }
            1 => {
                if let Some(f) = pfx {
                    draw_splat(out, f.frame, f.dmg, f.crit);
                } else {
                    ui::hp_bar(out, shown.player, max_hp, 16);
                }
            }
            2 => {
                print_estus_souls(out, player);
            }
            3 => {
                print_special_status(out, combat.player_elemental_cooldown);
            }
            _ => {}
        };

        execute!(out, Print("\r\n")).ok();
    }
    rows_u16
}

fn separator(out: &mut io::Stdout, w: usize) {
    execute!(out, Print("\r\n")).ok();
    ui::hairline(out, w);
    execute!(out, Print("\r\n")).ok();
}
