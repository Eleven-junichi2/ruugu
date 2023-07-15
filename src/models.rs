use std::collections::HashMap;
use std::error::Error;
use std::fmt;

use crossterm::{cursor, execute, style::Print};
use ndarray::{Array2, ShapeBuilder};

#[derive(Debug)]
pub struct IdOfNotInsertedEntityError;

impl Error for IdOfNotInsertedEntityError {}

impl fmt::Display for IdOfNotInsertedEntityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "entity of given id is not inserted")
    }
}

pub struct WorldMapCoordinate {
    pub x: u16,
    pub y: u16,
}

pub struct Mob {
    pub appearance: char,
}

pub struct Item {}

trait TopDown2DGridMapRenderer {
    // fn render(&self, buffer: &mut impl io::Write) -> Result<(), io::Error>;
    // fn render_to_lines(&self, string: &mut String);
    fn render_to_vec(&self) -> Vec<Vec<char>>;
    fn render_to_lines(&self) -> String;
}

pub struct WorldMap {
    /// key of `mobs` must be greater than or equal to 1 because 0 means empty
    /// key of `items` must be greater than or equal to 1 because 0 means empty
    pub path_layer: Array2<u32>,
    pub mapchip_layer: Array2<u32>,
    pub mapchip_to_display_dict: HashMap<u32, char>,
    pub placed_mobs: HashMap<u32, WorldMapCoordinate>,
    pub placed_items: HashMap<u32, WorldMapCoordinate>,
    pub items: HashMap<u32, Item>,
    pub mobs: HashMap<u32, Mob>,
}

impl WorldMap {
    pub fn from_size(width: usize, height: usize) -> Self {
        Self {
            path_layer: Array2::zeros((width, height).f()),
            mapchip_layer: Array2::zeros((width, height).f()),
            mapchip_to_display_dict: HashMap::new(),
            placed_mobs: HashMap::new(),
            placed_items: HashMap::new(),
            items: HashMap::new(),
            mobs: HashMap::new(),
        }
    }

    pub fn insert_mob(&mut self, mob: Mob, id: u32) -> u32 {
        self.mobs.insert(id, mob);
        id
    }

    pub fn insert_item(&mut self, item: Item, id: u32) -> u32 {
        self.items.insert(id, item);
        id
    }

    pub fn place_mob_in_world(
        &mut self,
        mob_id: u32,
        coordinate: WorldMapCoordinate,
    ) -> Result<(), IdOfNotInsertedEntityError> {
        if self.mobs.contains_key(&mob_id) {
            self.placed_mobs.insert(mob_id, coordinate);
            Ok(())
        } else {
            Err(IdOfNotInsertedEntityError)
        }
    }

    pub fn place_item_in_world(
        &mut self,
        item_id: u32,
        coordinate: WorldMapCoordinate,
    ) -> Result<(), IdOfNotInsertedEntityError> {
        if self.items.contains_key(&item_id) {
            self.placed_mobs.insert(item_id, coordinate);
            Ok(())
        } else {
            Err(IdOfNotInsertedEntityError)
        }
    }

    pub fn remove_mob_from_world(&mut self, mob_id: u32) {
        todo!();
    }

    pub fn remove_item_from_world(&mut self, item_id: u32) {
        todo!();
    }

    pub fn mob_exists(&mut self, mob_id: u32) -> Result<u32, IdOfNotInsertedEntityError> {
        todo!();
    }

    pub fn item_exists(&mut self, mob_id: u32) -> Result<u32, IdOfNotInsertedEntityError> {
        todo!();
    }
}

impl TopDown2DGridMapRenderer for WorldMap {
    fn render_to_vec(&self) -> Vec<Vec<char>> {
        let mut lines = Vec::<Vec<char>>::new();
        for mapchip_row in self.mapchip_layer.rows() {
            let line = mapchip_row
                .iter()
                .map(|mapchip_id| self.mapchip_to_display_dict[mapchip_id])
                .collect();
            lines.push(line);
        }
        for (mob_id, WorldMapCoordinate { x, y }) in self.placed_mobs.iter() {
            lines[*y as usize][*x as usize] = self.mobs[mob_id].appearance;
        }
        lines
    }
    fn render_to_lines(&self) -> String {
        let mut lines = String::new();
        for line in self.render_to_vec().into_iter() {
            lines.push_str(&line.into_iter().collect::<String>());
            lines.push_str("\n");
        }
        lines
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
    }

    #[test]
    fn test_render_worldmap_vec() {
        let mut worldmap = WorldMap::from_size(3, 3);
        let mut mapchip_to_display_dict = HashMap::<u32, char>::new();
        mapchip_to_display_dict.insert(0, 'a');
        mapchip_to_display_dict.insert(1, 'b');
        worldmap.mapchip_to_display_dict = mapchip_to_display_dict;
        worldmap.mobs.insert(1, Mob { appearance: '@' });
        worldmap
            .placed_mobs
            .insert(1, WorldMapCoordinate { x: 1, y: 1 });
        assert_eq!(
            vec![
                vec!['a', 'a', 'a'],
                vec!['a', '@', 'a'],
                vec!['a', 'a', 'a']
            ],
            worldmap.render_to_vec()
        );
    }

    #[test]
    fn test_render_worldmap_lines() {
        let mut worldmap = WorldMap::from_size(3, 3);
        let mut mapchip_to_display_dict = HashMap::<u32, char>::new();
        mapchip_to_display_dict.insert(0, 'a');
        mapchip_to_display_dict.insert(1, 'b');
        worldmap.mapchip_to_display_dict = mapchip_to_display_dict;
        worldmap.mobs.insert(1, Mob { appearance: '@' });
        worldmap
            .placed_mobs
            .insert(1, WorldMapCoordinate { x: 1, y: 1 });
        assert_eq!("aaa\na@a\naaa\n", worldmap.render_to_lines());
    }
}
