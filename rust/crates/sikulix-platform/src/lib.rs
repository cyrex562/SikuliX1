//! SikuliX Platform - OS-specific screen capture and input automation

pub mod keyboard;
pub mod mouse;
pub mod screen;

pub use keyboard::Keyboard;
pub use mouse::Mouse;
pub use screen::Screen;

// Platform-specific implementations
#[cfg(windows)]
pub mod platform {
    pub mod windows;
}

#[cfg(target_os = "linux")]
pub mod platform {
    pub mod linux;
}
