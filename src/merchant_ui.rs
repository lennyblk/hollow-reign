use std::io::{self, Write};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};

use crate::catalog::item_by_name;
use crate::player::Player;
use crate::zone::ZoneId;

// ─── SÉPARATEUR ───────────────────────────────────────────────────────────────

const ART_SEP: u16 = 82; // colonne du │

// ─── ART DU MARCHAND ─────────────────────────────────────────────────────────

const MERCHANT_ART: &str = concat!(
    "                                         . :.                                 \n",
    "                                       =+#*#***:                              \n",
    "                                     .*%%%**###*.   -. :-                     \n",
    "                                     .*%%*+**+#%:.: ::=:+-                    \n",
    "                                      -#*#######+=::--   .:          .:       \n",
    "                                       *#*-.::=*#-:.  .:-:-=.       -##+-.    \n",
    "                                     .+#*=-:.:-#*+      .--+*      =####=     \n",
    "                                 :=+=*###*****##*++      .-=-=    +###*. .    \n",
    "                            .=++:   -##*##***+*****+        .#*--++**: :+*=   \n",
    "                  :*+  :=**=       ****####****##=:-+   :---==##***+--+#*.    \n",
    "                 .=##+:-.          :=**++++***#*+----:=+=-::::-=*###*-==:     \n",
    "             .----=+=:-           :*-***+=++=======-----####******#%%#=       \n",
    "            .-.    ==.          :*+**###**+==++==----==-+*+=-::::--=*##==+:   \n",
    "                   +*          .+#####*#****+======-===--=+=-:::::::=##*+**.  \n",
    "                   -=         .+#####%*#******++==---==-:-+==-::::::=##*##*=  \n",
    "                  .+=.       +*##*###%##****+*=++*+=+++---**=+*-::::=*%***=   \n",
    "                  .#*.    .**#****#**#**#***+++**+++=++-=+++:-*++-::=###*#-   \n",
    "                  .#*.    .*#*****####*=*##***++==-========-=#+*-:::+#%*+*.   \n",
    "                   =-       ###*****#%%%##*##*++==+=-====+**##+==---**#:+*=   \n",
    "                   ..        :##++++**#%%#+*#*#####*########****++=*++*.=*=   \n",
    "                            .*+:-**#*==##%######%%%%######*+++++****+++.      \n",
    "                      .     .-:-=+++*+-+##%#%%####%%*==*#*++++++*##- .*.      \n",
    "                .           :: :+***+==*%%%%######%%#**=:..:=++**.   :#:      \n",
    "                           :=-.-=*#**###%%%%%%####%%%**---.           =.      \n",
    "                              .=*****##*#%##%%####%%#%%+             :**      \n",
    "                              :*****###*#####%%%###%#%%=             =*#      \n",
    "          .-.  ..       .   -:=**#######%%%%%##%###%%#%+             .+=      \n",
    "             -*##*+==++***=  .+*++####**#%%%############                      \n",
    "                             ++*++####***%%%#####*#*****                      \n",
    "                            -****+*%##***##******###*##*+                     \n",
    "                           .+##**+####***####*##########*.                    \n",
    "                          .:++**#*####***#####%###**####*-                    \n",
    "                          .=******#####**#######*****#*#-                     \n",
    "                         .-****+***#**#####*#. -+++:                          \n",
    "                          ++++*+*=+#-  :***-   :==:                           \n",
    "                              -+-#=*:   =--.   :==                            \n",
    "                                        -:.    -*+                            \n",
    "                                        ...    -*-                            \n",
    "                                         :..   =*:                            \n",
    "                                         =*=  .*+-                            \n",
    "                                         .++   +##                            \n",
    "                                          ++. .#*#-                           \n",
    "                                          +*+ :#*++                           \n",
    "                                         :###: -*+                            \n",
    "                                        :#*##                                 \n",
    "                                       .#*+*=                                 \n",
    "                                        .:.                                   ",
);

// ─── CATALOGUE PAR ZONE ──────────────────────────────────────────────────────

struct ShopEntry {
    item_name: &'static str,
    price: u32,
}

const ASHFELD_SHOP: &[ShopEntry] = &[
    ShopEntry {
        item_name: "Bandage",
        price: 10,
    },
    ShopEntry {
        item_name: "Antidote",
        price: 10,
    },
    ShopEntry {
        item_name: "Frost Salts",
        price: 20,
    },
    ShopEntry {
        item_name: "Throwing Knife",
        price: 30,
    },
];

const ROTWOOD_SHOP: &[ShopEntry] = &[
    ShopEntry {
        item_name: "Cooling Salve",
        price: 20,
    },
    ShopEntry {
        item_name: "Purifying Salt",
        price: 30,
    },
    ShopEntry {
        item_name: "Siege Ash",
        price: 20,
    },
    ShopEntry {
        item_name: "Haste Draught",
        price: 30,
    },
    ShopEntry {
        item_name: "Bandage",
        price: 10,
    },
];

const FROSTVEIL_SHOP: &[ShopEntry] = &[
    ShopEntry {
        item_name: "Grounding Wrap",
        price: 50,
    },
    ShopEntry {
        item_name: "Frost Salts",
        price: 20,
    },
    ShopEntry {
        item_name: "Haste Draught",
        price: 30,
    },
    ShopEntry {
        item_name: "Siege Ash",
        price: 20,
    },
    ShopEntry {
        item_name: "Antidote",
        price: 10,
    },
];

fn shop_for_zone(zone: ZoneId) -> (&'static str, &'static [ShopEntry]) {
    match zone {
        ZoneId::Ashfeld => (
            "Bienvenue. Je vends de la qualite. Pas de credit.",
            ASHFELD_SHOP,
        ),
        ZoneId::Rotwood => (
            "Hahaha... tu cherches quelque chose ? J'ai tout, TOUT ! A un prix raisonnable...",
            ROTWOOD_SHOP,
        ),
        ZoneId::Frostveil => (
            "... Tu arrives loin. Prends ce qu'il te faut. Le Vide n'attend pas.",
            FROSTVEIL_SHOP,
        ),
        _ => ("...", &[]),
    }
}

// ─── POINT D'ENTRÉE ──────────────────────────────────────────────────────────

pub fn open_merchant(out: &mut io::Stdout, player: &mut Player, zone: ZoneId) {
    let (greeting, items) = shop_for_zone(zone);
    if items.is_empty() {
        return;
    }

    terminal::enable_raw_mode().ok();
    execute!(out, Hide).ok();

    let mut selected: usize = 0;
    let mut flash: Option<(&str, bool)> = None; // (message, success)

    loop {
        draw(out, player, zone, greeting, items, selected, flash);

        flash = None;

        if let Ok(Event::Key(KeyEvent {
            code,
            kind: KeyEventKind::Press,
            ..
        })) = event::read()
        {
            match code {
                KeyCode::Up if selected > 0 => selected -= 1,
                KeyCode::Down if selected + 1 < items.len() => selected += 1,
                KeyCode::Enter | KeyCode::Char(' ') => {
                    let entry = &items[selected];
                    if player.inventory.len() >= 20 {
                        flash = Some(("Inventaire plein !", false));
                    } else if player.souls < entry.price {
                        flash = Some(("Pas assez d'ames !", false));
                    } else if let Some(item) = item_by_name(entry.item_name) {
                        player.souls -= entry.price;
                        player.pick_up(item);
                        flash = Some(("Achete !", true));
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
    out: &mut io::Stdout,
    player: &Player,
    _zone: ZoneId,
    greeting: &str,
    items: &[ShopEntry],
    selected: usize,
    flash: Option<(&str, bool)>,
) {
    let (tw, th) = terminal::size().unwrap_or((120, 40));
    execute!(out, Clear(ClearType::All), MoveTo(0, 0)).ok();

    // ── En-tête ───────────────────────────────────────────────────────────────
    let header = "  MARCHAND  ";
    let bar_len = (tw as usize).saturating_sub(header.len() + 4);
    execute!(
        out,
        SetForegroundColor(Color::DarkYellow),
        SetAttribute(Attribute::Bold),
        Print(format!("══{}{}══\r\n", header, "═".repeat(bar_len))),
        ResetColor,
    )
    .ok();

    // ── Art du marchand (colonne gauche) ──────────────────────────────────────
    for (row, line) in MERCHANT_ART.lines().enumerate() {
        let y = 1 + row as u16;
        if y >= th.saturating_sub(3) {
            break;
        }
        execute!(
            out,
            MoveTo(0, y),
            SetForegroundColor(Color::DarkYellow),
            Print(line),
            ResetColor,
        )
        .ok();
    }

    // ── Séparateur vertical ───────────────────────────────────────────────────
    for row in 1..th.saturating_sub(2) {
        execute!(
            out,
            MoveTo(ART_SEP, row),
            SetForegroundColor(Color::DarkGrey),
            Print("│"),
            ResetColor,
        )
        .ok();
    }

    // ── Panneau droit ─────────────────────────────────────────────────────────
    let rx = ART_SEP + 2; // colonne de début du panneau droit
    let rw = (tw.saturating_sub(rx + 1)) as usize; // largeur dispo

    // Âmes
    execute!(
        out,
        MoveTo(rx, 2),
        SetForegroundColor(Color::DarkYellow),
        SetAttribute(Attribute::Bold),
        Print(format!("Ames : {}", player.souls)),
        ResetColor,
    )
    .ok();

    // Séparateur
    execute!(
        out,
        MoveTo(rx, 3),
        SetForegroundColor(Color::DarkGrey),
        Print("─".repeat(rw.min(35))),
        ResetColor,
    )
    .ok();

    // Accueil du marchand (word-wrapped)
    let mut greet_row = 4u16;
    for line in wrap_str(greeting, rw.min(35)) {
        execute!(
            out,
            MoveTo(rx, greet_row),
            SetForegroundColor(Color::Cyan),
            Print(line),
            ResetColor,
        )
        .ok();
        greet_row += 1;
    }
    greet_row += 1;

    // Séparateur
    execute!(
        out,
        MoveTo(rx, greet_row),
        SetForegroundColor(Color::DarkGrey),
        Print("─".repeat(rw.min(35))),
        ResetColor,
    )
    .ok();
    greet_row += 1;

    // Liste des items
    let mut item_row = greet_row;
    for (i, entry) in items.iter().enumerate() {
        let is_sel = i == selected;
        let prefix = if is_sel { "> " } else { "  " };
        // Prix aligné à droite dans un champ de 6
        let name_max = rw.min(35).saturating_sub(8);
        let name_trunc = truncate_str(entry.item_name, name_max);
        let label = format!(
            "{}{:<width$}{:>5}a",
            prefix,
            name_trunc,
            entry.price,
            width = name_max
        );

        if is_sel {
            execute!(
                out,
                MoveTo(rx, item_row),
                SetForegroundColor(Color::DarkYellow),
                SetAttribute(Attribute::Bold),
                Print(&label),
                ResetColor,
            )
            .ok();
        } else {
            execute!(
                out,
                MoveTo(rx, item_row),
                SetForegroundColor(Color::White),
                Print(&label),
                ResetColor,
            )
            .ok();
        }
        item_row += 1;
    }

    // Description de l'item sélectionné
    item_row += 1;
    execute!(
        out,
        MoveTo(rx, item_row),
        SetForegroundColor(Color::DarkGrey),
        Print("─".repeat(rw.min(35))),
        ResetColor,
    )
    .ok();
    item_row += 1;

    if let Some(item) = item_by_name(items[selected].item_name) {
        let desc = match &item {
            crate::item::Item::Consumable(c) => c.description,
            crate::item::Item::Weapon(w) => w.description,
            crate::item::Item::Armor(a) => a.description,
            crate::item::Item::Shield(s) => s.description,
        };
        for line in wrap_str(desc, rw.min(35)) {
            if item_row >= th.saturating_sub(4) {
                break;
            }
            execute!(
                out,
                MoveTo(rx, item_row),
                SetForegroundColor(Color::Grey),
                Print(line),
                ResetColor,
            )
            .ok();
            item_row += 1;
        }
    }

    // Flash message
    if let Some((msg, success)) = flash {
        let color = if success { Color::Green } else { Color::Red };
        let frow = th.saturating_sub(4);
        execute!(
            out,
            MoveTo(rx, frow),
            SetForegroundColor(color),
            SetAttribute(Attribute::Bold),
            Print(msg),
            ResetColor,
        )
        .ok();
    }

    // Hint en bas
    let hint = "[↑↓] Choisir   [Entree] Acheter   [Esc] Partir";
    execute!(
        out,
        MoveTo(rx, th.saturating_sub(2)),
        SetForegroundColor(Color::DarkGrey),
        Print(hint),
        ResetColor,
    )
    .ok();

    out.flush().ok();
}

// ─── UTILITAIRES ─────────────────────────────────────────────────────────────

fn wrap_str(text: &str, width: usize) -> Vec<&str> {
    if width == 0 {
        return vec![text];
    }
    let mut lines = Vec::new();
    let mut start = 0;
    while start < text.len() {
        let remaining = &text[start..];
        if remaining.chars().count() <= width {
            lines.push(remaining);
            break;
        }
        let end_byte = remaining
            .char_indices()
            .nth(width)
            .map(|(i, _)| i)
            .unwrap_or(remaining.len());
        let cut_byte = remaining[..end_byte].rfind(' ').unwrap_or(end_byte);
        lines.push(&remaining[..cut_byte]);
        let skip = if remaining[cut_byte..].starts_with(' ') {
            1
        } else {
            0
        };
        start += cut_byte + skip;
    }
    lines
}

fn truncate_str(s: &str, max: usize) -> &str {
    if s.chars().count() <= max {
        return s;
    }
    let end = s.char_indices().nth(max).map(|(i, _)| i).unwrap_or(s.len());
    &s[..end]
}
