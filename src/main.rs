mod models;

use std::{array, collections::HashMap, default::Default};

use models::*;
use ndarray::{array, Array, ArrayView1};

fn main() {
    let player = Mob {
        world_coordinates: WorldCoordinates { x: 2, y: 2 },
        status: Status {
            hp: Hp {
                max: 100,
                current: 100,
            },
            exp: Exp {
                next: 100,
                current: 0,
                carrying: 0,
            },
            level: Level {
                max: 100,
                current: 1,
            },
            strength: Strength {
                max: 100,
                current: 3,
            },
            defense: Defense {
                max: 100,
                current: 0,
            },
            agility: Agility {
                max: 100,
                current: 3,
            },
            resilience: Resilience { factor: 0.1 },
        },
        relationship: RelationShip {
            hostile_to_self_as_enemies_of_self: true,
            friendly_to_self_as_allies_of_self: true,
            ..Default::default()
        },
    };
    let mut gameworld: GameWorld = GameWorld::default();
    let mut mapchip_to_display_dict = HashMap::new();
    mapchip_to_display_dict.insert(0, ' ');
    mapchip_to_display_dict.insert(1, '.');
    gameworld.maps.insert(
        "start_area".to_string(),
        WorldMap {
            path_layer: array![
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
            ],
            mapchip_layer: array![
                [1, 1, 1, 1, 1],
                [1, 1, 1, 1, 1],
                [1, 1, 1, 1, 1],
                [1, 1, 1, 1, 1],
                [1, 1, 1, 1, 1]
            ],
            mapchip_to_display_dict
        },
    );
    println!("{}", gameworld.maps["start_area"].render_to_string())
}
