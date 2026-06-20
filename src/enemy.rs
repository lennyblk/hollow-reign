pub enum EnemyType {
    Mob,
    MobLeader,
    MiniBoss,
    Boss,
}

pub enum Element {
    Fire,
    Ice,
    Lightning,
    Poison,
    Bleed,
    Rot,
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
        }
    }

    pub fn can_parry(&self) -> bool {
        matches!(self.enemy_type, EnemyType::Boss)
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    /// Les boss meurent définitivement. Les autres respawn à la grace.
    pub fn can_respawn(&self) -> bool {
        !matches!(self.enemy_type, EnemyType::Boss)
    }

    /// Remet l'ennemi à pleine vie. Ne fait rien pour les boss.
    pub fn respawn(&mut self) {
        if self.can_respawn() {
            self.hp = self.max_hp;
        }
    }

    /// Retourne les dégâts réellement subis après réduction par la defense.
    pub fn take_damage(&mut self, amount: u32) -> u32 {
        let reduced = amount.saturating_sub(self.defense);
        self.hp = self.hp.saturating_sub(reduced);
        reduced
    }
}
