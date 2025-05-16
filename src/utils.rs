use std::ops::RangeInclusive;

use crate::{bbox::BBox, constants::WEB_MERCATOR_EXTENT, tile_iterator::TileIterator};

/// Returns all tiles covering specified web mercator bounding box at specified zoom.
#[must_use]
pub fn bbox_covered_tiles(bbox: &BBox, zoom: u8) -> TileIterator<RangeInclusive<u32>> {
    let tile_size_meters = (WEB_MERCATOR_EXTENT * 2.0) / f64::from(1 << zoom);

    // Compute the tile range for the given bounding box
    let min_tile_x = ((bbox.min_x + WEB_MERCATOR_EXTENT) / tile_size_meters).floor() as u32;
    let max_tile_x = ((bbox.max_x + WEB_MERCATOR_EXTENT) / tile_size_meters).ceil() as u32 - 1;
    let min_tile_y = ((WEB_MERCATOR_EXTENT - bbox.max_y) / tile_size_meters).floor() as u32;
    let max_tile_y = ((WEB_MERCATOR_EXTENT - bbox.min_y) / tile_size_meters).ceil() as u32 - 1;

    TileIterator::new(zoom, min_tile_x..=max_tile_x, min_tile_y..=max_tile_y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Tile;

    #[test]
    fn test_bbox_covered_tiles() {
        let tiles: Vec<Tile> =
            bbox_covered_tiles(&BBox::new(1137489.0, 5980732.0, 1711100.0, 6428543.0), 7).collect();

        let expect: Result<Vec<Tile>, _> = [
            "7/67/43", "7/68/43", "7/69/43", "7/67/44", "7/68/44", "7/69/44",
        ]
        .iter()
        .map(|&s| s.parse())
        .collect();

        assert_eq!(Ok(tiles), expect);
    }
}
