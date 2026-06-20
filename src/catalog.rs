use crate::item::{
    ArmorData, ConsumableData, ConsumableEffect, Element, Item, ShieldData, StatScaling,
    WeaponAbility, WeaponData, WeaponEffect,
};

// ─── WEAPONS ────────────────────────────────────────────────────────────────

pub fn worn_shortsword() -> Item {
    Item::Weapon(WeaponData {
        name: "Worn Shortsword".into(),
        base_damage: 18,
        scaling: StatScaling::Dexterity,
        two_handed: false, element: Element::Bleed,
        ability: WeaponAbility { name: "Lacerate", effect: WeaponEffect::AoeElemental },
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
        base_damage: 26,
        scaling: StatScaling::Dexterity,
        two_handed: false, element: Element::Bleed,
        ability: WeaponAbility { name: "Blood Feast", effect: WeaponEffect::Lifesteal { percent: 25 } },
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
        base_damage: 32,
        scaling: StatScaling::Dexterity,
        two_handed: false, element: Element::Ice,
        ability: WeaponAbility { name: "Flash Freeze", effect: WeaponEffect::InstantStun },
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
        base_damage: 38,
        scaling: StatScaling::Dexterity,
        two_handed: true, element: Element::Fire,
        ability: WeaponAbility { name: "Ember Dance", effect: WeaponEffect::MultiHit { count: 2, damage_percent: 60 } },
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
        base_damage: 48,
        scaling: StatScaling::Dexterity,
        two_handed: false, element: Element::Lightning,
        ability: WeaponAbility { name: "Storm Call", effect: WeaponEffect::AoeElemental },
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
        base_damage: 22,
        scaling: StatScaling::Strength,
        two_handed: true, element: Element::Poison,
        ability: WeaponAbility { name: "Toxic Bludgeon", effect: WeaponEffect::ArmorBreak { reduction_percent: 50, turns: 3 } },
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
        base_damage: 42,
        scaling: StatScaling::Strength,
        two_handed: true, element: Element::Poison,
        ability: WeaponAbility { name: "Pestilence", effect: WeaponEffect::AoeElemental },
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
        base_damage: 62,
        scaling: StatScaling::Strength,
        two_handed: true, element: Element::Bleed,
        ability: WeaponAbility { name: "Obliterate", effect: WeaponEffect::Execute { hp_threshold_percent: 25, multiplier: 3 } },
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
        base_damage: 24,
        scaling: StatScaling::MindAndFaith,
        two_handed: false, element: Element::Rot,
        ability: WeaponAbility { name: "Entropy", effect: WeaponEffect::ExtendedRot { extra_turns: 2 } },
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
        base_damage: 46,
        scaling: StatScaling::Intelligence,
        two_handed: false, element: Element::Lightning,
        ability: WeaponAbility { name: "Arcane Surge", effect: WeaponEffect::EmpoweredStrike { bonus_percent: 100 } },
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
    Item::Armor(ArmorData { name: "Tattered Rags".into(), defense: 2, ascii: "" })
}

pub fn pilgrims_coat() -> Item {
    Item::Armor(ArmorData { name: "Pilgrim's Coat".into(), defense: 6, ascii: "" })
}

pub fn leather_vest() -> Item {
    Item::Armor(ArmorData { name: "Leather Vest".into(), defense: 10, ascii: "" })
}

pub fn shadowweave_mantle() -> Item {
    Item::Armor(ArmorData { name: "Shadowweave Mantle".into(), defense: 13, ascii: "" })
}

pub fn ashcaster_robes() -> Item {
    Item::Armor(ArmorData { name: "Ashcaster Robes".into(), defense: 8, ascii: "" })
}

pub fn chainmail() -> Item {
    Item::Armor(ArmorData { name: "Chainmail".into(), defense: 16, ascii: "" })
}

pub fn wardens_harness() -> Item {
    Item::Armor(ArmorData { name: "Warden's Harness".into(), defense: 20, ascii: "" })
}

pub fn knights_plate() -> Item {
    Item::Armor(ArmorData { name: "Knight's Plate".into(), defense: 26, ascii: "" })
}

pub fn hollowed_armor() -> Item {
    Item::Armor(ArmorData { name: "Hollowed Armor".into(), defense: 22, ascii: "" })
}

pub fn wraithplate() -> Item {
    Item::Armor(ArmorData { name: "Wraithplate".into(), defense: 32, ascii: "" })
}

// ─── SHIELDS ────────────────────────────────────────────────────────────────

pub fn wooden_buckler() -> Item {
    Item::Shield(ShieldData { name: "Wooden Buckler".into(), defense: 4, ascii: "" })
}

pub fn thornwood_shield() -> Item {
    Item::Shield(ShieldData { name: "Thornwood Shield".into(), defense: 9, ascii: "" })
}

pub fn iron_kite_shield() -> Item {
    Item::Shield(ShieldData { name: "Iron Kite Shield".into(), defense: 14, ascii: "" })
}

pub fn boneguard() -> Item {
    Item::Shield(ShieldData { name: "Boneguard".into(), defense: 17, ascii: "" })
}

pub fn ashen_bulwark() -> Item {
    Item::Shield(ShieldData { name: "Ashen Bulwark".into(), defense: 20, ascii: "" })
}

pub fn ghostveil() -> Item {
    Item::Shield(ShieldData { name: "Ghostveil".into(), defense: 12, ascii: "" })
}

pub fn runed_aegis() -> Item {
    Item::Shield(ShieldData { name: "Runed Aegis".into(), defense: 22, ascii: "" })
}

pub fn stormwall() -> Item {
    Item::Shield(ShieldData { name: "Stormwall".into(), defense: 27, ascii: "" })
}

pub fn tower_shield() -> Item {
    Item::Shield(ShieldData { name: "Tower Shield".into(), defense: 32, ascii: "" })
}

pub fn soulward() -> Item {
    Item::Shield(ShieldData { name: "Soulward".into(), defense: 38, ascii: "" })
}

// ─── CONSUMABLES ────────────────────────────────────────────────────────────

pub fn antidote() -> Item {
    Item::Consumable(ConsumableData { name: "Antidote".into(), effect: ConsumableEffect::CurePoison, ascii: "" })
}

pub fn frost_salts() -> Item {
    Item::Consumable(ConsumableData { name: "Frost Salts".into(), effect: ConsumableEffect::CureFrost, ascii: "" })
}

pub fn throwing_knife() -> Item {
    Item::Consumable(ConsumableData { name: "Throwing Knife".into(), effect: ConsumableEffect::DealDamage(35), ascii: "" })
}

pub fn siege_ash() -> Item {
    Item::Consumable(ConsumableData { name: "Siege Ash".into(), effect: ConsumableEffect::DealFireDamage(50), ascii: "" })
}

pub fn haste_draught() -> Item {
    Item::Consumable(ConsumableData { name: "Haste Draught".into(), effect: ConsumableEffect::BuffAttack { bonus: 20, turns: 3 }, ascii: "" })
}

pub fn map_fragment() -> Item {
    Item::Consumable(ConsumableData { name: "Torn Map Fragment".into(), effect: ConsumableEffect::QuestItem, ascii: "" })
}
