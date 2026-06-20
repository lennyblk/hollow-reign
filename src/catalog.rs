use crate::item::{
    ArmorData, ConsumableData, ConsumableEffect, Element, Item, ShieldData, StatScaling,
    WeaponAbility, WeaponData, WeaponEffect,
};

// ─── WEAPONS ────────────────────────────────────────────────────────────────

pub fn worn_shortsword() -> Item {
    Item::Weapon(WeaponData {
        name: "Worn Shortsword".into(),
        description: "Une lame courte usée par des années de combat. Inflige des saignements progressifs. Ability: Lacerate — applique Bleed à tous les ennemis.",
        base_damage: 18,
        scaling: StatScaling::Dexterity,
        two_handed: false,
        element: Element::Bleed,
        ability: WeaponAbility {
            name: "Lacerate",
            effect: WeaponEffect::AoeElemental,
        },
        ascii: r#"
                             ▓▓▓
                            ▓▓▒
                          ▒▓▓▒
                         ▓▓▓▒
                       ░▓▓▓▒
                      ▒▓▓▓▒
                     ▓▓▓▓░
                   ░▓▓▓▓░
                  ▓▓▓▓▓░
             ░░ ░▓▓▓▓▓░
             ░▓▓▓▓▓▓▓▒░░░
               ▓▓▓████▓░
              ▓▓▓▓░
             ▓▓▓▓░
           ▒▓▓▓▓
          ▒▓▓▓▓
         ▓▒▓▓░
       ░▒▓▓▓▒
      ░▓██▓▒░
        ░░░
"#,
    })
}

pub fn hunters_blade() -> Item {
    Item::Weapon(WeaponData {
        name: "Hunter's Blade".into(),
        description: "Lames de chasseur forgées pour traquer et saigner les proies. Ability: Blood Feast — récupère 25% des dégâts infligés en PV.",
        base_damage: 26,
        scaling: StatScaling::Dexterity,
        two_handed: false,
        element: Element::Bleed,
        ability: WeaponAbility {
            name: "Blood Feast",
            effect: WeaponEffect::Lifesteal { percent: 25 },
        },
        ascii: r#"
           ▒▓
          ▒▒▒▒▒▓▓▓▒▒▒
              ▒▓▓▓▒▒▓▓▒
                ▓███▓▒▓▓▓
                  ▓████▒▓▓▒
  ▒▓█▓▒            ▓████▓▓█▒
    ▓▒ ▒▒           ▒████▒▓▒
     ▓▒▓▒▒▒     ▒▓▒  ▓▓▓▓▒▓█▓
      ▓▓▒      ▓▒█▓▓    ▓▒▒▓█▓
       ▒▒  ▒▒▓█▓▓ ▓▒     ▓▒▓▒▓▓▒
       ▓▓██▓▓▒▓          ▓▓▒▒ ▓▓▒
    ▓███████▓             ▓█▓▒▓▓ ▒
        ▓▒▒▓█▒            ▓█████▓█▓▓
        ▓▓▒ ▒█           ▓▓ ████▓ ▒
         ▓█▓▒▒▒     ▒  ▓▓█▓█▒ ▒▒
           ▓█▓▓▒▒   ▓▓▒▓▓     ▓▒▓
            ▓█▒▓███▓ ▓▓     ▓▓▒▓▒▓
             ▓▓▓███▓          ▒▓▒▓▓
              ▓▓▒███▓▒           ▒▓▓▒
               ▓▓█▓▓▓█▓▒
                  ▓▓▓▓▓▓▒▒▒
                       ▓▒▒▓▓
"#,
    })
}

pub fn silverwind_rapier() -> Item {
    Item::Weapon(WeaponData {
        name: "Silverwind Rapier".into(),
        description: "Rapière fine ciselée d'argent glacial. Ses attaques élémentaires gèlent les chairs. Ability: Flash Freeze — gèle l'ennemi ciblé 1 tour.",
        base_damage: 32,
        scaling: StatScaling::Dexterity,
        two_handed: false,
        element: Element::Ice,
        ability: WeaponAbility {
            name: "Flash Freeze",
            effect: WeaponEffect::InstantStun,
        },
        ascii: r#"
                                     ▓▒
                                    ▓▒▒
                                   ▒▒▒
                                  ▒▒▒
                                 ▒▒▒
                                ▒▒▓
                               ▒▒▒
                              ▒▒▓
                             ▒▒▓
                            ▒▒▓
                           ▒▒▓
                         ▓▒▒
                        ▒▒▒
                       ▒▒▒
                      ▒▒▒
                     ▒▒▒
                    ▒▒▒
                ▓▓▓▓▓▓
             ▓▓▓▓▓▓▓▓▓▓
              ▓█ ▓▓▓▓▓▓▓▓
              ▓ ▓▓█
              ▓▓▓█
              ▓▒▓
"#,
    })
}

pub fn ashfall_katana() -> Item {
    Item::Weapon(WeaponData {
        name: "Ashfall Katana".into(),
        description: "Katana forgé dans les cendres d'un volcan mort. Requiert deux mains. Ability: Ember Dance — frappe deux fois à 60% de dégâts chaque coup.",
        base_damage: 38,
        scaling: StatScaling::Dexterity,
        two_handed: true,
        element: Element::Fire,
        ability: WeaponAbility {
            name: "Ember Dance",
            effect: WeaponEffect::MultiHit {
                count: 2,
                damage_percent: 60,
            },
        },
        ascii: r#"
                                            ░░░
                                           ░▓▒▒
                                          ░▒▒░
                               ░░░       ░▒▒░
                              ░██▒░     ░▒▒░
                             ▓██▒      ░▒▒░
                           ░▓█▓░      ▒▒░░
                          ░███▒    ░░▓▒░░
                        ░▒███▓▒▒▒▒▓▓██▓░
                        ▒█▓░▒█▓░  ░▒▒░░░
                      ░▓█▓░ ░▒   ░▒░
                     ░▓█▓░      ░▒░
                    ░▓█▒░      ░░░
                   ▒▓█▒      ░░░
                  ▒█▓▒      ░░░
                 ▒█▓▒     ░░░
                ▒█▓▒     ░░░
               ▒█▓▒     ░░░
              ▒█▓▒    ░▒░
             ▒█▓▒   ░░░
            ▒█▓░   ░░░
          ░▒██░  ░▒░
         ░▒█▓▒ ░░▒░
         ░█▓▒░░▒░
            ░▒▒
          ░░░░
"#,
    })
}

pub fn dawnbreaker() -> Item {
    Item::Weapon(WeaponData {
        name: "Dawnbreaker".into(),
        description: "Épée longue chargée de foudre, forgée à l'aube d'une tempête. Ability: Storm Call — applique Lightning à tous les ennemis en même temps.",
        base_damage: 48,
        scaling: StatScaling::Dexterity,
        two_handed: false,
        element: Element::Lightning,
        ability: WeaponAbility {
            name: "Storm Call",
            effect: WeaponEffect::AoeElemental,
        },
        ascii: r#"
                                            ░
                                            ▒
                                          ░░░
                                         ░░░
                                     ▒░░░░░
                                  ░▓▓▒░░░
                                ░▓▓▓░ ░
                               ░▓▓▒  ░
                             ░▒▓▓░░
                     ▒░    ▒▓▒░░
                   ▒█░░░▒▓▒░░░
                   ██▓▓▓░ ░░
                   ▓▓▓▓▓▒░░
                  ▓██▓▒░
                ▒▓   ▓▓█
              ▒▓    ░█▓
            ░▓░    ░▓░
          ░▒▓
        ░██▓
       ░▓██░
     ░▓██▓  ░▒
   ▒██████▓░
"#,
    })
}

pub fn ironwood_club() -> Item {
    Item::Weapon(WeaponData {
        name: "Ironwood Club".into(),
        description: "Massue lourde taillée dans du bois de fer, imbibée de toxines. Requiert deux mains. Ability: Toxic Bludgeon — réduit l'armure ennemie de 50% pendant 3 tours.",
        base_damage: 22,
        scaling: StatScaling::Strength,
        two_handed: true,
        element: Element::Poison,
        ability: WeaponAbility {
            name: "Toxic Bludgeon",
            effect: WeaponEffect::ArmorBreak {
                reduction_percent: 50,
                turns: 3,
            },
        },
        ascii: r#"
  ▒█▓░
 ░▓███▒
   ░▓███░
     ▒▓██▓░
      ░▒▓██▓░
        ░▒▓▓▓▓░
          ░▒▓▓▓▒░
            ░▓▓█▓▒░
             ░▒▓▓▓▓▒░
               ░▒▓▓▓▓▒░
                 ░▓▓▓▓▒░░
                  ░▒▓▓▒▒▒▓░░▒▒
                    ░▓▓█▓░▒▓░  ░░░
                     ░▒▓█▓▒▒▒▓▒░   ░░
                   ░▒▒░░▓██▓░▒▓▓░░▒░
                        ░▒████▓░▒▓░  ░▒░
                      ░▒░  ▒▓██▒▓▓▓▒▓░░
                          ▒▓▒▒████▓░▒▓░  ░░
                        ░░    ░▓██▓▒▒▓█▓▒░
                            ░▒▓░░▓███▓███▓░
                           ░░    ░▒▓███████░
                                ▒▒░░▒█████▓░
                               ░      ▓█▓░
"#,
    })
}

pub fn gravecrusher() -> Item {
    Item::Weapon(WeaponData {
        name: "Gravecrusher".into(),
        description: "Fléau massif imprégné de spores de charnier. Requiert deux mains. Ability: Pestilence — propage le Poison à tous les ennemis simultanément.",
        base_damage: 42,
        scaling: StatScaling::Strength,
        two_handed: true,
        element: Element::Poison,
        ability: WeaponAbility {
            name: "Pestilence",
            effect: WeaponEffect::AoeElemental,
        },
        ascii: r#"
                             ███
                            █▓▓▓▓█
                           ▓▓▒▓▓▓▓▓██
                          ▓▓▓▓▓▓▓▓▓▓▓▓█
                         ▓░░▒▓▓▓▒▓▓▓▓███
                         ▒░▓▓▒░░▒▒░░▒▒███
                        ▒▒▓▓▓▒▒▒░▒▓░░▓▓▓▓
                       ▒▒▒▒▒░▒░░▒░░░░░░▓▓
                     ░░░▒▒▒░░░▒▒▒▒░░▒▓▓▓
                    ░░░░▒▒▒▒▒░░░░▒▓▓
                   ░░░░░▒░░▒░▒▒▒▓▓
            ▓▓▓▓▓▓░░░░▒▒░░░░▒▓▓
           ▓▓▓▓▓▓▒░░░░░░░░▒▓▒
         ▓▓▒▒▓▓▓▓▒▒▒░░░░▒▓
         ▓▓▓▓▒▒▒▒▓▒░░░▒▓▓
         ▓▓▓▓▓▒▒▒░░░▓▓▓▓▓
           ▓▓▓▓▓▓▒░▒▒▓█▓
           ░░▒▓▓▒▓▓▓▓▓
         ░░▒▓   ▓▓▓▓
       ░░▒▓
    ▓▒░▓▓
    ▓▒▓▓
"#,
    })
}

pub fn sundermaul() -> Item {
    Item::Weapon(WeaponData {
        name: "Sundermaul".into(),
        description: "Marteau de guerre colossal qui brise les os et fait jaillir le sang. Requiert deux mains. Ability: Obliterate — dégâts ×3 si l'ennemi est sous 25% PV.",
        base_damage: 62,
        scaling: StatScaling::Strength,
        two_handed: true,
        element: Element::Bleed,
        ability: WeaponAbility {
            name: "Obliterate",
            effect: WeaponEffect::Execute {
                hp_threshold_percent: 25,
                multiplier: 3,
            },
        },
        ascii: r#"
              ███   ██████
             █▓▓█████▓▓█▓▒▓▓
       ████████████▓▓▓▓▓▓▓░░▓
     ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓█▓▓▓▓█▒▒▒▓
    ▓▓█▓▓▒░░▓▓▓▓▓▓▓█▓▓▓██▓▓▓██
    █████▓▓▒▒██▓▓▓███▓████████
    ▓████▓▓▒▒▓█████████████▓██
     ▓███▓▓▓▓███████████████
     █▓██▓▓████████████
       ██████████████
                ██████
                 ▓▓▓██
                 ▓▓▓██
                  ▓▓▓█
                  █████
                  ▓▓▓██
                  ▓▓▓██
                   ▓▓▓██
                   ▓▓▓██
                   ▓▓▓██
                    ▓▓██
                    █▓▓██
                    █▓▓██
                     █▓██
                     █▓█▓█
                     ▓▓▓███
                      ███
"#,
    })
}

pub fn voidstaff() -> Item {
    Item::Weapon(WeaponData {
        name: "Voidstaff".into(),
        description: "Bâton nourri de néant putride, corrodant tout ce qu'il touche. Scale sur Mind+Faith. Ability: Entropy — les ticks de Rot actifs durent 2 tours de plus.",
        base_damage: 24,
        scaling: StatScaling::MindAndFaith,
        two_handed: false,
        element: Element::Rot,
        ability: WeaponAbility {
            name: "Entropy",
            effect: WeaponEffect::ExtendedRot { extra_turns: 2 },
        },
        ascii: r#"
          ▓

         ▓

            ▒      ▒▒▓▓▓
                ▓▒▒▒▒▓▓█
             ▒ ▒▒  ▒▓▒█
          ▒   ▓▒    ▒▓▓
       ▓▒▓ ▒▒ ▒ ▒▒ ▓█
       ▒▒▓▒▒  ▒▒▓█▓█
         ███▓▓██  █▓▓▓█
                  ▓▓▓▓▓
                  ██▓███
                     ██▓▓
                      ▓▒▒

                         ▒
                          ▒▒▓▓▓
                           ▓▓▓▓▓█
                           █▓▓▓▓▓▓
                            █▓▓▒▓▓
                               ▒
"#,
    })
}

pub fn runebreaker() -> Item {
    Item::Weapon(WeaponData {
        name: "Runebreaker".into(),
        description: "Faux magique gravée de runes arcanes crépitantes d'éclairs. Scale sur Intelligence. Ability: Arcane Surge — la prochaine attaque normale inflige le double de dégâts.",
        base_damage: 46,
        scaling: StatScaling::Intelligence,
        two_handed: false,
        element: Element::Lightning,
        ability: WeaponAbility {
            name: "Arcane Surge",
            effect: WeaponEffect::EmpoweredStrike { bonus_percent: 100 },
        },
        ascii: r#"
                   █▓
                     ▒▓███▓
                       ▓██ ▓▓
                       ████████
                      ▒███▒▓▓▓████
                      ███▓▒   ▒████
                     ▓██         ▓███
                    ▒█▓▒          ▒▓█
                   ▓█▓             ▓█▓
                  ▓▓                ▓█
                 ▓▓                 ▒▓
               ▓█▒                  ▒▓
              ▓▓▓                   ▒
             ▓█
           ▓█▓
          ▓▓▓
         ▓█▒          ▒▒▓▒
       ▓▓▓
      ▓▓▓
     ▓█▒
    ▓▓▓
"#,
    })
}

// ─── ARMORS ─────────────────────────────────────────────────────────────────

pub fn tattered_rags() -> Item {
    Item::Armor(ArmorData {
        name: "Tattered Rags".into(),
        description: "Lambeaux de tissu à peine cousus ensemble. Offre peu de protection mais ne gêne pas les mouvements.",
        defense: 2,
        ascii: r#"


               █░▓▓▓░░▓█
            █░░ ░█▓░ ░░░▓▓█
           █░░░ ░█▓  ░░░▓▓▓██
          █░░   ░░  ░▓█▓▓██▓▓█
         █░░░      ░▓██▓██▓▓▓▓██
       █░░▓▓██   ░▓▓██▓███▓▓▓▓███
     ▓  ░█████░ ░░▓█▓▓█████▓▓██▓▓██
    █░▓████  █  ░░▓░██████ ███▓░  ░█
     ██     ▓░   ░░▓▓█████    █▓░░ ░▓█
            ▓    ░ ░░░▓▓██      ██░░░▓█
           █▓█ ░░░░▓██████        █▓██
           ▓█░░▓▓░█▓▓▓▓███
          █░█░░▓█░█░░▓▓███
          ███░░░█░█▓░░█▓███
            █░▓░█░▓░░░█░███
            █ █▓░░░░░░█▓██
            █░▓██▓▓█░░░██
             ░░█    ███

"#,
    })
}

pub fn pilgrims_coat() -> Item {
    Item::Armor(ArmorData {
        name: "Pilgrim's Coat".into(),
        description: "Manteau de voyage épais porté par les pèlerins errants. Résistant aux intempéries, légèrement renforcé aux épaules.",
        defense: 6,
        ascii: r#"


                    ░░

               ░░░░   ▓░░░░
            ░░░░░░░░░░▓░▓░▓▓░
           ░░▓░░░░░░  ▓░▓▓▓█▓░
           ░▓█░░░░░ ░ ▓▓▓▓██▓▓
           ░█▓░░░░░░░ ░▓▓▓██▓▓░
          ░░█▓░░░░░░░ ░▓▓▓██▓▓░
          ░░█ ░░░░░░░ ░█▓█░░▓▓▓
         ░░▓  ░░░▓░▓   ▓▓▓░ ▓▓▓
         ░░▓  ░░░░░░   ▓██░ █▓▓
         ░░▓░ ░░░░▓░   ▓██▓ ▓▓▓▓░
        ░░░█░░░░▓░▓    ░▓██ ▓░░▓
        ░░░▓▓▓░░░░░ ░   ▓███░░░▓
        █▓█▓ ░▓░░░  █░  ▓███ ▓██░
          ░░░░░░░░▓▓██▓▓░███ ░░
           ░█░░░░▓█████▓░███░ ▓
          ░▓▓░░░░███▓██▓ ███▓ ░
           ▓░░░░░██▓▓██▓ ████
          ░░░░░░░█▓▓▓█▓▓░▓███
          ░▓░░░░░▓▓▓▓█▓░░▓██▓▓
           ░░░░░    ▓    ░▓█░
              ░░    ░     ░
"#,
    })
}

pub fn leather_vest() -> Item {
    Item::Armor(ArmorData {
        name: "Leather Vest".into(),
        description: "Veste de cuir tanné clouté aux articulations. Bon compromis entre mobilité et protection.",
        defense: 10,
        ascii: r#"

                 ░     ░
                ░███   ▓
             ░██████░ ░▓▓▓░
         ░█▓█████████░▓▓▓▓▓▓██
          ░░▓▓▓█▓█▓▓▓█▓▓▓▓▓▓▓▓
           ░░▓██▓▓▓▓▓▓▓▓░░░░▓▓
            ░█████▓░░▓▓▓░░░▓▓
            ░██████▓▓▓▓▓▓▓▓░░
             ██████▓▓█▓▓▓▓█░
             ▓████████▓▓▓▓█▓
             ░██████████▓██░
             ░██████▓█▓▓█▓▓░
             ▓██████████▓▓▓▓
            ░█████████▓▓▓█▓█░
            ▓█████████▓▓▓▓▓█▓
           ░█████████▓█▓▓▓███░
           ▓█████████░███▓███░
             ░▓█████▓░▓▓▓▓░
             ░░ ░░░░░░░░
                    ▓░
                    █░
"#,
    })
}

pub fn shadowweave_mantle() -> Item {
    Item::Armor(ArmorData {
        name: "Shadowweave Mantle".into(),
        description: "Manteau tissé de fils d'ombre absorbant la lumière. Favori des assassins et des voleurs de la nuit.",
        defense: 13,
        ascii: r#"


                ▓       ░▓
              ░░▓▓▓██▓▓▓▓▓▓▓
           ░▓▓▓▓▓▓▓▓▓▓▓▓▓░░▓▓▓
          ░▓▓▓▓▓▓░ ░▓▓▓░░░▓░░▓▓▓░
          ░▓▓▓▓░    ░░░░▓░░░▓▓▓▓░
         ░▓▓▓▓░        ░▓▓▓▓░░▓▓░
         ░▓▓▓▓░░     ░░░▓▓▓█▓▓▓▓▓▓
          ░▓▓ ░░░░░░░░░▓░▓▓▓░▓▓▓▓▓
          ▓▓▓ ░░░░░░░▓▓▓▓▓▓▓░░▓▓▓▓
        ▓▓▓▓▓▓░▓▓▓▓░▓▓▓▓▓▓▓░ ░░▓▓▓
       ░▓▓▓▓  ░▓░░░░▓▓▓▓▓▓▓   ▓▓▓░▓
       ░▓▓▓   ▓▓▓░░░▓▓░▓▓▓░    ░▓▓▓
      ░▓▓▓   ░▓▓▓░░▓▓▓░▓▓▓▓░    ▓▓▓
      ▓▓▓    ▓▓▓▓▓▓▓▓░▓▓▓▓▓▓    ░▓▓
            ▓▓░▓▓░░▓▓░▓░░▓▓▓     ░▓
            ▓░ ▓▓░▓▓▓▓▓░░▓▓▓░      ░
                ▓░▓▓    ▓▓▓▓


░ ░░ ░   ░ ░▓   ▓███████▓ ▓░      ░ ░ ░ ░
"#,
    })
}

pub fn ashcaster_robes() -> Item {
    Item::Armor(ArmorData {
        name: "Ashcaster Robes".into(),
        description: "Robes de mage imprégnées de cendres arcanes. Légères mais délicates, conçues pour canaliser la magie sans entrave.",
        defense: 8,
        ascii: r#"

                 ░░░░░░░░░
              ░░░▓▓▓▓▓█▓▓▓░░
            ░░░░░▓▓▓▓█▓▓▓▓▓▓▓░░
          ░░░▓░░░░░▓▓▓░░▓░▓▓▓▓▓▓░
          ░▓▓▓▓▓▓▓▓▓▓░▓▓▓▓▓▓▓▓▓░
           ░▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓█▓▓░
          ░▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓██▓▓░
          ░▓▓██▓▓▓▓░░▓▓▓▓▓▓██▓▓▓
         ░▓▓▓█░▓▓▓▓▓▓░░▓▓▓▓░▓██▓▓░
        ░░▓▓▓░ ░▓▓▓▓▓░▓▓▓▓▓  ▓▓▓▓▓
       ░░░▓█▓  ▓▓░░░░░▓▓▓▓▓░ ░▓▓▓▓▓
       ░░▓▓▓  ░▓▓░░ ░░░░▓▓▓▓  ░▓▓▓▓░
       ░▓▓░   ░▓▓▓░░░░░▓▓▓▓▓   ░▓▓▓░
      ░░░░  ░▓░▓▓▓▓▓▓▓▓▓▓▓▓▓░   ░░░░
       ░░    ▓▓▓▓░▓▓▓▓▓▓▓▓▓▓▓░    ░
       ░     ░░▓▓░▓▓▓▓▓▓▓▓▓▓▓░    ░
            ░▓░▓░░▓▓▓▓▓▓▓▓▓▓▓░
            ░▓▓▓▓█▓▓▓▓▓▓▓▓▓▓▓▓
            ░▓▓▓░▓▓▓▓▓▓▓▓▓░▓▓▓
            ░▓▓▓░▓▓▓▓▓▓▓▓▓░▓▓▓░
           ░▓▓▓░░▓▓▓▓▓▓▓▓▓░▓▓▓▓
           ░▓▓▓░░██▓▓▓▓██▓░▓▓▓▓░
          ░▓▓▓▓▓░███▓▓███▓░░▓▓▓▓
          ▓▓▓▓▓░░███▓▓▓▓█▓░░▓▓▓▓░
         ░▓▓▓▓▓░▓██▓▓▓▓▓█▓░▓▓▓▓▓▓
         ▓▓▓▓▓▓░▓▓█▓░░▓██▓░▓▓▓▓▓▓▓
         ░▓▓▓▓▓░▓▓██░░▓▓▓▓░▓▓▓▓▓▓
          ░▓▓▓▓░▓░▓▓░░▓▓░░░▓▓▓▓▓░░   ░
          ░▓▓▓▓▓▓        ░▓▓▓▓▓▓░░░░░░
            ░▓▓▓▓░░░░░░░░░▓▓▓▓░░░░░░
              ░▓▓▓░░░░░  ░▓▓░  ░░░
                 ░░
"#,
    })
}

pub fn chainmail() -> Item {
    Item::Armor(ArmorData {
        name: "Chainmail".into(),
        description: "Cotte de mailles forgée à la main, anneau par anneau. Protection solide contre les lames sans sacrifier trop de mobilité.",
        defense: 16,
        ascii: r#"

           █▒▒█▒▒▒▒▒▒▒▒
      █▒▒▒▒▒▒▒█████▒▒▒▒▒▒▒▒▒█▒▒▒
     █▒▒▒▒▒▒▒███▒▒▒▒▒▒▒▒▒██▒▒▒▒▒▒▒
    █▒▒▒▒▒▒▒███▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒█▒▒▒▒
    █▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒██▒
    ██▒▒▒▒▒ ▒  ▒▒▒▒▒▒▒▒▒▒▒█▒▒▒▒▒▒█▒▒
    █▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒█▒█▒▒█▒▒▒▒
    ▒▒▒█▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒█▒█▒▒▒▒▒▒█▒▒
       █▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒█▒██▒▒▒▒▒▒█▒▒
       █▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒█▒▒▒▒█▒█▒
       ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒█▒█▒
        ██▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒██▒▒
        █▒▒▒▒▒▒▒▒▒▒▒▒██▒████▒
        █▒▒▒▒▒█▒▒▒▒▒▒▒▒▒▒█▒▒▒
        ███▒▒▒▒▒▒▒▒▒▒▒▒▒███▒▒
        ███▒▒▒▒▒▒▒▒▒███▒▒██▒▒
        █▒▒▒▒▒▒▒▒▒▒▒███▒██▒▒▒
        ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒█▒█▒█
         ▒▒▒▒▒▒▒▒▒▒▒▒▒▒█▒▒██▒
                ▒▒▒▒▒▒▒▒▒▒
"#,
    })
}

pub fn wardens_harness() -> Item {
    Item::Armor(ArmorData {
        name: "Warden's Harness".into(),
        description: "Harnais de garde-frontière en métal battu et cuir épais. Porte les marques de nombreux combats survivés.",
        defense: 20,
        ascii: r#"



                ▒▓  ▓▒▒▒▒▒
         ▓▒▒▒▒▓▓▒▒▓▒▒▒▒░▒▒▒▒▒▓
       ▓▒▒▒▒░░▒▓▒▒▒▒░▒▒▒▒▒▒▒▒▒▓▒▒
      ▓▒▒▒▒░░░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▓
      █▓▒▒▒░▒░░░░░▒▒▒░▒▒▒▓▒▒▒▓▓▒▒▓▓
      ▓▒▒▒▒░░░░░▒░░░▒▒▒▓▒▓▓▓▓▓▓▓▓▒▓▒
     █▒▒▒▓▓▒▒▒▓▓▓▒▒▒▒▒▒▓▓▒▓▓▓▓▒▒▒▓▓▓█
   █▒░▒▒▒▓▓▓▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓█▓▒▒▒▒▒▓▓
 █▒░▒▒▒▓▓▓▓▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓█▓▒▒░▒▒▒▓▓▓
  █▓▒▓▓▓▓█▒▒▒▒▓▒▓▓▓▓▓▓▓▓▓▓▒█   █▒░▒▒▒▓▓█
    ██▓▓  █▓▒▓▓▓▓▓▓▓▓▓▓▓▓▓▒     █▓▓▓▓▓
          █▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓     █
         ██▒▓▒▒▒▒▒▓▓▓▓▓▓▓▓▓▒
          █▒▒▒▒▓▓▓▓▓▓▓▓▓▒▓▓▒▒
          █▒▒▒▒▒▒▓▒▒▓▒▒▒▒▒▓▓▓
           ██▓▒▒▒▒▒▒▒▒▒▓▓▓█


"#,
    })
}

pub fn knights_plate() -> Item {
    Item::Armor(ArmorData {
        name: "Knight's Plate".into(),
        description: "Armure complète d'un chevalier d'élite. Lourde et imposante, elle offre une protection exceptionnelle au prix de la vitesse.",
        defense: 26,
        ascii: r#"



             ▓▓▓░░░░░░ ▓▓▓░
          ▓░▓░ ░▓▓▓▓▓▓██▓█▓░░
       ░▓▓▓▓▓░░░░▓░       ▓░░░░
       ▓█▓▓▓▓▓░░▓▓░▓░    ░▓▓▓▓░░
       ▓▓▓▓▓▓▓░░▓▓▓▓░░░░░░░▓▓▓▓▓░
      ▓▓▓▓▓▓▓▓███▓▓▓▓░▓░░▓░░▓▓█▓░
     ░▓▓▓▓░▓████▓▓▓▓▓▓▓▓▓▓▓░░▓██░░
    ▓▓█▓░▓▓▓███▓▓▓▓▓▓▓▓▓▓▓▓▓░▓▓▓▓█▓▓
  ▓▓▓▓▓███▓▓▓█▓▓▓▓▓▓▓▓▓▓▓▓▓░▓▓████▓█▓
 ░▓▓▓▓▓▓▓▓▓▓▓██▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓█▓▓ ░▓░
  ░▓▓████▓▓ ▓▓█▓▓▓▓▓▓▓▓▓▓▓▓▓▓█▓▓▓▓█▓░░░
  ▓▓░░░▓▓░  ▓▓█▓▓▓▓▓▓▓▓▓▓▓▓▓▓   ░▓▓▓▓░ ░
 ▓▓░░░▓▓      ░▓▓▓▓▓▓░░░░░▓░▓     ▓▓▓▓░░░
 ▓▓░░░▓                             ▓▓▓▓░
░▓░░░▓
▓▓░░▓░


"#,
    })
}

pub fn hollowed_armor() -> Item {
    Item::Armor(ArmorData {
        name: "Hollowed Armor".into(),
        description: "Armure d'un guerrier mort-vivant, rongée mais toujours solide. Dégage une aura menaçante qui perturbe les ennemis fragiles.",
        defense: 22,
        ascii: r#"

            █▒███
         █▒▒▒▒██▒▒      ▒▒▒▒▒█
        ▒▒▒▒▒ ▒▒▒▒▒▒▒▒▒▒▒▒ ▒▒▒▒█
       █▒▒▒▒▒▒▒▒▒▒▒    ▒      ▒█
       ▒█▒▒▒▒▒▒▒▒              ▒▒
         ██▒█▒▒         ▒    ▒ ▒▒█
         ██▒▒██▒▒▒      ▒▒▒▒▒▒▒▒▒█
         ▒▒▒▒▒███▒ ▒▒ ▒▒██▒▒▒▒██
        █▒▒ ▒▒▒▒▒▒▒ ▒ ▒███▒▒▒▒▒▒█
        █▒ ▒█▒▒▒▒▒▒▒▒▒▒████▒▒ ▒▒▒█
        █▒▒█▒██▒▒▒▒▒▒▒▒▒▒██▒  ▒▒██
        ▒▒▒▒ ▒▒█████▒▒▒▒███▒ ▒▒▒██
       █▒▒▒▒▒▒▒▒  ▒▒▒▒▒ ▒▒██▒▒▒▒█▒
       ▒▒▒▒▒█▒▒▒    ▒▒   ▒▒██ ▒▒██▒
      ▒ ▒ ▒██▒▒▒▒▒  ▒▒▒▒▒▒▒▒█ ▒▒███
      █▒▒▒█████▒▒▒▒▒█▒▒▒▒▒▒ ▒ ▒▒█▒█▒
       ▒▒██████▒▒▒▒█████▒▒▒▒▒ ▒▒█▒█▒
       ██ ██▒███████████▒▒▒▒▒▒▒███▒█▒
          ████████████████▒▒█▒██████▒
                           ███
"#,
    })
}

pub fn wraithplate() -> Item {
    Item::Armor(ArmorData {
        name: "Wraithplate".into(),
        description: "Armure forgée de métal fantôme, à mi-chemin entre le monde des vivants et celui des morts. La protection la plus élevée qui soit.",
        defense: 32,
        ascii: r#"
                 ░░░░
           ░░░░░░░░░░░░░░░█
         ░░██░░░░░████████░░
       ░░░░░███░░██░░░░░░░░██
      ░░░██░░░█░░░█░░███████░░
      ██████░█████████████░░░░░░░
      ███░░██░░░░██████░██████░░░░█
    █████░████░██░░░░████████████░░░
   ░█████░██░░░██░░░░░░██████████░░░░░
  ██████████░░░█░████░░█████████████░░░
░█████  ██░████░░█░░█░░░█████████████░░█
  ██   ████████████░░░░░███████████████
     ██████████░███████░░  ████████████
     ████████████░░░████░█ █  ██████████
    ████████████░████░░░░░█    ██████████
    ██████░██████░░█░██░░░█     █████████
   ███ ███████████░░█░░██░██     ██████ █
       █████░░█████░░░░░░░░█░     ██████
      ███████░░███████░░█░░░██        ██
     ░████████░████░███░░░░░░░        █
    ██░██████░░█ ██░██  █░░░░░░
   ░███████████░   █     █░░░░██
"#,
    })
}

// ─── SHIELDS ────────────────────────────────────────────────────────────────

pub fn wooden_buckler() -> Item {
    Item::Shield(ShieldData {
        name: "Wooden Buckler".into(),
        description: "Petit bouclier rond taillé dans du chêne. Léger et maniable, idéal pour débutants ou builds axés sur la vitesse.",
        defense: 4,
        ascii: "",
    })
}

pub fn thornwood_shield() -> Item {
    Item::Shield(ShieldData {
        name: "Thornwood Shield".into(),
        description: "Bouclier de bois épineux récupéré dans une forêt maudite. Les épines blessent les ennemis qui frappent trop fort.",
        defense: 9,
        ascii: "",
    })
}

pub fn iron_kite_shield() -> Item {
    Item::Shield(ShieldData {
        name: "Iron Kite Shield".into(),
        description: "Bouclier de cavalier en forme de cerf-volant. Large et robuste, couvre le corps entier contre les attaques directes.",
        defense: 14,
        ascii: "",
    })
}

pub fn boneguard() -> Item {
    Item::Shield(ShieldData {
        name: "Boneguard".into(),
        description: "Bouclier fait d'os fusionnés de créatures tombées. Étrangement résistant malgré son aspect fragile.",
        defense: 17,
        ascii: "",
    })
}

pub fn ashen_bulwark() -> Item {
    Item::Shield(ShieldData {
        name: "Ashen Bulwark".into(),
        description: "Bouclier de métal noirci par la cendre volcanique. Résiste particulièrement bien aux attaques de feu.",
        defense: 20,
        ascii: "",
    })
}

pub fn ghostveil() -> Item {
    Item::Shield(ShieldData {
        name: "Ghostveil".into(),
        description: "Bouclier semi-translucide fait d'énergie spectrales solidifiées. Léger comme une plume, absorbe les dégâts magiques avec efficacité.",
        defense: 12,
        ascii: "",
    })
}

pub fn runed_aegis() -> Item {
    Item::Shield(ShieldData {
        name: "Runed Aegis".into(),
        description: "Aegis gravée de runes de protection ancienne. Les runes brillent faiblement lorsque le bouclier absorbe un coup.",
        defense: 22,
        ascii: "",
    })
}

pub fn stormwall() -> Item {
    Item::Shield(ShieldData {
        name: "Stormwall".into(),
        description: "Grand bouclier chargé d'électricité statique. Les frapper déclenche parfois une décharge sur l'attaquant.",
        defense: 27,
        ascii: "",
    })
}

pub fn tower_shield() -> Item {
    Item::Shield(ShieldData {
        name: "Tower Shield".into(),
        description: "Bouclier tour massif couvrant presque tout le corps. Extrêmement lourd — prévu pour les guerriers à force maximale.",
        defense: 32,
        ascii: "",
    })
}

pub fn soulward() -> Item {
    Item::Shield(ShieldData {
        name: "Soulward".into(),
        description: "Bouclier forgé de l'essence d'âmes condensées. La protection ultime — chaque âme absorbée renforce sa résistance.",
        defense: 38,
        ascii: "",
    })
}

// ─── CONSUMABLES ────────────────────────────────────────────────────────────

pub fn antidote() -> Item {
    Item::Consumable(ConsumableData {
        name: "Antidote".into(),
        description: "Fiole de contrepoison distillée à partir d'herbes rares. Neutralise immédiatement le Poison actif et ses stacks.",
        effect: ConsumableEffect::CurePoison,
        ascii: "",
    })
}

pub fn frost_salts() -> Item {
    Item::Consumable(ConsumableData {
        name: "Frost Salts".into(),
        description: "Sels thermiques récoltés près de sources volcaniques. Dissout les cristaux de glace dans les veines et lève le gel.",
        effect: ConsumableEffect::CureFrost,
        ascii: "",
    })
}

pub fn throwing_knife() -> Item {
    Item::Consumable(ConsumableData {
        name: "Throwing Knife".into(),
        description: "Couteau de lancer affûté. Inflige 35 dégâts directs à un ennemi ciblé. Usage unique.",
        effect: ConsumableEffect::DealDamage(35),
        ascii: "",
    })
}

pub fn siege_ash() -> Item {
    Item::Consumable(ConsumableData {
        name: "Siege Ash".into(),
        description: "Poudre explosive incendiaire utilisée par les légions de siège. Inflige 50 dégâts de feu à un ennemi ciblé. Usage unique.",
        effect: ConsumableEffect::DealFireDamage(50),
        ascii: "",
    })
}

pub fn haste_draught() -> Item {
    Item::Consumable(ConsumableData {
        name: "Haste Draught".into(),
        description: "Élixir de célérité qui aiguise les réflexes et la puissance de frappe. +20 dégâts d'attaque pendant 3 tours.",
        effect: ConsumableEffect::BuffAttack {
            bonus: 20,
            turns: 3,
        },
        ascii: "",
    })
}

pub fn bandage() -> Item {
    Item::Consumable(ConsumableData {
        name: "Bandage".into(),
        description: "Bandage de compression traité avec de la cire hémostatique. Stoppe immédiatement les saignements actifs et leurs stacks.",
        effect: ConsumableEffect::CureBleed,
        ascii: "",
    })
}

pub fn cooling_salve() -> Item {
    Item::Consumable(ConsumableData {
        name: "Cooling Salve".into(),
        description: "Onguent à base d'argile réfrigérante. Éteint les brûlures actives et réinitialise les stacks de feu sur le corps.",
        effect: ConsumableEffect::CureFire,
        ascii: "",
    })
}

pub fn purifying_salt() -> Item {
    Item::Consumable(ConsumableData {
        name: "Purifying Salt".into(),
        description: "Sel blanc béni qui purifie la chair en décomposition. Élimine la Rot active et ses stacks accumulés.",
        effect: ConsumableEffect::CureRot,
        ascii: "",
    })
}

pub fn grounding_wrap() -> Item {
    Item::Consumable(ConsumableData {
        name: "Grounding Wrap".into(),
        description: "Bandage isolant en fibre de cuivre tressée. Dissipe l'électricité résiduelle dans les muscles et lève l'électrocution.",
        effect: ConsumableEffect::CureLightning,
        ascii: "",
    })
}

pub fn map_fragment() -> Item {
    Item::Consumable(ConsumableData {
        name: "Torn Map Fragment".into(),
        description: "Morceau de parchemin arraché d'une carte plus grande. Quelqu'un cherche peut-être ce qu'il révèle.",
        effect: ConsumableEffect::QuestItem,
        ascii: "",
    })
}
