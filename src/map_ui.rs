use std::io::{self, Write};

use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyEvent, KeyEventKind},
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};

// ─── CARTE ASCII ──────────────────────────────────────────────────────────────

const MAP: &[&str] = &[
    "╔═════════════════════════════════════════════════════════════════════════════════════╗",
    "║                                                                                     ║",
    "║  ┌───────────────┐   ══════▶   ┌───────────────┐   ══════▶   ┌───────────────┐      ║",
    "║  │    ASHFELD    │             │   GRAVEMOOR   │             │    ROTWOOD    │      ║",
    "║  ├───────────────┤             ├───────────────┤             ├───────────────┤      ║",
    "║  ┌───────────────┐   ◀══════   ┌───────────────┐   ◀══════   ┌───────┴───────┐      ║",
    "║  │   THE  VOID   │             │   FROSTVEIL   │             │  THE CINDERS  │      ║",
    "║  ├───────────────┤             ├───────────────┤             ├───────────────┤      ║",
    "║                                                                                     ║",
    "╚═════════════════════════════════════════════════════════════════════════════════════╝",
];

const MAP_HINT: &str = "[ Appuyez sur une touche pour fermer ]";

// ─── RENDU ────────────────────────────────────────────────────────────────────

pub fn show_world_map(out: &mut io::Stdout) {
    let (tw, th) = terminal::size().unwrap_or((120, 40));
    let map_w = MAP[0].chars().count() as u16;
    let map_h = MAP.len() as u16;

    let col0 = (tw.saturating_sub(map_w)) / 2;
    let row0 = (th.saturating_sub(map_h + 2)) / 2;

    execute!(out, Clear(ClearType::All)).ok();

    for (i, line) in MAP.iter().enumerate() {
        execute!(out, MoveTo(col0, row0 + i as u16)).ok();

        if i == 2 {
            // Titre en or
            execute!(
                out,
                SetForegroundColor(Color::White),
                SetAttribute(Attribute::Bold),
                Print(line),
                ResetColor,
            )
            .ok();
        } else if is_terrain_row(i) {
            draw_terrain_line(out, line, i);
        } else if i == 0 || i == 4 || i == map_h as usize - 1 {
            // Bordures horizontales
            execute!(
                out,
                SetForegroundColor(Color::DarkGrey),
                Print(line),
                ResetColor,
            )
            .ok();
        } else {
            execute!(
                out,
                SetForegroundColor(Color::White),
                Print(line),
                ResetColor,
            )
            .ok();
        }
    }

    // Hint centré sous la carte
    let hint_col = (tw.saturating_sub(MAP_HINT.len() as u16)) / 2;
    execute!(
        out,
        MoveTo(hint_col, row0 + map_h + 1),
        SetForegroundColor(Color::DarkGrey),
        Print(MAP_HINT),
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
}

fn is_terrain_row(row: usize) -> bool {
    (9..=12).contains(&row) || (19..=22).contains(&row)
}

// Colore les 3 blocs de terrain (zone A / B / C) sur la même ligne.
// Structure : ║  │<A>│   gap   │<B>│   gap   │<C>│    ║
fn draw_terrain_line(out: &mut io::Stdout, line: &str, map_row: usize) {
    let (ca, cb, cc) = match map_row {
        9..=12 => (Color::Grey, Color::Cyan, Color::Green),
        19..=22 => (Color::Magenta, Color::Blue, Color::Red),
        _ => (Color::White, Color::White, Color::White),
    };

    let chars: Vec<char> = line.chars().collect();
    let bars: Vec<usize> = chars
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c == '│')
        .map(|(i, _)| i)
        .collect();

    if bars.len() < 6 {
        execute!(
            out,
            SetForegroundColor(Color::White),
            Print(line),
            ResetColor
        )
        .ok();
        return;
    }

    // Segments : [before A] [zone A] [gap AB] [zone B] [gap BC] [zone C] [after C]
    let ranges: &[(usize, usize, Color)] = &[
        (0, bars[0] + 1, Color::DarkGrey),
        (bars[0] + 1, bars[1], ca),
        (bars[1], bars[2] + 1, Color::DarkGrey),
        (bars[2] + 1, bars[3], cb),
        (bars[3], bars[4] + 1, Color::DarkGrey),
        (bars[4] + 1, bars[5], cc),
        (bars[5], chars.len(), Color::DarkGrey),
    ];

    for (start, end, color) in ranges {
        let segment: String = chars[*start..*end].iter().collect();
        execute!(out, SetForegroundColor(*color), Print(&segment), ResetColor).ok();
    }
}
