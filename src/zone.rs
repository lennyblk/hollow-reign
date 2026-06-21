use crate::enemy::EnemyType;
use crate::item::Element;

// ─── ZONE ID ─────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ZoneId {
    Ashfeld,
    Gravemoor,
    Rotwood,
    TheCinders,
    Frostveil,
    TheVoid,
}

impl ZoneId {
    pub fn name(self) -> &'static str {
        match self {
            ZoneId::Ashfeld => "Ashfeld",
            ZoneId::Gravemoor => "Gravemoor",
            ZoneId::Rotwood => "Rotwood",
            ZoneId::TheCinders => "The Cinders",
            ZoneId::Frostveil => "Frostveil",
            ZoneId::TheVoid => "The Void",
        }
    }
}

// ─── LOCATION CONNECTION ─────────────────────────────────────────────────────

pub enum LocationTarget {
    /// Autre location dans la même zone (id local)
    Here(u32),
    /// Transition vers une autre zone
    OtherZone(ZoneId),
}

pub struct LocationConnection {
    pub label: &'static str, // "→ vers le nord", "↓ descendre la falaise"
    pub target: LocationTarget,
    pub one_way: bool, // true = sens unique (pas de retour automatique)
}

// ─── LOCATION CONTENTS ───────────────────────────────────────────────────────

pub struct EnemySpawn {
    pub enemy_type: EnemyType,
    pub element: Element,
    pub count: u32,
    pub respawns: bool,
}

pub struct GracePoint {
    pub id: u32,
    pub name: &'static str,
}

/// Coffre avec un id numérique — le contenu est défini dans enemy_catalog::open_chest().
pub struct Chest {
    pub id: u32,
}

pub struct LocationContents {
    pub grace: Option<GracePoint>,
    pub npc: Option<&'static str>,
    pub merchant: bool,
    pub enemies: Vec<EnemySpawn>,
    pub chest: Option<Chest>,
}

impl LocationContents {
    fn empty() -> Self {
        Self {
            grace: None,
            npc: None,
            merchant: false,
            enemies: vec![],
            chest: None,
        }
    }
}

// ─── LOCATION ────────────────────────────────────────────────────────────────

pub struct Location {
    pub id: u32, // (commence à 1)
    pub name: &'static str,
    pub description: &'static str,
    pub ascii: &'static str,
    pub connections: Vec<LocationConnection>,
    pub contents: LocationContents,
}

// ─── ZONE ────────────────────────────────────────────────────────────────────

pub struct Zone {
    pub id: ZoneId,
    pub name: &'static str,
    pub description: &'static str,
    pub ascii: &'static str,
    pub entry_location: u32, // id de la location d'entrée
    pub locations: Vec<Location>,
}

impl Zone {
    pub fn get_location(&self, id: u32) -> Option<&Location> {
        self.locations.iter().find(|l| l.id == id)
    }
}

// ─── HELPERS INTERNES ────────────────────────────────────────────────────────

fn mob(element: Element, count: u32) -> EnemySpawn {
    EnemySpawn {
        enemy_type: EnemyType::Mob,
        element,
        count,
        respawns: true,
    }
}
fn leader(element: Element) -> EnemySpawn {
    EnemySpawn {
        enemy_type: EnemyType::MobLeader,
        element,
        count: 1,
        respawns: true,
    }
}
fn miniboss(element: Element) -> EnemySpawn {
    EnemySpawn {
        enemy_type: EnemyType::MiniBoss,
        element,
        count: 1,
        respawns: false,
    }
}
fn boss(element: Element) -> EnemySpawn {
    EnemySpawn {
        enemy_type: EnemyType::Boss,
        element,
        count: 1,
        respawns: false,
    }
}
fn grace(id: u32, name: &'static str) -> GracePoint {
    GracePoint { id, name }
}
fn go(label: &'static str, id: u32) -> LocationConnection {
    LocationConnection {
        label,
        target: LocationTarget::Here(id),
        one_way: false,
    }
}
fn go_one_way(label: &'static str, id: u32) -> LocationConnection {
    LocationConnection {
        label,
        target: LocationTarget::Here(id),
        one_way: true,
    }
}
fn chest_here(id: u32) -> Chest {
    Chest { id }
}
fn go_zone(label: &'static str, zone: ZoneId) -> LocationConnection {
    LocationConnection {
        label,
        target: LocationTarget::OtherZone(zone),
        one_way: false,
    }
}

// ─── ASHFELD ─────────────────────────────────────────────────────────────────

pub fn ashfeld() -> Zone {
    Zone {
        id: ZoneId::Ashfeld,
        name: "Ashfeld",
        description: "Un village en ruines perdu sous une pluie de cendres. Les premiers hollows errent dans les rues, attires par l'eclat de votre ame.",
        ascii: "",
        entry_location: 1,
        locations: vec![
            Location {
                id: 1,
                name: "Chemin des Cendres",
                description: "Un sentier de pierre fracassee longe les premiers batiments effondres. Des cendres tombent sans arret du ciel gris.",
                ascii: "",
                connections: vec![go("→ Marche en ruines", 2)],
                contents: LocationContents::empty(),
            },
            Location {
                id: 2,
                name: "Marche en ruines",
                description: "Les etals renverses et les corps calcines d'anciens marchands jonchent le sol. Des hollows errent entre les decombres.",
                ascii: "",
                connections: vec![
                    go("← Chemin des Cendres", 1),
                    go("→ Tour de guet", 3),
                    go("↓ Sanctuaire d'Ashfeld", 4),
                ],
                contents: LocationContents {
                    enemies: vec![mob(Element::Bleed, 3)],
                    ..LocationContents::empty()
                },
            },
            Location {
                id: 3,
                name: "Tour de guet",
                description: "Une tour de guet a moitie effondree. Du haut des ruines on voit toute la zone, mais le chef des hollows la surveille aussi.",
                ascii: "",
                connections: vec![
                    go("← Marche en ruines", 2),
                    go_one_way("↓ Sauter dans la cour (sens unique)", 4),
                ],
                contents: LocationContents {
                    enemies: vec![leader(Element::Bleed)],
                    chest: Some(chest_here(1)),
                    ..LocationContents::empty()
                },
            },
            Location {
                id: 4,
                name: "Sanctuaire d'Ashfeld",
                description: "Une vieille chapelle miraculeusement intacte. Une grace brule dans l'obscurite. Edric le Creux s'y est refugie.",
                ascii: "",
                connections: vec![go("↑ Marche en ruines", 2), go("→ Porte des ruines", 5)],
                contents: LocationContents {
                    grace: Some(grace(1, "Sanctuaire d'Ashfeld")),
                    npc: Some("Edric le Creux"),
                    merchant: true,
                    enemies: vec![],
                    chest: None,
                },
            },
            Location {
                id: 5,
                name: "Porte des ruines",
                description: "Une arche de pierre fendue marque la frontiere d'Ashfeld. Au-dela commencent les terres plus dangereuses. Une derniere grace brule ici.",
                ascii: "",
                connections: vec![
                    go("← Sanctuaire d'Ashfeld", 4),
                    go_zone("→ Rotwood", ZoneId::Rotwood),
                    go_zone("↓ Gravemoor", ZoneId::Gravemoor),
                ],
                contents: LocationContents {
                    grace: Some(grace(2, "Porte des Ruines")),
                    ..LocationContents::empty()
                },
            },
        ],
    }
}

// ─── GRAVEMOOR ───────────────────────────────────────────────────────────────

pub fn gravemoor() -> Zone {
    Zone {
        id: ZoneId::Gravemoor,
        name: "Gravemoor",
        description: "Un cimetiere sans fin ou les morts refusent de rester enterres. La terre noire suinte de poison et de sang fige.",
        ascii: "",
        entry_location: 1,
        locations: vec![
            Location {
                id: 1,
                name: "Lisiere du Mouroir",
                description: "La boue noire avale les pas. Des pierres tombales brisees emergent du sol comme des dents. Des morts-vivants rampent hors de leurs fosses.",
                ascii: "",
                connections: vec![
                    go_zone("↑ Ashfeld", ZoneId::Ashfeld),
                    go("→ Sentier des Tombes", 2),
                ],
                contents: LocationContents {
                    enemies: vec![mob(Element::Bleed, 2)],
                    ..LocationContents::empty()
                },
            },
            Location {
                id: 2,
                name: "Sentier des Tombes",
                description: "Un chemin etroit serpente entre des mausolees en ruine. L'air sent le poison. Les gardes de la crypte patrouillent sans relache.",
                ascii: "",
                connections: vec![
                    go("← Lisiere du Mouroir", 1),
                    go("→ Croix de Mossbound", 3),
                    go_one_way("↓ Descendre dans la fosse (sens unique)", 4),
                ],
                contents: LocationContents {
                    enemies: vec![mob(Element::Poison, 3)],
                    chest: Some(chest_here(2)),
                    ..LocationContents::empty()
                },
            },
            Location {
                id: 3,
                name: "Croix de Mossbound",
                description: "Une croix de mousse ancienne marque le centre du cimetiere. Soeur Vael prie ici, indifferente aux morts qui l'entourent.",
                ascii: "",
                connections: vec![
                    go("← Sentier des Tombes", 2),
                    go("→ Crypte des Sans-Nom", 4),
                ],
                contents: LocationContents {
                    grace: Some(grace(3, "Croix de Mossbound")),
                    npc: Some("Soeur Vael"),
                    ..LocationContents::empty()
                },
            },
            Location {
                id: 4,
                name: "Crypte des Sans-Nom",
                description: "Une crypte souterraine ou sont entasses ceux dont personne ne se souvient. Quelque chose de massif garde l'entree du tombeau du seigneur.",
                ascii: "",
                connections: vec![
                    go("← Croix de Mossbound", 3),
                    go("→ Tombeau du Seigneur", 5),
                ],
                contents: LocationContents {
                    enemies: vec![mob(Element::Poison, 2), miniboss(Element::Poison)],
                    ..LocationContents::empty()
                },
            },
            Location {
                id: 5,
                name: "Tombeau du Seigneur",
                description: "Une chambre funeraire immense, vide de tout sauf d'une grace solitaire. Une porte au fond mene vers des terres plus froides.",
                ascii: "",
                connections: vec![
                    go("← Crypte des Sans-Nom", 4),
                    go_zone("→ Frostveil", ZoneId::Frostveil),
                ],
                contents: LocationContents {
                    grace: Some(grace(4, "Tombeau du Seigneur")),
                    ..LocationContents::empty()
                },
            },
        ],
    }
}

// ─── ROTWOOD ─────────────────────────────────────────────────────────────────

pub fn rotwood() -> Zone {
    Zone {
        id: ZoneId::Rotwood,
        name: "Rotwood",
        description: "Une foret corrompue dont les arbres transpirent une seve noire. Chaque pas repand des spores qui rongent la chair.",
        ascii: "",
        entry_location: 1,
        locations: vec![
            Location {
                id: 1,
                name: "Bordure Corrompue",
                description: "Les premiers arbres de Rotwood sont deja pourris jusqu'a la racine. Une brume verdatre stagne au sol.",
                ascii: "",
                connections: vec![
                    go_zone("← Ashfeld", ZoneId::Ashfeld),
                    go("→ Sous-bois Maudit", 2),
                ],
                contents: LocationContents {
                    enemies: vec![mob(Element::Poison, 2)],
                    ..LocationContents::empty()
                },
            },
            Location {
                id: 2,
                name: "Sous-bois Maudit",
                description: "La canopee bloque toute lumiere. Des champignons luminescents eclairent faiblement un marchand itinerant qui a perdu la raison.",
                ascii: "",
                connections: vec![
                    go("← Bordure Corrompue", 1),
                    go("→ Sanctuaire du Baldaquin", 3),
                ],
                contents: LocationContents {
                    enemies: vec![mob(Element::Rot, 3)],
                    merchant: true,
                    chest: Some(chest_here(3)),
                    ..LocationContents::empty()
                },
            },
            Location {
                id: 3,
                name: "Sanctuaire du Baldaquin",
                description: "Un vieux sanctuaire de chasse envahi par les lianes. Malgre la corruption, une grace y brule encore.",
                ascii: "",
                connections: vec![go("← Sous-bois Maudit", 2), go("→ Clairiere des Spores", 4)],
                contents: LocationContents {
                    grace: Some(grace(5, "Sanctuaire du Baldaquin")),
                    ..LocationContents::empty()
                },
            },
            Location {
                id: 4,
                name: "Clairiere des Spores",
                description: "Une clairiere ou les spores sont si denses qu'on ne voit pas a deux metres. Un chef de meute et ses sbires y patrouillent.",
                ascii: "",
                connections: vec![
                    go("← Sanctuaire du Baldaquin", 3),
                    go("→ Coeur de Rotwood", 5),
                ],
                contents: LocationContents {
                    enemies: vec![mob(Element::Poison, 2), leader(Element::Rot)],
                    ..LocationContents::empty()
                },
            },
            Location {
                id: 5,
                name: "Coeur de Rotwood",
                description: "L'arbre-mere de la corruption. Ses racines couvrent tout. Le gardien de la foret, une chose informe, en emerge lentement.",
                ascii: "",
                connections: vec![
                    go("← Clairiere des Spores", 4),
                    go_zone("→ The Cinders", ZoneId::TheCinders),
                    go_zone("↑ Frostveil", ZoneId::Frostveil),
                ],
                contents: LocationContents {
                    grace: Some(grace(6, "Coeur de Rotwood")),
                    enemies: vec![miniboss(Element::Poison)],
                    ..LocationContents::empty()
                },
            },
        ],
    }
}

// ─── THE CINDERS ─────────────────────────────────────────────────────────────

pub fn the_cinders() -> Zone {
    Zone {
        id: ZoneId::TheCinders,
        name: "The Cinders",
        description: "Les ruines d'une forge titanesque engloutie par un volcan. La chaleur deforme l'air et fait fondre les faibles.",
        ascii: "",
        entry_location: 1,
        locations: vec![
            Location {
                id: 1,
                name: "Porte de Braise",
                description: "Une porte de metal fondu marque l'entree des Cinders. La chaleur est suffocante. Une grace brule ici, ecrasee par la touffeur.",
                ascii: "",
                connections: vec![
                    go_zone("← Rotwood", ZoneId::Rotwood),
                    go("→ Fonderie Effondree", 2),
                ],
                contents: LocationContents {
                    grace: Some(grace(7, "Porte de Braise")),
                    enemies: vec![mob(Element::Fire, 2)],
                    ..LocationContents::empty()
                },
            },
            Location {
                id: 2,
                name: "Fonderie Effondree",
                description: "Les murs de la forge ont cede sous la chaleur. Du metal en fusion coule encore dans les rainures du sol. Des gardes ignes patrouillent.",
                ascii: "",
                connections: vec![
                    go("← Porte de Braise", 1),
                    go("→ Atelier des Ames", 3),
                    go("↓ Salle de Fonte", 4),
                ],
                contents: LocationContents {
                    enemies: vec![mob(Element::Fire, 2), leader(Element::Fire)],
                    ..LocationContents::empty()
                },
            },
            Location {
                id: 3,
                name: "Atelier des Ames",
                description: "Une salle preservee de la chaleur par des runes anciennes. Osryn le Chevalier Braise y attend, entre la vie et la mort.",
                ascii: "",
                connections: vec![go("← Fonderie Effondree", 2), go("→ Salle de Fonte", 4)],
                contents: LocationContents {
                    grace: Some(grace(8, "Atelier des Ames")),
                    npc: Some("Osryn le Chevalier Braise"),
                    ..LocationContents::empty()
                },
            },
            Location {
                id: 4,
                name: "Salle de Fonte",
                description: "L'ancienne salle de coulage des armes. Des statues de metal fondu figent d'anciens guerriers dans des poses de combat.",
                ascii: "",
                connections: vec![
                    go("← Fonderie Effondree", 2),
                    go("← Atelier des Ames", 3),
                    go("→ Trone de Cendre", 5),
                ],
                contents: LocationContents {
                    enemies: vec![mob(Element::Fire, 3)],
                    chest: Some(chest_here(4)),
                    ..LocationContents::empty()
                },
            },
            Location {
                id: 5,
                name: "Trone de Cendre",
                description: "Au sommet de la forge, un trone de cendres solidifiees. Le Seigneur du Feu y siege, immobile depuis des siecles, attendant un challenger.",
                ascii: "",
                connections: vec![
                    go("← Salle de Fonte", 4),
                    go_zone("→ The Void", ZoneId::TheVoid),
                ],
                contents: LocationContents {
                    enemies: vec![boss(Element::Fire)],
                    ..LocationContents::empty()
                },
            },
        ],
    }
}

// ─── FROSTVEIL ───────────────────────────────────────────────────────────────

pub fn frostveil() -> Zone {
    Zone {
        id: ZoneId::Frostveil,
        name: "Frostveil",
        description: "Des pics eternellement geles ou la foudre frappe sans prevenir. Le froid paralyse avant meme que les ennemis n'attaquent.",
        ascii: "",
        entry_location: 1,
        locations: vec![
            Location {
                id: 1,
                name: "Seuil du Gel",
                description: "La temperature chute brutalement a l'entree de Frostveil. La neige crisse sous les pas. Une grace tremble dans le vent glace.",
                ascii: "",
                connections: vec![
                    go_zone("← Rotwood", ZoneId::Rotwood),
                    go_zone("← Gravemoor", ZoneId::Gravemoor),
                    go("→ Falaise Brisee", 2),
                    go("↑ Chemin du Temple (long)", 4),
                ],
                contents: LocationContents {
                    grace: Some(grace(9, "Seuil du Gel")),
                    enemies: vec![mob(Element::Ice, 2)],
                    ..LocationContents::empty()
                },
            },
            Location {
                id: 2,
                name: "Falaise Brisee",
                description: "Une corniche de glace surplombe un gouffre. Des sentinelles eclair patrouillent. On peut sauter en bas pour atteindre le Pic, mais pas remonter.",
                ascii: "",
                connections: vec![
                    go("← Seuil du Gel", 1),
                    go_one_way("↓ Sauter sur le Pic Fracasse (sens unique)", 3),
                ],
                contents: LocationContents {
                    enemies: vec![mob(Element::Lightning, 3)],
                    ..LocationContents::empty()
                },
            },
            Location {
                id: 3,
                name: "Pic Fracasse",
                description: "Un plateau de glace brisee au pied de la falaise. Des cristaux de glace geants emergent du sol. Un chef de meute surveille les alentours.",
                ascii: "",
                connections: vec![go("→ Temple de la Foudre", 4)],
                contents: LocationContents {
                    grace: Some(grace(10, "Pic Fracasse")),
                    enemies: vec![mob(Element::Ice, 2), leader(Element::Ice)],
                    ..LocationContents::empty()
                },
            },
            Location {
                id: 4,
                name: "Temple de la Foudre",
                description: "Un temple en ruine ou la foudre frappe les colonnes sans cesse. Le Pelerin Pale y medite, indifferent aux eclairs.",
                ascii: "",
                connections: vec![
                    go("← Seuil du Gel (long)", 1),
                    go("← Pic Fracasse", 3),
                    go("→ Sommet des Tempetes", 5),
                ],
                contents: LocationContents {
                    npc: Some("Le Pelerin Pale"),
                    merchant: true,
                    chest: Some(chest_here(5)),
                    ..LocationContents::empty()
                },
            },
            Location {
                id: 5,
                name: "Sommet des Tempetes",
                description: "Le pic le plus haut de Frostveil. La tempete de foudre est permanente. La Reine de Glace attend dans l'oeil du cyclone.",
                ascii: "",
                connections: vec![
                    go("← Temple de la Foudre", 4),
                    go_zone("→ The Void", ZoneId::TheVoid),
                ],
                contents: LocationContents {
                    enemies: vec![boss(Element::Lightning)],
                    ..LocationContents::empty()
                },
            },
        ],
    }
}

// ─── THE VOID ────────────────────────────────────────────────────────────────

pub fn the_void() -> Zone {
    Zone {
        id: ZoneId::TheVoid,
        name: "The Void",
        description: "Le coeur pourri du royaume. La ou le roi s'est creux lui-meme pour fuir la mort. Rien ne survit ici par accident.",
        ascii: "",
        entry_location: 1,
        locations: vec![
            Location {
                id: 1,
                name: "Porte de l'Abime",
                description: "Une porte de pierre noire marque l'entree du Vide. L'air y est immobile et froid. La derniere grace avant la fin brule ici.",
                ascii: "",
                connections: vec![
                    go_zone("← The Cinders", ZoneId::TheCinders),
                    go_zone("← Frostveil", ZoneId::Frostveil),
                    go("→ Couloir du Neant", 2),
                ],
                contents: LocationContents {
                    grace: Some(grace(11, "Porte de l'Abime")),
                    enemies: vec![mob(Element::Rot, 2)],
                    ..LocationContents::empty()
                },
            },
            Location {
                id: 2,
                name: "Couloir du Neant",
                description: "Un long couloir ou le sol et le plafond se confondent dans l'obscurite. Des sentinelles de foudre en gardent le passage.",
                ascii: "",
                connections: vec![go("← Porte de l'Abime", 1), go("→ Sanctuaire Maudit", 3)],
                contents: LocationContents {
                    enemies: vec![mob(Element::Lightning, 2), mob(Element::Rot, 1)],
                    ..LocationContents::empty()
                },
            },
            Location {
                id: 3,
                name: "Sanctuaire Maudit",
                description: "Une salle circulaire ou une grace brule d'une flamme noire. Le silence y est total. C'est le dernier repos avant les chambres du roi.",
                ascii: "",
                connections: vec![go("← Couloir du Neant", 2), go("→ Chambre du Roi Creux", 4)],
                contents: LocationContents {
                    grace: Some(grace(12, "Sanctuaire Maudit")),
                    chest: Some(chest_here(6)),
                    ..LocationContents::empty()
                },
            },
            Location {
                id: 4,
                name: "Chambre du Roi Creux",
                description: "Une antichambre immense gardee par le lieutenant du roi. Des trones vides bordent les murs — les fantomes de la cour.",
                ascii: "",
                connections: vec![go("← Sanctuaire Maudit", 3), go("→ Coeur du Vide", 5)],
                contents: LocationContents {
                    enemies: vec![mob(Element::Rot, 2), miniboss(Element::Rot)],
                    ..LocationContents::empty()
                },
            },
            Location {
                id: 5,
                name: "Coeur du Vide",
                description: "Le trone du Roi Creux. Il n'est plus homme ni monstre — juste une volonte de survivre qui a devore tout le reste. La fin du chemin.",
                ascii: "",
                connections: vec![go("← Chambre du Roi Creux", 4)],
                contents: LocationContents {
                    enemies: vec![boss(Element::Rot)],
                    ..LocationContents::empty()
                },
            },
        ],
    }
}
