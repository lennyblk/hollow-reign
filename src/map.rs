use std::collections::HashMap;

use crate::zone::{Zone, ZoneId, LocationTarget, ashfeld, gravemoor, rotwood, the_cinders, frostveil, the_void};

// ─── WORLD ───────────────────────────────────────────────────────────────────

pub struct World {
    zones: HashMap<ZoneId, Zone>,
}

impl World {
    pub fn new() -> Self {
        let mut zones = HashMap::new();
        zones.insert(ZoneId::Ashfeld,    ashfeld());
        zones.insert(ZoneId::Gravemoor,  gravemoor());
        zones.insert(ZoneId::Rotwood,    rotwood());
        zones.insert(ZoneId::TheCinders, the_cinders());
        zones.insert(ZoneId::Frostveil,  frostveil());
        zones.insert(ZoneId::TheVoid,    the_void());
        Self { zones }
    }

    pub fn get(&self, id: ZoneId) -> &Zone {
        self.zones.get(&id).expect("zone inconnue")
    }

    pub fn starting_zone(&self) -> ZoneId {
        ZoneId::Ashfeld
    }

    /// Zones accessibles depuis la zone actuelle (via transitions inter-zones).
    pub fn connections(&self, id: ZoneId) -> Vec<ZoneId> {
        let zone = self.get(id);
        let mut result = Vec::new();
        for loc in &zone.locations {
            for conn in &loc.connections {
                if let LocationTarget::OtherZone(zid) = conn.target {
                    if !result.contains(&zid) {
                        result.push(zid);
                    }
                }
            }
        }
        result
    }

    /// Retourne toutes les grâces du monde, triées par id (pour les menus).
    pub fn all_graces(&self) -> Vec<(u32, &str, ZoneId)> {
        let mut graces: Vec<(u32, &str, ZoneId)> = self
            .zones
            .values()
            .flat_map(|z| {
                z.locations.iter().filter_map(|l| {
                    l.contents.grace.as_ref().map(|g| (g.id, g.name, z.id))
                })
            })
            .collect();
        graces.sort_by_key(|(id, _, _)| *id);
        graces
    }

    /// Trouve la zone qui contient une grâce donnée.
    pub fn zone_of_grace(&self, grace_id: u32) -> Option<ZoneId> {
        self.zones
            .values()
            .find(|z| {
                z.locations.iter().any(|l| {
                    l.contents.grace.as_ref().map_or(false, |g| g.id == grace_id)
                })
            })
            .map(|z| z.id)
    }
}
