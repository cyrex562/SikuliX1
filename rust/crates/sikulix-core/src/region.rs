//! Region type representing a rectangular area on the screen

use crate::{Location, Offset};
use serde::{Deserialize, Serialize};

/// A rectangular area on the screen
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Region {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Region {
    /// Create a new region
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { x, y, w, h }
    }

    /// Create a region from two locations (top-left and bottom-right)
    pub fn from_locations(top_left: Location, bottom_right: Location) -> Self {
        Self {
            x: top_left.x,
            y: top_left.y,
            w: bottom_right.x - top_left.x,
            h: bottom_right.y - top_left.y,
        }
    }

    /// Get the top-left location of this region
    pub fn top_left(&self) -> Location {
        Location::new(self.x, self.y)
    }

    /// Get the center location of this region
    pub fn center(&self) -> Location {
        Location::new(self.x + self.w / 2, self.y + self.h / 2)
    }

    /// Get the bottom-right location of this region
    pub fn bottom_right(&self) -> Location {
        Location::new(self.x + self.w, self.y + self.h)
    }

    /// Check if this region contains a location
    pub fn contains(&self, loc: Location) -> bool {
        loc.x >= self.x
            && loc.x < self.x + self.w
            && loc.y >= self.y
            && loc.y < self.y + self.h
    }

    /// Check if this region overlaps with another region
    pub fn overlaps(&self, other: &Region) -> bool {
        self.x < other.x + other.w
            && self.x + self.w > other.x
            && self.y < other.y + other.h
            && self.y + self.h > other.y
    }

    /// Calculate the intersection of two regions
    pub fn intersection(&self, other: &Region) -> Option<Region> {
        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        let w = (self.x + self.w).min(other.x + other.w) - x;
        let h = (self.y + self.h).min(other.y + other.h) - y;

        if w > 0 && h > 0 {
            Some(Region { x, y, w, h })
        } else {
            None
        }
    }

    /// Get the area of this region
    pub fn area(&self) -> i32 {
        self.w * self.h
    }

    /// Move this region by an offset
    pub fn offset(&self, offset: Offset) -> Self {
        Self {
            x: self.x + offset.dx,
            y: self.y + offset.dy,
            w: self.w,
            h: self.h,
        }
    }

    /// Grow (or shrink if negative) this region by a margin
    pub fn grow(&self, margin: i32) -> Self {
        Self {
            x: self.x - margin,
            y: self.y - margin,
            w: self.w + 2 * margin,
            h: self.h + 2 * margin,
        }
    }
}

impl From<(i32, i32, i32, i32)> for Region {
    fn from((x, y, w, h): (i32, i32, i32, i32)) -> Self {
        Self::new(x, y, w, h)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_region_creation() {
        let region = Region::new(10, 20, 100, 50);
        assert_eq!(region.x, 10);
        assert_eq!(region.y, 20);
        assert_eq!(region.w, 100);
        assert_eq!(region.h, 50);
    }

    #[test]
    fn test_region_center() {
        let region = Region::new(0, 0, 100, 100);
        let center = region.center();
        assert_eq!(center.x, 50);
        assert_eq!(center.y, 50);
    }

    #[test]
    fn test_region_contains() {
        let region = Region::new(0, 0, 100, 100);
        assert!(region.contains(Location::new(50, 50)));
        assert!(region.contains(Location::new(0, 0)));
        assert!(!region.contains(Location::new(100, 100)));
        assert!(!region.contains(Location::new(-1, 50)));
    }

    #[test]
    fn test_region_overlaps() {
        let r1 = Region::new(0, 0, 100, 100);
        let r2 = Region::new(50, 50, 100, 100);
        let r3 = Region::new(200, 200, 100, 100);

        assert!(r1.overlaps(&r2));
        assert!(r2.overlaps(&r1));
        assert!(!r1.overlaps(&r3));
    }

    #[test]
    fn test_region_intersection() {
        let r1 = Region::new(0, 0, 100, 100);
        let r2 = Region::new(50, 50, 100, 100);

        let intersection = r1.intersection(&r2).unwrap();
        assert_eq!(intersection.x, 50);
        assert_eq!(intersection.y, 50);
        assert_eq!(intersection.w, 50);
        assert_eq!(intersection.h, 50);

        let r3 = Region::new(200, 200, 100, 100);
        assert!(r1.intersection(&r3).is_none());
    }

    #[test]
    fn test_region_area() {
        let region = Region::new(0, 0, 100, 50);
        assert_eq!(region.area(), 5000);
    }

    #[test]
    fn test_region_grow() {
        let region = Region::new(50, 50, 100, 100);
        let grown = region.grow(10);
        assert_eq!(grown.x, 40);
        assert_eq!(grown.y, 40);
        assert_eq!(grown.w, 120);
        assert_eq!(grown.h, 120);
    }
}
