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
        Print("  PARADE !  Tapez la phrase :\r\n"),
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

    let (label, color) = match result {
        ParryResult::Perfect => ("  >> PARFAIT !   +50% dmg  /  0 dmg recu <<", Color::Cyan),
        ParryResult::Good => (
            "  >> BIEN !       +20% dmg  /  50% dmg recu <<",
            Color::Yellow,
        ),
        ParryResult::Miss => (
            "  >> RATE !       -30% dmg  /  130% dmg recu <<",
            Color::Red,
        ),
    };

    execute!(
        out,
        Clear(ClearType::CurrentLine),
        SetForegroundColor(color),
        SetAttribute(Attribute::Bold),
        Print(label),
        ResetColor,
        Print("\r\n"),
    )
    .ok();
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
            "  PARRY !  Appuyez sur [{}] quand le curseur est dessus :\r\n",
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

// ─── CHALLENGE ALÉATOIRE ─────────────────────────────────────────────────────

/// Choisit aléatoirement entre typing_challenge (40%) et parry_challenge (60%).
pub fn combat_challenge(phrase: &str, limit_ms: u64, perfect_pct: u64) -> ParryResult {
    if rand::random::<u8>() < 80 {
        parry_challenge()
    } else {
        typing_challenge(phrase, limit_ms, perfect_pct)
    }
}
