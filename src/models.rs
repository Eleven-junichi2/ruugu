use std::{collections::HashMap, error::Error};

use ndarray::Array2;

#[derive(Default)]
pub struct WorldCoordinates {
    pub x: u32,
    pub y: u32,
}

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
    pub world_coordinates: WorldCoordinates,
    pub status: Status,
    pub relationship: RelationShip,
}

pub struct WorldMap {
    pub path_layer: Array2<u32>,
    pub mapchip_layer: Array2<u32>,
    pub mapchip_to_display_dict: HashMap<u32, char>,
}

impl WorldMap {
    pub fn render_to_string(&self) -> String {
        let mut map_display = Vec::<String>::new();
        for row in self.mapchip_layer.rows() {
            let mut map_display_row = String::new();
            for mapchip in row.iter() {
                map_display_row.push(self.mapchip_to_display_dict[mapchip]);
            }
            map_display.push(map_display_row);
        }
        map_display.join("\n")
    }
}

pub struct GameWorld {
    pub maps: HashMap<String, WorldMap>,
    pub mobs: Vec<Mob>,
}

impl GameWorld {
    pub fn new(maps: HashMap<String, WorldMap>, mobs: Vec<Mob>) -> Self {
        Self { maps, mobs }
    }

    pub fn add_mob(&mut self, mob: Mob) {
        self.mobs.push(mob);
    }

    pub fn render_world_to_string(&self, map_key: &String) -> String {
        self.maps[map_key].render_to_string()
    }
}

impl Default for GameWorld {
    fn default() -> Self {
        Self {
            maps: HashMap::new(),
            mobs: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_to_string() {
        use ndarray::array;
        let mut mapchip_to_display_dict: HashMap<u32, char> = HashMap::new();
        mapchip_to_display_dict.insert(0, ' ');
        mapchip_to_display_dict.insert(1, '.');
        mapchip_to_display_dict.insert(2, '#');
        let example_worldmap = WorldMap {
            path_layer: array![
                [0, 0, 0, 0, 0],
                [0, 1, 1, 1, 2],
                [0, 1, 1, 1, 2],
                [0, 2, 2, 2, 2],
                [0, 0, 0, 0, 0],
            ],
            mapchip_layer: array![
                [0, 0, 0, 0, 0],
                [0, 1, 1, 1, 2],
                [0, 1, 1, 1, 2],
                [0, 2, 2, 2, 2],
                [0, 0, 0, 0, 0]
            ],
            mapchip_to_display_dict,
        };
        assert_eq!(
            "     \n ...#\n ...#\n ####\n     ",
            example_worldmap.render_to_string()
        )
    }
}
