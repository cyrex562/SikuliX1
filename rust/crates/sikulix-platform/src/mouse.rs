//! Mouse control

use sikulix_core::{Location, Result};

/// Mouse control interface
pub struct Mouse {
    // Will be implemented in Phase 2
}

impl Mouse {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }

    pub fn move_to(&self, _location: Location) -> Result<()> {
        unimplemented!("Mouse control - Phase 2")
    }

    pub fn click(&self) -> Result<()> {
        unimplemented!("Mouse control - Phase 2")
    }
}
