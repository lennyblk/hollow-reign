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

/// Cycle : Fire → Ice → Lightning → Bleed → Poison → Rot → Fire
/// Chaque élément bat le suivant (+50% dmg), faible contre le précédent (-25% dmg).
pub enum Element {
    Fire,
    Ice,
    Lightning,
    Bleed,
    Poison,
    Rot,
}

impl Element {
    /// Retourne true si self bat other dans le cycle.
    pub fn beats(&self, other: &Element) -> bool {
        matches!(
            (self, other),
            (Element::Fire,      Element::Ice)
          | (Element::Ice,       Element::Lightning)
          | (Element::Lightning, Element::Bleed)
          | (Element::Bleed,     Element::Poison)
          | (Element::Poison,    Element::Rot)
          | (Element::Rot,       Element::Fire)
        )
    }

    /// Multiplicateur de dégâts élémentaires : attaquant vs défenseur.
    pub fn multiplier_against(&self, defender: &Element) -> f32 {
        if self.beats(defender) {
            1.5
        } else if defender.beats(self) {
            0.75
        } else {
            1.0
        }
    }
}

pub enum WeaponEffect {
    /// Applique la stack élémentaire de l'arme à TOUS les ennemis.
    AoeElemental,
    /// Soigne le joueur pour X% des dégâts infligés.
    Lifesteal { percent: u32 },
    /// Stun l'ennemi ciblé pour 1 tour.
    InstantStun,
    /// Frappe count fois, chaque coup = damage_percent% des dégâts de base.
    MultiHit { count: u32, damage_percent: u32 },
    /// Réduit la défense de l'ennemi de reduction_percent% pendant turns tours.
    ArmorBreak { reduction_percent: u32, turns: u32 },
    /// Si l'ennemi est sous hp_threshold_percent% de hp, dégâts × multiplier.
    Execute { hp_threshold_percent: u32, multiplier: u32 },
    /// Les ticks de rot actifs durent extra_turns tours supplémentaires.
    ExtendedRot { extra_turns: u32 },
    /// La prochaine attaque normale est boostée de bonus_percent%.
    EmpoweredStrike { bonus_percent: u32 },
}

pub struct WeaponAbility {
    pub name: &'static str,
    pub effect: WeaponEffect,
}

pub struct WeaponData {
    pub name: String,
    pub description: &'static str,
    pub base_damage: u32,
    pub scaling: StatScaling,
    pub two_handed: bool,
    pub element: Element,
    pub ability: WeaponAbility,
    pub ascii: &'static str,
}

pub struct ArmorData {
    pub name: String,
    pub description: &'static str,
    pub defense: u32,
    pub ascii: &'static str,
}

pub struct ShieldData {
    pub name: String,
    pub description: &'static str,
    pub defense: u32,
    pub ascii: &'static str,
}

pub enum ConsumableEffect {
    CurePoison,
    CureBleed,
    CureFire,
    CureRot,
    CureFrost,
    CureLightning,
    DealDamage(u32),
    DealFireDamage(u32),
    BuffAttack { bonus: u32, turns: u32 },
    QuestItem,
}

pub struct ConsumableData {
    pub name: String,
    pub description: &'static str,
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
