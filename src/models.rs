use std::{
    collections::HashMap,
    io::{self, Error},
};

use crossterm::{execute, style::Print};
use ndarray::{Array2, ShapeBuilder};

// pub struct Hp {
//     pub max: u32,
//     pub current: u32,
// }

// pub struct Exp {
//     pub next: u32,
//     pub current: u32,
//     pub carrying: u32,
// }

// pub struct Level {
//     pub max: u32,
//     pub current: u32,
// }

// pub struct Strength {
//     pub max: u32,
//     pub current: u32,
// }

// pub struct Defense {
//     pub max: u32,
//     pub current: u32,
// }

// pub struct Agility {
//     pub max: u32,
//     pub current: u32,
// }

// pub struct Status {
//     pub hp: Hp,
//     pub exp: Exp,
//     pub level: Level,
//     pub strength: Strength,
//     pub defense: Defense,
//     pub agility: Agility,
// }

// impl Status {
//     fn calc_actual_values(&self) -> Result<(), Box<dyn Error>> {
//         todo!("not implemented")
//     }
// }

// pub struct Mob {
//     pub coordinate: DungeonMapCoordinate,
//     pub status: Status,
//     pub appearance_char: char,
// }

// struct DungeonMap {
//     pub path_layer: Array2<u32>,
//     pub mapchip_layer: Array2<u32>,
//     pub mapchip_to_display_dict: HashMap<u32, char>,
//     pub mob_layer: Array2<u32>,
// }

// struct GameWorld {
//     pub maps: HashMap<String, DungeonMap>,
//     pub mobs: HashMap<u32, Mob>,
//     pub current_map: String,
// }

pub struct WorldMapCoordinate {
    pub x: u32,
    pub y: u32,
}

pub struct Mob {
    pub coordinate: WorldMapCoordinate,
    pub appearance: char,
}

pub struct Item {}

trait TopDown2DGridMapRenderer {
    fn render_lines(&self, out: &mut impl io::Write) -> Result<(), io::Error>;
}

pub struct WorldMap {
    pub path_layer: Array2<u32>,
    pub mapchip_layer: Array2<u32>,
    pub mapchip_to_display_dict: HashMap<u32, char>,
    pub mob_layer: Array2<u32>,
    pub item_layer: Array2<u32>,
    pub items: HashMap<u32, Item>,
    pub mobs: HashMap<u32, Mob>,
}

impl WorldMap {
    pub fn from_size(width: usize, height: usize) -> Self {
        Self {
            path_layer: Array2::zeros((width, height).f()),
            mapchip_layer: Array2::zeros((width, height).f()),
            mapchip_to_display_dict: HashMap::new(),
            mob_layer: Array2::zeros((width, height).f()),
            item_layer: Array2::zeros((width, height).f()),
            items: HashMap::new(),
            mobs: HashMap::new(),
        }
    }
}

impl TopDown2DGridMapRenderer for WorldMap {
    fn render_lines(&self, out: &mut impl io::Write) -> Result<(), io::Error> {
        for row in self.mapchip_layer.rows() {
            dbg!();
            execute!(
                out,
                Print(
                    row.iter()
                        .map(|mapchip_id| self.mapchip_to_display_dict[mapchip_id])
                        .collect::<String>()+"\n"
                )
            )?;
        }
        Ok(())
    }
}

pub struct GameWorld {
    pub maps: HashMap<String, WorldMap>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_worldmap() {
        let mut buf = Vec::new();
        let mut worldmap = WorldMap::from_size(3, 3);
        let mut mapchip_to_display_dict = HashMap::<u32, char>::new();
        mapchip_to_display_dict.insert(0, 'a');
        mapchip_to_display_dict.insert(1, 'b');
        worldmap.mapchip_to_display_dict = mapchip_to_display_dict;
        worldmap.render_lines(&mut buf).unwrap();
        // dbg!(String::frombuf);
        assert_eq!("aaa\naaa\naaa\n", String::from_utf8(buf).unwrap());
    }
}
