use std::collections::HashSet;

use crate::class::Class;
use crate::equipment::Equipment;
use crate::item::{ConsumableEffect, Item};
use crate::stats::Stats;
use crate::status::{ElementalEffect, FrostEffect, Status};

// FrostEffect réutilisé pour lightning — même logique de stacks sans tick dmg.
type LightningEffect = FrostEffect;

const INVENTORY_MAX: usize = 20;

#[allow(dead_code)]
pub struct DroppedSouls {
    pub amount: u32,
    pub x: f32,
    pub y: f32,
}

pub struct Player {
    pub name: String,
    pub class: Class,
    pub hp: u32,
    pub souls: u32,
    pub level: u32,
    pub stats: Stats,
    pub status: Status,
    pub inventory: Vec<Item>,
    pub equipment: Vec<Equipment>,
    pub position: (f32, f32),
    pub dropped_souls: Option<DroppedSouls>,
    pub poison: ElementalEffect,
    pub bleed: ElementalEffect,
    pub rot: ElementalEffect,
    pub frost: FrostEffect,
    pub fire: ElementalEffect,
    pub lightning: LightningEffect,
    pub estus_charges: u32,
    pub last_grace: Option<u32>,
    pub visited_graces: HashSet<u32>,
    pub active_quests: HashSet<String>,
    pub completed_quests: HashSet<String>,
}

impl Player {
    pub fn new(name: String, class: Class) -> Self {
        let stats = class.base_stats();
        Player {
            name,
            class,
            hp: 100,
            souls: 0,
            level: 1,
            stats,
            status: Status::default(),
            inventory: Vec::new(),
            equipment: Vec::new(),
            position: (0.0, 0.0),
            dropped_souls: None,
            poison: ElementalEffect::new(),
            bleed: ElementalEffect::new(),
            rot: ElementalEffect::new(),
            frost: FrostEffect::new(),
            fire: ElementalEffect::new(),
            lightning: FrostEffect::new(),
            estus_charges: 1,
            last_grace: None,
            visited_graces: HashSet::new(),
            active_quests: HashSet::new(),
            completed_quests: HashSet::new(),
        }
    }

    /// Nombre max de fioles selon le niveau.
    pub fn max_estus(&self) -> u32 {
        match self.level {
            1..=9   => 1,
            10..=19 => 2,
            20..=34 => 3,
            35..=49 => 4,
            50..=69 => 5,
            70..=98 => 6,
            _       => 7,
        }
    }

    /// Utilise une fiole d'estus. Retourne false si plus de charges.
    pub fn use_estus(&mut self) -> bool {
        if self.estus_charges == 0 {
            return false;
        }
        self.estus_charges -= 1;
        let heal = self.stats.max_hp() / 2;
        self.hp = (self.hp + heal).min(self.stats.max_hp());
        true
    }

    /// S'asseoir à une grâce — reset estus, sauvegarde la grâce, ennemis respawn géré par la map.
    pub fn rest_at_grace(&mut self, grace_id: u32) {
        self.last_grace = Some(grace_id);
        self.visited_graces.insert(grace_id);
        self.estus_charges = self.max_estus();
        self.status = Status::default();
    }

    /// Respawn à la dernière grâce après une mort. La position est mise à jour par la map.
    pub fn respawn_at_grace(&mut self) {
        self.hp = self.stats.max_hp();
        self.status = Status::default();
        self.estus_charges = self.max_estus();
        self.poison = ElementalEffect::new();
        self.bleed = ElementalEffect::new();
        self.rot = ElementalEffect::new();
        self.frost = FrostEffect::new();
        self.fire = ElementalEffect::new();
        self.lightning = FrostEffect::new();
    }

    pub fn take_damage(&mut self, amount: u32) {
        self.hp = self.hp.saturating_sub(amount);
        if self.hp == 0 {
            self.die();
        }
    }

    pub fn pick_up(&mut self, item: Item) -> bool {
        if self.inventory.len() >= INVENTORY_MAX {
            return false;
        }
        self.inventory.push(item);
        true
    }

    /// Coût pour passer du niveau actuel au suivant.
    pub fn soul_cost(&self) -> u32 {
        10 + self.level * self.level / 5
    }

    /// Coût total pour acheter n niveaux (preview avant confirmation).
    #[allow(dead_code)]
    pub fn total_cost(&self, n: u32) -> u32 {
        (0..n).map(|i| 10 + (self.level + i) * (self.level + i) / 5).sum()
    }

    /// Achète 1 niveau dans une stat. Retourne false si souls insuffisants ou stat inconnue.
    pub fn level_up(&mut self, stat: &str) -> bool {
        let cost = self.soul_cost();
        if self.souls < cost {
            return false;
        }
        if !self.stats.modify(stat, 1) {
            return false;
        }
        self.souls -= cost;
        self.level += 1;
        true
    }

    /// Achète n niveaux dans une stat. Retourne le nombre de niveaux réellement achetés.
    #[allow(dead_code)]
    pub fn level_up_n(&mut self, stat: &str, n: u32) -> u32 {
        (0..n).take_while(|_| self.level_up(stat)).count() as u32
    }

    /// Applique un effet de consommable sur le joueur.
    /// Retourne false si rien à soigner (DoT inactif) ou si l'effet nécessite un contexte de combat.
    pub fn use_consumable(&mut self, effect: &ConsumableEffect) -> bool {
        match effect {
            ConsumableEffect::CurePoison => {
                if !self.poison.is_ticking() && self.poison.stacks == 0 { return false; }
                self.poison = ElementalEffect::new();
                true
            }
            ConsumableEffect::CureBleed => {
                if !self.bleed.is_ticking() && self.bleed.stacks == 0 { return false; }
                self.bleed = ElementalEffect::new();
                true
            }
            ConsumableEffect::CureFire => {
                if !self.fire.is_ticking() && self.fire.stacks == 0 { return false; }
                self.fire = ElementalEffect::new();
                true
            }
            ConsumableEffect::CureRot => {
                if !self.rot.is_ticking() && self.rot.stacks == 0 { return false; }
                self.rot = ElementalEffect::new();
                true
            }
            ConsumableEffect::CureFrost => {
                if self.frost.stacks == 0 && !matches!(self.status, Status::Frozen { .. }) { return false; }
                self.frost = FrostEffect::new();
                if matches!(self.status, Status::Frozen { .. }) {
                    self.status = Status::Alive;
                }
                true
            }
            ConsumableEffect::CureLightning => {
                if self.lightning.stacks == 0 && !matches!(self.status, Status::Electrocuted { .. }) { return false; }
                self.lightning = FrostEffect::new();
                if matches!(self.status, Status::Electrocuted { .. }) {
                    self.status = Status::Alive;
                }
                true
            }
            // Ces effets nécessitent un contexte de combat — géré dans combat.rs
            ConsumableEffect::DealDamage(_)
            | ConsumableEffect::DealFireDamage(_)
            | ConsumableEffect::BuffAttack { .. }
            | ConsumableEffect::QuestItem
            | ConsumableEffect::ShowMap => false,
        }
    }

    pub fn die(&mut self) {
        self.status = Status::Dead;
        self.hp = 0;
        self.dropped_souls = Some(DroppedSouls {
            amount: self.souls,
            x: self.position.0,
            y: self.position.1,
        });
        self.souls = 0;
    }
}
