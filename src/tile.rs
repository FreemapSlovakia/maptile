use crate::{bbox::BBox, constants::WEB_MERCATOR_EXTENT};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tile {
    pub zoom: u8,
    pub x: u32,
    pub y: u32,
}

impl Tile {
    pub const fn reversed_y(&self) -> u32 {
        (1 << self.zoom) - 1 - self.y
    }

    pub fn bounds(&self, tile_size: u16) -> BBox {
        let tile_size = f64::from(tile_size);

        let total_pixels = tile_size * f64::from(self.zoom).exp2();
        let pixel_size = (2.0 * WEB_MERCATOR_EXTENT) / total_pixels;

        let min_x = (f64::from(self.x) * tile_size).mul_add(pixel_size, -WEB_MERCATOR_EXTENT);
        let max_y = (f64::from(self.y) * tile_size).mul_add(-pixel_size, WEB_MERCATOR_EXTENT);

        let max_x = tile_size.mul_add(pixel_size, min_x);
        let min_y = tile_size.mul_add(-pixel_size, max_y);

        BBox {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    pub const fn parent(&self) -> Option<Self> {
        if self.zoom == 0 {
            None
        } else {
            Some(Self {
                x: self.x >> 1,
                y: self.y >> 1,
                zoom: self.zoom - 1,
            })
        }
    }

    pub fn ancestor(&self, level: u8) -> Option<Self> {
        let mut tile = Some(*self);

        for _ in 0..level {
            let Some(ref r_tile) = tile else {
                break;
            };

            tile = r_tile.parent();
        }

        tile
    }

    pub fn descendants(&self, level: u8) -> Vec<Tile> {
        let mut tiles = vec![*self];

        for _ in 0..level {
            tiles = tiles.iter().flat_map(|tile| tile.children()).collect();
        }

        tiles
    }

    pub const fn sector_in_parent(&self, levels: u8) -> (u32, u32) {
        (
            self.x & ((1_u32 << levels) - 1),
            self.y & ((1_u32 << levels) - 1),
        )
    }

    pub const fn children(&self) -> [Self; 4] {
        let zoom = self.zoom + 1;

        let x = self.x << 1;
        let y = self.y << 1;

        [
            Self { x, y, zoom },
            Self { x: x + 1, y, zoom },
            Self { x, y: y + 1, zoom },
            Self {
                x: x + 1,
                y: y + 1,
                zoom,
            },
        ]
    }

    pub fn sort_by_zorder(tiles: &mut [Self]) {
        tiles.sort_by_cached_key(Self::morton_code);
    }

    fn interleave(v: u32) -> u64 {
        let mut result = 0u64;

        for i in 0..32 {
            result |= ((u64::from(v) >> i) & 1) << (i << 1);
        }

        result
    }

    pub fn morton_code(&self) -> u64 {
        Self::interleave(self.x) | (Self::interleave(self.y) << 1)
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}/{}", self.zoom, self.x, self.y)
    }
}

pub fn mercator_to_tile_coords(x: f64, y: f64, z: u8) -> (u32, u32) {
    let scale = (1 << z) as f64;

    (
        ((x + WEB_MERCATOR_EXTENT) / (2.0 * WEB_MERCATOR_EXTENT) * scale).floor() as u32,
        ((1.0 - (y + WEB_MERCATOR_EXTENT) / (2.0 * WEB_MERCATOR_EXTENT)) * scale).floor() as u32,
    )
}
