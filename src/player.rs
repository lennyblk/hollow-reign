use crate::class::Class;
use crate::equipment::Equipment;
use crate::item::Item;
use crate::stats::Stats;
use crate::status::Status;

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
}

impl Player {
    // TODO: créer un nouveau joueur selon sa classe
    // pub fn new(name: String, class: Class) -> Self {}

    // TODO: recevoir des dégâts
    // pub fn take_damage(&mut self, amount: u32) { ... }

    // TODO: mourir et perdre les souls
    // pub fn die(&mut self) { ... }

    // TODO: ramasser un item dans l'inventaire
    // pub fn pick_up(&mut self, item: Item) { ... }

    // TODO: monter de niveau
    // pub fn level_up(&mut self, stat: &str) { ... }
}
