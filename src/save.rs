use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::catalog::item_by_name;
use crate::class::Class;
use crate::equipment::Equipment;
use crate::map::World;
use crate::navigation::NavigationState;
use crate::player::Player;
use crate::stats::Stats;
use crate::zone::ZoneId;

const SAVE_PATH: &str = "save.json";

// ─── STRUCTURE DE SAUVEGARDE ─────────────────────────────────────────────────

#[derive(Serialize, Deserialize)]
struct SaveData {
    // Joueur
    player_name: String,
    player_class: String,
    player_hp: u32,
    player_souls: u32,
    player_level: u32,
    stat_vigor: u32,
    stat_intelligence: u32,
    stat_faith: u32,
    stat_dexterity: u32,
    stat_arcane: u32,
    stat_mind: u32,
    stat_strength: u32,
    player_estus: u32,
    player_last_grace: Option<u32>,
    // Inventaire (noms exacts)
    inventory: Vec<String>,
    eq_weapon: Option<String>,
    eq_armor: Option<String>,
    eq_shield: Option<String>,
    eq_consumables: Vec<String>,
    // Navigation
    nav_zone: String,
    nav_location_id: u32,
    nav_opened_chests: Vec<u32>,
    nav_last_locations: Vec<(String, u32)>,
    nav_killed_bosses: Vec<String>,
    visited_graces: Vec<u32>,
    // Quêtes
    active_quests: Vec<String>,
    completed_quests: Vec<String>,
}

// ─── SAUVEGARDE ──────────────────────────────────────────────────────────────

pub fn save(player: &Player, state: &NavigationState) {
    let eq = player.equipment.first();

    let data = SaveData {
        player_name: player.name.clone(),
        player_class: class_to_str(&player.class).to_string(),
        player_hp: player.hp,
        player_souls: player.souls,
        player_level: player.level,
        stat_vigor: player.stats.vigor,
        stat_intelligence: player.stats.intelligence,
        stat_faith: player.stats.faith,
        stat_dexterity: player.stats.dexterity,
        stat_arcane: player.stats.arcane,
        stat_mind: player.stats.mind,
        stat_strength: player.stats.strength,
        player_estus: player.estus_charges,
        player_last_grace: player.last_grace,
        inventory: player
            .inventory
            .iter()
            .map(|i| i.name().to_string())
            .collect(),
        eq_weapon: eq
            .and_then(|e| e.weapon.as_ref())
            .map(|i| i.name().to_string()),
        eq_armor: eq
            .and_then(|e| e.armor.as_ref())
            .map(|i| i.name().to_string()),
        eq_shield: eq
            .and_then(|e| e.shield.as_ref())
            .map(|i| i.name().to_string()),
        eq_consumables: eq
            .map(|e| e.consumables.iter().map(|i| i.name().to_string()).collect())
            .unwrap_or_default(),
        nav_zone: zone_to_str(state.zone).to_string(),
        nav_location_id: state.location_id,
        nav_opened_chests: state.opened_chests.iter().copied().collect(),
        nav_killed_bosses: state
            .killed_bosses
            .iter()
            .map(|z| zone_to_str(*z).to_string())
            .collect(),
        visited_graces: player.visited_graces.iter().copied().collect(),
        active_quests: player.active_quests.iter().cloned().collect(),
        completed_quests: player.completed_quests.iter().cloned().collect(),
        nav_last_locations: state
            .last_location
            .iter()
            .map(|(z, &l)| (zone_to_str(*z).to_string(), l))
            .collect(),
    };

    match serde_json::to_string_pretty(&data) {
        Ok(json) => {
            if let Err(e) = std::fs::write(SAVE_PATH, json) {
                eprintln!("Erreur sauvegarde : {}", e);
            }
        }
        Err(e) => eprintln!("Erreur sérialisation : {}", e),
    }
}

// ─── CHARGEMENT ──────────────────────────────────────────────────────────────

/// Charge la sauvegarde si elle existe. Retourne `None` si absent ou corrompu.
pub fn load(world: &World) -> Option<(Player, NavigationState)> {
    let json = std::fs::read_to_string(SAVE_PATH).ok()?;
    let data: SaveData = serde_json::from_str(&json).ok()?;

    let class = class_from_str(&data.player_class)?;
    let stats = Stats::new(
        data.stat_vigor,
        data.stat_intelligence,
        data.stat_faith,
        data.stat_dexterity,
        data.stat_arcane,
        data.stat_mind,
        data.stat_strength,
    );

    let mut player = Player::new(data.player_name, class);
    player.stats = stats;
    player.hp = data.player_hp;
    player.souls = data.player_souls;
    player.level = data.player_level;
    player.estus_charges = data.player_estus;
    player.last_grace = data.player_last_grace;
    player.visited_graces = data.visited_graces.into_iter().collect();
    player.active_quests = data.active_quests.into_iter().collect();
    player.completed_quests = data.completed_quests.into_iter().collect();

    // Inventaire
    player.inventory = data
        .inventory
        .iter()
        .filter_map(|name| item_by_name(name))
        .collect();

    // Équipement
    let mut eq = Equipment::empty();
    if let Some(ref name) = data.eq_weapon {
        if let Some(i) = item_by_name(name) {
            eq.equip(i);
        }
    }
    if let Some(ref name) = data.eq_armor {
        if let Some(i) = item_by_name(name) {
            eq.equip(i);
        }
    }
    if let Some(ref name) = data.eq_shield {
        if let Some(i) = item_by_name(name) {
            eq.equip(i);
        }
    }
    for name in &data.eq_consumables {
        if let Some(i) = item_by_name(name) {
            eq.equip(i);
        }
    }
    player.equipment.push(eq);

    // Navigation
    let zone = zone_from_str(&data.nav_zone)?;
    let mut state = NavigationState::new(world);
    state.zone = zone;
    state.location_id = data.nav_location_id;
    state.opened_chests = data.nav_opened_chests.into_iter().collect::<HashSet<_>>();
    state.killed_bosses = data
        .nav_killed_bosses
        .iter()
        .filter_map(|s| zone_from_str(s))
        .collect();
    for (zone_str, loc) in data.nav_last_locations {
        if let Some(z) = zone_from_str(&zone_str) {
            state.last_location.insert(z, loc);
        }
    }

    Some((player, state))
}

pub fn save_exists() -> bool {
    std::path::Path::new(SAVE_PATH).exists()
}

// ─── CONVERSIONS ─────────────────────────────────────────────────────────────

fn class_to_str(class: &Class) -> &'static str {
    match class {
        Class::Knight => "Knight",
        Class::Mage => "Mage",
        Class::Rogue => "Rogue",
    }
}

fn class_from_str(s: &str) -> Option<Class> {
    match s {
        "Knight" => Some(Class::Knight),
        "Mage" => Some(Class::Mage),
        "Rogue" => Some(Class::Rogue),
        _ => None,
    }
}

fn zone_to_str(zone: ZoneId) -> &'static str {
    match zone {
        ZoneId::Ashfeld => "Ashfeld",
        ZoneId::Gravemoor => "Gravemoor",
        ZoneId::Rotwood => "Rotwood",
        ZoneId::TheCinders => "TheCinders",
        ZoneId::Frostveil => "Frostveil",
        ZoneId::TheVoid => "TheVoid",
    }
}

fn zone_from_str(s: &str) -> Option<ZoneId> {
    match s {
        "Ashfeld" => Some(ZoneId::Ashfeld),
        "Gravemoor" => Some(ZoneId::Gravemoor),
        "Rotwood" => Some(ZoneId::Rotwood),
        "TheCinders" => Some(ZoneId::TheCinders),
        "Frostveil" => Some(ZoneId::Frostveil),
        "TheVoid" => Some(ZoneId::TheVoid),
        _ => None,
    }
}
