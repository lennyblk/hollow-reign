pub enum EquipmentSlot {
    Weapon,
    Armor,
    Shield,
}

pub enum Item {
    Weapon(String),
    Armor(String),
    Shield(String),
    Consumable(String),
}

impl Item {
    // TODO: retourner le nom de l'item
    // pub fn name(&self) -> &str { ... }

    // TODO: retourner le slot compatible
    // pub fn slot(&self) -> Option<EquipmentSlot> { ... }
}
