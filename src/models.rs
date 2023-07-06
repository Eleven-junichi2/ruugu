use std::collections::HashMap;

use ndarray::Array2;

struct Mob {
    
}

struct DungeonMap {
    path_layer: Array2<usize>,
    mapchip_layer: Array2<usize>,
    mapchip_to_display_dict: HashMap<usize, char>,
    mob_layer: Array2<usize>,
}

struct GameWorld {
    maps: HashMap<String, DungeonMap>,
    mobs: HashMap<usize, Mob>
}

#[cfg(test)]
mod tests {
    use super::*;
}
