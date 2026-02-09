//! Property-based tests for image loading operations

use opencv::core::{Mat, Vector, CV_8UC3, CV_8UC1};
use opencv::imgcodecs::{imencode, IMWRITE_PNG_COMPRESSION, IMWRITE_JPEG_QUALITY};
use proptest::prelude::*;
use sikulix_vision::ImageLoader;

/// Generate valid image dimensions
fn image_dimensions() -> impl Strategy<Value = (i32, i32)> {
    (1i32..=1000, 1i32..=1000)
}

/// Generate valid pixel values for BGR images
fn bgr_pixel() -> impl Strategy<Value = (u8, u8, u8, u8)> {
    (any::<u8>(), any::<u8>(), any::<u8>(), Just(0u8))
}

proptest! {
    /// Property: Loading an image from memory should preserve dimensions
    #[test]
    fn prop_load_preserves_dimensions(
        (width, height) in image_dimensions(),
        (b, g, r, _a) in bgr_pixel(),
    ) {
        // Create an image with OpenCV
        let mat = Mat::new_rows_cols_with_default(
            height,
            width,
            CV_8UC3,
            (b, g, r, 0).into(),
        ).unwrap();

        // Encode as PNG
        let mut buf = Vector::<u8>::new();
        let params = Vector::<i32>::from_slice(&[IMWRITE_PNG_COMPRESSION, 3]);
        imencode(".png", &mat, &mut buf, &params).unwrap();

        // Load from memory
        let loaded = ImageLoader::load_from_memory(&buf.to_vec(), true).unwrap();

        // Check dimensions are preserved
        let (loaded_width, loaded_height) = loaded.size().unwrap();
        prop_assert_eq!(loaded_width, width, "Width mismatch");
        prop_assert_eq!(loaded_height, height, "Height mismatch");
    }

    /// Property: Loading as color should always produce 3 channels
    #[test]
    fn prop_load_color_has_three_channels(
        (width, height) in image_dimensions(),
    ) {
        // Create a BGR image
        let mat = Mat::new_rows_cols_with_default(
            height,
            width,
            CV_8UC3,
            (128, 128, 128, 0).into(),
        ).unwrap();

        // Encode as PNG
        let mut buf = Vector::<u8>::new();
        imencode(".png", &mat, &mut buf, &Vector::new()).unwrap();

        // Load as color
        let loaded = ImageLoader::load_from_memory(&buf.to_vec(), true).unwrap();

        // Should have 3 channels (BGR)
        prop_assert_eq!(loaded.channels().unwrap(), 3);
    }

    /// Property: Loading as grayscale should produce 1 channel
    #[test]
    fn prop_load_grayscale_has_one_channel(
        (width, height) in image_dimensions(),
        gray_value in any::<u8>(),
    ) {
        // Create a grayscale image
        let mat = Mat::new_rows_cols_with_default(
            height,
            width,
            CV_8UC1,
            gray_value.into(),
        ).unwrap();

        // Encode as PNG
        let mut buf = Vector::<u8>::new();
        imencode(".png", &mat, &mut buf, &Vector::new()).unwrap();

        // Load with UNCHANGED flag (preserves grayscale)
        let loaded = ImageLoader::load_from_memory(&buf.to_vec(), false).unwrap();

        // Should have 1 channel
        prop_assert_eq!(loaded.channels().unwrap(), 1);
    }

    /// Property: Encoded image size should be non-zero and reasonable
    #[test]
    fn prop_encoded_size_reasonable(
        (width, height) in image_dimensions(),
    ) {
        let mat = Mat::new_rows_cols_with_default(
            height,
            width,
            CV_8UC3,
            (0, 0, 0, 0).into(),
        ).unwrap();

        // Encode as PNG
        let mut png_buf = Vector::<u8>::new();
        imencode(".png", &mat, &mut png_buf, &Vector::new()).unwrap();

        // Encode as JPEG
        let mut jpg_buf = Vector::<u8>::new();
        let params = Vector::<i32>::from_slice(&[IMWRITE_JPEG_QUALITY, 85]);
        imencode(".jpg", &mat, &mut jpg_buf, &params).unwrap();

        // Both should be non-empty
        prop_assert!(png_buf.len() > 0, "PNG buffer is empty");
        prop_assert!(jpg_buf.len() > 0, "JPEG buffer is empty");

        // Should be able to decode both
        let png_loaded = ImageLoader::load_from_memory(&png_buf.to_vec(), true).unwrap();
        let jpg_loaded = ImageLoader::load_from_memory(&jpg_buf.to_vec(), true).unwrap();

        prop_assert!(!png_loaded.is_empty());
        prop_assert!(!jpg_loaded.is_empty());
    }

    /// Property: MatWrapper clone should have same properties as original
    #[test]
    fn prop_mat_clone_preserves_properties(
        (width, height) in image_dimensions(),
    ) {
        let mat = Mat::new_rows_cols_with_default(
            height,
            width,
            CV_8UC3,
            (100, 150, 200, 0).into(),
        ).unwrap();

        // Encode and load
        let mut buf = Vector::<u8>::new();
        imencode(".png", &mat, &mut buf, &Vector::new()).unwrap();
        let original = ImageLoader::load_from_memory(&buf.to_vec(), true).unwrap();

        // Clone it
        let cloned = original.clone_mat().unwrap();

        // Properties should match
        prop_assert_eq!(original.size().unwrap(), cloned.size().unwrap());
        prop_assert_eq!(original.channels().unwrap(), cloned.channels().unwrap());
        prop_assert_eq!(original.mat_type().unwrap(), cloned.mat_type().unwrap());
        prop_assert_eq!(original.is_empty(), cloned.is_empty());
    }
}

/// Additional property tests for edge cases
#[cfg(test)]
mod edge_cases {
    use super::*;

    proptest! {
        /// Property: Very small images (1x1) should work
        #[test]
        fn prop_tiny_image_works(pixel_value in any::<u8>()) {
            let mat = Mat::new_rows_cols_with_default(
                1, 1, CV_8UC3,
                (pixel_value, pixel_value, pixel_value, 0).into(),
            ).unwrap();

            let mut buf = Vector::<u8>::new();
            imencode(".png", &mat, &mut buf, &Vector::new()).unwrap();

            let loaded = ImageLoader::load_from_memory(&buf.to_vec(), true).unwrap();
            prop_assert_eq!(loaded.size().unwrap(), (1, 1));
        }

        /// Property: Square images preserve aspect ratio
        #[test]
        fn prop_square_images(size in 1i32..=500) {
            let mat = Mat::new_rows_cols_with_default(
                size, size, CV_8UC3,
                (128, 128, 128, 0).into(),
            ).unwrap();

            let mut buf = Vector::<u8>::new();
            imencode(".png", &mat, &mut buf, &Vector::new()).unwrap();

            let loaded = ImageLoader::load_from_memory(&buf.to_vec(), true).unwrap();
            let (w, h) = loaded.size().unwrap();

            prop_assert_eq!(w, h, "Square image lost aspect ratio");
            prop_assert_eq!(w, size);
        }

        /// Property: Empty buffer should always fail
        #[test]
        fn prop_empty_buffer_fails(_dummy in any::<u8>()) {
            let result = ImageLoader::load_from_memory(&[], true);
            prop_assert!(result.is_err());
        }

        /// Property: Invalid data should always fail
        #[test]
        fn prop_invalid_data_fails(invalid_bytes in prop::collection::vec(any::<u8>(), 1..100)) {
            // Filter out any accidentally valid image headers
            if invalid_bytes.starts_with(&[0x89, 0x50, 0x4E, 0x47]) ||  // PNG
               invalid_bytes.starts_with(&[0xFF, 0xD8, 0xFF]) {         // JPEG
                return Ok(());
            }

            let result = ImageLoader::load_from_memory(&invalid_bytes, true);
            prop_assert!(result.is_err(), "Should fail on invalid data");
        }
    }
}
