use crate::class::Class;
use crate::equipment::Equipment;
use crate::item::Item;
use crate::stats::Stats;
use crate::status::{ElementalEffect, FrostEffect, Status};

// FrostEffect réutilisé pour lightning — même logique de stacks sans tick dmg.
type LightningEffect = FrostEffect;

const INVENTORY_MAX: usize = 20;

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
        }
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
    pub fn level_up_n(&mut self, stat: &str, n: u32) -> u32 {
        (0..n).take_while(|_| self.level_up(stat)).count() as u32
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
