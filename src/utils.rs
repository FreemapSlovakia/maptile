use crate::{bbox::BBox, constants::WEB_MERCATOR_EXTENT, tile_iterator::TileIterator};

/// Returns all tiles covering specified web mercator bounding box at specified zoom.
pub fn bbox_covered_tiles(bbox: &BBox, zoom: u8) -> TileIterator {
    let tile_size_meters = (WEB_MERCATOR_EXTENT * 2.0) / f64::from(1 << zoom);

    // Compute the tile range for the given bounding box
    let min_tile_x = ((bbox.min_x + WEB_MERCATOR_EXTENT) / tile_size_meters).floor() as u32;
    let max_tile_x = ((bbox.max_x + WEB_MERCATOR_EXTENT) / tile_size_meters).ceil() as u32 - 1;
    let min_tile_y = ((WEB_MERCATOR_EXTENT - bbox.max_y) / tile_size_meters).floor() as u32;
    let max_tile_y = ((WEB_MERCATOR_EXTENT - bbox.min_y) / tile_size_meters).ceil() as u32 - 1;

    TileIterator::new(zoom, min_tile_x, min_tile_y, max_tile_x, max_tile_y)
}
