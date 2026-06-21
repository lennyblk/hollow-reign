use crate::item::Element;
use crate::status::{ElementalEffect, FrostEffect, Status};

type LightningEffect = FrostEffect;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EnemyType {
    Mob,
    MobLeader,
    MiniBoss,
    Boss,
}

pub struct Enemy {
    pub name: String,
    pub enemy_type: EnemyType,
    pub hp: u32,
    pub max_hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub elements: Vec<Element>,
    pub soul_drop: u32,
    pub ascii: &'static str,
    // état élémentaire (buildup appliqué par le joueur)
    pub poison: ElementalEffect,
    pub bleed: ElementalEffect,
    pub rot: ElementalEffect,
    pub frost: FrostEffect,
    pub fire: ElementalEffect,
    pub lightning: LightningEffect,
    pub status: Status,
    pub temp_defense_reduction: u32,
    pub temp_defense_turns: u32,
}

impl Enemy {
    pub fn new(
        name: String,
        enemy_type: EnemyType,
        hp: u32,
        attack: u32,
        defense: u32,
        elements: Vec<Element>,
        soul_drop: u32,
        ascii: &'static str,
    ) -> Self {
        Enemy {
            name,
            enemy_type,
            max_hp: hp,
            hp,
            attack,
            defense,
            elements,
            soul_drop,
            ascii,
            poison: ElementalEffect::new(),
            bleed: ElementalEffect::new(),
            rot: ElementalEffect::new(),
            frost: FrostEffect::new(),
            fire: ElementalEffect::new(),
            lightning: FrostEffect::new(),
            status: Status::default(),
            temp_defense_reduction: 0,
            temp_defense_turns: 0,
        }
    }

    #[allow(dead_code)]
    pub fn can_parry(&self) -> bool {
        matches!(self.enemy_type, EnemyType::Boss)
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    pub fn can_act(&self) -> bool {
        self.status.can_act()
    }

    #[allow(dead_code)]
    pub fn can_respawn(&self) -> bool {
        !matches!(self.enemy_type, EnemyType::Boss)
    }

    #[allow(dead_code)]
    pub fn respawn(&mut self) {
        if self.can_respawn() {
            self.hp = self.max_hp;
            self.poison = ElementalEffect::new();
            self.bleed = ElementalEffect::new();
            self.rot = ElementalEffect::new();
            self.frost = FrostEffect::new();
            self.fire = ElementalEffect::new();
            self.lightning = FrostEffect::new();
            self.status = Status::default();
        }
    }

    /// Dégâts normaux réduits par la défense (armor break pris en compte).
    pub fn take_damage(&mut self, amount: u32) -> u32 {
        let effective_defense = self.defense.saturating_sub(self.temp_defense_reduction);
        let reduced = amount.saturating_sub(effective_defense);
        self.hp = self.hp.saturating_sub(reduced);
        reduced
    }

    #[allow(dead_code)]
    pub fn apply_armor_break(&mut self, reduction_percent: u32, turns: u32) {
        self.temp_defense_reduction = self.defense * reduction_percent / 100;
        self.temp_defense_turns = turns;
    }

    /// Dégâts élémentaires directs (burst/tick) — ignore la défense.
    pub fn take_elemental_damage(&mut self, amount: u32) {
        self.hp = self.hp.saturating_sub(amount);
    }

    /// Applique une stack élémentaire du joueur. Retourne le burst si 3 stacks atteints.
    pub fn apply_elemental_stack(&mut self, element: &Element) -> u32 {
        match element {
            Element::Poison => self.poison.add_stack(80),
            Element::Bleed  => self.bleed.add_stack(100),
            Element::Rot    => self.rot.add_stack(60),
            Element::Fire   => self.fire.add_stack(90),
            Element::Ice => {
                if self.frost.add_stack() {
                    self.status.apply_freeze();
                }
                0
            }
            Element::Lightning => {
                if self.lightning.add_stack() {
                    self.status.apply_electrocute();
                }
                0
            }
        }
    }

    /// Avance les ticks élémentaires d'un tour. Retourne les dégâts totaux.
    pub fn tick_effects(&mut self) -> u32 {
        let dmg = self.poison.tick(15)
            + self.bleed.tick(20)
            + self.rot.tick(10)
            + self.fire.tick(18);
        self.take_elemental_damage(dmg);
        self.status.tick();
        if self.temp_defense_turns > 0 {
            self.temp_defense_turns -= 1;
            if self.temp_defense_turns == 0 {
                self.temp_defense_reduction = 0;
            }
        }
        dmg
    }
}
