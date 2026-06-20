use crate::item::{EquipmentSlot, Item};

pub struct Equipment {
    pub weapon: Option<Item>,
    pub armor: Option<Item>,
    pub shield: Option<Item>,
    pub consumables: Vec<Item>,
}

impl Equipment {
    pub fn empty() -> Self {
        Equipment {
            weapon: None,
            armor: None,
            shield: None,
            consumables: Vec::new(),
        }
    }

    pub fn equip(&mut self, item: Item) {
        match item {
            Item::Weapon(_) => self.weapon = Some(item),
            Item::Armor(_) => self.armor = Some(item),
            Item::Shield(_) => self.shield = Some(item),
            Item::Consumable(_) => {
                if self.consumables.len() < 5 {
                    self.consumables.push(item);
                }
            }
        }
    }

    pub fn unequip(&mut self, slot: EquipmentSlot) -> Option<Item> {
        match slot {
            EquipmentSlot::Weapon => self.weapon.take(),
            EquipmentSlot::Armor => self.armor.take(),
            EquipmentSlot::Shield => self.shield.take(),
        }
    }

    pub fn unequip_consumable(&mut self, index: usize) -> Option<Item> {
        if index < self.consumables.len() {
            Some(self.consumables.remove(index))
        } else {
            None
        }
    }
}
