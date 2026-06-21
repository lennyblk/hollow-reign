use std::io::{self, Write};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};

use std::collections::{HashMap, HashSet};

use crate::enemy::EnemyType;
use crate::map::World;
use crate::zone::{LocationTarget, ZoneId};

// ─── STATE ────────────────────────────────────────────────────────────────────

pub struct NavigationState {
    pub zone: ZoneId,
    pub location_id: u32,
    /// Dernière location visitée par zone (pour le retour).
    pub last_location: HashMap<ZoneId, u32>,
    /// Coffres déjà ouverts (par chest_id).
    pub opened_chests: HashSet<u32>,
    /// Zones dont le boss a été définitivement tué.
    pub killed_bosses: HashSet<ZoneId>,
}

impl NavigationState {
    pub fn new(world: &World) -> Self {
        let zone = world.starting_zone();
        let location_id = world.get(zone).entry_location;
        NavigationState {
            zone,
            location_id,
            last_location: HashMap::new(),
            opened_chests: HashSet::new(),
            killed_bosses: HashSet::new(),
        }
    }

    /// Déplacement intra-zone.
    fn navigate_here(&mut self, new_loc: u32) {
        self.last_location.insert(self.zone, self.location_id);
        self.location_id = new_loc;
    }

    /// Transition vers une autre zone.
    /// Retourne à la dernière location visitée dans cette zone, ou entry_location si première visite.
    fn navigate_zone(&mut self, world: &World, zid: ZoneId) {
        self.last_location.insert(self.zone, self.location_id);
        let loc = self.last_location.get(&zid)
            .copied()
            .unwrap_or_else(|| world.get(zid).entry_location);
        self.zone = zid;
        self.location_id = loc;
    }
}

// ─── EVENTS ──────────────────────────────────────────────────────────────────

pub enum NavigationEvent {
    RestAtGrace(u32),
    TalkToNpc(&'static str),
    EnterCombat,
    OpenChest(u32),
    OpenInventory,
    OpenMerchant(ZoneId),
    Save,
    Quit,
}

// ─── MAIN LOOP ────────────────────────────────────────────────────────────────

pub fn run_navigation(world: &World, state: &mut NavigationState) -> NavigationEvent {
    let mut out = io::stdout();
    terminal::enable_raw_mode().expect("raw mode requis");
    execute!(out, Hide).ok();

    let mut flash: Option<String> = None;
    let event = loop {
        draw(&mut out, world, state, flash.as_deref());
        flash = None;

        let zone = world.get(state.zone);
        let location = zone.get_location(state.location_id).unwrap();

        // Capture des actions disponibles ici
        let zone_boss_dead = state.killed_bosses.contains(&state.zone);
        let has_grace = location.contents.grace.is_some();
        let has_npc = location.contents.npc.is_some();
        let has_enemies = location.contents.enemies.iter().any(|s| {
            s.enemy_type != EnemyType::Boss || !zone_boss_dead
        });
        let has_merchant = location.contents.merchant;
        let num_connections = location.connections.len();
        let chest_id = location.contents.chest.as_ref()
            .map(|c| c.id)
            .filter(|id| !state.opened_chests.contains(id));

        if let Ok(Event::Key(KeyEvent { code, kind: KeyEventKind::Press, .. })) = event::read() {
            match code {
                // Navigation par numéro
                KeyCode::Char(c) if c.is_ascii_digit() => {
                    let idx = (c as usize).saturating_sub('1' as usize);
                    let conns = &location.connections;
                    if idx < num_connections {
                        match conns[idx].target {
                            LocationTarget::Here(id) => {
                                state.navigate_here(id);
                            }
                            LocationTarget::OtherZone(zid) => {
                                if zid == ZoneId::TheVoid
                                    && !state.killed_bosses.contains(&state.zone)
                                {
                                    flash = Some(
                                        "Vaincre le boss de cette zone pour acceder au Vide."
                                            .to_string(),
                                    );
                                } else {
                                    state.navigate_zone(world, zid);
                                }
                            }
                        }
                    }
                }
                // Reposer à la grâce
                KeyCode::Char('r') | KeyCode::Char('R') if has_grace => {
                    let grace_id = location.contents.grace.as_ref().unwrap().id;
                    break NavigationEvent::RestAtGrace(grace_id);
                }
                // Ouvrir le coffre
                KeyCode::Char('c') | KeyCode::Char('C') if chest_id.is_some() => {
                    let id = chest_id.unwrap();
                    state.opened_chests.insert(id);
                    break NavigationEvent::OpenChest(id);
                }
                // Parler au NPC
                KeyCode::Char('t') | KeyCode::Char('T') if has_npc => {
                    let npc = location.contents.npc.unwrap();
                    break NavigationEvent::TalkToNpc(npc);
                }
                // Combat
                KeyCode::Char('e') | KeyCode::Char('E') if has_enemies => {
                    break NavigationEvent::EnterCombat;
                }
                // Marchand
                KeyCode::Char('m') | KeyCode::Char('M') if has_merchant => {
                    break NavigationEvent::OpenMerchant(state.zone);
                }
                // Inventaire (toujours disponible)
                KeyCode::Char('i') | KeyCode::Char('I') => {
                    break NavigationEvent::OpenInventory;
                }
                // Sauvegarde manuelle
                KeyCode::Char('s') | KeyCode::Char('S') => {
                    break NavigationEvent::Save;
                }
                // Quitter
                KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                    break NavigationEvent::Quit;
                }
                _ => {}
            }
        }
    };

    execute!(out, Show).ok();
    terminal::disable_raw_mode().ok();
    event
}

// ─── RENDU ────────────────────────────────────────────────────────────────────

fn draw(out: &mut io::Stdout, world: &World, state: &NavigationState, flash: Option<&str>) {
    let w = terminal::size().map(|(w, _)| w as usize).unwrap_or(80);
    let zone = world.get(state.zone);
    let location = zone.get_location(state.location_id).unwrap();

    execute!(out, Clear(ClearType::All), MoveTo(0, 0)).ok();

    // ── En-tête : Zone > Location ────────────────────────────────────────────
    let header = format!("  {}  >  {}  ", zone.name, location.name);
    let bar_len = w.saturating_sub(header.len() + 4);
    execute!(
        out,
        SetForegroundColor(Color::DarkYellow),
        SetAttribute(Attribute::Bold),
        Print(format!("══{}{}══\r\n", header, "═".repeat(bar_len))),
        ResetColor,
    )
    .ok();

    // ── ASCII art (si présent) ────────────────────────────────────────────────
    if !location.ascii.is_empty() {
        execute!(out, Print("\r\n")).ok();
        for line in location.ascii.lines() {
            execute!(
                out,
                SetForegroundColor(Color::DarkGrey),
                Print(format!("  {}\r\n", line)),
                ResetColor,
            )
            .ok();
        }
    }

    // ── Description ──────────────────────────────────────────────────────────
    execute!(out, Print("\r\n")).ok();
    for line in wrap_text(location.description, w.saturating_sub(4)) {
        execute!(
            out,
            SetForegroundColor(Color::Grey),
            Print(format!("  {}\r\n", line)),
            ResetColor,
        )
        .ok();
    }

    // ── Contenu ──────────────────────────────────────────────────────────────
    execute!(out, Print("\r\n")).ok();

    if let Some(grace) = &location.contents.grace {
        execute!(
            out,
            SetForegroundColor(Color::Yellow),
            Print(format!("  [Grace]    {}\r\n", grace.name)),
            ResetColor,
        )
        .ok();
    }
    if let Some(npc) = location.contents.npc {
        execute!(
            out,
            SetForegroundColor(Color::Cyan),
            Print(format!("  [NPC]      {}\r\n", npc)),
            ResetColor,
        )
        .ok();
    }
    if location.contents.merchant {
        execute!(
            out,
            SetForegroundColor(Color::Magenta),
            Print("  [Marchand] present\r\n"),
            ResetColor,
        )
        .ok();
    }
    if let Some(chest) = &location.contents.chest {
        let already_open = state.opened_chests.contains(&chest.id);
        let label = if already_open {
            "  [Coffre]   (deja ouvert)\r\n"
        } else {
            "  [Coffre]   ferme — [C] pour ouvrir\r\n"
        };
        execute!(
            out,
            SetForegroundColor(if already_open { Color::DarkGrey } else { Color::Green }),
            Print(label),
            ResetColor,
        )
        .ok();
    }
    {
        let zone_boss_dead = state.killed_bosses.contains(&state.zone);
        let has_boss_only = !location.contents.enemies.is_empty()
            && location.contents.enemies.iter().all(|s| s.enemy_type == EnemyType::Boss);

        if has_boss_only && zone_boss_dead {
            execute!(
                out,
                SetForegroundColor(Color::DarkGrey),
                Print("  [Boss]     VAINCU\r\n"),
                ResetColor,
            ).ok();
        } else if !location.contents.enemies.is_empty() {
            let live: u32 = location.contents.enemies.iter()
                .filter(|s| s.enemy_type != EnemyType::Boss || !zone_boss_dead)
                .map(|s| s.count)
                .sum();
            if live > 0 {
                execute!(
                    out,
                    SetForegroundColor(Color::Red),
                    Print(format!("  [Ennemis]  {} ennemi(s) en vue\r\n", live)),
                    ResetColor,
                ).ok();
            }
        }
    }

    // ── Séparateur ───────────────────────────────────────────────────────────
    execute!(
        out,
        Print("\r\n"),
        SetForegroundColor(Color::DarkGrey),
        Print(format!("  {}\r\n", "─".repeat(w.saturating_sub(4)))),
        ResetColor,
    )
    .ok();

    // ── Chemins disponibles ───────────────────────────────────────────────────
    execute!(
        out,
        SetForegroundColor(Color::White),
        SetAttribute(Attribute::Bold),
        Print("  Chemins :\r\n"),
        ResetColor,
    )
    .ok();

    let zone_boss_dead = state.killed_bosses.contains(&state.zone);
    for (i, conn) in location.connections.iter().enumerate() {
        let (dest_name, locked) = match conn.target {
            LocationTarget::Here(id) => (
                zone.get_location(id).map(|l| l.name).unwrap_or("?"),
                false,
            ),
            LocationTarget::OtherZone(zid) => {
                let gated = zid == ZoneId::TheVoid && !zone_boss_dead;
                (zid.name(), gated)
            }
        };
        if locked {
            execute!(
                out,
                SetForegroundColor(Color::DarkGrey),
                Print(format!("  [{}] {} ", i + 1, conn.label)),
                Print(format!("({}) [VERROU - vaincre le boss]\r\n", dest_name)),
                ResetColor,
            ).ok();
        } else {
            execute!(
                out,
                SetForegroundColor(Color::White),
                Print("  "),
                SetForegroundColor(Color::DarkYellow),
                SetAttribute(Attribute::Bold),
                Print(format!("[{}]", i + 1)),
                ResetColor,
                Print(format!(" {}", conn.label)),
                SetForegroundColor(Color::DarkGrey),
                Print(format!("  ({})\r\n", dest_name)),
                ResetColor,
            ).ok();
        }
    }

    // ── Actions ──────────────────────────────────────────────────────────────
    execute!(
        out,
        Print("\r\n"),
        SetForegroundColor(Color::DarkGrey),
        Print(format!("  {}\r\n", "─".repeat(w.saturating_sub(4)))),
        ResetColor,
        Print("  "),
    )
    .ok();

    let has_chest = location.contents.chest.as_ref()
        .map_or(false, |c| !state.opened_chests.contains(&c.id));
    let mut actions: Vec<(&str, &str)> = vec![];
    if location.contents.grace.is_some() {
        actions.push(("R", "Reposer"));
    }
    if location.contents.npc.is_some() {
        actions.push(("T", "Parler"));
    }
    if location.contents.merchant {
        actions.push(("M", "Marchand"));
    }
    let zone_boss_dead_actions = state.killed_bosses.contains(&state.zone);
    let has_live_enemies = location.contents.enemies.iter().any(|s| {
        s.enemy_type != EnemyType::Boss || !zone_boss_dead_actions
    });
    if has_live_enemies {
        actions.push(("E", "Combattre"));
    }
    if has_chest {
        actions.push(("C", "Coffre"));
    }
    actions.push(("I", "Inventaire"));
    actions.push(("S", "Sauvegarder"));
    actions.push(("Q", "Quitter"));

    for (key, label) in &actions {
        execute!(
            out,
            SetForegroundColor(Color::DarkYellow),
            SetAttribute(Attribute::Bold),
            Print(format!("[{}]", key)),
            ResetColor,
            Print(format!(" {}   ", label)),
        )
        .ok();
    }

    execute!(out, Print("\r\n")).ok();

    if let Some(msg) = flash {
        execute!(
            out,
            Print("\r\n  "),
            SetForegroundColor(Color::Red),
            SetAttribute(Attribute::Bold),
            Print(msg),
            ResetColor,
            Print("\r\n"),
        ).ok();
    }

    out.flush().ok();
}

// ─── UTILITAIRE ──────────────────────────────────────────────────────────────

/// Coupe le texte en lignes de max `width` caractères (word-wrap simple).
fn wrap_text(text: &str, width: usize) -> Vec<&str> {
    if width == 0 {
        return vec![text];
    }
    let mut lines = Vec::new();
    let mut start = 0;
    while start < text.len() {
        let remaining = &text[start..];
        if remaining.len() <= width {
            lines.push(remaining);
            break;
        }
        // Cherche le dernier espace avant `width`
        let cut = remaining[..width]
            .rfind(' ')
            .unwrap_or(width);
        lines.push(&remaining[..cut]);
        start += cut + 1; // +1 pour sauter l'espace
    }
    lines
}
