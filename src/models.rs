use std::{
    collections::HashMap,
    io::{self, Error},
};

use crossterm::{cursor, execute, style::Print};
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
    pub x: u16,
    pub y: u16,
}

pub struct Mob {
    pub coordinate: WorldMapCoordinate,
    pub appearance: char,
}

pub struct Item {}

trait TopDown2DGridMapRenderer {
    fn render(&self, buffer: &mut impl io::Write) -> Result<(), io::Error>;
}

pub struct WorldMap {
    /// key of `mobs` must be greater than or equal to 1 because 0 means empty
    /// key of `items` must be greater than or equal to 1 because 0 means empty
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
    fn render(&self, buffer: &mut impl io::Write) -> Result<(), io::Error> {
        for row in self.mapchip_layer.rows() {
            execute!(
                buffer,
                Print(
                    row.iter()
                        .map(|mapchip_id| self.mapchip_to_display_dict[mapchip_id])
                        .collect::<String>()
                        + "\n"
                )
            )?;
        }
        for row in self.mob_layer.rows() {
            for mob_id in row.iter() {
                if *mob_id == 0 { continue; }
                dbg!(mob_id);
                execute!(
                    buffer,
                    cursor::MoveTo(
                        self.mobs[mob_id].coordinate.x,
                        self.mobs[mob_id].coordinate.y
                    ),
                    Print(self.mobs[mob_id].appearance)
                )?;
            }
        }
        Ok(())
    }
}

pub struct GameWorld {
    pub maps: HashMap<String, WorldMap>,
}

#[cfg(test)]
mod tests {
    use ndarray::array;

    use super::*;

    #[test]
    fn test_worldmap_from_size() {
        let worldmap = WorldMap::from_size(4, 4);
        assert_eq!(
            array![[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
            worldmap.path_layer
        );
        assert_eq!(
            array![[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
            worldmap.mapchip_layer
        );
        assert_eq!(
            array![[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
            worldmap.mob_layer
        );
        assert_eq!(
            array![[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
            worldmap.item_layer
        )
    }

    #[test]
    fn test_render_worldmap() {
        let mut buf = Vec::new();
        let mut worldmap = WorldMap::from_size(3, 3);
        let mut mapchip_to_display_dict = HashMap::<u32, char>::new();
        mapchip_to_display_dict.insert(0, 'a');
        mapchip_to_display_dict.insert(1, 'b');
        worldmap.mapchip_to_display_dict = mapchip_to_display_dict;
        worldmap.render(&mut buf).unwrap();
        assert_eq!("aaa\naaa\naaa\n", String::from_utf8(buf).unwrap());
    }
}
