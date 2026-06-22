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

    /// Retourne false si l'équipement est refusé (arme 2 mains + bouclier).
    pub fn equip(&mut self, item: Item) -> bool {
        match &item {
            Item::Weapon(w) => {
                if w.two_handed {
                    self.shield = None; // retire le bouclier si arme 2 mains
                }
                self.weapon = Some(item);
                true
            }
            Item::Armor(_) => { self.armor = Some(item); true }
            Item::Shield(_) => {
                // Refuser si arme 2 mains équipée
                let is_two_handed = self.weapon.as_ref()
                    .and_then(|i| if let Item::Weapon(w) = i { Some(w.two_handed) } else { None })
                    .unwrap_or(false);
                if is_two_handed { return false; }
                self.shield = Some(item);
                true
            }
            Item::Consumable(_) => {
                if self.consumables.len() < 5 {
                    self.consumables.push(item);
                }
                true
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

    #[allow(dead_code)]
    pub fn unequip_consumable(&mut self, index: usize) -> Option<Item> {
        if index < self.consumables.len() {
            Some(self.consumables.remove(index))
        } else {
            None
        }
    }
}
