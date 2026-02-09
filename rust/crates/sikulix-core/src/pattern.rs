//! Pattern and Match types for template matching

use crate::{Image, Location, Offset, Region};
use serde::{Deserialize, Serialize};

/// A pattern to search for, containing an image and matching parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    /// The image to search for
    pub image: Image,

    /// Minimum similarity score (0.0 to 1.0)
    pub similarity: f32,

    /// Target offset from the match center
    pub target_offset: Offset,
}

impl Pattern {
    /// Create a new pattern from an image path
    pub fn new(image: Image) -> Self {
        Self {
            image,
            similarity: 0.7, // Default similarity threshold
            target_offset: Offset::zero(),
        }
    }

    /// Set the minimum similarity threshold
    pub fn similar(mut self, similarity: f32) -> Self {
        self.similarity = similarity.clamp(0.0, 1.0);
        self
    }

    /// Set the target offset
    pub fn target_offset(mut self, offset: Offset) -> Self {
        self.target_offset = offset;
        self
    }

    /// Get the target location for a match
    pub fn get_target_location(&self, match_center: Location) -> Location {
        match_center.offset(self.target_offset)
    }
}

/// The result of a successful pattern match
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Match {
    /// The region where the match was found
    pub region: Region,

    /// The similarity score (0.0 to 1.0)
    pub score: f32,

    /// Target offset from the match center
    pub target_offset: Offset,
}

impl Match {
    /// Create a new match
    pub fn new(region: Region, score: f32) -> Self {
        Self {
            region,
            score,
            target_offset: Offset::zero(),
        }
    }

    /// Create a match with a target offset
    pub fn with_offset(mut self, offset: Offset) -> Self {
        self.target_offset = offset;
        self
    }

    /// Get the center location of the match
    pub fn center(&self) -> Location {
        self.region.center()
    }

    /// Get the target location (center + offset)
    pub fn target(&self) -> Location {
        self.center().offset(self.target_offset)
    }

    /// Get the top-left location
    pub fn top_left(&self) -> Location {
        self.region.top_left()
    }

    /// Get the bottom-right location
    pub fn bottom_right(&self) -> Location {
        self.region.bottom_right()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_similarity() {
        let img = Image::new("test.png".to_string(), 10, 10);
        let pattern = Pattern::new(img).similar(0.8);
        assert_eq!(pattern.similarity, 0.8);

        // Test clamping
        let pattern = Pattern::new(Image::new("test.png".to_string(), 10, 10)).similar(1.5);
        assert_eq!(pattern.similarity, 1.0);
    }

    #[test]
    fn test_match_target() {
        let region = Region::new(100, 100, 50, 50);
        let match_result = Match::new(region, 0.9).with_offset(Offset::new(10, -5));

        let target = match_result.target();
        assert_eq!(target.x, 135); // center (125) + offset (10)
        assert_eq!(target.y, 120); // center (125) - offset (5)
    }

    #[test]
    fn test_match_score() {
        let region = Region::new(0, 0, 10, 10);
        let match_result = Match::new(region, 0.95);
        assert_eq!(match_result.score, 0.95);
    }
}
