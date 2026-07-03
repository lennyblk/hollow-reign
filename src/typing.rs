use crossterm::{
    cursor::{Hide, MoveUp, RestorePosition, SavePosition, Show},
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    style::{
        Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
    },
    terminal::{self, Clear, ClearType},
};
use std::io::{self, Write};
use std::time::{Duration, Instant};

use crate::combat::ParryResult;
use crate::phrases::Difficulty;
use crate::ui;

// ─── CONFIG PAR DIFFICULTÉ ───────────────────────────────────────────────────

pub fn time_limit_ms(diff: &Difficulty) -> u64 {
    match diff {
        Difficulty::Short => 4_000,
        Difficulty::Medium => 7_000,
        Difficulty::Long => 22_000,
    }
}

/// % du temps imparti en dessous duquel on obtient Perfect
pub fn perfect_threshold(diff: &Difficulty) -> u64 {
    match diff {
        Difficulty::Short => 40,
        Difficulty::Medium => 45,
        Difficulty::Long => 58,
    }
}

// ─── TYPING CHALLENGE ────────────────────────────────────────────────────────

/// Perfect  = phrase complétée sous le seuil de temps
/// Good     = phrase complétée mais plus lentement
/// Miss     = temps écoulé avant la fin
pub fn typing_challenge(phrase: &str, limit_ms: u64, perfect_pct: u64) -> ParryResult {
    let mut out = io::stdout();
    terminal::enable_raw_mode().expect("crossterm: raw mode requis");
    execute!(out, Hide).ok();

    // Réserve 8 lignes et ancre le curseur pour les redraws
    print!("\n\n\n\n\n\n\n\n");
    out.flush().ok();
    execute!(out, MoveUp(8), SavePosition).ok();

    let chars: Vec<char> = phrase.chars().collect();
    let total = chars.len();
    let mut typed = 0usize;
    let mut wrong = false;
    let start = Instant::now();
    let limit = Duration::from_millis(limit_ms);

    let result = loop {
        let elapsed_ms = start.elapsed().as_millis() as u64;

        if elapsed_ms >= limit_ms {
            break ParryResult::Miss;
        }

        redraw(&mut out, &chars, typed, wrong, limit_ms, elapsed_ms);

        let remaining = limit.saturating_sub(start.elapsed());
        if event::poll(remaining.min(Duration::from_millis(80))).unwrap_or(false) {
            if let Ok(Event::Key(key)) = event::read() {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::Char(c) => {
                        if c == chars[typed] {
                            typed += 1;
                            wrong = false;
                            if typed == total {
                                // Redraw une dernière fois tout en vert avant le résultat
                                let elapsed_ms = start.elapsed().as_millis() as u64;
                                redraw(&mut out, &chars, typed, false, limit_ms, elapsed_ms);

                                let pct = elapsed_ms * 100 / limit_ms;
                                break if pct <= perfect_pct {
                                    ParryResult::Perfect
                                } else {
                                    ParryResult::Good
                                };
                            }
                        } else {
                            wrong = true;
                        }
                    }
                    KeyCode::Esc => break ParryResult::Miss,
                    _ => {}
                }
            }
        }
    };

    show_result(&mut out, &result);

    execute!(out, Show).ok();
    terminal::disable_raw_mode().ok();
    result
}

// ─── REDRAW ──────────────────────────────────────────────────────────────────

fn bar_width() -> usize {
    // "[" + bar + "] XX.Xs\r\n" — on laisse 4 chars de marge de chaque côté
    let term_w = terminal::size().map(|(w, _)| w as usize).unwrap_or(60);
    term_w.saturating_sub(12).max(10)
}

fn redraw(
    out: &mut io::Stdout,
    phrase: &[char],
    typed: usize,
    wrong: bool,
    limit_ms: u64,
    elapsed_ms: u64,
) {
    execute!(out, RestorePosition).ok();

    // Ligne 1 — vide
    execute!(out, Clear(ClearType::CurrentLine), Print("\r\n")).ok();

    // Ligne 2 — label
    execute!(
        out,
        Clear(ClearType::CurrentLine),
        SetForegroundColor(Color::DarkYellow),
        SetAttribute(Attribute::Bold),
        Print("  ▍PARADE   Tapez la phrase :\r\n"),
        ResetColor,
    )
    .ok();

    // Ligne 3 — vide
    execute!(out, Clear(ClearType::CurrentLine), Print("\r\n")).ok();

    // Ligne 4 — phrase colorée (bold)
    execute!(out, Clear(ClearType::CurrentLine), Print("  ")).ok();
    for (i, &ch) in phrase.iter().enumerate() {
        if i < typed {
            execute!(
                out,
                SetForegroundColor(Color::Green),
                SetAttribute(Attribute::Bold),
                Print(ch),
                ResetColor,
            )
            .ok();
        } else if i == typed {
            let color = if wrong { Color::Red } else { Color::White };
            execute!(
                out,
                SetForegroundColor(color),
                SetAttribute(Attribute::Bold),
                SetAttribute(Attribute::Underlined),
                Print(ch),
                ResetColor,
            )
            .ok();
        } else {
            execute!(
                out,
                SetForegroundColor(Color::DarkGrey),
                SetAttribute(Attribute::Bold),
                Print(ch),
                ResetColor,
            )
            .ok();
        }
    }
    execute!(out, Print("\r\n")).ok();

    // Ligne 5 — vide
    execute!(out, Clear(ClearType::CurrentLine), Print("\r\n")).ok();

    // Ligne 6 — barre pleine largeur
    let bw = bar_width();
    let bw_u64 = bw as u64;
    let filled = ((elapsed_ms * bw_u64) / limit_ms).min(bw_u64) as usize;
    let empty = bw - filled;
    let remaining_s = limit_ms.saturating_sub(elapsed_ms) as f32 / 1000.0;
    let bar_color = if filled * 100 / bw < 40 {
        Color::Green
    } else if filled * 100 / bw < 80 {
        Color::Yellow
    } else {
        Color::Red
    };
    let bar = format!("{}{}", "█".repeat(filled), "░".repeat(empty));
    execute!(
        out,
        Clear(ClearType::CurrentLine),
        Print("  ["),
        SetForegroundColor(bar_color),
        Print(&bar),
        ResetColor,
        Print(format!("] {:.1}s\r\n", remaining_s)),
    )
    .ok();

    // Lignes 7-8 — padding
    execute!(out, Clear(ClearType::CurrentLine), Print("\r\n")).ok();
    execute!(out, Clear(ClearType::CurrentLine), Print("\r\n")).ok();

    out.flush().ok();
}

// ─── RESULT FLASH ────────────────────────────────────────────────────────────

fn show_result(out: &mut io::Stdout, result: &ParryResult) {
    execute!(out, RestorePosition).ok();

    // Efface les 8 lignes
    for _ in 0..8 {
        execute!(out, Clear(ClearType::CurrentLine), Print("\r\n")).ok();
    }
    execute!(out, RestorePosition).ok();

    let (label, bg) = match result {
        ParryResult::Perfect => (
            "✦ PARFAIT    +50% degats · aucun degat recu",
            Color::Rgb {
                r: 90,
                g: 220,
                b: 230,
            },
        ),
        ParryResult::Good => (
            "✔ BIEN       +20% degats · 50% degats recus",
            Color::Rgb {
                r: 235,
                g: 190,
                b: 70,
            },
        ),
        ParryResult::Miss => (
            "✖ RATE       -30% degats · 130% degats recus",
            Color::Rgb {
                r: 235,
                g: 90,
                b: 90,
            },
        ),
    };

    execute!(out, Clear(ClearType::CurrentLine), Print("  ")).ok();
    ui::banner(out, label, bg);
    execute!(out, Print("\r\n")).ok();
    out.flush().ok();

    std::thread::sleep(Duration::from_millis(1_200));
}

// ─── PARRY CHALLENGE (timing bar) ────────────────────────────────────────────

/// Perfect  = curseur pile sur la lettre
/// Good     = curseur dans la zone colorée (±2 autour de la lettre)
/// Miss     = raté ou mauvaise touche ou temps écoulé
pub fn parry_challenge() -> ParryResult {
    let mut out = io::stdout();
    terminal::enable_raw_mode().expect("crossterm: raw mode requis");
    execute!(out, Hide).ok();

    print!("\n\n\n\n\n\n\n\n");
    out.flush().ok();
    execute!(out, MoveUp(8), SavePosition).ok();

    let term_w = terminal::size().map(|(w, _)| w as usize).unwrap_or(80);
    let bar_w = term_w.saturating_sub(10).max(20);

    let letter = (b'a' + rand::random::<u8>() % 26) as char;
    const ZONE_HALF: usize = 2;
    const ZONE_W: usize = ZONE_HALF * 2 + 1;

    let range = bar_w.saturating_sub(ZONE_W + ZONE_HALF * 2);
    let target_col = ZONE_HALF
        + if range > 0 {
            rand::random::<usize>() % range
        } else {
            0
        };

    let duration_ms = 3_000u64;
    let start = Instant::now();

    let result = loop {
        let elapsed_ms = start.elapsed().as_millis() as u64;
        if elapsed_ms >= duration_ms {
            break ParryResult::Miss;
        }

        // Curseur qui rebondit 1 fois sur toute la largeur (3s)
        let cycle = (bar_w * 2) as u64;
        let pixel = (elapsed_ms * cycle / duration_ms) % cycle;
        let cursor_col = if pixel < bar_w as u64 {
            pixel as usize
        } else {
            (cycle - pixel) as usize
        };

        draw_parry_bar(
            &mut out,
            bar_w,
            target_col,
            ZONE_HALF,
            letter,
            cursor_col,
            elapsed_ms,
            duration_ms,
        );

        if event::poll(Duration::from_millis(16)).unwrap_or(false) {
            if let Ok(Event::Key(key)) = event::read() {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::Char(c) if c == letter || c == letter.to_ascii_uppercase() => {
                        let diff = (cursor_col as i32 - target_col as i32).unsigned_abs() as usize;
                        break if diff == 0 {
                            ParryResult::Perfect
                        } else if diff <= ZONE_HALF {
                            ParryResult::Good
                        } else {
                            ParryResult::Miss
                        };
                    }
                    KeyCode::Esc | KeyCode::Char(_) => break ParryResult::Miss,
                    _ => {}
                }
            }
        }
    };

    show_result(&mut out, &result);

    execute!(out, Show).ok();
    terminal::disable_raw_mode().ok();
    result
}

fn draw_parry_bar(
    out: &mut io::Stdout,
    bar_w: usize,
    target_col: usize,
    zone_half: usize,
    letter: char,
    cursor_col: usize,
    elapsed_ms: u64,
    duration_ms: u64,
) {
    execute!(out, RestorePosition).ok();

    execute!(out, Clear(ClearType::CurrentLine), Print("\r\n")).ok();

    execute!(
        out,
        Clear(ClearType::CurrentLine),
        SetForegroundColor(Color::DarkYellow),
        SetAttribute(Attribute::Bold),
        Print(format!(
            "  ▍PARRY   Appuyez sur [{}] quand le curseur est dessus :\r\n",
            letter.to_ascii_uppercase()
        )),
        ResetColor,
    )
    .ok();

    execute!(out, Clear(ClearType::CurrentLine), Print("\r\n")).ok();

    let remaining_s = duration_ms.saturating_sub(elapsed_ms) as f32 / 1000.0;
    execute!(out, Clear(ClearType::CurrentLine), Print("  [")).ok();

    for c in 0..bar_w {
        let dist = (c as i32 - target_col as i32).unsigned_abs() as usize;
        let on_zone = dist <= zone_half;
        let is_center = c == target_col;
        let is_cursor = c == cursor_col;

        if is_cursor && on_zone {
            execute!(
                out,
                SetBackgroundColor(Color::Rgb {
                    r: 200,
                    g: 140,
                    b: 0
                }),
                SetForegroundColor(Color::White),
                SetAttribute(Attribute::Bold),
                Print("▌"),
                ResetColor,
            )
            .ok();
        } else if is_cursor {
            execute!(
                out,
                SetForegroundColor(Color::White),
                SetAttribute(Attribute::Bold),
                Print("▌"),
                ResetColor,
            )
            .ok();
        } else if on_zone {
            let ch = if is_center {
                letter.to_ascii_uppercase()
            } else {
                ' '
            };
            execute!(
                out,
                SetBackgroundColor(Color::Rgb {
                    r: 200,
                    g: 140,
                    b: 0
                }),
                SetForegroundColor(Color::Black),
                SetAttribute(Attribute::Bold),
                Print(ch),
                ResetColor,
            )
            .ok();
        } else {
            execute!(
                out,
                SetForegroundColor(Color::DarkGrey),
                Print("░"),
                ResetColor
            )
            .ok();
        }
    }

    execute!(out, Print(format!("] {:.1}s\r\n", remaining_s))).ok();

    for _ in 0..4 {
        execute!(out, Clear(ClearType::CurrentLine), Print("\r\n")).ok();
    }

    out.flush().ok();
}

// ─── COMBO CHALLENGE (séquence QTE) ──────────────────────────────────────────

#[derive(Clone, Copy)]
enum ComboKey {
    Up,
    Down,
    Left,
    Right,
    Char(char),
}

impl ComboKey {
    fn random() -> Self {
        match rand::random::<u8>() % 8 {
            0 => ComboKey::Up,
            1 => ComboKey::Down,
            2 => ComboKey::Left,
            3 => ComboKey::Right,
            _ => ComboKey::Char((b'a' + rand::random::<u8>() % 26) as char),
        }
    }

    fn symbol(&self) -> String {
        match self {
            ComboKey::Up => "↑".to_string(),
            ComboKey::Down => "↓".to_string(),
            ComboKey::Left => "←".to_string(),
            ComboKey::Right => "→".to_string(),
            ComboKey::Char(c) => c.to_ascii_uppercase().to_string(),
        }
    }

    fn matches(&self, code: &KeyCode) -> bool {
        match (self, code) {
            (ComboKey::Up, KeyCode::Up)
            | (ComboKey::Down, KeyCode::Down)
            | (ComboKey::Left, KeyCode::Left)
            | (ComboKey::Right, KeyCode::Right) => true,
            (ComboKey::Char(c), KeyCode::Char(k)) => k.to_ascii_lowercase() == *c,
            _ => false,
        }
    }
}

/// Perfect  = séquence complétée sous 55% du temps
/// Good     = complétée mais plus lentement
/// Miss     = mauvaise touche (combo brisé) ou temps écoulé
///
/// Difficulté : Mob 4 touches/4.5s — Chef/MiniBoss 5 touches/5s — Boss 7 touches/6s
pub fn combo_challenge(diff: Difficulty) -> ParryResult {
    let mut out = io::stdout();
    terminal::enable_raw_mode().expect("crossterm: raw mode requis");
    execute!(out, Hide).ok();

    print!("\n\n\n\n\n\n\n\n");
    out.flush().ok();
    execute!(out, MoveUp(8), SavePosition).ok();

    let (seq_len, limit_ms) = match diff {
        Difficulty::Short => (4usize, 4_500u64),
        Difficulty::Medium => (5, 5_000),
        Difficulty::Long => (7, 6_000),
    };
    const PERFECT_PCT: u64 = 55;

    let seq: Vec<ComboKey> = (0..seq_len).map(|_| ComboKey::random()).collect();
    let mut done = 0usize;
    let start = Instant::now();

    let result = loop {
        let elapsed_ms = start.elapsed().as_millis() as u64;
        if elapsed_ms >= limit_ms {
            break ParryResult::Miss;
        }

        draw_combo(&mut out, &seq, done, limit_ms, elapsed_ms);

        if event::poll(Duration::from_millis(30)).unwrap_or(false) {
            if let Ok(Event::Key(key)) = event::read() {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::Esc => break ParryResult::Miss,
                    KeyCode::Char(_)
                    | KeyCode::Up
                    | KeyCode::Down
                    | KeyCode::Left
                    | KeyCode::Right => {
                        if seq[done].matches(&key.code) {
                            done += 1;
                            if done == seq_len {
                                let elapsed_ms = start.elapsed().as_millis() as u64;
                                draw_combo(&mut out, &seq, done, limit_ms, elapsed_ms);
                                let pct = elapsed_ms * 100 / limit_ms;
                                break if pct <= PERFECT_PCT {
                                    ParryResult::Perfect
                                } else {
                                    ParryResult::Good
                                };
                            }
                        } else {
                            // une seule faute brise le combo
                            break ParryResult::Miss;
                        }
                    }
                    _ => {}
                }
            }
        }
    };

    show_result(&mut out, &result);

    execute!(out, Show).ok();
    terminal::disable_raw_mode().ok();
    result
}

fn draw_combo(out: &mut io::Stdout, seq: &[ComboKey], done: usize, limit_ms: u64, elapsed_ms: u64) {
    execute!(out, RestorePosition).ok();

    // Ligne 1 — vide
    execute!(out, Clear(ClearType::CurrentLine), Print("\r\n")).ok();

    // Ligne 2 — label
    execute!(
        out,
        Clear(ClearType::CurrentLine),
        SetForegroundColor(Color::DarkYellow),
        SetAttribute(Attribute::Bold),
        Print("  ▍COMBO   Reproduisez la séquence (flèches + lettres) :\r\n"),
        ResetColor,
    )
    .ok();

    // Ligne 3 — vide
    execute!(out, Clear(ClearType::CurrentLine), Print("\r\n")).ok();

    // Ligne 4 — séquence
    execute!(out, Clear(ClearType::CurrentLine), Print("    ")).ok();
    for (i, key) in seq.iter().enumerate() {
        let (bg, fg) = if i < done {
            (
                Color::Rgb {
                    r: 70,
                    g: 160,
                    b: 100,
                },
                ui::INK,
            )
        } else if i == done {
            (Color::White, ui::INK)
        } else {
            (ui::KEY_BG, ui::DIM)
        };
        execute!(
            out,
            SetBackgroundColor(bg),
            SetForegroundColor(fg),
            SetAttribute(Attribute::Bold),
            Print(format!(" {} ", key.symbol())),
            ResetColor,
            Print("  "),
        )
        .ok();
    }
    execute!(out, Print("\r\n")).ok();

    // Ligne 5 — vide
    execute!(out, Clear(ClearType::CurrentLine), Print("\r\n")).ok();

    // Ligne 6 — barre de temps (même style que typing)
    let bw = bar_width();
    let bw_u64 = bw as u64;
    let filled = ((elapsed_ms * bw_u64) / limit_ms).min(bw_u64) as usize;
    let empty = bw - filled;
    let remaining_s = limit_ms.saturating_sub(elapsed_ms) as f32 / 1000.0;
    let bar_color = if filled * 100 / bw < 40 {
        Color::Green
    } else if filled * 100 / bw < 80 {
        Color::Yellow
    } else {
        Color::Red
    };
    let bar = format!("{}{}", "█".repeat(filled), "░".repeat(empty));
    execute!(
        out,
        Clear(ClearType::CurrentLine),
        Print("  ["),
        SetForegroundColor(bar_color),
        Print(&bar),
        ResetColor,
        Print(format!("] {:.1}s\r\n", remaining_s)),
    )
    .ok();

    // Lignes 7-8 — padding
    execute!(out, Clear(ClearType::CurrentLine), Print("\r\n")).ok();
    execute!(out, Clear(ClearType::CurrentLine), Print("\r\n")).ok();

    out.flush().ok();
}

// ─── DODGE CHALLENGE (esquive directionnelle) ────────────────────────────────

/// Perfect  = bonne flèche très vite après la révélation
/// Good     = bonne flèche dans la fenêtre
/// Miss     = mauvaise flèche, flèche pressée pendant la feinte, ou trop tard
///
/// Difficulté : Mob fenêtre 1.1s — Chef/MiniBoss 0.9s — Boss 0.65s
pub fn dodge_challenge(diff: Difficulty) -> ParryResult {
    let mut out = io::stdout();
    terminal::enable_raw_mode().expect("crossterm: raw mode requis");
    execute!(out, Hide).ok();

    print!("\n\n\n\n\n\n\n\n");
    out.flush().ok();
    execute!(out, MoveUp(8), SavePosition).ok();

    let (dir_code, dir_label) = match rand::random::<u8>() % 4 {
        0 => (KeyCode::Left, "← GAUCHE"),
        1 => (KeyCode::Right, "→ DROITE"),
        2 => (KeyCode::Up, "↑ HAUT"),
        _ => (KeyCode::Down, "↓ BAS"),
    };

    // Durée d'armement aléatoire : impossible d'anticiper
    let windup_ms = 700 + rand::random::<u64>() % 900;
    let (window_ms, perfect_ms) = match diff {
        Difficulty::Short => (1_100u64, 400u64),
        Difficulty::Medium => (900, 300),
        Difficulty::Long => (650, 230),
    };

    let start = Instant::now();
    let mut result: Option<ParryResult> = None;

    // Phase 1 — l'ennemi arme son coup : presser une flèche = tomber dans la feinte
    while (start.elapsed().as_millis() as u64) < windup_ms {
        draw_dodge_windup(&mut out);
        if event::poll(Duration::from_millis(30)).unwrap_or(false) {
            if let Ok(Event::Key(key)) = event::read() {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::Esc | KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => {
                        result = Some(ParryResult::Miss);
                        break;
                    }
                    _ => {}
                }
            }
        }
        if result.is_some() {
            break;
        }
    }

    // Phase 2 — direction révélée : fenêtre de réaction courte
    if result.is_none() {
        let reveal = Instant::now();
        loop {
            let elapsed_ms = reveal.elapsed().as_millis() as u64;
            if elapsed_ms >= window_ms {
                result = Some(ParryResult::Miss);
                break;
            }

            draw_dodge_reveal(&mut out, dir_label, elapsed_ms, window_ms);

            if event::poll(Duration::from_millis(16)).unwrap_or(false) {
                if let Ok(Event::Key(key)) = event::read() {
                    if key.kind != KeyEventKind::Press {
                        continue;
                    }
                    match key.code {
                        code if code == dir_code => {
                            let t = reveal.elapsed().as_millis() as u64;
                            result = Some(if t <= perfect_ms {
                                ParryResult::Perfect
                            } else {
                                ParryResult::Good
                            });
                            break;
                        }
                        KeyCode::Esc
                        | KeyCode::Up
                        | KeyCode::Down
                        | KeyCode::Left
                        | KeyCode::Right => {
                            result = Some(ParryResult::Miss);
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    let result = result.unwrap_or(ParryResult::Miss);

    show_result(&mut out, &result);

    execute!(out, Show).ok();
    terminal::disable_raw_mode().ok();
    result
}

fn draw_dodge_windup(out: &mut io::Stdout) {
    execute!(out, RestorePosition).ok();

    execute!(out, Clear(ClearType::CurrentLine), Print("\r\n")).ok();

    execute!(
        out,
        Clear(ClearType::CurrentLine),
        SetForegroundColor(Color::DarkYellow),
        SetAttribute(Attribute::Bold),
        Print("  ▍ESQUIVE   L'ennemi arme son coup...\r\n"),
        ResetColor,
    )
    .ok();

    execute!(out, Clear(ClearType::CurrentLine), Print("\r\n")).ok();

    execute!(
        out,
        Clear(ClearType::CurrentLine),
        SetForegroundColor(Color::DarkGrey),
        Print("      attendez la direction — trop tôt = feinte !\r\n"),
        ResetColor,
    )
    .ok();

    for _ in 0..4 {
        execute!(out, Clear(ClearType::CurrentLine), Print("\r\n")).ok();
    }

    out.flush().ok();
}

fn draw_dodge_reveal(out: &mut io::Stdout, dir_label: &str, elapsed_ms: u64, window_ms: u64) {
    execute!(out, RestorePosition).ok();

    execute!(out, Clear(ClearType::CurrentLine), Print("\r\n")).ok();

    execute!(
        out,
        Clear(ClearType::CurrentLine),
        SetForegroundColor(Color::Red),
        SetAttribute(Attribute::Bold),
        Print(format!("  ▍ESQUIVEZ   {}\r\n", dir_label)),
        ResetColor,
    )
    .ok();

    execute!(out, Clear(ClearType::CurrentLine), Print("\r\n")).ok();

    // Barre de fenêtre qui se vide
    let bw = bar_width();
    let bw_u64 = bw as u64;
    let remaining = window_ms.saturating_sub(elapsed_ms);
    let filled = ((remaining * bw_u64) / window_ms).min(bw_u64) as usize;
    let empty = bw - filled;
    let bar = format!("{}{}", "█".repeat(filled), "░".repeat(empty));
    execute!(
        out,
        Clear(ClearType::CurrentLine),
        Print("  ["),
        SetForegroundColor(Color::Red),
        Print(&bar),
        ResetColor,
        Print("]\r\n"),
    )
    .ok();

    for _ in 0..4 {
        execute!(out, Clear(ClearType::CurrentLine), Print("\r\n")).ok();
    }

    out.flush().ok();
}

// ─── CHALLENGE ALÉATOIRE ─────────────────────────────────────────────────────

/// Choisit aléatoirement le mini-jeu :
/// typing 35% / parry 25% / combo 20% / esquive 20%.
pub fn combat_challenge(phrase: &str, limit_ms: u64, perfect_pct: u64, diff: Difficulty) -> ParryResult {
    match rand::random::<u8>() % 100 {
        0..=24 => parry_challenge(),
        25..=44 => combo_challenge(diff),
        45..=64 => dodge_challenge(diff),
        _ => typing_challenge(phrase, limit_ms, perfect_pct),
    }
}
