use std::default::Default;

#[derive(Default)]
struct Hp {
    max: u32,
    current: u32,
}

#[derive(Default)]
struct Exp {
    next: u32,
    current: u32,
    carrying: u32,
}

#[derive(Default)]
struct Level {
    max: u32,
    current: u32,
}

#[derive(Default)]
struct Strength {
    max: u32,
    current: u32,
}

#[derive(Default)]
struct Defense {
    max: u32,
    current: u32,
}

#[derive(Default)]
struct Agility {
    max: u32,
    current: u32,
}

#[derive(Default)]
struct Resilience {
    factor: f32,
}

#[derive(Default)]
struct Status {
    hp: Hp,
    exp: Exp,
    level: Level,
    strength: Strength,
    defense: Defense,
    agility: Agility,
    resilience: Resilience,
}

#[derive(Default)]
struct RelationShip {
    enemies: Vec<Mob>,
    allies: Vec<Mob>,
}

#[derive(Default)]
struct Mob {
    status: Status,
    relationship: RelationShip,
}

fn main() {
    let player = Mob {
        ..Default::default()
    };
    println!("Hello, world!");
}
