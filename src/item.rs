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
    pub two_handed: bool,
    pub ascii: &'static str,
}

pub struct ArmorData {
    pub name: String,
    pub defense: u32,
    pub ascii: &'static str,
}

pub struct ShieldData {
    pub name: String,
    pub defense: u32,
    pub ascii: &'static str,
}

pub enum ConsumableEffect {
    CurePoison,
    CureFrost,
    DealDamage(u32),
    DealFireDamage(u32),
    BuffAttack { bonus: u32, turns: u32 },
    QuestItem,
}

pub struct ConsumableData {
    pub name: String,
    pub effect: ConsumableEffect,
    pub ascii: &'static str,
}

pub enum Item {
    Weapon(WeaponData),
    Armor(ArmorData),
    Shield(ShieldData),
    Consumable(ConsumableData),
}

impl Item {
    pub fn name(&self) -> &str {
        match self {
            Item::Weapon(w)     => &w.name,
            Item::Armor(a)      => &a.name,
            Item::Shield(s)     => &s.name,
            Item::Consumable(c) => &c.name,
        }
    }

    pub fn ascii(&self) -> &'static str {
        match self {
            Item::Weapon(w)     => w.ascii,
            Item::Armor(a)      => a.ascii,
            Item::Shield(s)     => s.ascii,
            Item::Consumable(c) => c.ascii,
        }
    }

    pub fn slot(&self) -> Option<EquipmentSlot> {
        match self {
            Item::Weapon(_)     => Some(EquipmentSlot::Weapon),
            Item::Armor(_)      => Some(EquipmentSlot::Armor),
            Item::Shield(_)     => Some(EquipmentSlot::Shield),
            Item::Consumable(_) => None,
        }
    }
}
