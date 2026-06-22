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
    /// Locations dont les ennemis normaux ont été tués (reset au repos à la grace).
    pub defeated_locations: HashSet<u32>,
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
            defeated_locations: HashSet::new(),
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
        let enemies_defeated_here = state.defeated_locations.contains(&state.location_id);
        let has_enemies = !enemies_defeated_here && location.contents.enemies.iter().any(|s| {
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

        let enemies_defeated_here = state.defeated_locations.contains(&state.location_id);
        if has_boss_only && zone_boss_dead {
            execute!(
                out,
                SetForegroundColor(Color::DarkGrey),
                Print("  [Boss]     VAINCU\r\n"),
                ResetColor,
            ).ok();
        } else if !location.contents.enemies.is_empty() {
            if enemies_defeated_here {
                execute!(
                    out,
                    SetForegroundColor(Color::DarkGrey),
                    Print("  [Ennemis]  vaincus\r\n"),
                    ResetColor,
                ).ok();
            } else {
                let max_in_combat: u32 = match state.zone {
                    ZoneId::Ashfeld | ZoneId::Gravemoor => 3,
                    _ => 1,
                };
                let live: u32 = location.contents.enemies.iter()
                    .filter(|s| s.enemy_type != EnemyType::Boss || !zone_boss_dead)
                    .map(|s| s.count)
                    .sum::<u32>()
                    .min(max_in_combat);
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
        actions.push(("R", "Reposer (ennemis respawn)"));
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

    draw_zone_map(out, world, state);
    out.flush().ok();
}

// ─── MINI-MAP ────────────────────────────────────────────────────────────────

const MAP_VIOLET: Color = Color::Rgb { r: 95, g: 81, b: 194 };

fn draw_zone_map(out: &mut io::Stdout, world: &World, state: &NavigationState) {
    let (tw, th) = terminal::size().unwrap_or((120, 30));
    if th < 10 { return; }

    let zone = world.get(state.zone);
    let mut locs: Vec<_> = zone.locations.iter().collect();
    locs.sort_by_key(|l| l.id);
    let n = locs.len();
    if n == 0 { return; }

    let sep = "     →     ";
    let sep_w = sep.chars().count(); // 11, pas sep.len() (13 bytes à cause de →)
    let pad = "      ";
    let pad_w = pad.len(); // que des espaces ASCII, len() == colonnes

    // Trouve le premier noeud branch (connexion forward non-adjacente)
    let mut branch: Option<(usize, usize)> = None; // (branch_idx, skip_target_idx)
    'outer: for (i, loc) in locs.iter().enumerate() {
        for conn in &loc.connections {
            if let LocationTarget::Here(tid) = conn.target {
                if let Some(j) = locs.iter().position(|l| l.id == tid) {
                    if j > i + 1 { branch = Some((i, j)); break 'outer; }
                }
            }
        }
    }

    if let Some((branch_idx, skip_idx)) = branch {
        // ── LAYOUT BRANCH : upper row + lower row ─────────────────────────────
        let upper_locs = &locs[..=branch_idx];
        let lower_locs = &locs[branch_idx + 1..];
        let n_lower = lower_locs.len();

        // name_max basé sur la lower row (plus large)
        let name_max = ((tw as usize).saturating_sub(
            pad_w * 2 * n_lower + sep_w * n_lower.saturating_sub(1) + 6
        )) / n_lower.max(1);

        let lower_lens: Vec<usize> = lower_locs.iter()
            .map(|l| l.name.chars().count().min(name_max)).collect();
        let upper_lens: Vec<usize> = upper_locs.iter()
            .map(|l| l.name.chars().count().min(name_max)).collect();

        // Calcule les centres des nodes lower row (relatif à lower_start=0)
        let mut lower_centers: Vec<isize> = Vec::new();
        let mut cur: isize = 0;
        for i in 0..n_lower {
            cur += pad_w as isize;
            lower_centers.push(cur + lower_lens[i] as isize / 2);
            cur += lower_lens[i] as isize + pad_w as isize;
            if i + 1 < n_lower { cur += sep_w as isize; }
        }
        let lower_content_w = cur as usize;

        let skip_local = skip_idx - branch_idx - 1;
        let arrow_l = lower_centers[0];
        let arrow_r = lower_centers[skip_local.min(n_lower - 1)];
        let branch_center_rel = (arrow_l + arrow_r) / 2; // relatif à lower_start

        // Calcule la position de upper row :
        // on veut que le centre du noeud branch soit à branch_center_rel (relatif à lower_start)
        // pre_width = contenu des locs avant le branch node
        let pre_w: isize = if upper_locs.len() > 1 {
            (upper_lens[..upper_locs.len() - 1].iter().sum::<usize>()
                + pad_w * 2 * (upper_locs.len() - 1)
                + sep_w * (upper_locs.len() - 1)) as isize
        } else { 0 };
        let branch_len = *upper_lens.last().unwrap_or(&10) as isize;
        // centre du branch node depuis upper_start = pre_w + pad + branch_len/2
        let branch_from_upper: isize = pre_w + pad_w as isize + branch_len / 2;
        // upper_start relatif à lower_start
        let upper_start_rel: isize = branch_center_rel - branch_from_upper;

        // Le noeud branch doit visuellement couvrir la zone [arrow_l, arrow_r]
        // On étend le display du noeud branch si nécessaire
        let branch_natural_start = branch_center_rel - branch_len / 2 - pad_w as isize;
        let extra_l = (arrow_l - pad_w as isize - branch_natural_start).max(0) as usize;
        let branch_natural_end = branch_center_rel + branch_len / 2 + pad_w as isize;
        let extra_r = (arrow_r + pad_w as isize - branch_natural_end).max(0) as usize;

        // upper row content width
        let upper_content_w = (pre_w as usize)
            + pad_w + extra_l + upper_lens.last().unwrap_or(&0) + extra_r + pad_w;

        // leftmost de tout (pour calculer col)
        let leftmost = upper_start_rel.min(0);
        let rightmost = ((upper_start_rel + upper_content_w as isize)
            .max(lower_content_w as isize)) as usize;
        let content_span = (rightmost as isize - leftmost.min(0)).max(1) as usize;
        let box_w = content_span + 4;
        let col = ((tw as usize).saturating_sub(box_w)) / 2;

        // Positions terminales
        let lower_term = (col as isize + 2 - leftmost) as usize;
        let upper_term = (lower_term as isize + upper_start_rel) as usize;
        let arrow_l_term = (lower_term as isize + arrow_l) as usize;
        let arrow_r_term = (lower_term as isize + arrow_r) as usize;
        let branch_center_term = (lower_term as isize + branch_center_rel) as usize;
        let right_col = col + box_w - 1;

        // 8 lignes: name + ┌┐ + upper + │ + ┌─┴─┐ + ↓↓ + lower + └┘
        let name_row = th.saturating_sub(8);

        // Nom de zone
        let zone_name_upper = zone.name.to_uppercase();
        let zone_col = col + box_w.saturating_sub(zone_name_upper.len()) / 2;
        execute!(out, MoveTo(zone_col as u16, name_row),
            SetForegroundColor(Color::Rgb { r: 255, g: 140, b: 30 }), SetAttribute(Attribute::Bold),
            Print(&zone_name_upper), ResetColor).ok();

        // Coins hauts
        execute!(out, MoveTo(col as u16, name_row + 1),
            SetForegroundColor(Color::DarkGrey), Print("┌"), ResetColor).ok();
        execute!(out, MoveTo(right_col as u16, name_row + 1),
            SetForegroundColor(Color::DarkGrey), Print("┐"), ResetColor).ok();

        // Upper row
        execute!(out, MoveTo(upper_term as u16, name_row + 2)).ok();
        for (i, loc) in upper_locs.iter().enumerate() {
            let is_cur = loc.id == state.location_id;
            let name = trunc(loc.name, name_max);
            let (pl, pr) = if i == upper_locs.len() - 1 {
                (pad_w + extra_l, pad_w + extra_r)
            } else {
                (pad_w, pad_w)
            };
            execute!(out,
                SetForegroundColor(if is_cur { MAP_VIOLET } else { Color::DarkGrey }),
                SetAttribute(if is_cur { Attribute::Bold } else { Attribute::NormalIntensity }),
                Print(format!("{}{}{}", " ".repeat(pl), name, " ".repeat(pr))),
                ResetColor).ok();
            if i + 1 < upper_locs.len() {
                execute!(out, SetForegroundColor(Color::DarkGrey), Print(sep), ResetColor).ok();
            }
        }

        // T-connector: │ stem
        execute!(out, MoveTo(branch_center_term as u16, name_row + 3),
            SetForegroundColor(Color::DarkGrey), Print("│"), ResetColor).ok();

        // T-connector: ┌──┴──┐ spanning arrow_l_term..arrow_r_term
        {
            let span = arrow_r_term.saturating_sub(arrow_l_term) + 1;
            let mut row = String::new();
            for i in 0..span {
                let abs = arrow_l_term + i;
                if abs == arrow_l_term { row.push('┌'); }
                else if abs == arrow_r_term { row.push('┐'); }
                else if abs == branch_center_term { row.push('┴'); }
                else { row.push('─'); }
            }
            execute!(out, MoveTo(arrow_l_term as u16, name_row + 4),
                SetForegroundColor(Color::DarkGrey), Print(&row), ResetColor).ok();
        }

        // Flèches ↓ vers lower row
        execute!(out, MoveTo(arrow_l_term as u16, name_row + 5),
            SetForegroundColor(Color::DarkGrey), Print("↓"), ResetColor).ok();
        execute!(out, MoveTo(arrow_r_term as u16, name_row + 5),
            SetForegroundColor(Color::DarkGrey), Print("↓"), ResetColor).ok();

        // Lower row
        execute!(out, MoveTo(lower_term as u16, name_row + 6)).ok();
        for (i, loc) in lower_locs.iter().enumerate() {
            let is_cur = loc.id == state.location_id;
            let name = trunc(loc.name, name_max);
            execute!(out,
                SetForegroundColor(if is_cur { MAP_VIOLET } else { Color::DarkGrey }),
                SetAttribute(if is_cur { Attribute::Bold } else { Attribute::NormalIntensity }),
                Print(format!("{}{}{}", pad, name, pad)), ResetColor).ok();
            if i + 1 < n_lower {
                execute!(out, SetForegroundColor(Color::DarkGrey), Print(sep), ResetColor).ok();
            }
        }

        // Coins bas
        execute!(out, MoveTo(col as u16, name_row + 7),
            SetForegroundColor(Color::DarkGrey), Print("└"), ResetColor).ok();
        execute!(out, MoveTo(right_col as u16, name_row + 7),
            SetForegroundColor(Color::DarkGrey), Print("┘"), ResetColor).ok();

    } else {
        // ── LAYOUT LINÉAIRE ───────────────────────────────────────────────────
        let name_max = ((tw as usize).saturating_sub(
            pad_w * 2 * n + sep_w * n.saturating_sub(1) + 6
        )) / n.max(1);

        let actual_lens: Vec<usize> = locs.iter()
            .map(|l| l.name.chars().count().min(name_max)).collect();
        let content_w: usize = actual_lens.iter().sum::<usize>()
            + pad_w * 2 * n + sep_w * n.saturating_sub(1);
        let box_w = content_w + 4;
        let col = ((tw as usize).saturating_sub(box_w)) / 2;
        let right_col = col + box_w - 1;
        let name_row = th.saturating_sub(4);

        let zone_name_upper = zone.name.to_uppercase();
        let zone_col = col + box_w.saturating_sub(zone_name_upper.len()) / 2;
        execute!(out, MoveTo(zone_col as u16, name_row),
            SetForegroundColor(Color::Rgb { r: 255, g: 140, b: 30 }), SetAttribute(Attribute::Bold),
            Print(&zone_name_upper), ResetColor).ok();
        execute!(out, MoveTo(col as u16, name_row + 1),
            SetForegroundColor(Color::DarkGrey), Print("┌"), ResetColor).ok();
        execute!(out, MoveTo(right_col as u16, name_row + 1),
            SetForegroundColor(Color::DarkGrey), Print("┐"), ResetColor).ok();

        execute!(out, MoveTo((col + 2) as u16, name_row + 2)).ok();
        for (i, loc) in locs.iter().enumerate() {
            let is_cur = loc.id == state.location_id;
            let name = trunc(loc.name, name_max);
            execute!(out,
                SetForegroundColor(if is_cur { MAP_VIOLET } else { Color::DarkGrey }),
                SetAttribute(if is_cur { Attribute::Bold } else { Attribute::NormalIntensity }),
                Print(format!("{}{}{}", pad, name, pad)), ResetColor).ok();
            if i + 1 < n {
                execute!(out, SetForegroundColor(Color::DarkGrey), Print(sep), ResetColor).ok();
            }
        }

        execute!(out, MoveTo(col as u16, name_row + 3),
            SetForegroundColor(Color::DarkGrey), Print("└"), ResetColor).ok();
        execute!(out, MoveTo(right_col as u16, name_row + 3),
            SetForegroundColor(Color::DarkGrey), Print("┘"), ResetColor).ok();
    }
}

// ─── UTILITAIRE ──────────────────────────────────────────────────────────────

fn trunc(s: &str, max: usize) -> &str {
    let mut end = s.len();
    let mut count = 0;
    for (i, _) in s.char_indices() {
        if count == max { end = i; break; }
        count += 1;
    }
    &s[..end]
}

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
