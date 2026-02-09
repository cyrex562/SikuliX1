//! Image loading from files and memory buffers

use crate::mat_wrapper::MatWrapper;
use opencv::imgcodecs::{imread, imdecode, IMREAD_COLOR, IMREAD_UNCHANGED};
use opencv::core::{Vector, Mat, AlgorithmHint};
use opencv::prelude::*;
use sikulix_core::{Error, Result};
use std::path::Path;
use tracing::{debug, trace};

/// Image loader for reading images from various sources
pub struct ImageLoader;

impl ImageLoader {
    /// Load an image from a file path
    ///
    /// Supports: PNG, JPEG, BMP, TIFF, WebP
    ///
    /// # Arguments
    /// * `path` - Path to the image file
    /// * `color` - If true, load as color image (BGR). If false, load as-is with alpha if present
    ///
    /// # Returns
    /// MatWrapper containing the loaded image
    ///
    /// # Errors
    /// Returns error if:
    /// - File doesn't exist
    /// - File is not a valid image format
    /// - OpenCV fails to decode the image
    pub fn load_from_file<P: AsRef<Path>>(path: P, color: bool) -> Result<MatWrapper> {
        let path_str = path
            .as_ref()
            .to_str()
            .ok_or_else(|| Error::InvalidParameter("Invalid UTF-8 in path".to_string()))?;

        debug!("Loading image from file: {}", path_str);

        // Check if file exists
        if !path.as_ref().exists() {
            return Err(Error::ImageNotFound(format!(
                "File not found: {}",
                path_str
            )));
        }

        // Load image with OpenCV
        let flags = if color { IMREAD_COLOR } else { IMREAD_UNCHANGED };
        let mat = imread(path_str, flags).map_err(|e| {
            Error::Platform(format!("OpenCV imread failed for {}: {}", path_str, e))
        })?;

        // Check if image was loaded successfully
        if mat.empty() {
            return Err(Error::ImageNotFound(format!(
                "OpenCV loaded empty image from: {}",
                path_str
            )));
        }

        let size = mat.size().map_err(|e| {
            Error::Platform(format!("Failed to get image size: {}", e))
        })?;
        trace!(
            "Loaded image: {}x{}, channels: {}",
            size.width,
            size.height,
            mat.channels()
        );

        Ok(MatWrapper::new(mat))
    }

    /// Load an image from a memory buffer
    ///
    /// # Arguments
    /// * `buffer` - Byte buffer containing encoded image data (PNG, JPEG, etc.)
    /// * `color` - If true, decode as color image (BGR). If false, decode with alpha if present
    ///
    /// # Returns
    /// MatWrapper containing the decoded image
    ///
    /// # Errors
    /// Returns error if buffer doesn't contain valid image data
    pub fn load_from_memory(buffer: &[u8], color: bool) -> Result<MatWrapper> {
        debug!("Loading image from memory buffer ({} bytes)", buffer.len());

        if buffer.is_empty() {
            return Err(Error::InvalidParameter("Empty buffer".to_string()));
        }

        // Create OpenCV Vector from buffer
        let vec = Vector::<u8>::from_slice(buffer);

        // Decode image from memory
        let flags = if color { IMREAD_COLOR } else { IMREAD_UNCHANGED };
        let mat = imdecode(&vec, flags).map_err(|e| {
            Error::Platform(format!("OpenCV imdecode failed: {}", e))
        })?;

        // Check if image was decoded successfully
        if mat.empty() {
            return Err(Error::InvalidParameter(
                "Failed to decode image from buffer".to_string(),
            ));
        }

        let size = mat.size().map_err(|e| {
            Error::Platform(format!("Failed to get image size: {}", e))
        })?;
        trace!(
            "Decoded image: {}x{}, channels: {}",
            size.width,
            size.height,
            mat.channels()
        );

        Ok(MatWrapper::new(mat))
    }

    /// Load an image from a file, converting to grayscale
    pub fn load_as_grayscale<P: AsRef<Path>>(path: P) -> Result<MatWrapper> {
        use opencv::imgproc::{cvt_color, COLOR_BGR2GRAY};

        let color_image = Self::load_from_file(path, true)?;

        // Convert BGR to GRAY
        let mut gray_mat = Mat::default();
        cvt_color(color_image.as_mat(), &mut gray_mat, COLOR_BGR2GRAY, 0, AlgorithmHint::ALGO_HINT_DEFAULT).map_err(|e| {
            Error::Platform(format!("Failed to convert to grayscale: {}", e))
        })?;

        Ok(MatWrapper::new(gray_mat))
    }

    /// Get image dimensions without fully loading it (if possible)
    ///
    /// For now, this loads the full image. Future optimization could use
    /// format-specific header parsing.
    pub fn get_dimensions<P: AsRef<Path>>(path: P) -> Result<(u32, u32)> {
        let mat = Self::load_from_file(path, false)?;
        let (w, h) = mat.size()?;
        Ok((w as u32, h as u32))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use opencv::core::{Mat, Vector, CV_8UC3};
    use opencv::imgcodecs::{imencode, IMWRITE_PNG_COMPRESSION};
    use std::fs;
    use tempfile::TempDir;

    /// Helper to create a test image file
    fn create_test_image(dir: &TempDir, filename: &str, width: i32, height: i32) -> std::path::PathBuf {
        let path = dir.path().join(filename);

        // Create a test image with OpenCV
        let mat = Mat::new_rows_cols_with_default(
            height,
            width,
            CV_8UC3,
            (255, 128, 64, 0).into(),
        )
        .unwrap();

        // Encode as PNG
        let mut buf = Vector::<u8>::new();
        let params = Vector::<i32>::from_slice(&[IMWRITE_PNG_COMPRESSION, 3]);
        imencode(".png", &mat, &mut buf, &params).unwrap();

        // Write to file
        fs::write(&path, buf.to_vec()).unwrap();
        path
    }

    #[test]
    fn test_load_from_file_success() {
        let temp_dir = TempDir::new().unwrap();
        let image_path = create_test_image(&temp_dir, "test.png", 100, 50);

        let result = ImageLoader::load_from_file(&image_path, true);
        assert!(result.is_ok());

        let mat = result.unwrap();
        assert!(!mat.is_empty());
        assert_eq!(mat.size().unwrap(), (100, 50));
        assert_eq!(mat.channels().unwrap(), 3); // BGR
    }

    #[test]
    fn test_load_from_file_not_found() {
        let result = ImageLoader::load_from_file("nonexistent.png", true);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::ImageNotFound(_)));
    }

    #[test]
    fn test_load_from_file_invalid_format() {
        let temp_dir = TempDir::new().unwrap();
        let invalid_file = temp_dir.path().join("invalid.png");
        fs::write(&invalid_file, b"not an image").unwrap();

        let result = ImageLoader::load_from_file(&invalid_file, true);
        // Should fail to load
        assert!(result.is_err());
    }

    #[test]
    fn test_load_from_memory_success() {
        // Create a small PNG in memory
        let mat = Mat::new_rows_cols_with_default(20, 30, CV_8UC3, (100, 150, 200, 0).into()).unwrap();
        let mut buf = Vector::<u8>::new();
        imencode(".png", &mat, &mut buf, &Vector::new()).unwrap();

        let result = ImageLoader::load_from_memory(&buf.to_vec(), true);
        assert!(result.is_ok());

        let loaded = result.unwrap();
        assert!(!loaded.is_empty());
        assert_eq!(loaded.size().unwrap(), (30, 20));
    }

    #[test]
    fn test_load_from_memory_empty_buffer() {
        let result = ImageLoader::load_from_memory(&[], true);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::InvalidParameter(_)));
    }

    #[test]
    fn test_load_from_memory_invalid_data() {
        let invalid_data = b"this is not an image";
        let result = ImageLoader::load_from_memory(invalid_data, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_load_as_grayscale() {
        let temp_dir = TempDir::new().unwrap();
        let image_path = create_test_image(&temp_dir, "color.png", 50, 50);

        let result = ImageLoader::load_as_grayscale(&image_path);
        assert!(result.is_ok());

        let gray_mat = result.unwrap();
        assert_eq!(gray_mat.channels().unwrap(), 1); // Grayscale
        assert_eq!(gray_mat.size().unwrap(), (50, 50));
    }

    #[test]
    fn test_get_dimensions() {
        let temp_dir = TempDir::new().unwrap();
        let image_path = create_test_image(&temp_dir, "sized.png", 123, 456);

        let result = ImageLoader::get_dimensions(&image_path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), (123, 456));
    }

    #[test]
    fn test_load_color_vs_unchanged() {
        let temp_dir = TempDir::new().unwrap();
        let image_path = create_test_image(&temp_dir, "modes.png", 50, 50);

        let color = ImageLoader::load_from_file(&image_path, true).unwrap();
        let unchanged = ImageLoader::load_from_file(&image_path, false).unwrap();

        // Both should load, but may have different channel counts depending on source
        assert!(!color.is_empty());
        assert!(!unchanged.is_empty());
        assert_eq!(color.size().unwrap(), unchanged.size().unwrap());
    }
}
