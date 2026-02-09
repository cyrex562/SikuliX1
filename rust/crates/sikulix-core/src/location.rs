//! Location and Offset types for screen coordinates

use serde::{Deserialize, Serialize};

/// A point on the screen with x, y coordinates
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub x: i32,
    pub y: i32,
}

impl Location {
    /// Create a new location at (x, y)
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Move this location by an offset
    pub fn offset(&self, offset: Offset) -> Self {
        Self {
            x: self.x + offset.dx,
            y: self.y + offset.dy,
        }
    }

    /// Calculate the offset from this location to another
    pub fn offset_to(&self, other: Location) -> Offset {
        Offset {
            dx: other.x - self.x,
            dy: other.y - self.y,
        }
    }

    /// Calculate distance to another location
    pub fn distance_to(&self, other: Location) -> f64 {
        let dx = (other.x - self.x) as f64;
        let dy = (other.y - self.y) as f64;
        (dx * dx + dy * dy).sqrt()
    }
}

impl From<(i32, i32)> for Location {
    fn from((x, y): (i32, i32)) -> Self {
        Self::new(x, y)
    }
}

/// A relative displacement with dx, dy components
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Offset {
    pub dx: i32,
    pub dy: i32,
}

impl Offset {
    /// Create a new offset (dx, dy)
    pub fn new(dx: i32, dy: i32) -> Self {
        Self { dx, dy }
    }

    /// Zero offset (no displacement)
    pub fn zero() -> Self {
        Self { dx: 0, dy: 0 }
    }
}

impl From<(i32, i32)> for Offset {
    fn from((dx, dy): (i32, i32)) -> Self {
        Self::new(dx, dy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_creation() {
        let loc = Location::new(100, 200);
        assert_eq!(loc.x, 100);
        assert_eq!(loc.y, 200);
    }

    #[test]
    fn test_location_offset() {
        let loc = Location::new(100, 200);
        let offset = Offset::new(50, -30);
        let new_loc = loc.offset(offset);
        assert_eq!(new_loc.x, 150);
        assert_eq!(new_loc.y, 170);
    }

    #[test]
    fn test_distance() {
        let loc1 = Location::new(0, 0);
        let loc2 = Location::new(3, 4);
        assert_eq!(loc1.distance_to(loc2), 5.0);
    }

    #[test]
    fn test_offset_to() {
        let loc1 = Location::new(100, 100);
        let loc2 = Location::new(150, 80);
        let offset = loc1.offset_to(loc2);
        assert_eq!(offset.dx, 50);
        assert_eq!(offset.dy, -20);
    }
}
