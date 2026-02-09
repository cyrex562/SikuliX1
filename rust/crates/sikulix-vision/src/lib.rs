//! SikuliX Vision - Computer vision capabilities using OpenCV
//!
//! This crate provides template matching, image processing, and OCR capabilities.

pub mod finder;
pub mod image_loader;
pub mod mat_wrapper;
pub mod matcher;
pub mod ocr;
pub mod resize;

pub use finder::Finder;
pub use image_loader::ImageLoader;
pub use mat_wrapper::MatWrapper;
// pub use matcher::TemplateMatcher;
// pub use ocr::TextRecognizer;
