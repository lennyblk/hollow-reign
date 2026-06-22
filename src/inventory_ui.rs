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
use crate::item::{ConsumableEffect, EquipmentSlot, Item, StatScaling};
use crate::map_ui;
use crate::player::Player;

// ─── RÉSULTAT ────────────────────────────────────────────────────────────────

pub enum InventoryResult {
    Closed,
    ConsumedItem,
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
    ShowMap,
}

// Colonnes plein écran — art Knight ~43 chars large depuis x=2 → stats à x=48
const MID_X:    u16 = 48;
const LIST_X:   u16 = 68;
const DETAIL_X: u16 = 105;
const START_ROW: u16 = 2;

// ─── POINT D'ENTRÉE ──────────────────────────────────────────────────────────

pub fn open_inventory(out: &mut io::Stdout, player: &mut Player, inline: bool) -> InventoryResult {
    if player.equipment.is_empty() {
        player.equipment.push(Equipment::empty());
    }

    if !inline {
        terminal::enable_raw_mode().ok();
        execute!(out, Hide).ok();
    }

    let mut tab      = Tab::Inventory;
    let mut selected = 0usize;
    let mut consumed = false;

    'main: loop {
        let (tw, th) = terminal::size().unwrap_or((120, 30));

        if inline {
            draw_inline(out, player, tab, selected, tw as usize);
        } else {
            draw_fullscreen(out, player, tab, selected, tw as usize, th as usize);
        }

        let list_len = slot_count(player, tab);

        if let Ok(Event::Key(KeyEvent { code, kind: KeyEventKind::Press, .. })) = event::read() {
            match code {
                KeyCode::Char('i') | KeyCode::Char('I') if tab != Tab::Inventory => {
                    tab = Tab::Inventory;
                    selected = 0;
                }
                KeyCode::Char('p') | KeyCode::Char('P') if tab != Tab::Equipment => {
                    tab = Tab::Equipment;
                    selected = 0;
                }

                KeyCode::Up   if selected > 0              => selected -= 1,
                KeyCode::Down if selected + 1 < list_len   => selected += 1,

                KeyCode::Enter | KeyCode::Char('u') | KeyCode::Char('U')
                    if tab == Tab::Inventory =>
                {
                    match use_or_equip(player, selected) {
                        ItemAction::Consumed => {
                            consumed = true;
                            clamp_selected(&mut selected, player.inventory.len());
                            if inline { break 'main; }
                        }
                        ItemAction::Equipped => {
                            clamp_selected(&mut selected, player.inventory.len());
                        }
                        ItemAction::ShowMap => {
                            map_ui::show_world_map(out);
                        }
                        ItemAction::None => {}
                    }
                }

                KeyCode::Enter
                | KeyCode::Char('r') | KeyCode::Char('R')
                | KeyCode::Char('u') | KeyCode::Char('U')
                    if tab == Tab::Equipment =>
                {
                    unequip_slot(player, selected);
                    clamp_selected(&mut selected, slot_count(player, Tab::Equipment));
                }

                KeyCode::Esc => break 'main,
                _ => {}
            }
        }
    }

    if !inline {
        execute!(out, Show).ok();
        terminal::disable_raw_mode().ok();
    }

    if consumed { InventoryResult::ConsumedItem } else { InventoryResult::Closed }
}

// ─── ACTIONS ─────────────────────────────────────────────────────────────────

fn use_or_equip(player: &mut Player, index: usize) -> ItemAction {
    if index >= player.inventory.len() { return ItemAction::None; }

    let is_consumable = matches!(&player.inventory[index], Item::Consumable(_));
    let slot = player.inventory[index].slot();

    if is_consumable {
        let item = player.inventory.remove(index);
        if let Item::Consumable(cd) = item {
            if matches!(cd.effect, ConsumableEffect::ShowMap) {
                // La carte reste dans l'inventaire — on la consulte sans la consommer
                player.inventory.insert(index, Item::Consumable(cd));
                return ItemAction::ShowMap;
            }
            if player.use_consumable(&cd.effect) {
                ItemAction::Consumed
            } else {
                player.inventory.insert(index, Item::Consumable(cd));
                ItemAction::None
            }
        } else {
            ItemAction::None
        }
    } else if let Some(slot) = slot {
        // Vérifie si l'équipement est autorisé avant de modifier quoi que ce soit
        let blocked = matches!(&player.inventory[index], Item::Shield(_))
            && player.equipment[0].weapon.as_ref()
                .and_then(|i| if let Item::Weapon(w) = i { Some(w.two_handed) } else { None })
                .unwrap_or(false);
        if blocked { return ItemAction::None; }

        let item = player.inventory.remove(index);
        let eq   = &mut player.equipment[0];
        let old  = eq.unequip(slot);
        eq.equip(item);
        if let Some(old_item) = old { player.inventory.push(old_item); }
        ItemAction::Equipped
    } else {
        ItemAction::None
    }
}

fn unequip_slot(player: &mut Player, selected: usize) {
    let eq   = &mut player.equipment[0];
    let item = match selected {
        0 => eq.unequip(EquipmentSlot::Weapon),
        1 => eq.unequip(EquipmentSlot::Armor),
        2 => eq.unequip(EquipmentSlot::Shield),
        _ => None,
    };
    if let Some(item) = item { player.pick_up(item); }
}

fn slot_count(player: &Player, tab: Tab) -> usize {
    match tab {
        Tab::Inventory => player.inventory.len(),
        Tab::Equipment => 3, // arme + armure + bouclier uniquement
    }
}

fn clamp_selected(selected: &mut usize, len: usize) {
    if len == 0           { *selected = 0; }
    else if *selected >= len { *selected = len - 1; }
}

// ─── HELPERS AFFICHAGE ───────────────────────────────────────────────────────

fn item_type_label(item: &Item) -> &'static str {
    match item {
        Item::Weapon(_)     => "Arme",
        Item::Armor(_)      => "Armure",
        Item::Shield(_)     => "Bouclier",
        Item::Consumable(_) => "Consommable",
    }
}

fn item_color(item: &Item) -> Color {
    match item {
        Item::Weapon(_)     => Color::Red,
        Item::Armor(_)      => Color::Blue,
        Item::Shield(_)     => Color::Cyan,
        Item::Consumable(_) => Color::Green,
    }
}

fn item_description(item: &Item) -> &'static str {
    match item {
        Item::Weapon(w)     => w.description,
        Item::Armor(a)      => a.description,
        Item::Shield(s)     => s.description,
        Item::Consumable(c) => c.description,
    }
}

fn item_stat_lines(item: &Item) -> Vec<(&'static str, String)> {
    match item {
        Item::Weapon(w) => {
            let scaling = match w.scaling {
                StatScaling::Strength      => "Force",
                StatScaling::Dexterity     => "Dexterite",
                StatScaling::Faith         => "Foi",
                StatScaling::Intelligence  => "Intelligence",
                StatScaling::MindAndFaith  => "Esprit/Foi",
            };
            vec![
                ("Degats base", w.base_damage.to_string()),
                ("Scaling",     scaling.to_string()),
                ("Deux mains",  if w.two_handed { "Oui" } else { "Non" }.to_string()),
                ("Element",     w.element.label().to_string()),
                ("Capacite",    w.ability.name.to_string()),
            ]
        }
        Item::Armor(a)      => vec![("Defense", a.defense.to_string())],
        Item::Shield(s)     => vec![("Defense", s.defense.to_string())],
        Item::Consumable(c) => vec![("Effet", consumable_effect_label(&c.effect).to_string())],
    }
}

fn consumable_effect_label(effect: &ConsumableEffect) -> &'static str {
    match effect {
        ConsumableEffect::CurePoison           => "Soigne le poison",
        ConsumableEffect::CureBleed            => "Soigne le saignement",
        ConsumableEffect::CureFire             => "Soigne les brulures",
        ConsumableEffect::CureRot              => "Soigne la pourriture",
        ConsumableEffect::CureFrost            => "Soigne le gel",
        ConsumableEffect::CureLightning        => "Soigne la foudre",
        ConsumableEffect::DealDamage(_)        => "Inflige des degats",
        ConsumableEffect::DealFireDamage(_)    => "Inflige des degats de feu",
        ConsumableEffect::BuffAttack { .. }    => "Augmente l'attaque temporairement",
        ConsumableEffect::QuestItem            => "Objet de quete",
        ConsumableEffect::ShowMap              => "Voir la carte du monde",
    }
}

fn class_label(class: &Class) -> &'static str {
    match class {
        Class::Knight => "Chevalier",
        Class::Mage   => "Mage",
        Class::Rogue  => "Rodeur",
    }
}

fn player_atk(player: &Player) -> u32 {
    5 + player.stats.strength + player.stats.dexterity / 2
}

fn player_def(player: &Player) -> u32 {
    let eq = match player.equipment.first() { Some(e) => e, None => return 0 };
    let armor  = match &eq.armor  { Some(Item::Armor(a))  => a.defense, _ => 0 };
    let is_2h  = matches!(&eq.weapon, Some(Item::Weapon(w)) if w.two_handed);
    let shield = if !is_2h { match &eq.shield { Some(Item::Shield(s)) => s.defense, _ => 0 } } else { 0 };
    armor + shield
}

fn wrap_str(text: &str, width: usize) -> Vec<&str> {
    if width == 0 { return vec![text]; }
    let mut lines = Vec::new();
    let mut start = 0;
    while start < text.len() {
        let remaining = &text[start..];
        if remaining.chars().count() <= width { lines.push(remaining); break; }
        // Byte index du width-ième caractère
        let end_byte = remaining.char_indices()
            .nth(width)
            .map(|(i, _)| i)
            .unwrap_or(remaining.len());
        // Dernier espace avant end_byte
        let cut_byte = remaining[..end_byte].rfind(' ').unwrap_or(end_byte);
        lines.push(&remaining[..cut_byte]);
        // Sauter l'espace lui-même
        let skip = if remaining[cut_byte..].starts_with(' ') { 1 } else { 0 };
        start += cut_byte + skip;
    }
    lines
}

// ─── RENDU PLEIN ÉCRAN ───────────────────────────────────────────────────────

fn draw_fullscreen(
    out:      &mut io::Stdout,
    player:   &Player,
    tab:      Tab,
    selected: usize,
    w:        usize,
    h:        usize,
) {
    execute!(out, Clear(ClearType::All), MoveTo(0, 0)).ok();

    // En-tête
    let title  = if tab == Tab::Inventory { "Inventaire" } else { "Equipement" };
    let header = format!("  {}  ", title);
    let bar    = "═".repeat(w.saturating_sub(header.len() + 4));
    execute!(
        out,
        SetForegroundColor(Color::DarkYellow),
        SetAttribute(Attribute::Bold),
        Print(format!("══{}{}══\r\n", header, bar)),
        ResetColor,
    ).ok();

    // ── Colonne gauche : ASCII joueur ─────────────────────────────────────────
    let pc = player.class.color();
    for (i, line) in player.class.ascii().lines().enumerate() {
        execute!(
            out,
            MoveTo(2, START_ROW + i as u16),
            SetForegroundColor(pc),
            SetAttribute(Attribute::Bold),
            Print(line),
            ResetColor,
        ).ok();
    }

    // ── Colonne milieu : stats joueur ─────────────────────────────────────────
    let max_hp   = player.stats.max_hp();
    let mid_data: Vec<(Option<&str>, String, Color)> = vec![
        (None,              player.name.clone(),                         Color::White),
        (None,              class_label(&player.class).to_string(),      Color::DarkYellow),
        (None,              "─".repeat(18),                              Color::DarkGrey),
        (Some("PV       "), format!("{}/{}", player.hp, max_hp),         Color::White),
        (Some("Vigueur  "), format!("{}", player.stats.vigor),           Color::Grey),
        (Some("Force    "), format!("{}", player.stats.strength),        Color::Grey),
        (Some("Dexterite"), format!("{}", player.stats.dexterity),       Color::Grey),
        (Some("Intel.   "), format!("{}", player.stats.intelligence),    Color::Grey),
        (Some("Foi      "), format!("{}", player.stats.faith),           Color::Grey),
        (Some("Arcane   "), format!("{}", player.stats.arcane),          Color::Grey),
        (Some("Mental   "), format!("{}", player.stats.mind),            Color::Grey),
        (None,              "─".repeat(18),                              Color::DarkGrey),
        (Some("Ames     "), format!("{}", player.souls),                 Color::Yellow),
        (Some("ATK      "), format!("{}", player_atk(player)),           Color::Red),
        (Some("DEF      "), format!("{}", player_def(player)),           Color::Blue),
    ];

    for (i, (label, value, color)) in mid_data.iter().enumerate() {
        execute!(out, MoveTo(MID_X, START_ROW + i as u16)).ok();
        match label {
            None => execute!(
                out,
                SetForegroundColor(*color),
                SetAttribute(if i == 0 { Attribute::Bold } else { Attribute::Reset }),
                Print(value),
                ResetColor,
            ).ok(),
            Some(lbl) => execute!(
                out,
                SetForegroundColor(Color::DarkGrey),
                Print(format!("{}: ", lbl)),
                ResetColor,
                SetForegroundColor(*color),
                Print(value),
                ResetColor,
            ).ok(),
        };
    }

    // ── Colonne liste : onglets + items ───────────────────────────────────────
    execute!(out, MoveTo(LIST_X, START_ROW)).ok();
    print_tab_label(out, "I", "Inventaire", tab == Tab::Inventory);
    execute!(out, Print("   ")).ok();
    print_tab_label(out, "P", "Equipement", tab == Tab::Equipment);

    let list_sep_w = (DETAIL_X as usize).saturating_sub(LIST_X as usize + 2);
    execute!(
        out,
        MoveTo(LIST_X, START_ROW + 1),
        SetForegroundColor(Color::DarkGrey),
        Print("─".repeat(list_sep_w)),
        ResetColor,
    ).ok();

    let list_row = START_ROW + 2;
    let visible  = h.saturating_sub(list_row as usize + 3);
    let scroll   = if selected >= visible && visible > 0 { selected + 1 - visible } else { 0 };

    match tab {
        Tab::Inventory => draw_inventory_list(out, player, selected, LIST_X, list_row, visible, scroll),
        Tab::Equipment => draw_equipment_list(out, player, selected, LIST_X, list_row),
    }

    // ── Séparateur vertical entre liste et détail ─────────────────────────────
    for r in START_ROW..(h as u16).saturating_sub(2) {
        execute!(
            out,
            MoveTo(DETAIL_X - 1, r),
            SetForegroundColor(Color::DarkGrey),
            Print("│"),
            ResetColor,
        ).ok();
    }

    // ── Colonne détail : item survolé ─────────────────────────────────────────
    let hovered: Option<&Item> = match tab {
        Tab::Inventory => player.inventory.get(selected),
        Tab::Equipment => {
            let eq = &player.equipment[0];
            match selected {
                0 => eq.weapon.as_ref(),
                1 => eq.armor.as_ref(),
                2 => eq.shield.as_ref(),
                _ => None,
            }
        }
    };
    draw_item_detail(out, hovered, DETAIL_X, START_ROW, h, w);

    // ── Pied de page ─────────────────────────────────────────────────────────
    let footer = h as u16 - 2;
    execute!(
        out,
        MoveTo(0, footer),
        SetForegroundColor(Color::DarkGrey),
        Print(format!("  {}", "─".repeat(w.saturating_sub(4)))),
        ResetColor,
    ).ok();
    let help = match tab {
        Tab::Inventory => "[haut/bas] Naviguer   [U/Entree] Utiliser ou Equiper   [P] Equipement   [Esc] Fermer",
        Tab::Equipment => "[haut/bas] Naviguer   [U/R/Entree] Desequiper          [I] Inventaire   [Esc] Fermer",
    };
    execute!(
        out,
        MoveTo(2, footer + 1),
        SetForegroundColor(Color::DarkGrey),
        Print(help),
        ResetColor,
    ).ok();

    out.flush().ok();
}

// ─── PANEL DÉTAIL ITEM ───────────────────────────────────────────────────────

fn draw_item_detail(
    out:   &mut io::Stdout,
    item:  Option<&Item>,
    x:     u16,
    start: u16,
    h:     usize,
    tw:    usize,
) {
    let detail_w = tw.saturating_sub(x as usize + 2);

    let item = match item {
        Some(i) => i,
        None => {
            execute!(
                out,
                MoveTo(x + 1, start + 2),
                SetForegroundColor(Color::DarkGrey),
                Print("(rien de selectionne)"),
                ResetColor,
            ).ok();
            return;
        }
    };

    let mut row = start;

    // Nom + type
    execute!(
        out,
        MoveTo(x + 1, row),
        SetForegroundColor(item_color(item)),
        SetAttribute(Attribute::Bold),
        Print(item.name()),
        ResetColor,
        SetForegroundColor(Color::DarkGrey),
        Print(format!("  [{}]", item_type_label(item))),
        ResetColor,
    ).ok();
    row += 1;

    execute!(
        out,
        MoveTo(x + 1, row),
        SetForegroundColor(Color::DarkGrey),
        Print("─".repeat(detail_w)),
        ResetColor,
    ).ok();
    row += 1;

    // ASCII art
    let ascii = item.ascii();
    if !ascii.is_empty() {
        for line in ascii.lines() {
            if row as usize >= h.saturating_sub(6) { break; }
            execute!(
                out,
                MoveTo(x + 1, row),
                SetForegroundColor(Color::DarkGrey),
                Print(line),
                ResetColor,
            ).ok();
            row += 1;
        }
        row += 1;
    }

    // Stats
    for (label, value) in item_stat_lines(item) {
        if row as usize >= h.saturating_sub(5) { break; }
        execute!(
            out,
            MoveTo(x + 1, row),
            SetForegroundColor(Color::DarkGrey),
            Print(format!("{}: ", label)),
            ResetColor,
            SetForegroundColor(Color::White),
            Print(&value),
            ResetColor,
        ).ok();
        row += 1;
    }

    row += 1;
    if (row as usize) < h.saturating_sub(4) {
        execute!(
            out,
            MoveTo(x + 1, row),
            SetForegroundColor(Color::DarkGrey),
            Print("─".repeat(detail_w)),
            ResetColor,
        ).ok();
        row += 1;
    }

    // Description (word-wrap)
    for chunk in wrap_str(item_description(item), detail_w) {
        if row as usize >= h.saturating_sub(3) { break; }
        execute!(
            out,
            MoveTo(x + 1, row),
            SetForegroundColor(Color::Grey),
            Print(chunk),
            ResetColor,
        ).ok();
        row += 1;
    }
}

// ─── LISTE INVENTAIRE ────────────────────────────────────────────────────────

fn draw_inventory_list(
    out:       &mut io::Stdout,
    player:    &Player,
    selected:  usize,
    x:         u16,
    start_row: u16,
    visible:   usize,
    scroll:    usize,
) {
    if player.inventory.is_empty() {
        execute!(
            out,
            MoveTo(x, start_row),
            SetForegroundColor(Color::DarkGrey),
            Print("(inventaire vide)"),
            ResetColor,
        ).ok();
        return;
    }
    for (vi, inv_idx) in (scroll..).take(visible).enumerate() {
        if inv_idx >= player.inventory.len() { break; }
        let row    = start_row + vi as u16;
        let item   = &player.inventory[inv_idx];
        let is_sel = inv_idx == selected;

        execute!(out, MoveTo(x, row)).ok();
        if is_sel {
            execute!(
                out,
                SetForegroundColor(Color::DarkYellow),
                SetAttribute(Attribute::Bold),
                Print(format!("{:>2} > ", inv_idx + 1)),
                ResetColor,
            ).ok();
        } else {
            execute!(
                out,
                SetForegroundColor(Color::DarkGrey),
                Print(format!("{:>2}   ", inv_idx + 1)),
                ResetColor,
            ).ok();
        }
        execute!(
            out,
            SetForegroundColor(item_color(item)),
            Print(item.name()),
            ResetColor,
            SetForegroundColor(Color::DarkGrey),
            Print(format!("  [{}]", item_type_label(item))),
            ResetColor,
        ).ok();
    }
}

// ─── LISTE ÉQUIPEMENT ────────────────────────────────────────────────────────

fn draw_equipment_list(
    out:       &mut io::Stdout,
    player:    &Player,
    selected:  usize,
    x:         u16,
    start_row: u16,
) {
    let eq = &player.equipment[0];
    let slots: [(&str, Option<&Item>); 3] = [
        ("Arme    ", eq.weapon.as_ref()),
        ("Armure  ", eq.armor.as_ref()),
        ("Bouclier", eq.shield.as_ref()),
    ];
    for (i, (label, opt)) in slots.iter().enumerate() {
        let row    = start_row + i as u16;
        let is_sel = i == selected;
        execute!(out, MoveTo(x, row)).ok();
        if is_sel {
            execute!(
                out,
                SetForegroundColor(Color::DarkYellow),
                SetAttribute(Attribute::Bold),
                Print("> "),
                ResetColor,
            ).ok();
        } else {
            execute!(out, Print("  ")).ok();
        }
        execute!(
            out,
            SetForegroundColor(Color::DarkGrey),
            Print(format!("{}: ", label)),
            ResetColor,
        ).ok();
        match opt {
            Some(item) => execute!(
                out,
                SetForegroundColor(item_color(item)),
                Print(item.name()),
                ResetColor,
            ).ok(),
            None => execute!(
                out,
                SetForegroundColor(Color::DarkGrey),
                Print("(aucun)"),
                ResetColor,
            ).ok(),
        };
    }
}

// ─── TAB LABEL ───────────────────────────────────────────────────────────────

fn print_tab_label(out: &mut io::Stdout, key: &str, label: &str, active: bool) {
    if active {
        execute!(
            out,
            SetForegroundColor(Color::DarkYellow),
            SetAttribute(Attribute::Bold),
            Print(format!("[{}] {}", key, label)),
            ResetColor,
        ).ok();
    } else {
        execute!(
            out,
            SetForegroundColor(Color::DarkGrey),
            Print(format!("[{}] {}", key, label)),
            ResetColor,
        ).ok();
    }
}

// ─── RENDU INLINE (COMBAT) ───────────────────────────────────────────────────

fn draw_inline(out: &mut io::Stdout, player: &Player, tab: Tab, selected: usize, w: usize) {
    let sep = format!("  {}", "─".repeat(w.saturating_sub(4)));

    execute!(out, Print(format!("\r\n{}\r\n  ", sep))).ok();
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
                ).ok();
            } else {
                for (i, item) in player.inventory.iter().enumerate() {
                    let is_sel = i == selected;
                    execute!(out, Print("  ")).ok();
                    if is_sel {
                        execute!(
                            out,
                            SetForegroundColor(Color::DarkYellow),
                            SetAttribute(Attribute::Bold),
                            Print(format!("{} > ", i + 1)),
                            ResetColor,
                        ).ok();
                    } else {
                        execute!(
                            out,
                            SetForegroundColor(Color::DarkGrey),
                            Print(format!("{}   ", i + 1)),
                            ResetColor,
                        ).ok();
                    }
                    execute!(
                        out,
                        SetForegroundColor(item_color(item)),
                        Print(item.name()),
                        ResetColor,
                        SetForegroundColor(Color::DarkGrey),
                        Print(format!("  [{}]\r\n", item_type_label(item))),
                        ResetColor,
                    ).ok();
                }
            }
        }
        Tab::Equipment => {
            let eq = &player.equipment[0];
            let slots = [
                ("Arme    ", eq.weapon.as_ref()),
                ("Armure  ", eq.armor.as_ref()),
                ("Bouclier", eq.shield.as_ref()),
            ];
            for (i, (label, opt)) in slots.iter().enumerate() {
                let is_sel = i == selected;
                execute!(out, Print("  ")).ok();
                if is_sel {
                    execute!(
                        out,
                        SetForegroundColor(Color::DarkYellow),
                        SetAttribute(Attribute::Bold),
                        Print("> "),
                        ResetColor,
                    ).ok();
                } else {
                    execute!(out, Print("  ")).ok();
                }
                execute!(
                    out,
                    SetForegroundColor(Color::DarkGrey),
                    Print(format!("{}: ", label)),
                    ResetColor,
                ).ok();
                match opt {
                    Some(item) => execute!(
                        out,
                        SetForegroundColor(item_color(item)),
                        Print(format!("{}\r\n", item.name())),
                        ResetColor,
                    ).ok(),
                    None => execute!(
                        out,
                        SetForegroundColor(Color::DarkGrey),
                        Print("(aucun)\r\n"),
                        ResetColor,
                    ).ok(),
                };
            }
        }
    }

    execute!(
        out,
        Print(format!("{}\r\n  ", sep)),
        SetForegroundColor(Color::DarkGrey),
    ).ok();
    let help = match tab {
        Tab::Inventory => "[haut/bas] Naviguer   [U/Entree] Utiliser/Equiper   [P] Equipement   [Esc] Fermer",
        Tab::Equipment => "[haut/bas] Naviguer   [R/Entree] Desequiper         [I] Inventaire   [Esc] Fermer",
    };
    execute!(out, Print(help), ResetColor, Print("\r\n")).ok();
    out.flush().ok();
}
