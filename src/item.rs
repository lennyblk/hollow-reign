pub enum EquipmentSlot {
    Weapon,
    Armor,
    Shield,
}

pub enum StatScaling {
    Strength,
    Dexterity,
    Intelligence,
    Faith,
    MindAndFaith,
}

pub struct WeaponData {
    pub name: String,
    pub base_damage: u32,
    pub scaling: StatScaling,
    /// Si true, pas de shield possible (épée longue, marteau, katana...).
    pub two_handed: bool,
}

pub struct ArmorData {
    pub name: String,
    pub defense: u32,
}

pub struct ShieldData {
    pub name: String,
    pub defense: u32,
}

pub enum Item {
    Weapon(WeaponData),
    Armor(ArmorData),
    Shield(ShieldData),
    Consumable(String),
}

impl Item {
    pub fn name(&self) -> &str {
        match self {
            Item::Weapon(w)    => &w.name,
            Item::Armor(a)     => &a.name,
            Item::Shield(s)    => &s.name,
            Item::Consumable(n) => n,
        }
    }

    pub fn slot(&self) -> Option<EquipmentSlot> {
        match self {
            Item::Weapon(_)    => Some(EquipmentSlot::Weapon),
            Item::Armor(_)     => Some(EquipmentSlot::Armor),
            Item::Shield(_)    => Some(EquipmentSlot::Shield),
            Item::Consumable(_) => None,
        }
    }
}
