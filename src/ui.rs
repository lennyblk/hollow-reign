use std::io::{self};

use crossterm::{
    execute,
    style::{
        Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
    },
};

use crate::item::Element;

// ─── PALETTE ─────────────────────────────────────────────────────────────────

/// Texte atténué (hiérarchie visuelle).
pub const DIM: Color = Color::Rgb {
    r: 115,
    g: 115,
    b: 130,
};
/// Filets / séparateurs discrets.
pub const HAIRLINE: Color = Color::Rgb {
    r: 62,
    g: 62,
    b: 74,
};
/// Fond des touches de clavier (keycaps).
pub const KEY_BG: Color = Color::Rgb {
    r: 52,
    g: 52,
    b: 64,
};
/// Fond des keycaps désactivés.
pub const KEY_BG_OFF: Color = Color::Rgb {
    r: 38,
    g: 38,
    b: 46,
};
/// Texte sombre posé sur les fonds colorés (badges, bannières).
pub const INK: Color = Color::Rgb {
    r: 18,
    g: 18,
    b: 24,
};
/// Piste vide des barres de progression.
pub const TRACK: Color = Color::Rgb {
    r: 55,
    g: 55,
    b: 66,
};

// ─── LOG ─────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
pub enum LogKind {
    /// Le joueur inflige des dégâts.
    PlayerHit,
    /// Le joueur en reçoit.
    EnemyHit,
    Heal,
    /// Ticks / statuts élémentaires.
    Effect,
    Info,
}

impl LogKind {
    pub fn icon(&self) -> &'static str {
        match self {
            LogKind::PlayerHit => "▸",
            LogKind::EnemyHit => "▾",
            LogKind::Heal => "✚",
            LogKind::Effect => "◆",
            LogKind::Info => "·",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            LogKind::PlayerHit => Color::Rgb {
                r: 120,
                g: 220,
                b: 160,
            },
            LogKind::EnemyHit => Color::Rgb {
                r: 235,
                g: 100,
                b: 100,
            },
            LogKind::Heal => Color::Rgb {
                r: 120,
                g: 230,
                b: 120,
            },
            LogKind::Effect => Color::Rgb {
                r: 190,
                g: 130,
                b: 255,
            },
            LogKind::Info => DIM,
        }
    }
}

// ─── ÉLÉMENTS ────────────────────────────────────────────────────────────────

pub fn element_color(e: &Element) -> Color {
    match e {
        Element::Fire => Color::Rgb {
            r: 235,
            g: 110,
            b: 50,
        },
        Element::Ice => Color::Rgb {
            r: 90,
            g: 200,
            b: 230,
        },
        Element::Lightning => Color::Rgb {
            r: 240,
            g: 220,
            b: 90,
        },
        Element::Bleed => Color::Rgb {
            r: 205,
            g: 65,
            b: 75,
        },
        Element::Poison => Color::Rgb {
            r: 110,
            g: 200,
            b: 90,
        },
        Element::Rot => Color::Rgb {
            r: 145,
            g: 165,
            b: 60,
        },
    }
}

pub fn element_label(e: &Element) -> &'static str {
    match e {
        Element::Fire => "Feu",
        Element::Ice => "Glace",
        Element::Lightning => "Foudre",
        Element::Bleed => "Saignement",
        Element::Poison => "Poison",
        Element::Rot => "Pourriture",
    }
}

// ─── COMPOSANTS ──────────────────────────────────────────────────────────────

/// En-tête d'écran moderne :
/// `  ▍TITRE  détail`
/// `  ━━━━━━━━──────────────`  (segment accentué + reste discret)
pub fn screen_header(out: &mut io::Stdout, title: &str, detail: &str, accent: Color, w: usize) {
    execute!(
        out,
        Print("\r\n"),
        SetForegroundColor(accent),
        SetAttribute(Attribute::Bold),
        Print(format!("  ▍{}", title)),
        ResetColor,
    )
    .ok();
    if !detail.is_empty() {
        execute!(
            out,
            SetForegroundColor(DIM),
            Print(format!("   {}", detail)),
            ResetColor,
        )
        .ok();
    }
    let seg = 12usize.min(w.saturating_sub(4));
    execute!(
        out,
        Print("\r\n"),
        SetForegroundColor(accent),
        Print(format!("  {}", "━".repeat(seg))),
        SetForegroundColor(HAIRLINE),
        Print(format!("{}\r\n", "─".repeat(w.saturating_sub(seg + 4)))),
        ResetColor,
    )
    .ok();
}

/// Filet discret pleine largeur.
pub fn hairline(out: &mut io::Stdout, w: usize) {
    execute!(
        out,
        SetForegroundColor(HAIRLINE),
        Print(format!("  {}\r\n", "─".repeat(w.saturating_sub(4)))),
        ResetColor,
    )
    .ok();
}

pub fn hp_color(current: u32, max: u32) -> Color {
    if max == 0 {
        return DIM;
    }
    let pct = current * 100 / max;
    if pct > 60 {
        Color::Rgb {
            r: 95,
            g: 210,
            b: 120,
        }
    } else if pct > 30 {
        Color::Rgb {
            r: 235,
            g: 190,
            b: 70,
        }
    } else {
        Color::Rgb {
            r: 235,
            g: 90,
            b: 90,
        }
    }
}

/// Barre de vie moderne : remplissage au 1/8e de cellule près + valeurs.
/// `██████▋░░░░░░ 84/100`
pub fn hp_bar(out: &mut io::Stdout, current: u32, max: u32, width: usize) {
    const PARTIALS: [&str; 8] = ["", "▏", "▎", "▍", "▌", "▋", "▊", "▉"];
    let eighths = if max == 0 {
        0
    } else {
        ((current as u64 * width as u64 * 8) / max as u64).min(width as u64 * 8) as usize
    };
    let full = eighths / 8;
    let part = eighths % 8;
    let color = hp_color(current, max);

    execute!(out, SetForegroundColor(color), Print("█".repeat(full))).ok();
    let mut used = full;
    if part > 0 && full < width {
        execute!(out, Print(PARTIALS[part])).ok();
        used += 1;
    }
    execute!(
        out,
        SetForegroundColor(TRACK),
        Print("░".repeat(width.saturating_sub(used))),
        SetForegroundColor(color),
        SetAttribute(Attribute::Bold),
        Print(format!(" {}", current)),
        ResetColor,
        SetForegroundColor(DIM),
        Print(format!("/{}", max)),
        ResetColor,
    )
    .ok();
}

/// Petit badge : texte sombre gras sur fond coloré. ` Feu `
pub fn badge(out: &mut io::Stdout, label: &str, bg: Color) {
    execute!(
        out,
        SetBackgroundColor(bg),
        SetForegroundColor(INK),
        SetAttribute(Attribute::Bold),
        Print(format!(" {} ", label)),
        ResetColor,
    )
    .ok();
}

/// Touche d'action style keycap : ` A ` sur fond sombre + label.
pub fn keycap(out: &mut io::Stdout, key: &str, label: &str, enabled: bool) {
    if enabled {
        execute!(
            out,
            SetBackgroundColor(KEY_BG),
            SetForegroundColor(Color::White),
            SetAttribute(Attribute::Bold),
            Print(format!(" {} ", key)),
            ResetColor,
            SetForegroundColor(Color::Rgb {
                r: 205,
                g: 205,
                b: 215
            }),
            Print(format!(" {}", label)),
            ResetColor,
            Print("   "),
        )
        .ok();
    } else {
        execute!(
            out,
            SetBackgroundColor(KEY_BG_OFF),
            SetForegroundColor(DIM),
            Print(format!(" {} ", key)),
            ResetColor,
            SetForegroundColor(Color::Rgb {
                r: 82,
                g: 82,
                b: 95
            }),
            Print(format!(" {}", label)),
            ResetColor,
            Print("   "),
        )
        .ok();
    }
}

pub fn banner(out: &mut io::Stdout, text: &str, bg: Color) {
    execute!(
        out,
        SetBackgroundColor(bg),
        SetForegroundColor(INK),
        SetAttribute(Attribute::Bold),
        Print(format!("  {}  ", text)),
        ResetColor,
    )
    .ok();
}
