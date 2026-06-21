const STACK_THRESHOLD: u32 = 3;
const TICK_DURATION: u32 = 3;
const FROZEN_DURATION: u32 = 3;
const ELECTROCUTED_DURATION: u32 = 3;

/// Effet élémentaire (poison, bleed, rot) — stacks + tick dégâts.
pub struct ElementalEffect {
    pub stacks: u32,
    pub tick_turns: u32,
}

impl ElementalEffect {
    pub fn new() -> Self {
        Self { stacks: 0, tick_turns: 0 }
    }

    /// Ajoute une stack. Retourne le burst de dégâts si 3 stacks atteints (puis reset).
    pub fn add_stack(&mut self, burst_damage: u32) -> u32 {
        self.stacks += 1;
        if self.stacks >= STACK_THRESHOLD {
            self.stacks = 0;
            self.tick_turns = TICK_DURATION;
            return burst_damage;
        }
        0
    }

    /// Avance d'un tour. Retourne les dégâts du tick (0 si inactif).
    pub fn tick(&mut self, damage_per_turn: u32) -> u32 {
        if self.tick_turns > 0 {
            self.tick_turns -= 1;
            return damage_per_turn;
        }
        0
    }

    pub fn is_ticking(&self) -> bool {
        self.tick_turns > 0
    }
}

/// Effet frost — stacks puis freeze X tours (pas de tick dmg).
pub struct FrostEffect {
    pub stacks: u32,
}

impl FrostEffect {
    pub fn new() -> Self {
        Self { stacks: 0 }
    }

    /// Ajoute une stack. Retourne true si 3 stacks atteints (déclenche le freeze, puis reset).
    pub fn add_stack(&mut self) -> bool {
        self.stacks += 1;
        if self.stacks >= STACK_THRESHOLD {
            self.stacks = 0;
            return true;
        }
        false
    }
}

#[allow(dead_code)]
pub enum Status {
    Alive,
    Dead,
    Sleeping,
    Frozen { turns_remaining: u32 },
    Electrocuted { turns_remaining: u32 },
    Stunned { turns_remaining: u32 },
}

impl Default for Status {
    fn default() -> Self {
        Status::Alive
    }
}

impl Status {
    pub fn can_act(&self) -> bool {
        match self {
            Status::Frozen { .. }
            | Status::Electrocuted { .. }
            | Status::Stunned { .. }
            | Status::Dead
            | Status::Sleeping => false,
            _ => true,
        }
    }

    #[allow(dead_code)]
    pub fn apply_stun(&mut self) {
        *self = Status::Stunned { turns_remaining: 1 };
    }

    /// Déclenche le freeze (appelé quand FrostEffect atteint 3 stacks).
    pub fn apply_freeze(&mut self) {
        *self = Status::Frozen { turns_remaining: FROZEN_DURATION };
    }

    /// Déclenche l'électrocution (appelé quand LightningEffect atteint 3 stacks).
    pub fn apply_electrocute(&mut self) {
        *self = Status::Electrocuted { turns_remaining: ELECTROCUTED_DURATION };
    }

    /// Avance d'un tour — décrémente les statuts temporaires.
    pub fn tick(&mut self) {
        match self {
            Status::Frozen { turns_remaining }
            | Status::Electrocuted { turns_remaining }
            | Status::Stunned { turns_remaining } => {
                if *turns_remaining <= 1 {
                    *self = Status::Alive;
                } else {
                    *turns_remaining -= 1;
                }
            }
            _ => {}
        }
    }
}
