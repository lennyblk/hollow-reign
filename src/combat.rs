use crate::enemy::Enemy;
use crate::item::{Element, Item, WeaponAbility, WeaponEffect};
use crate::player::Player;

const ELEMENTAL_COOLDOWN: u32 = 3;

#[derive(Debug)]
pub enum ParryResult {
    Perfect, // frappe rapide et sans faute → plein bonus
    Good,    // frappe correcte mais lente → bonus partiel
    Miss,    // faute ou timeout → malus
}

impl ParryResult {
    pub fn attack_multiplier(&self) -> f32 {
        match self {
            ParryResult::Perfect => 1.5,
            ParryResult::Good    => 1.2,
            ParryResult::Miss    => 0.7,
        }
    }

    pub fn defense_multiplier(&self) -> f32 {
        match self {
            ParryResult::Perfect => 0.0,
            ParryResult::Good    => 0.5,
            ParryResult::Miss    => 1.3,
        }
    }
}

pub struct Combat {
    pub enemies: Vec<Enemy>,
    pub turn: u32,
    pub player_elemental_cooldown: u32,
    /// Prochaine attaque normale boostée (Arcane Surge).
    pub player_empowered: bool,
}

impl Combat {
    pub fn new(enemies: Vec<Enemy>) -> Self {
        assert!(enemies.len() <= 3, "max 3 enemies per combat");
        Combat { enemies, turn: 0, player_elemental_cooldown: 0, player_empowered: false }
    }

    pub fn is_over(&self, player: &Player) -> bool {
        player.hp == 0 || self.enemies.iter().all(|e| !e.is_alive())
    }

    pub fn next_turn(&mut self) {
        self.turn += 1;
        self.player_elemental_cooldown = self.player_elemental_cooldown.saturating_sub(1);
    }

    /// Attaque normale du joueur sur un ennemi. Consomme Empowered si actif.
    pub fn player_attack(&mut self, enemy_index: usize, base_damage: u32) -> u32 {
        let damage = if self.player_empowered {
            self.player_empowered = false;
            base_damage * 2
        } else {
            base_damage
        };
        self.enemies[enemy_index].take_damage(damage)
    }

    /// Attaque spéciale de l'arme (tous les 3 tours).
    #[allow(dead_code)]
    pub fn apply_weapon_ability(
        &mut self,
        player: &mut Player,
        target_index: usize,
        element: &Element,
        ability: &WeaponAbility,
        base_damage: u32,
        parry_result: &ParryResult,
    ) -> u32 {
        if self.player_elemental_cooldown > 0 {
            return 0;
        }
        self.player_elemental_cooldown = ELEMENTAL_COOLDOWN;

        let mult = parry_result.attack_multiplier();

        // Burst élémentaire sur la cible (toujours présent)
        let burst_raw = self.enemies[target_index].apply_elemental_stack(element);
        let burst = (burst_raw as f32 * mult) as u32;
        self.enemies[target_index].take_elemental_damage(burst);

        // Effet spécial de l'arme
        let special = match &ability.effect {
            WeaponEffect::AoeElemental => {
                let mut total = 0u32;
                for i in 0..self.enemies.len() {
                    let raw = self.enemies[i].apply_elemental_stack(element);
                    let dealt = (raw as f32 * mult) as u32;
                    self.enemies[i].take_elemental_damage(dealt);
                    total += dealt;
                }
                total
            }
            WeaponEffect::Lifesteal { percent } => {
                let heal = (base_damage as f32 * mult) as u32 * percent / 100;
                player.hp = (player.hp + heal).min(player.stats.max_hp());
                0
            }
            WeaponEffect::InstantStun => {
                self.enemies[target_index].status.apply_stun();
                0
            }
            WeaponEffect::MultiHit { count, damage_percent } => {
                let hit = ((base_damage * damage_percent / 100) as f32 * mult) as u32;
                let mut total = 0u32;
                for _ in 0..*count {
                    total += self.enemies[target_index].take_damage(hit);
                }
                total
            }
            WeaponEffect::ArmorBreak { reduction_percent, turns } => {
                self.enemies[target_index].apply_armor_break(*reduction_percent, *turns);
                0
            }
            WeaponEffect::Execute { hp_threshold_percent, multiplier } => {
                let threshold = self.enemies[target_index].max_hp * hp_threshold_percent / 100;
                if self.enemies[target_index].hp <= threshold {
                    let bonus = ((base_damage as f32 * mult) as u32) * (multiplier - 1);
                    self.enemies[target_index].take_damage(bonus)
                } else {
                    0
                }
            }
            WeaponEffect::ExtendedRot { extra_turns } => {
                self.enemies[target_index].rot.tick_turns += extra_turns;
                0
            }
            WeaponEffect::EmpoweredStrike { .. } => {
                self.player_empowered = true;
                0
            }
        };

        burst + special
    }

    /// Attaque élémentaire du joueur (disponible tous les 3 tours).
    /// Applique le multiplicateur de cycle + stack élémentaire sur l'ennemi.
    /// Retourne (dégâts infligés, burst élémentaire déclenché).
    pub fn player_elemental_attack(
        &mut self,
        player: &Player,
        enemy_index: usize,
        base_damage: u32,
        parry_result: ParryResult,
    ) -> (u32, u32) {
        if self.player_elemental_cooldown > 0 {
            return (0, 0);
        }

        let weapon_element = player_weapon_element(player);

        let elemental_mult = match (&weapon_element, self.enemies[enemy_index].elements.first()) {
            (Some(atk), Some(def)) => atk.multiplier_against(def),
            _ => 1.0,
        };

        let damage = (base_damage as f32
            * parry_result.attack_multiplier()
            * elemental_mult) as u32;

        let dealt = self.enemies[enemy_index].take_damage(damage);

        let burst = if let Some(element) = &weapon_element {
            let burst_raw = self.enemies[enemy_index].apply_elemental_stack(element);
            let burst_dealt = (burst_raw as f32 * parry_result.attack_multiplier()) as u32;
            self.enemies[enemy_index].take_elemental_damage(burst_dealt);
            burst_dealt
        } else {
            0
        };

        self.player_elemental_cooldown = ELEMENTAL_COOLDOWN;
        (dealt, burst)
    }

    /// Attaque normale d'un ennemi sur le joueur.
    pub fn enemy_attack(&self, player: &mut Player, enemy_index: usize) -> u32 {
        if !self.enemies[enemy_index].can_act() {
            return 0;
        }
        let raw = self.enemies[enemy_index]
            .attack
            .saturating_sub(player_defense(player));
        player.take_damage(raw);
        raw
    }

    /// Attaque élémentaire d'un ennemi sur le joueur (typing challenge).
    pub fn enemy_elemental_attack(
        &mut self,
        player: &mut Player,
        enemy_index: usize,
        parry_result: ParryResult,
    ) -> u32 {
        if !self.enemies[enemy_index].can_act() {
            return 0;
        }
        if matches!(parry_result, ParryResult::Perfect) {
            return 0;
        }

        let element = self.enemies[enemy_index].elements.first();
        let burst = match element {
            Some(Element::Poison)    => player.poison.add_stack(80),
            Some(Element::Bleed)     => player.bleed.add_stack(100),
            Some(Element::Rot)       => player.rot.add_stack(60),
            Some(Element::Fire)      => player.fire.add_stack(90),
            Some(Element::Ice) => {
                if player.frost.add_stack() { player.status.apply_freeze(); }
                0
            }
            Some(Element::Lightning) => {
                if player.lightning.add_stack() { player.status.apply_electrocute(); }
                0
            }
            None => 0,
        };

        let burst = (burst as f32 * parry_result.defense_multiplier()) as u32;
        player.take_damage(burst);
        burst
    }

    /// Ticks élémentaires du joueur en début de tour.
    pub fn tick_player_effects(player: &mut Player) -> u32 {
        let dmg = player.poison.tick(15)
            + player.bleed.tick(20)
            + player.rot.tick(10)
            + player.fire.tick(18);
        player.take_damage(dmg);
        player.status.tick();
        dmg
    }

    /// Ticks élémentaires d'un ennemi en début de son tour.
    pub fn tick_enemy_effects(&mut self, enemy_index: usize) -> u32 {
        self.enemies[enemy_index].tick_effects()
    }

    pub fn collect_souls(&self) -> u32 {
        self.enemies.iter().filter(|e| !e.is_alive()).map(|e| e.soul_drop).sum()
    }
}

fn player_weapon_element(player: &Player) -> Option<&Element> {
    player.equipment.first()
        .and_then(|e| e.weapon.as_ref())
        .and_then(|w| if let Item::Weapon(wd) = w { Some(&wd.element) } else { None })
}

fn player_defense(player: &Player) -> u32 {
    let equip = match player.equipment.first() {
        Some(e) => e,
        None => return 0,
    };
    let armor_def = match &equip.armor {
        Some(Item::Armor(a)) => a.defense,
        _ => 0,
    };
    let is_two_handed = matches!(&equip.weapon, Some(Item::Weapon(w)) if w.two_handed);
    let shield_def = if !is_two_handed {
        match &equip.shield {
            Some(Item::Shield(s)) => s.defense,
            _ => 0,
        }
    } else {
        0
    };
    armor_def + shield_def
}
