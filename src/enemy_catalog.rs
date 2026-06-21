use crate::catalog::{
    antidote, ashfall_katana, ashcaster_robes, ashen_bulwark, bandage, boneguard, chainmail,
    cooling_salve, dawnbreaker, frost_salts, gravecrusher, grounding_wrap, haste_draught,
    hollowed_armor, hunters_blade, iron_kite_shield, ironwood_club, knights_plate, leather_vest,
    pilgrims_coat, purifying_salt, runebreaker, shadowweave_mantle, siege_ash, silverwind_rapier,
    sundermaul, tattered_rags, thornwood_shield, throwing_knife, voidstaff, wardens_harness,
    wooden_buckler, worn_shortsword, wraithplate,
};
use crate::enemy::{Enemy, EnemyType};
use crate::item::{Element, Item};
use crate::zone::ZoneId;

// ─── ASHFELD ─────────────────────────────────────────────────────────────────
// Ennemis de base, element Bleed. Zone tutoriel.

fn creux_errant() -> Enemy {
    Enemy::new(
        "Creux Errant".to_string(),
        EnemyType::Mob,
        80, 10, 2,
        vec![Element::Bleed],
        15,
        "",
    )
}

fn chef_des_creux() -> Enemy {
    Enemy::new(
        "Chef des Creux".to_string(),
        EnemyType::MobLeader,
        150, 16, 4,
        vec![Element::Bleed],
        40,
        "",
    )
}

// ─── GRAVEMOOR ───────────────────────────────────────────────────────────────
// Morts-vivants, Bleed et Poison.

fn mort_rampant() -> Enemy {
    Enemy::new(
        "Mort-Rampant".to_string(),
        EnemyType::Mob,
        100, 12, 3,
        vec![Element::Bleed],
        20,
        "",
    )
}

fn garde_de_crypte() -> Enemy {
    Enemy::new(
        "Garde de Crypte".to_string(),
        EnemyType::Mob,
        110, 13, 3,
        vec![Element::Poison],
        22,
        "",
    )
}

fn gardien_du_tombeau() -> Enemy {
    Enemy::new(
        "Gardien du Tombeau".to_string(),
        EnemyType::MiniBoss,
        240, 20, 8,
        vec![Element::Poison, Element::Bleed],
        85,
        "",
    )
}

// ─── ROTWOOD ─────────────────────────────────────────────────────────────────
// Creatures fongiques, Poison et Rot.

fn spore_marcheur() -> Enemy {
    Enemy::new(
        "Spore-Marcheur".to_string(),
        EnemyType::Mob,
        120, 14, 4,
        vec![Element::Poison],
        25,
        "",
    )
}

fn champignon_ambulant() -> Enemy {
    Enemy::new(
        "Champignon Ambulant".to_string(),
        EnemyType::Mob,
        130, 13, 5,
        vec![Element::Rot],
        25,
        "",
    )
}

fn seigneur_des_spores() -> Enemy {
    Enemy::new(
        "Seigneur des Spores".to_string(),
        EnemyType::MobLeader,
        190, 20, 6,
        vec![Element::Rot],
        65,
        "",
    )
}

fn l_excroissance() -> Enemy {
    Enemy::new(
        "L'Excroissance".to_string(),
        EnemyType::MiniBoss,
        300, 24, 9,
        vec![Element::Poison, Element::Rot],
        110,
        "",
    )
}

// ─── THE CINDERS ─────────────────────────────────────────────────────────────
// Gardes de forge, element Fire.

fn garde_igne() -> Enemy {
    Enemy::new(
        "Garde Igne".to_string(),
        EnemyType::Mob,
        150, 18, 6,
        vec![Element::Fire],
        30,
        "",
    )
}

fn capitaine_des_braises() -> Enemy {
    Enemy::new(
        "Capitaine des Braises".to_string(),
        EnemyType::MobLeader,
        230, 26, 9,
        vec![Element::Fire],
        75,
        "",
    )
}

fn seigneur_du_feu() -> Enemy {
    Enemy::new(
        "Seigneur du Feu".to_string(),
        EnemyType::Boss,
        520, 36, 16,
        vec![Element::Fire],
        300,
        "",
    )
}

// ─── FROSTVEIL ───────────────────────────────────────────────────────────────
// Sentinelles, Ice et Lightning.

fn sentinelle_gelee() -> Enemy {
    Enemy::new(
        "Sentinelle Gelee".to_string(),
        EnemyType::Mob,
        170, 20, 8,
        vec![Element::Ice],
        35,
        "",
    )
}

fn sentinelle_eclair() -> Enemy {
    Enemy::new(
        "Sentinelle Eclair".to_string(),
        EnemyType::Mob,
        165, 22, 7,
        vec![Element::Lightning],
        35,
        "",
    )
}

fn chasseur_des_glaces() -> Enemy {
    Enemy::new(
        "Chasseur des Glaces".to_string(),
        EnemyType::MobLeader,
        270, 29, 11,
        vec![Element::Ice],
        85,
        "",
    )
}

fn la_reine_tonnerre() -> Enemy {
    Enemy::new(
        "La Reine Tonnerre".to_string(),
        EnemyType::Boss,
        620, 42, 19,
        vec![Element::Lightning, Element::Ice],
        350,
        "",
    )
}

// ─── THE VOID ────────────────────────────────────────────────────────────────
// Vestiges corrompus, Rot et Lightning.

fn vestige_corrompu() -> Enemy {
    Enemy::new(
        "Vestige Corrompu".to_string(),
        EnemyType::Mob,
        200, 25, 10,
        vec![Element::Rot],
        45,
        "",
    )
}

fn ombre_electrique() -> Enemy {
    Enemy::new(
        "Ombre Electrique".to_string(),
        EnemyType::Mob,
        195, 27, 9,
        vec![Element::Lightning],
        45,
        "",
    )
}

fn lieutenant_du_roi() -> Enemy {
    Enemy::new(
        "Lieutenant du Roi".to_string(),
        EnemyType::MiniBoss,
        390, 33, 14,
        vec![Element::Rot, Element::Lightning],
        160,
        "",
    )
}

fn le_roi_creux() -> Enemy {
    Enemy::new(
        "Le Roi Creux".to_string(),
        EnemyType::Boss,
        820, 52, 22,
        vec![Element::Rot],
        500,
        "",
    )
}

// ─── DISPATCH ────────────────────────────────────────────────────────────────

/// Instancie un ennemi concret depuis les données de spawn d'une zone.
/// Panic si la combinaison (zone, type, element) n'est pas cataloguee.
pub fn spawn(zone: ZoneId, enemy_type: EnemyType, element: Element) -> Enemy {
    match (zone, enemy_type, element) {
        // Ashfeld
        (ZoneId::Ashfeld, EnemyType::Mob,       Element::Bleed) => creux_errant(),
        (ZoneId::Ashfeld, EnemyType::MobLeader, Element::Bleed) => chef_des_creux(),

        // Gravemoor
        (ZoneId::Gravemoor, EnemyType::Mob,       Element::Bleed)  => mort_rampant(),
        (ZoneId::Gravemoor, EnemyType::Mob,       Element::Poison) => garde_de_crypte(),
        (ZoneId::Gravemoor, EnemyType::MiniBoss,  Element::Poison) => gardien_du_tombeau(),

        // Rotwood
        (ZoneId::Rotwood, EnemyType::Mob,       Element::Poison) => spore_marcheur(),
        (ZoneId::Rotwood, EnemyType::Mob,       Element::Rot)    => champignon_ambulant(),
        (ZoneId::Rotwood, EnemyType::MobLeader, Element::Rot)    => seigneur_des_spores(),
        (ZoneId::Rotwood, EnemyType::MiniBoss,  Element::Poison) => l_excroissance(),

        // The Cinders
        (ZoneId::TheCinders, EnemyType::Mob,       Element::Fire) => garde_igne(),
        (ZoneId::TheCinders, EnemyType::MobLeader, Element::Fire) => capitaine_des_braises(),
        (ZoneId::TheCinders, EnemyType::Boss,      Element::Fire) => seigneur_du_feu(),

        // Frostveil
        (ZoneId::Frostveil, EnemyType::Mob,       Element::Ice)       => sentinelle_gelee(),
        (ZoneId::Frostveil, EnemyType::Mob,       Element::Lightning) => sentinelle_eclair(),
        (ZoneId::Frostveil, EnemyType::MobLeader, Element::Ice)       => chasseur_des_glaces(),
        (ZoneId::Frostveil, EnemyType::Boss,      Element::Lightning) => la_reine_tonnerre(),

        // The Void
        (ZoneId::TheVoid, EnemyType::Mob,      Element::Rot)       => vestige_corrompu(),
        (ZoneId::TheVoid, EnemyType::Mob,      Element::Lightning) => ombre_electrique(),
        (ZoneId::TheVoid, EnemyType::MiniBoss, Element::Rot)       => lieutenant_du_roi(),
        (ZoneId::TheVoid, EnemyType::Boss,     Element::Rot)       => le_roi_creux(),

        _ => panic!(
            "spawn non cataloguee: {:?} / {:?} / {:?}",
            zone, enemy_type, element
        ),
    }
}

// ─── DROPS ───────────────────────────────────────────────────────────────────

/// Items droppés à la mort d'un ennemi.
/// Mobs → rien. Leaders → 1 consommable. Minibosses → 1 arme/armure. Bosses → set complet.
pub fn drops(zone: ZoneId, enemy_type: EnemyType, element: Element) -> Vec<Item> {
    match (zone, enemy_type, element) {
        // ── Leaders ──────────────────────────────────────────────────────────
        // Arme légère ou bouclier basique selon la zone
        (ZoneId::Ashfeld,    EnemyType::MobLeader, Element::Bleed) => vec![wooden_buckler()],
        (ZoneId::Rotwood,    EnemyType::MobLeader, Element::Rot)   => vec![thornwood_shield()],
        (ZoneId::TheCinders, EnemyType::MobLeader, Element::Fire)  => vec![throwing_knife()],
        (ZoneId::Frostveil,  EnemyType::MobLeader, Element::Ice)   => vec![chainmail()],

        // ── Minibosses ───────────────────────────────────────────────────────
        (ZoneId::Gravemoor, EnemyType::MiniBoss, Element::Poison) => vec![hunters_blade(),       pilgrims_coat()],
        (ZoneId::Rotwood,   EnemyType::MiniBoss, Element::Poison) => vec![gravecrusher(),        shadowweave_mantle()],
        (ZoneId::TheVoid,   EnemyType::MiniBoss, Element::Rot)    => vec![runebreaker(),         haste_draught()],

        // ── Bosses : weapon + armor + shield + consumable ─────────────────────
        (ZoneId::TheCinders, EnemyType::Boss, Element::Fire) => vec![
            ashfall_katana(),
            ashcaster_robes(),
            ashen_bulwark(),
            cooling_salve(),
        ],
        (ZoneId::Frostveil, EnemyType::Boss, Element::Lightning) => vec![
            dawnbreaker(),
            knights_plate(),
            iron_kite_shield(),
            grounding_wrap(),
        ],
        (ZoneId::TheVoid, EnemyType::Boss, Element::Rot) => vec![
            voidstaff(),
            wraithplate(),
            boneguard(),
            purifying_salt(),
        ],

        // Mobs et le reste → rien
        _ => vec![],
    }
}

// ─── COFFRES ─────────────────────────────────────────────────────────────────

/// Contenu d'un coffre par son id (défini dans zone.rs).
pub fn open_chest(id: u32) -> Vec<Item> {
    match id {
        1 => vec![worn_shortsword(), bandage()],            // Ashfeld    - Tour de guet       (arme de base + soin)
        2 => vec![antidote(), tattered_rags()],           // Gravemoor  - Sentier des Tombes  (conso + armure basique)
        3 => vec![ironwood_club(), leather_vest()],       // Rotwood    - Sous-bois Maudit    (arme + armure mid)
        4 => vec![siege_ash(), wardens_harness()],        // Cinders    - Salle de Fonte      (buff + armure mid-late)
        5 => vec![frost_salts(), silverwind_rapier()],    // Frostveil  - Temple de la Foudre (conso + arme Ice)
        6 => vec![sundermaul(), hollowed_armor()],        // The Void   - Sanctuaire Maudit   (arme + armure late)
        _ => vec![],
    }
}
