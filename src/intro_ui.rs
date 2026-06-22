use std::io::{self, Write};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};

use crate::class::Class;

// ─── SÉLECTION DE CLASSE ─────────────────────────────────────────────────────

pub fn class_select() -> Class {
    let mut out = io::stdout();
    terminal::enable_raw_mode().ok();
    execute!(out, Hide).ok();

    let mut selected = 0usize;

    loop {
        draw_class_select(&mut out, selected);

        if let Ok(Event::Key(KeyEvent {
            code,
            kind: KeyEventKind::Press,
            ..
        })) = event::read()
        {
            match code {
                KeyCode::Left => {
                    if selected > 0 {
                        selected -= 1;
                    }
                }
                KeyCode::Right => {
                    if selected < 2 {
                        selected += 1;
                    }
                }
                KeyCode::Enter | KeyCode::Char(' ') => break,
                _ => {}
            }
        }
    }

    execute!(out, Show).ok();
    terminal::disable_raw_mode().ok();

    match selected {
        0 => Class::Knight,
        1 => Class::Mage,
        _ => Class::Rogue,
    }
}

fn draw_class_select(out: &mut io::Stdout, selected: usize) {
    let (tw, th) = terminal::size().unwrap_or((120, 35));
    execute!(out, Clear(ClearType::All), MoveTo(0, 0)).ok();

    // En-tête
    let title = "  CHOISIR VOTRE CLASSE  ";
    let bar = "═".repeat((tw as usize).saturating_sub(title.len() + 4));
    execute!(
        out,
        SetForegroundColor(Color::DarkYellow),
        SetAttribute(Attribute::Bold),
        Print(format!("══{}{}══\r\n", title, bar)),
        ResetColor,
    )
    .ok();

    let names = ["CHEVALIER", "MAGE", "VOLEUR"];
    let descs = [
        "Guerrier. Solide. Implacable.",
        "Mage. Devastateur. Fragile.",
        "Ombre. Rapide. Precise.",
    ];
    let arts = [
        Class::Knight.ascii(),
        Class::Mage.ascii(),
        Class::Rogue.ascii(),
    ];
    let stats_v = [
        Class::Knight.base_stats(),
        Class::Mage.base_stats(),
        Class::Rogue.base_stats(),
    ];

    let col_w = (tw as usize) / 3;
    let art_w = col_w.saturating_sub(3);

    for col in 0..3usize {
        let cx = (col * col_w + 1) as u16;
        let is_sel = col == selected;
        let sel_color = match col {
            0 => Color::Rgb { r: 80, g: 140, b: 255 },  // Knight - bleu
            1 => Color::Rgb { r: 255, g: 140, b: 30 },  // Mage - orange
            _ => Color::Rgb { r: 60, g: 200, b: 80 },   // Voleur - vert
        };
        let color = if is_sel { sel_color } else { Color::DarkGrey };

        // Nom de classe
        let label = if is_sel {
            format!("▶ {}", names[col])
        } else {
            format!("  {}", names[col])
        };
        execute!(
            out,
            MoveTo(cx, 2),
            SetForegroundColor(color),
            SetAttribute(Attribute::Bold),
            Print(&label),
            ResetColor
        )
        .ok();

        // Description
        execute!(
            out,
            MoveTo(cx, 3),
            SetForegroundColor(if is_sel { Color::Grey } else { Color::DarkGrey }),
            Print(trunc(descs[col], art_w)),
            ResetColor
        )
        .ok();

        // Séparateur haut
        execute!(
            out,
            MoveTo(cx, 4),
            SetForegroundColor(Color::DarkGrey),
            Print("─".repeat(art_w)),
            ResetColor
        )
        .ok();

        // Art complet
        let mut art_line_count = 0u16;
        for (i, line) in arts[col].lines().enumerate() {
            let row = 5 + i as u16;
            if row >= th.saturating_sub(3) { break; }
            execute!(
                out,
                MoveTo(cx, row),
                SetForegroundColor(color),
                Print(trunc(line, art_w)),
                ResetColor
            )
            .ok();
            art_line_count = i as u16 + 1;
        }

        // Séparateur bas
        let stat_y = 5 + art_line_count + 1;
        execute!(
            out,
            MoveTo(cx, stat_y - 1),
            SetForegroundColor(Color::DarkGrey),
            Print("─".repeat(art_w)),
            ResetColor
        )
        .ok();

        // Stats
        let s = &stats_v[col];
        let rows: &[(&str, u32)] = &[
            ("Vigueur", s.vigor),
            ("Force", s.strength),
            ("Dexterite", s.dexterity),
            ("Intelligence", s.intelligence),
            ("Foi", s.faith),
            ("Arcane", s.arcane),
            ("Esprit", s.mind),
        ];
        for (j, (name, val)) in rows.iter().enumerate() {
            if stat_y + j as u16 >= th.saturating_sub(3) {
                break;
            }
            execute!(
                out,
                MoveTo(cx, stat_y + j as u16),
                SetForegroundColor(color),
                Print(format!("{:<14}{:>3}", name, val)),
                ResetColor
            )
            .ok();
        }
    }

    // Hint
    let hint = "[←→] Choisir    [Entree] Confirmer";
    let hx = (tw.saturating_sub(hint.len() as u16)) / 2;
    execute!(
        out,
        MoveTo(0, th.saturating_sub(2)),
        SetForegroundColor(Color::DarkGrey),
        Print(format!("  {}", "─".repeat((tw as usize).saturating_sub(4)))),
        ResetColor
    )
    .ok();
    execute!(
        out,
        MoveTo(hx, th.saturating_sub(1)),
        SetForegroundColor(Color::DarkGrey),
        Print(hint),
        ResetColor
    )
    .ok();

    out.flush().ok();
}

// ─── ÉCRANS DE LORE ──────────────────────────────────────────────────────────

const PAGES: &[(&str, &[&str])] = &[
    (
        "LES TERRES D'HOLLOW REIGN",
        &[
            "",
            "Il y a une eternite, les terres que l'on nomme Hollow Reign",
            "baignaient dans la lumiere.",
            "",
            "Six royaumes vivaient sous la Couronne Unifiee —",
            "Ashfeld le forgeron, Gravemoor la gardienne des morts,",
            "Rotwood la sauvage, les Cendres brulantes,",
            "le Voile de Givre, et en leur coeur : le Vide silencieux.",
            "",
            "Puis vint le Roi Creux.",
            "",
            "Nul ne sait ce qu'il fut avant sa chute.",
            "On dit qu'il but a la source interdite pour arracher l'immortalite",
            "— et qu'il y perdit son ame, sa chair, tout ce qui le rendait humain.",
            "",
            "Il ne reste de lui qu'une couronne vide et une volonte de destruction.",
            "",
            "Aujourd'hui, ses ombres rongent les terres.",
            "Le Vide s'etend. Les graces s'eteignent une a une.",
        ],
    ),
    (
        "LE REVENANT",
        &[
            "",
            "Vous etiez mort.",
            "",
            "Une Grace vous a trouve dans le noir —",
            "une flamme oubliee, refusant de s'eteindre.",
            "Elle vous a rendu a la vie sans vous demander votre avis.",
            "",
            "Sa mission, et donc la votre :",
            "traverser les terres brisees d'Hollow Reign,",
            "atteindre Le Vide,",
            "et mettre fin au regne du Roi Creux.",
            "",
            "D'autres avant vous ont essaye.",
            "Leurs cendres reposent quelque part sur la route.",
            "",
            "Vous n'etes pas les premiers.",
            "Vous esperez etre les derniers.",
        ],
    ),
    (
        "LA LANGUE DES CENDRES",
        &[
            "",
            "Mais vous n'etes pas ordinaire.",
            "",
            "Des votre reveil, une Voix s'est installee dans votre esprit.",
            "Ancienne. Brisee. Terriblement puissante.",
            "",
            "En plein coeur du combat, elle murmure —",
            "des mots, des noms, des formules que rien ne devrait",
            "encore prononcer.",
            "",
            "Si vous les retranscrivez dans le chaos de la bataille,",
            "vos ennemis s'effondrent avant meme de comprendre",
            "ce qui leur arrive.",
            "",
            "Les anciens appelaient ce don : la Langue des Cendres.",
            "",
            "Apprenez a l'ecouter.",
            "Votre survie en depend.",
        ],
    ),
    (
        "EN ROUTE",
        &[
            "",
            "La route est longue. Cinq terres a traverser.",
            "",
            "Reposez-vous aux Graces — elles vous soigneront",
            "et vous permettront de voyager entre elles.",
            "",
            "Chaque ame prelevee sur vos ennemis peut vous rendre plus fort.",
            "Ne les gaspillez pas.",
            "",
            "Le Roi Creux attend dans l'obscurite du Vide.",
            "",
            "Allez.",
        ],
    ),
];

pub fn show_intro() {
    let mut out = io::stdout();
    terminal::enable_raw_mode().ok();
    execute!(out, Hide).ok();

    let total = PAGES.len();
    for (i, (title, lines)) in PAGES.iter().enumerate() {
        draw_lore_page(&mut out, lines, title, i + 1, total);
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

    execute!(out, Show).ok();
    terminal::disable_raw_mode().ok();
}

fn draw_lore_page(out: &mut io::Stdout, lines: &[&str], title: &str, page: usize, total: usize) {
    let (tw, th) = terminal::size().unwrap_or((120, 35));
    execute!(out, Clear(ClearType::All), MoveTo(0, 0)).ok();

    // En-tête
    let header = format!("  {}  ", title);
    let bar = "═".repeat((tw as usize).saturating_sub(header.len() + 4));
    execute!(
        out,
        SetForegroundColor(Color::DarkYellow),
        SetAttribute(Attribute::Bold),
        Print(format!("══{}{}══\r\n", header, bar)),
        ResetColor,
    )
    .ok();

    // Corps centré verticalement
    let text_h = lines.len() as u16;
    let usable = th.saturating_sub(6);
    let start_y = (usable.saturating_sub(text_h) / 2 + 2).max(2);

    for (i, &line) in lines.iter().enumerate() {
        let row = start_y + i as u16;
        if row >= th.saturating_sub(3) {
            break;
        }
        if line.is_empty() {
            continue;
        }
        let cx = (tw.saturating_sub(line.len() as u16)) / 2;
        execute!(
            out,
            MoveTo(cx, row),
            SetForegroundColor(Color::Grey),
            Print(line),
            ResetColor
        )
        .ok();
    }

    // Barre + hint + pagination
    execute!(
        out,
        MoveTo(0, th.saturating_sub(3)),
        SetForegroundColor(Color::DarkGrey),
        Print(format!("  {}", "─".repeat((tw as usize).saturating_sub(4)))),
        ResetColor
    )
    .ok();

    let page_label = format!("[{}/{}]", page, total);
    execute!(
        out,
        MoveTo(2, th.saturating_sub(2)),
        SetForegroundColor(Color::DarkGrey),
        Print(&page_label),
        ResetColor
    )
    .ok();

    let hint = if page == total {
        "[ Appuyez pour commencer ]"
    } else {
        "[ Appuyez pour continuer ]"
    };
    let hx = (tw.saturating_sub(hint.len() as u16)) / 2;
    execute!(
        out,
        MoveTo(hx, th.saturating_sub(2)),
        SetForegroundColor(Color::DarkGrey),
        Print(hint),
        ResetColor
    )
    .ok();

    out.flush().ok();
}

// ─── UTILITAIRE ──────────────────────────────────────────────────────────────

fn trunc(s: &str, max: usize) -> &str {
    let mut end = s.len();
    let mut count = 0;
    for (i, _) in s.char_indices() {
        if count == max {
            end = i;
            break;
        }
        count += 1;
    }
    &s[..end]
}
