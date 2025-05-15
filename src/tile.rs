use crate::{bbox::BBox, constants::WEB_MERCATOR_EXTENT};
use itertools::iproduct;
use std::{error::Error, fmt::Display, str::FromStr};

/// Map tile
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tile {
    pub zoom: u8,
    pub x: u32,
    pub y: u32,
}

impl Tile {
    /// Returns reversed y coordinate: 2 ^ zoom - 1 - y
    pub const fn reversed_y(&self) -> u32 {
        (1 << self.zoom) - 1 - self.y
    }

    /// Returns new `Tile`` with reversed y coordinate: 2 ^ zoom - 1 - y
    pub const fn to_reversed_y(&self) -> Self {
        Tile {
            y: self.reversed_y(),
            ..*self
        }
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

    /// Returns parent tile - tile of one less zoom containing this tile.
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

    /// Returns ancestor tile of specified relative level containing this tile.
    pub fn ancestor(&self, rel_level: u8) -> Option<Self> {
        let mut tile = Some(*self);

        for _ in 0..rel_level {
            let Some(ref r_tile) = tile else {
                break;
            };

            tile = r_tile.parent();
        }

        tile
    }

    /// Returns all descendant tiles of relative level.
    pub fn descendants(&self, rel_level: u8) -> Vec<Tile> {
        let mut tiles = vec![*self];

        for _ in 0..rel_level {
            tiles = tiles.iter().flat_map(|tile| tile.children()).collect();
        }

        tiles
    }

    /// Returns sector number of the tile in its ancestor. Counting goes from top-left corner and goes by columns then rows, eg:
    /// ```
    /// 1 2
    /// 3 4
    /// ```
    pub const fn sector_in_ancestor(&self, rel_level: u8) -> (u32, u32) {
        (
            self.x & ((1_u32 << rel_level) - 1),
            self.y & ((1_u32 << rel_level) - 1),
        )
    }

    /// Returns all children of the tile.
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

    /// Returns all children tiles of the tile including their adjacent tiles of specified buffer.
    pub fn children_buffered(&self, buffer: u8) -> impl Iterator<Item = Self> {
        let zoom = self.zoom + 1;

        let x = self.x << 1;
        let y = self.y << 1;

        let buffer = buffer as u32;

        let range = 0..(buffer + 1) * 2;

        let max = 2 << zoom;

        iproduct!(range.clone(), range).map(move |(dx, dy)| {
            let wrap = |v: u32| {
                let v = (if buffer > v { max } else { 0 }) + v - buffer;

                v - if v >= max { max } else { 0 }
            };

            Self {
                x: wrap(x + dx),
                y: wrap(y + dy),
                zoom,
            }
        })
    }

    /// Sort tiles according to morton code. Currently it does not take the zoom into the account.
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

    /// Returns tile's [Morton code](https://en.wikipedia.org/wiki/Z-order_curve).
    pub fn morton_code(&self) -> u64 {
        Self::interleave(self.x) | (Self::interleave(self.y) << 1)
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}/{}", self.zoom, self.x, self.y)
    }
}

/// Converts web mercator coordinates to tile coordinates of the specified zoom level.
pub fn mercator_to_tile_coords(x: f64, y: f64, zoom: u8) -> (u32, u32) {
    let scale = (1 << zoom) as f64;

    (
        ((x + WEB_MERCATOR_EXTENT) / (2.0 * WEB_MERCATOR_EXTENT) * scale).floor() as u32,
        ((1.0 - (y + WEB_MERCATOR_EXTENT) / (2.0 * WEB_MERCATOR_EXTENT)) * scale).floor() as u32,
    )
}

#[derive(Debug)]
pub struct ParseError;

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid tile format")
    }
}

impl Error for ParseError {}

impl FromStr for Tile {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(3, '/').collect();

        if parts.len() != 3 {
            return Err(ParseError);
        }

        Ok(Tile {
            zoom: parts[0].parse().map_err(|_| ParseError)?,
            x: parts[1].parse().map_err(|_| ParseError)?,
            y: parts[2].parse().map_err(|_| ParseError)?,
        })
    }
}
