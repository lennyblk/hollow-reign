mod catalog;
mod enemy_catalog;
mod class;
mod combat;
mod enemy;
mod equipment;
mod grace;
mod item;
mod map;
mod navigation;
mod phrases;
mod player;
mod stats;
mod status;
mod typing;
mod zone;

use enemy_catalog::open_chest;
use map::World;
use navigation::{NavigationEvent, NavigationState, run_navigation};

fn main() {
    let world = World::new();
    let mut state = NavigationState::new(&world);

    loop {
        match run_navigation(&world, &mut state) {
            NavigationEvent::RestAtGrace(id) => {
                println!("\r\n  [Grace] Repos a la grace #{}. PV restaures.\r\n", id);
                println!("  Appuie sur Entree...");
                let mut buf = String::new();
                std::io::stdin().read_line(&mut buf).ok();
            }
            NavigationEvent::TalkToNpc(npc) => {
                println!("\r\n  [NPC] {} : \"...\" (dialogue a implementer)\r\n", npc);
                println!("  Appuie sur Entree...");
                let mut buf = String::new();
                std::io::stdin().read_line(&mut buf).ok();
            }
            NavigationEvent::EnterCombat => {
                println!("\r\n  [Combat] (a implementer)\r\n");
                println!("  Appuie sur Entree...");
                let mut buf = String::new();
                std::io::stdin().read_line(&mut buf).ok();
            }
            NavigationEvent::OpenChest(id) => {
                let items = open_chest(id);
                println!("\r\n  [Coffre] Ouvert !");
                for item in &items {
                    println!("    + {}", item.name());
                }
                println!("\r\n  Appuie sur Entree...");
                let mut buf = String::new();
                std::io::stdin().read_line(&mut buf).ok();
            }
            NavigationEvent::Quit => {
                println!("\r\n  Au revoir.\r\n");
                break;
            }
        }
    }
}
