//! Image representation for SikuliX

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Represents an image to be used in pattern matching
///
/// This is a lightweight representation that stores the path and dimensions.
/// The actual image data is loaded lazily when needed by the vision module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    /// Path to the image file
    pub path: String,

    /// Width of the image in pixels
    pub width: u32,

    /// Height of the image in pixels
    pub height: u32,
}

impl Image {
    /// Create a new image reference
    pub fn new(path: String, width: u32, height: u32) -> Self {
        Self {
            path,
            width,
            height,
        }
    }

    /// Create an image reference from a path (dimensions will be loaded lazily)
    pub fn from_path(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            width: 0, // Will be loaded when needed
            height: 0,
        }
    }

    /// Get the image path
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Get the image dimensions
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Check if dimensions are loaded
    pub fn has_dimensions(&self) -> bool {
        self.width > 0 && self.height > 0
    }

    /// Convert path to PathBuf
    pub fn path_buf(&self) -> PathBuf {
        PathBuf::from(&self.path)
    }
}

impl From<String> for Image {
    fn from(path: String) -> Self {
        Self::from_path(path)
    }
}

impl From<&str> for Image {
    fn from(path: &str) -> Self {
        Self::from_path(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_creation() {
        let img = Image::new("test.png".to_string(), 100, 50);
        assert_eq!(img.path(), "test.png");
        assert_eq!(img.dimensions(), (100, 50));
        assert!(img.has_dimensions());
    }

    #[test]
    fn test_image_from_path() {
        let img = Image::from_path("image.png");
        assert_eq!(img.path(), "image.png");
        assert!(!img.has_dimensions());
    }

    #[test]
    fn test_image_from_string() {
        let img: Image = "button.png".into();
        assert_eq!(img.path(), "button.png");
    }
}
