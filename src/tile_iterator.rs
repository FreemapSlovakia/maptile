use std::iter::successors;

use itertools::Itertools;

use crate::tile::Tile;

/// Tile iterator
pub struct TileIterator<T> {
    zoom: u8,
    x_range: T,
    x_range_fresh: T,
    y_range: T,
    y: Option<u32>,
}

impl<T: Iterator<Item = u32> + Clone> TileIterator<T> {
    /// Creates new tile iterator.
    pub fn new(zoom: u8, x_range: T, mut y_range: T) -> Self {
        Self {
            zoom,
            x_range_fresh: x_range.clone(),
            x_range,
            y: y_range.next(),
            y_range,
        }
    }

    pub fn pyramid(self) -> impl Iterator<Item = Tile> {
        self.flat_map(|tile| successors(Some(tile), Tile::parent))
            .unique()
    }
}

impl<T: Iterator<Item = u32> + Clone> Iterator for TileIterator<T> {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        let y = self.y?;

        let x = self.x_range.next();

        let Some(x) = x else {
            self.x_range = self.x_range_fresh.clone();
            self.y = self.y_range.next();

            return self.next();
        };

        Some(Tile {
            zoom: self.zoom,
            x,
            y,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator() {
        let tiles: Vec<Tile> = TileIterator::new(3, 1..=2, 2..=4).collect();

        let expect: Result<Vec<Tile>, _> = ["3/1/2", "3/2/2", "3/1/3", "3/2/3", "3/1/4", "3/2/4"]
            .iter()
            .map(|s| s.parse())
            .collect();

        assert_eq!(Ok(tiles), expect);
    }

    #[test]
    fn test_pyramid() {
        let expect: Result<Vec<Tile>, _> = [
            "2/1/1", "1/0/0", "0/0/0", "2/2/1", "1/1/0", "2/1/2", "1/0/1", "2/2/2", "1/1/1",
            "2/1/3", "2/2/3",
        ]
        .iter()
        .map(|&s| s.parse())
        .collect();

        let pyramid: Vec<Tile> = TileIterator::new(2, 1..=2, 1..=3).pyramid().collect();

        assert_eq!(Ok(pyramid), expect);
    }
}
