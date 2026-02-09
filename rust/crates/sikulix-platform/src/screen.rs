//! Screen capture functionality

use sikulix_core::{Region, Result};

/// Screen capture interface
pub struct Screen {
    // Will be implemented in Phase 2
}

impl Screen {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }

    pub fn capture(&self, _region: Region) -> Result<Vec<u8>> {
        unimplemented!("Screen capture - Phase 2")
    }
}
