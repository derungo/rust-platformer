// tile.rs
pub struct Tile {
    pub tile_index: usize,
    pub position: (f32, f32),
}

pub struct TileMap {
    pub tiles: Vec<Tile>,
    pub tile_width: f32,
    pub tile_height: f32,
    pub tileset_columns: usize,
    pub tileset_rows: usize,
}

impl TileMap {
    pub fn new_ground(tile_width: f32, tile_height: f32, tileset_columns: usize, tileset_rows: usize) -> Self {
        let mut tiles = Vec::new();

        // Define the number of ground tiles you want
        let ground_length = 20; // Adjust as needed

        // Choose a tile index that corresponds to the ground tile in your tileset
        let ground_tile_index = 21; // Replace with the actual index in your tileset

        for i in 0..ground_length {
            tiles.push(Tile {
                tile_index: ground_tile_index,
                position: (i as f32 * tile_width, -0.5), // Adjust y position as needed
            });
        }

        Self {
            tiles,
            tile_width,
            tile_height,
            tileset_columns,
            tileset_rows,
        }
    }
}