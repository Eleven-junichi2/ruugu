mod models;

use std::{default::Default};

use models::*;

fn main() {
    let player = Mob {
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
    let gameworld: World2DTopDown = World2DTopDown::default();
    loop {
    }
}
