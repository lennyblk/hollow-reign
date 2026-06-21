mod catalog;
mod class;
mod combat;
mod combat_ui;
mod enemy;
mod enemy_catalog;
mod equipment;
mod grace;
mod inventory_ui;
mod item;
mod map;
mod navigation;
mod phrases;
mod player;
mod stats;
mod status;
mod typing;
mod zone;

use class::Class;
use combat_ui::{CombatResult, run_combat};
use enemy_catalog::open_chest;
use equipment::Equipment;
use inventory_ui::{open_inventory, InventoryResult};
use map::World;
use navigation::{NavigationEvent, NavigationState, run_navigation};
use player::Player;

fn main() {
    let world = World::new();
    let mut state = NavigationState::new(&world);
    let mut player = Player::new("Héros".to_string(), Class::Knight);
    player.hp = player.stats.max_hp();
    player.equipment.push(Equipment::empty());

    loop {
        match run_navigation(&world, &mut state) {
            NavigationEvent::RestAtGrace(id) => {
                player.rest_at_grace(id);
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
                let zone = state.zone;
                let zone_data = world.get(zone);
                let location = zone_data.get_location(state.location_id).unwrap();
                let spawns = &location.contents.enemies;
                match run_combat(&mut player, zone, spawns) {
                    CombatResult::Victory { items, souls } => {
                        player.souls += souls;
                        let item_count = items.len();
                        for item in items {
                            player.pick_up(item);
                        }
                        println!("\r\n  Victoire ! +{} ames, {} objet(s) recupere(s).\r\n", souls, item_count);
                        println!("  Appuie sur Entree...");
                        let mut buf = String::new();
                        std::io::stdin().read_line(&mut buf).ok();
                    }
                    CombatResult::Defeat => {
                        println!("\r\n  Tu es mort. Retour a la derniere grace.\r\n");
                        player.respawn_at_grace();
                        println!("  Appuie sur Entree...");
                        let mut buf = String::new();
                        std::io::stdin().read_line(&mut buf).ok();
                    }
                    CombatResult::Fled => {
                        println!("\r\n  Tu fuis le combat.\r\n");
                        println!("  Appuie sur Entree...");
                        let mut buf = String::new();
                        std::io::stdin().read_line(&mut buf).ok();
                    }
                }
            }
            NavigationEvent::OpenChest(id) => {
                let items = open_chest(id);
                println!("\r\n  [Coffre] Ouvert !");
                for item in items {
                    let name = item.name();
                    println!("    + {}", name);
                    player.pick_up(item);
                }
                println!("\r\n  Appuie sur Entree...");
                let mut buf = String::new();
                std::io::stdin().read_line(&mut buf).ok();
            }
            NavigationEvent::OpenInventory => {
                let mut out = std::io::stdout();
                open_inventory(&mut out, &mut player, false);
            }
            NavigationEvent::Quit => {
                println!("\r\n  Au revoir.\r\n");
                break;
            }
        }
    }
}
