use ndarray::Array2;

struct DungeonMap {
    path_layer: Array2<usize>,
}

struct GameWorld {
    maps: HashMap<String, DungeonMap>
}

#[cfg(test)]
mod tests {
    use super::*;
}
