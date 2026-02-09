//! SikuliX Core - Platform-agnostic automation primitives
//!
//! This crate provides the fundamental types used throughout SikuliX:
//! - `Location`: A point on the screen (x, y coordinates)
//! - `Offset`: Relative displacement from a location
//! - `Region`: A rectangular area of the screen
//! - `Pattern`: An image pattern to search for
//! - `Match`: The result of a successful pattern match
//! - `Image`: Representation of an image

pub mod error;
pub mod image;
pub mod location;
pub mod pattern;
pub mod region;

pub use error::{Error, Result};
pub use image::Image;
pub use location::{Location, Offset};
pub use pattern::{Match, Pattern};
pub use region::Region;
