use crate::item::Item;

pub struct Equipment {
    pub weapon: Option<Item>,
    pub armor: Option<Item>,
    pub shield: Option<Item>,
}

impl Equipment {
    pub fn empty() -> Self {
        Equipment {
            weapon: None,
            armor: None,
            shield: None,
        }
    }

    // TODO: équiper un item dans le bon slot
    // pub fn equip(&mut self, item: Item) { ... }

    // TODO: déséquiper un slot
    // pub fn unequip(&mut self, slot: EquipmentSlot) -> Option<Item> { ... }
}
