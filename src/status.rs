pub enum Status {
    Alive,
    Dead,
    Poisoned,
    Bleeded,
    Frozen,
    Rotted,
    Sleeping,
}

impl Default for Status {
    fn default() -> Self {
        Status::Alive
    }
}

impl Status {
    // TODO: vérifier si le joueur peut agir
    // pub fn can_act(&self) -> bool { ... }

    // TODO: appliquer dégâts du status par tour
    // pub fn tick_damage(&self) -> u32 { ... }
}
