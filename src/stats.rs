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

    pub fn get(&self, stat: &str) -> Option<u32> {
        match stat {
            "vigor" => Some(self.vigor),
            "intelligence" => Some(self.intelligence),
            "faith" => Some(self.faith),
            "dexterity" => Some(self.dexterity),
            "arcane" => Some(self.arcane),
            "mind" => Some(self.mind),
            "strength" => Some(self.strength),
            _ => None,
        }
    }

    /// pour afficher le menu grâce.
    #[allow(dead_code)]
    pub fn all(&self) -> [(&str, u32); 7] {
        [
            ("vigor", self.vigor),
            ("intelligence", self.intelligence),
            ("faith", self.faith),
            ("dexterity", self.dexterity),
            ("arcane", self.arcane),
            ("mind", self.mind),
            ("strength", self.strength),
        ]
    }

    pub fn modify(&mut self, stat: &str, amount: u32) -> bool {
        match stat {
            "vigor" => self.vigor += amount,
            "intelligence" => self.intelligence += amount,
            "faith" => self.faith += amount,
            "dexterity" => self.dexterity += amount,
            "arcane" => self.arcane += amount,
            "mind" => self.mind += amount,
            "strength" => self.strength += amount,
            _ => return false,
        }
        true
    }

    pub fn max_hp(&self) -> u32 {
        100 + self.vigor * 10
    }
}
