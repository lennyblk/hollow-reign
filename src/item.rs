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
    pub fn name(&self) -> &str {
        match self {
            Item::Weapon(name) => name,
            Item::Armor(name) => name,
            Item::Shield(name) => name,
            Item::Consumable(name) => name,
        }
    }

    pub fn slot(&self) -> Option<EquipmentSlot> {
        match self {
            Item::Weapon(_) => Some(EquipmentSlot::Weapon),
            Item::Armor(_) => Some(EquipmentSlot::Armor),
            Item::Shield(_) => Some(EquipmentSlot::Shield),
            Item::Consumable(_) => None,
        }
    }
}
