mod catalog;
mod enemy_catalog;
mod class;
mod combat;
mod enemy;
mod equipment;
mod grace;
mod item;
mod map;
mod phrases;
mod player;
mod stats;
mod status;
mod typing;
mod zone;

use phrases::{Difficulty, PhrasePool};
use typing::{perfect_threshold, time_limit_ms, typing_challenge};

fn main() {
    let mut pool = PhrasePool::new();

    for (label, diff) in [
        ("MOB        (Short  / 5s) ", Difficulty::Short),
        ("MOB LEADER (Medium / 10s)", Difficulty::Medium),
        ("BOSS       (Long   / 18s)", Difficulty::Long),
    ] {
        println!("\n--- {} ---", label);
        println!("Appuie sur Entree pour lancer...");
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).ok();

        let limit = time_limit_ms(&diff);
        let pct = perfect_threshold(&diff);
        let phrase = pool.next(diff);
        let result = typing_challenge(phrase, limit, pct);

        println!("Resultat : {:?}\n", result);
    }
}
