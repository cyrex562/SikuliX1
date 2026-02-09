//! Keyboard control

use sikulix_core::Result;

/// Keyboard control interface
pub struct Keyboard {
    // Will be implemented in Phase 2
}

impl Keyboard {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }

    pub fn type_text(&self, _text: &str) -> Result<()> {
        unimplemented!("Keyboard control - Phase 2")
    }
}
