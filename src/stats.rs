pub struct Stats {
    pub vigor: u32,
    pub intelligence: u32,
    pub faith: u32,
    pub dexterity: u32,
    pub arcane: u32,
    pub mind: u32,
    pub strength: u32,
}

impl Stats {
    pub fn new(
        vigor: u32,
        intelligence: u32,
        faith: u32,
        dexterity: u32,
        arcane: u32,
        mind: u32,
        strength: u32,
    ) -> Self {
        Stats {
            vigor,
            intelligence,
            faith,
            dexterity,
            arcane,
            mind,
            strength,
        }
    }

    // TODO: calculer HP max depuis vigor
    // pub fn max_hp(&self) -> u32 { ... }

    // TODO: calculer stamina max depuis mind
    // pub fn max_stamina(&self) -> u32 { ... }
}
