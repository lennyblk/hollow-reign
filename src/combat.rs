use crate::enemy::{Element, Enemy};
use crate::item::Item;
use crate::player::Player;

/// Résultat de la frappe (mécanique de typing).
/// Normal attacks n'ont pas de ParryResult — seulement les attaques élémentaires.
pub enum ParryResult {
    Perfect, // frappe rapide et sans faute → plein bonus
    Good,    // frappe correcte mais lente → bonus partiel
    Miss,    // faute ou timeout → malus
}

impl ParryResult {
    /// Multiplicateur appliqué aux dégâts offensifs du joueur.
    pub fn attack_multiplier(&self) -> f32 {
        match self {
            ParryResult::Perfect => 1.5,
            ParryResult::Good    => 1.2,
            ParryResult::Miss    => 0.7,
        }
    }

    /// Multiplicateur appliqué aux dégâts reçus en défense.
    pub fn defense_multiplier(&self) -> f32 {
        match self {
            ParryResult::Perfect => 0.0, // parry parfait = 0 dégâts
            ParryResult::Good    => 0.5,
            ParryResult::Miss    => 1.3,
        }
    }
}

pub struct Combat {
    pub enemies: Vec<Enemy>, // max 3
    pub turn: u32,
}

impl Combat {
    pub fn new(enemies: Vec<Enemy>) -> Self {
        assert!(enemies.len() <= 3, "max 3 enemies per combat");
        Combat { enemies, turn: 0 }
    }

    pub fn is_over(&self, player: &Player) -> bool {
        !player.status.can_act() || self.enemies.iter().all(|e| !e.is_alive())
    }

    pub fn next_turn(&mut self) {
        self.turn += 1;
    }

    /// Joueur attaque un ennemi (attaque normale, pas d'élément).
    /// base_damage = weapon.base_damage + stat_ratio du joueur (calculé avant l'appel).
    /// Retourne les dégâts réellement infligés.
    pub fn player_attack(&mut self, enemy_index: usize, base_damage: u32) -> u32 {
        self.enemies[enemy_index].take_damage(base_damage)
    }

    /// Joueur attaque avec un élément (typing challenge réussi).
    /// Retourne les dégâts réellement infligés.
    pub fn player_elemental_attack(
        &mut self,
        enemy_index: usize,
        base_damage: u32,
        parry_result: ParryResult,
    ) -> u32 {
        let damage = (base_damage as f32 * parry_result.attack_multiplier()) as u32;
        self.enemies[enemy_index].take_damage(damage)
    }

    /// Ennemi fait une attaque normale sur le joueur.
    /// Retourne les dégâts reçus.
    pub fn enemy_attack(&self, player: &mut Player, enemy_index: usize) -> u32 {
        let raw = self.enemies[enemy_index]
            .attack
            .saturating_sub(player_defense(player));
        player.take_damage(raw);
        raw
    }

    /// Ennemi fait une attaque élémentaire (typing challenge côté joueur).
    /// Perfect = parry total, aucun effet. Good/Miss = stack appliqué.
    /// Retourne les dégâts burst si 3ème stack atteint.
    pub fn enemy_elemental_attack(
        &mut self,
        player: &mut Player,
        enemy_index: usize,
        parry_result: ParryResult,
    ) -> u32 {
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
                if player.frost.add_stack() {
                    player.status.apply_freeze();
                }
                0
            }
            Some(Element::Lightning) => {
                if player.lightning.add_stack() {
                    player.status.apply_electrocute();
                }
                0
            }
            None => 0,
        };

        let burst = (burst as f32 * parry_result.defense_multiplier()) as u32;
        player.take_damage(burst);
        burst
    }

    /// À appeler en début de chaque tour joueur — applique les ticks élémentaires actifs.
    /// Retourne les dégâts totaux des ticks.
    pub fn tick_player_effects(player: &mut Player) -> u32 {
        let dmg = player.poison.tick(15)
            + player.bleed.tick(20)
            + player.rot.tick(10)
            + player.fire.tick(18);
        player.take_damage(dmg);
        player.status.tick();
        dmg
    }

    /// Retourne les souls totaux à donner au joueur après combat (ennemis morts seulement).
    pub fn collect_souls(&self) -> u32 {
        self.enemies
            .iter()
            .filter(|e| !e.is_alive())
            .map(|e| e.soul_drop)
            .sum()
    }
}

/// Défense du joueur = armure + shield (sauf si arme two-handed).
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
