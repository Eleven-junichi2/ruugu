use std::{collections::HashMap, error::Error};

use ndarray::Array3;

#[derive(Default)]
pub struct Hp {
    pub max: u32,
    pub current: u32,
}

#[derive(Default)]
pub struct Exp {
    pub next: u32,
    pub current: u32,
    pub carrying: u32,
}

#[derive(Default)]
pub struct Level {
    pub max: u32,
    pub current: u32,
}

#[derive(Default)]
pub struct Strength {
    pub max: u32,
    pub current: u32,
}

#[derive(Default)]
pub struct Defense {
    pub max: u32,
    pub current: u32,
}

#[derive(Default)]
pub struct Agility {
    pub max: u32,
    pub current: u32,
}

#[derive(Default)]
pub struct Resilience {
    pub factor: f32,
}

#[derive(Default)]
pub struct Status {
    pub hp: Hp,
    pub exp: Exp,
    pub level: Level,
    pub strength: Strength,
    pub defense: Defense,
    pub agility: Agility,
    pub resilience: Resilience,
}

impl Status {
    fn calc_actual_values(&self) -> Result<(), Box<dyn Error>> {
        todo!("not implemented")
    }
}

#[derive(Default)]
pub struct RelationShip {
    pub enemies: Vec<Mob>,
    pub allies: Vec<Mob>,
    pub hostile_to_self_as_enemies_of_self: bool,
    pub friendly_to_self_as_allies_of_self: bool,
}

#[derive(Default)]
pub struct Mob {
    pub status: Status,
    pub relationship: RelationShip,
}

pub struct World2DTopDown {
    maps: HashMap<String, Array3<char>>,
}

impl World2DTopDown {
    fn new(maps: HashMap<String, Array3<char>>) -> Self {
        Self { maps }
    }
}

impl Default for World2DTopDown {
    fn default() -> Self {
        Self {
            maps: HashMap::new(),
        }
    }
}
