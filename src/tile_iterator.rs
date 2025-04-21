use crate::tile::Tile;

/// Tile iterator
pub struct TileIterator {
    zoom: u8,
    x: u32,
    y: u32,
    min_tile_x: u32,
    max_tile_x: u32,
    max_tile_y: u32,
    init: bool,
}

impl TileIterator {
    /// Creates new tile iterator.
    pub fn new(
        zoom: u8,
        min_tile_x: u32,
        min_tile_y: u32,
        max_tile_x: u32,
        max_tile_y: u32,
    ) -> Self {
        TileIterator {
            zoom,
            x: min_tile_x,
            y: min_tile_y,
            min_tile_x,
            max_tile_x,
            max_tile_y,
            init: true,
        }
    }
}

impl Iterator for TileIterator {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        if self.init {
            self.init = false;
        } else if self.x > self.max_tile_x {
            self.x = self.min_tile_x;

            self.y += 1;

            if self.y > self.max_tile_y {
                return None;
            }
        } else {
            self.x += 1;
        }

        Some(Tile {
            zoom: self.zoom,
            x: self.x,
            y: self.y,
        })
    }
}
