use crate::class::Class;
use crate::equipment::Equipment;
use crate::item::Item;
use crate::stats::Stats;
use crate::status::Status;

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

    pub fn soul_cost(&self) -> u32 {
        10 + self.level * self.level / 5
    }

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
