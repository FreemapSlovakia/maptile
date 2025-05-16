use std::{error::Error, fmt::Display, num::ParseFloatError, str::FromStr};

/// 2D bounding box.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BBox {
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}

impl BBox {
    /// Creates new bounding box.
    #[must_use]
    pub const fn new(min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> Self {
        Self {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    /// Tests if bounding box completly contains another bounding box
    #[must_use]
    pub fn contains(&self, x: f64, y: f64) -> bool {
        x >= self.min_x && y >= self.min_y && x < self.max_x && y < self.max_y
    }

    /// Get width of the bounding box
    #[must_use]
    pub fn width(&self) -> f64 {
        self.max_x - self.min_x
    }

    /// Get height of the bounding box
    #[must_use]
    pub fn height(&self) -> f64 {
        self.max_y - self.min_y
    }

    /// Create buffered bouding box
    #[must_use]
    pub fn to_buffered(&self, buffer: f64) -> Self {
        Self {
            min_x: self.min_x - buffer,
            min_y: self.min_y - buffer,
            max_x: self.max_x + buffer,
            max_y: self.max_y + buffer,
        }
    }
}

impl From<[f64; 4]> for BBox {
    fn from(bbox: [f64; 4]) -> Self {
        Self {
            min_x: bbox[0],
            min_y: bbox[1],
            max_x: bbox[2],
            max_y: bbox[3],
        }
    }
}

impl From<BBox> for [f64; 4] {
    fn from(bbox: BBox) -> Self {
        [bbox.min_x, bbox.min_y, bbox.max_x, bbox.max_y]
    }
}

impl From<(f64, f64, f64, f64)> for BBox {
    fn from(bbox: (f64, f64, f64, f64)) -> Self {
        Self {
            min_x: bbox.0,
            min_y: bbox.1,
            max_x: bbox.2,
            max_y: bbox.3,
        }
    }
}

impl From<BBox> for (f64, f64, f64, f64) {
    fn from(bbox: BBox) -> Self {
        (bbox.min_x, bbox.min_y, bbox.max_x, bbox.max_y)
    }
}

impl Display for BBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{},{}",
            self.min_x, self.min_y, self.max_x, self.max_y
        )
    }
}

#[derive(Debug)]
pub enum BBoxParseError {
    ParseFloatError(ParseFloatError),
    NumberOfElementsError,
}

impl Display for BBoxParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseFloatError(parse_float_error) => parse_float_error.fmt(f),
            Self::NumberOfElementsError => {
                write!(f, "Expected exactly 4 comma-separated values")
            }
        }
    }
}

impl Error for BBoxParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl From<ParseFloatError> for BBoxParseError {
    fn from(value: ParseFloatError) -> Self {
        Self::ParseFloatError(value)
    }
}

/// Parse bbox from string `min_x,min_y,max_x,max_y`; tollerating spaces
impl FromStr for BBox {
    type Err = BBoxParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').map(str::trim).collect();

        if parts.len() != 4 {
            return Err(BBoxParseError::NumberOfElementsError);
        }

        let min_x = parts[0].parse::<f64>()?;
        let min_y = parts[1].parse::<f64>()?;
        let max_x = parts[2].parse::<f64>()?;
        let max_y = parts[3].parse::<f64>()?;

        Ok(Self {
            min_x,
            min_y,
            max_x,
            max_y,
        })
    }
}
