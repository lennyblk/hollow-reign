mod class;
mod equipment;
mod item;
mod player;
mod stats;
mod status;

use class::Class;
use equipment::Equipment;
use player::Player;
use stats::Stats;
use status::Status;

fn main() {
    let player = Player {
        name: String::from("Bozo"),
        class: Class::Knight,
        hp: 100,
        souls: 0,
        level: 1,
        stats: Stats {
            vigor: 20,
            intelligence: 5,
            faith: 5,
            dexterity: 5,
            arcane: 5,
            mind: 5,
            strength: 14,
        },
        status: Status::Alive,
        inventory: Vec::new(),
        equipment: Vec::new(),
    };

    println!(
        "Welcome, {} the {:?} et ta {} hp et {}",
        player.name, player.class, player.hp, player.stats.vigor
    );
}
