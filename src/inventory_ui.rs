use std::io::{self, Write};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};

use crate::class::Class;
use crate::equipment::Equipment;
use crate::item::{EquipmentSlot, Item};
use crate::player::Player;

// ─── RÉSULTAT ────────────────────────────────────────────────────────────────

pub enum InventoryResult {
    Closed,
    ConsumedItem, // en combat = consomme le tour du joueur
}

#[derive(PartialEq, Clone, Copy)]
enum Tab {
    Inventory,
    Equipment,
}

enum ItemAction {
    None,
    Consumed,
    Equipped,
}

// ─── POINT D'ENTRÉE ──────────────────────────────────────────────────────────

/// `inline = false` → plein écran (navigation).
/// `inline = true`  → panel compact en dessous du combat (raw mode déjà actif).
pub fn open_inventory(out: &mut io::Stdout, player: &mut Player, inline: bool) -> InventoryResult {
    // Garantit qu'il existe au moins un slot d'équipement
    if player.equipment.is_empty() {
        player.equipment.push(Equipment::empty());
    }

    if !inline {
        terminal::enable_raw_mode().ok();
        execute!(out, Hide).ok();
    }

    let mut tab = Tab::Inventory;
    let mut selected: usize = 0;
    let mut consumed = false;

    'main: loop {
        let w = terminal::size().map(|(w, _)| w as usize).unwrap_or(80);
        let h = terminal::size().map(|(_, h)| h as usize).unwrap_or(24);

        if inline {
            draw_inline(out, player, tab, selected, w);
        } else {
            draw_fullscreen(out, player, tab, selected, w, h);
        }

        let list_len = slot_count(player, tab);

        if let Ok(Event::Key(KeyEvent {
            code,
            kind: KeyEventKind::Press,
            ..
        })) = event::read()
        {
            match code {
                // Changer d'onglet
                KeyCode::Char('i') | KeyCode::Char('I') if tab != Tab::Inventory => {
                    tab = Tab::Inventory;
                    selected = 0;
                }
                KeyCode::Char('p') | KeyCode::Char('P') if tab != Tab::Equipment => {
                    tab = Tab::Equipment;
                    selected = 0;
                }

                // Navigation
                KeyCode::Up if selected > 0 => selected -= 1,
                KeyCode::Down if selected + 1 < list_len => selected += 1,

                // Utiliser / Équiper (onglet Inventaire)
                KeyCode::Enter | KeyCode::Char('u') | KeyCode::Char('U')
                    if tab == Tab::Inventory =>
                {
                    match use_or_equip(player, selected) {
                        ItemAction::Consumed => {
                            consumed = true;
                            clamp_selected(&mut selected, player.inventory.len());
                            if inline {
                                break 'main; // en combat, le tour est consommé
                            }
                        }
                        ItemAction::Equipped => {
                            clamp_selected(&mut selected, player.inventory.len());
                        }
                        ItemAction::None => {}
                    }
                }

                // Déséquiper (onglet Equipement)
                KeyCode::Enter
                | KeyCode::Char('r')
                | KeyCode::Char('R')
                | KeyCode::Char('u')
                | KeyCode::Char('U')
                    if tab == Tab::Equipment =>
                {
                    unequip_slot(player, selected);
                    clamp_selected(&mut selected, slot_count(player, Tab::Equipment));
                }

                // Fermer
                KeyCode::Esc => break 'main,
                _ => {}
            }
        }
    }

    if !inline {
        execute!(out, Show).ok();
        terminal::disable_raw_mode().ok();
    }

    if consumed {
        InventoryResult::ConsumedItem
    } else {
        InventoryResult::Closed
    }
}

// ─── ACTIONS ITEMS ───────────────────────────────────────────────────────────

fn use_or_equip(player: &mut Player, index: usize) -> ItemAction {
    if index >= player.inventory.len() {
        return ItemAction::None;
    }

    // Détermine le type AVANT de muter (borrow immutable se termine ici)
    let is_consumable = matches!(&player.inventory[index], Item::Consumable(_));
    let slot = player.inventory[index].slot();

    if is_consumable {
        let item = player.inventory.remove(index);
        if let Item::Consumable(cd) = item {
            if player.use_consumable(&cd.effect) {
                ItemAction::Consumed
            } else {
                // Effet impossible hors combat (DealDamage etc.) → remet en place
                player.inventory.insert(index, Item::Consumable(cd));
                ItemAction::None
            }
        } else {
            ItemAction::None
        }
    } else if let Some(slot) = slot {
        let item = player.inventory.remove(index);
        let eq = &mut player.equipment[0];
        let old = eq.unequip(slot);
        eq.equip(item);
        if let Some(old_item) = old {
            player.inventory.push(old_item);
        }
        ItemAction::Equipped
    } else {
        ItemAction::None
    }
}

fn unequip_slot(player: &mut Player, selected: usize) {
    let eq = &mut player.equipment[0];
    let item = match selected {
        0 => eq.unequip(EquipmentSlot::Weapon),
        1 => eq.unequip(EquipmentSlot::Armor),
        2 => eq.unequip(EquipmentSlot::Shield),
        n => eq.unequip_consumable(n - 3),
    };
    if let Some(item) = item {
        player.pick_up(item);
    }
}

fn slot_count(player: &Player, tab: Tab) -> usize {
    match tab {
        Tab::Inventory => player.inventory.len(),
        Tab::Equipment => 8, // weapon + armor + shield + 5 consommables
    }
}

fn clamp_selected(selected: &mut usize, len: usize) {
    if len == 0 {
        *selected = 0;
    } else if *selected >= len {
        *selected = len - 1;
    }
}

// ─── HELPERS AFFICHAGE ───────────────────────────────────────────────────────

fn item_type_label(item: &Item) -> &'static str {
    match item {
        Item::Weapon(_) => "Arme",
        Item::Armor(_) => "Armure",
        Item::Shield(_) => "Bouclier",
        Item::Consumable(_) => "Consommable",
    }
}

fn item_color(item: &Item) -> Color {
    match item {
        Item::Weapon(_) => Color::Red,
        Item::Armor(_) => Color::Blue,
        Item::Shield(_) => Color::Cyan,
        Item::Consumable(_) => Color::Green,
    }
}

fn class_label(class: &Class) -> &'static str {
    match class {
        Class::Knight => "Chevalier",
        Class::Mage => "Mage",
        Class::Rogue => "Rodeur",
    }
}

fn player_atk(player: &Player) -> u32 {
    5 + player.stats.strength + player.stats.dexterity / 2
}

fn player_def(player: &Player) -> u32 {
    let eq = match player.equipment.first() {
        Some(e) => e,
        None => return 0,
    };
    let armor = match &eq.armor {
        Some(Item::Armor(a)) => a.defense,
        _ => 0,
    };
    let is_2h = matches!(&eq.weapon, Some(Item::Weapon(w)) if w.two_handed);
    let shield = if !is_2h {
        match &eq.shield {
            Some(Item::Shield(s)) => s.defense,
            _ => 0,
        }
    } else {
        0
    };
    armor + shield
}

// ─── RENDU PLEIN ÉCRAN ───────────────────────────────────────────────────────

fn draw_fullscreen(
    out: &mut io::Stdout,
    player: &Player,
    tab: Tab,
    selected: usize,
    w: usize,
    h: usize,
) {
    execute!(out, Clear(ClearType::All), MoveTo(0, 0)).ok();

    // ── En-tête ───────────────────────────────────────────────────────────────
    let title = if tab == Tab::Inventory {
        "Inventaire"
    } else {
        "Équipement"
    };
    let header = format!("  {}  ", title);
    let bar_len = w.saturating_sub(header.len() + 4);
    execute!(
        out,
        SetForegroundColor(Color::DarkYellow),
        SetAttribute(Attribute::Bold),
        Print(format!("══{}{}══\r\n", header, "═".repeat(bar_len))),
        ResetColor,
    )
    .ok();

    // Positions des colonnes
    const MID_X: u16 = 15;
    const RIGHT_X: u16 = 40;
    const START_ROW: u16 = 2;

    // ── Colonne gauche : ASCII ────────────────────────────────────────────────
    let ascii_lines: Vec<&str> = player.class.ascii().lines().collect();
    for (i, line) in ascii_lines.iter().enumerate() {
        execute!(
            out,
            MoveTo(2, START_ROW + i as u16),
            SetForegroundColor(Color::Cyan),
            SetAttribute(Attribute::Bold),
            Print(line),
            ResetColor,
        )
        .ok();
    }

    // ── Colonne milieu : Stats ────────────────────────────────────────────────
    let max_hp = player.stats.max_hp();
    let stat_rows: &[(&str, &dyn std::fmt::Display)] = &[]; // placeholder — drawn manually below

    let mid_data: Vec<(Option<&str>, String, Color)> = vec![
        (None, player.name.clone(), Color::White),
        (
            None,
            class_label(&player.class).to_string(),
            Color::DarkYellow,
        ),
        (None, "─".repeat(18), Color::DarkGrey),
        (
            Some("PV       "),
            format!("{}/{}", player.hp, max_hp),
            Color::White,
        ),
        (
            Some("Vigueur  "),
            format!("{}", player.stats.vigor),
            Color::Grey,
        ),
        (
            Some("Force    "),
            format!("{}", player.stats.strength),
            Color::Grey,
        ),
        (
            Some("Dexterite"),
            format!("{}", player.stats.dexterity),
            Color::Grey,
        ),
        (
            Some("Intel.   "),
            format!("{}", player.stats.intelligence),
            Color::Grey,
        ),
        (
            Some("Foi      "),
            format!("{}", player.stats.faith),
            Color::Grey,
        ),
        (
            Some("Arcane   "),
            format!("{}", player.stats.arcane),
            Color::Grey,
        ),
        (
            Some("Mental   "),
            format!("{}", player.stats.mind),
            Color::Grey,
        ),
        (None, "─".repeat(18), Color::DarkGrey),
        (
            Some("Ames     "),
            format!("{}", player.souls),
            Color::Yellow,
        ),
        (
            Some("ATK      "),
            format!("{}", player_atk(player)),
            Color::Red,
        ),
        (
            Some("DEF      "),
            format!("{}", player_def(player)),
            Color::Blue,
        ),
    ];

    for (i, (label, value, color)) in mid_data.iter().enumerate() {
        execute!(out, MoveTo(MID_X, START_ROW + i as u16)).ok();
        match label {
            None => execute!(
                out,
                SetForegroundColor(*color),
                SetAttribute(if i == 0 {
                    Attribute::Bold
                } else {
                    Attribute::Reset
                }),
                Print(value),
                ResetColor,
            )
            .ok(),
            Some(lbl) => execute!(
                out,
                SetForegroundColor(Color::DarkGrey),
                Print(format!("{}: ", lbl)),
                ResetColor,
                SetForegroundColor(*color),
                Print(value),
                ResetColor,
            )
            .ok(),
        };
    }

    // ── Colonne droite : Onglets ──────────────────────────────────────────────
    execute!(out, MoveTo(RIGHT_X, START_ROW)).ok();
    print_tab_label(out, "I", "Inventaire", tab == Tab::Inventory);
    execute!(out, Print("   ")).ok();
    print_tab_label(out, "P", "Equipement", tab == Tab::Equipment);

    execute!(
        out,
        MoveTo(RIGHT_X, START_ROW + 1),
        SetForegroundColor(Color::DarkGrey),
        Print("─".repeat(w.saturating_sub(RIGHT_X as usize + 1))),
        ResetColor,
    )
    .ok();

    // ── Colonne droite : Contenu ──────────────────────────────────────────────
    let list_row = START_ROW + 2;
    let visible = h.saturating_sub(list_row as usize + 3);
    let scroll = if selected >= visible && visible > 0 {
        selected + 1 - visible
    } else {
        0
    };

    match tab {
        Tab::Inventory => {
            draw_inventory_list(out, player, selected, RIGHT_X, list_row, visible, scroll)
        }
        Tab::Equipment => draw_equipment_list(out, player, selected, RIGHT_X, list_row),
    }

    // ── Pied de page ─────────────────────────────────────────────────────────
    let footer = h as u16 - 2;
    execute!(
        out,
        MoveTo(0, footer),
        SetForegroundColor(Color::DarkGrey),
        Print(format!("  {}", "─".repeat(w.saturating_sub(4)))),
        ResetColor,
    )
    .ok();
    let help = match tab {
        Tab::Inventory => {
            "[↑↓] Naviguer   [U/Enter] Utiliser ou Equiper   [P] Equipement   [Esc] Fermer"
        }
        Tab::Equipment => {
            "[↑↓] Naviguer   [U/R/Enter] Desequiper          [I] Inventaire [Esc] Fermer"
        }
    };
    execute!(
        out,
        MoveTo(2, footer + 1),
        SetForegroundColor(Color::DarkGrey),
        Print(help),
        ResetColor,
    )
    .ok();

    out.flush().ok();
}

fn draw_inventory_list(
    out: &mut io::Stdout,
    player: &Player,
    selected: usize,
    x: u16,
    start_row: u16,
    visible: usize,
    scroll: usize,
) {
    if player.inventory.is_empty() {
        execute!(
            out,
            MoveTo(x, start_row),
            SetForegroundColor(Color::DarkGrey),
            Print("(inventaire vide)"),
            ResetColor,
        )
        .ok();
        return;
    }
    for (vi, inv_idx) in (scroll..).take(visible).enumerate() {
        if inv_idx >= player.inventory.len() {
            break;
        }
        let row = start_row + vi as u16;
        let item = &player.inventory[inv_idx];
        let is_sel = inv_idx == selected;
        execute!(out, MoveTo(x, row)).ok();
        if is_sel {
            execute!(
                out,
                SetForegroundColor(Color::DarkYellow),
                SetAttribute(Attribute::Bold),
                Print(format!("{:>2} > ", inv_idx + 1)),
                ResetColor,
            )
            .ok();
        } else {
            execute!(
                out,
                SetForegroundColor(Color::DarkGrey),
                Print(format!("{:>2}   ", inv_idx + 1)),
                ResetColor,
            )
            .ok();
        }
        execute!(
            out,
            SetForegroundColor(item_color(item)),
            Print(item.name()),
            ResetColor,
            SetForegroundColor(Color::DarkGrey),
            Print(format!("  [{}]", item_type_label(item))),
            ResetColor,
        )
        .ok();
    }
}

fn draw_equipment_list(
    out: &mut io::Stdout,
    player: &Player,
    selected: usize,
    x: u16,
    start_row: u16,
) {
    let eq = &player.equipment[0];

    let main_slots: [(&str, Option<&Item>); 3] = [
        ("Arme    ", eq.weapon.as_ref()),
        ("Armure  ", eq.armor.as_ref()),
        ("Bouclier", eq.shield.as_ref()),
    ];

    for (i, (label, opt)) in main_slots.iter().enumerate() {
        let row = start_row + i as u16;
        let is_sel = i == selected;
        execute!(out, MoveTo(x, row)).ok();
        if is_sel {
            execute!(
                out,
                SetForegroundColor(Color::DarkYellow),
                SetAttribute(Attribute::Bold),
                Print("> "),
                ResetColor,
            )
            .ok();
        } else {
            execute!(out, Print("  ")).ok();
        }
        execute!(
            out,
            SetForegroundColor(Color::DarkGrey),
            Print(format!("{}: ", label)),
            ResetColor,
        )
        .ok();
        match opt {
            Some(item) => execute!(
                out,
                SetForegroundColor(item_color(item)),
                Print(item.name()),
                ResetColor,
            )
            .ok(),
            None => execute!(
                out,
                SetForegroundColor(Color::DarkGrey),
                Print("(aucun)"),
                ResetColor,
            )
            .ok(),
        };
    }

    // Consommables
    execute!(
        out,
        MoveTo(x, start_row + 4),
        SetForegroundColor(Color::DarkGrey),
        Print("Consommables :"),
        ResetColor,
    )
    .ok();

    for ci in 0..5usize {
        let row = start_row + 5 + ci as u16;
        let slot_idx = ci + 3;
        let is_sel = slot_idx == selected;
        execute!(out, MoveTo(x, row)).ok();
        if is_sel {
            execute!(
                out,
                SetForegroundColor(Color::DarkYellow),
                SetAttribute(Attribute::Bold),
                Print(format!("{} > ", ci + 1)),
                ResetColor,
            )
            .ok();
        } else {
            execute!(
                out,
                SetForegroundColor(Color::DarkGrey),
                Print(format!("{}   ", ci + 1)),
                ResetColor,
            )
            .ok();
        }
        match eq.consumables.get(ci) {
            Some(item) => execute!(
                out,
                SetForegroundColor(Color::Green),
                Print(item.name()),
                ResetColor,
            )
            .ok(),
            None => execute!(
                out,
                SetForegroundColor(Color::DarkGrey),
                Print("-"),
                ResetColor,
            )
            .ok(),
        };
    }
}

fn print_tab_label(out: &mut io::Stdout, key: &str, label: &str, active: bool) {
    if active {
        execute!(
            out,
            SetForegroundColor(Color::DarkYellow),
            SetAttribute(Attribute::Bold),
            Print(format!("[{}] {}", key, label)),
            ResetColor,
        )
        .ok();
    } else {
        execute!(
            out,
            SetForegroundColor(Color::DarkGrey),
            Print(format!("[{}] {}", key, label)),
            ResetColor,
        )
        .ok();
    }
}

// ─── RENDU INLINE (COMBAT) ───────────────────────────────────────────────────

fn draw_inline(out: &mut io::Stdout, player: &Player, tab: Tab, selected: usize, w: usize) {
    let sep = format!("  {}", "─".repeat(w.saturating_sub(4)));

    execute!(out, Print(format!("\r\n{}\r\n  ", sep)),).ok();
    print_tab_label(out, "I", "Inventaire", tab == Tab::Inventory);
    execute!(out, Print("   ")).ok();
    print_tab_label(out, "P", "Equipement", tab == Tab::Equipment);
    execute!(out, Print(format!("\r\n{}\r\n", sep))).ok();

    match tab {
        Tab::Inventory => {
            if player.inventory.is_empty() {
                execute!(
                    out,
                    SetForegroundColor(Color::DarkGrey),
                    Print("  (inventaire vide)\r\n"),
                    ResetColor,
                )
                .ok();
            } else {
                for (i, item) in player.inventory.iter().enumerate() {
                    execute!(out, Print("  ")).ok();
                    let is_sel = i == selected;
                    if is_sel {
                        execute!(
                            out,
                            SetForegroundColor(Color::DarkYellow),
                            SetAttribute(Attribute::Bold),
                            Print(format!("{} > ", i + 1)),
                            ResetColor,
                        )
                        .ok();
                    } else {
                        execute!(
                            out,
                            SetForegroundColor(Color::DarkGrey),
                            Print(format!("{}   ", i + 1)),
                            ResetColor,
                        )
                        .ok();
                    }
                    execute!(
                        out,
                        SetForegroundColor(item_color(item)),
                        Print(item.name()),
                        ResetColor,
                        SetForegroundColor(Color::DarkGrey),
                        Print(format!("  [{}]\r\n", item_type_label(item))),
                        ResetColor,
                    )
                    .ok();
                }
            }
        }
        Tab::Equipment => {
            let eq = &player.equipment[0];
            let main_slots = [
                ("Arme    ", eq.weapon.as_ref()),
                ("Armure  ", eq.armor.as_ref()),
                ("Bouclier", eq.shield.as_ref()),
            ];
            for (i, (label, opt)) in main_slots.iter().enumerate() {
                let is_sel = i == selected;
                execute!(out, Print("  ")).ok();
                if is_sel {
                    execute!(
                        out,
                        SetForegroundColor(Color::DarkYellow),
                        SetAttribute(Attribute::Bold),
                        Print("> "),
                        ResetColor,
                    )
                    .ok();
                } else {
                    execute!(out, Print("  ")).ok();
                }
                execute!(
                    out,
                    SetForegroundColor(Color::DarkGrey),
                    Print(format!("{}: ", label)),
                    ResetColor,
                )
                .ok();
                match opt {
                    Some(item) => execute!(
                        out,
                        SetForegroundColor(item_color(item)),
                        Print(format!("{}\r\n", item.name())),
                        ResetColor,
                    )
                    .ok(),
                    None => execute!(
                        out,
                        SetForegroundColor(Color::DarkGrey),
                        Print("(aucun)\r\n"),
                        ResetColor,
                    )
                    .ok(),
                };
            }
            // Consommables inline
            execute!(
                out,
                Print("  "),
                SetForegroundColor(Color::DarkGrey),
                Print("Consomm. : "),
                ResetColor,
            )
            .ok();
            for (ci, item) in eq.consumables.iter().enumerate() {
                let is_sel = ci + 3 == selected;
                if is_sel {
                    execute!(
                        out,
                        SetForegroundColor(Color::DarkYellow),
                        SetAttribute(Attribute::Bold),
                        Print(format!("[>{}<]  ", item.name())),
                        ResetColor,
                    )
                    .ok();
                } else {
                    execute!(
                        out,
                        SetForegroundColor(Color::Green),
                        Print(format!("[{}]  ", item.name())),
                        ResetColor,
                    )
                    .ok();
                }
            }
            if eq.consumables.is_empty() {
                execute!(
                    out,
                    SetForegroundColor(Color::DarkGrey),
                    Print("(aucun)"),
                    ResetColor,
                )
                .ok();
            }
            execute!(out, Print("\r\n")).ok();
        }
    }

    execute!(
        out,
        Print(format!("{}\r\n  ", sep)),
        SetForegroundColor(Color::DarkGrey),
    )
    .ok();
    let help = match tab {
        Tab::Inventory => {
            "[↑↓] Naviguer   [U/Enter] Utiliser/Equiper   [P] Equipement   [Esc] Fermer"
        }
        Tab::Equipment => {
            "[↑↓] Naviguer   [R/Enter] Desequiper         [I] Inventaire [Esc] Fermer"
        }
    };
    execute!(out, Print(help), ResetColor, Print("\r\n")).ok();
    out.flush().ok();
}
