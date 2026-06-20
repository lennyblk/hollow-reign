mod catalog;
mod class;
mod combat;
mod enemy;
mod equipment;
mod grace;
mod item;
mod player;
mod stats;
mod status;

use class::Class;
use player::Player;

fn main() {
    let player = Player::new(String::from("Bozo"), Class::Knight);

    println!(
        "Welcome, {} the {:?} — {} hp — vigor {}",
        player.name, player.class, player.hp, player.stats.vigor
    );
}
