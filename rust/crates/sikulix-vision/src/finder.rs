//! Template matching and pattern finding

use sikulix_core::{Error, Match, Pattern, Region, Result};

/// Finds patterns in images using template matching
pub struct Finder {
    // Will be implemented in Phase 1
}

impl Finder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn find(&self, _region: Region, _pattern: Pattern) -> Result<Option<Match>> {
        Err(Error::Other(anyhow::anyhow!("Not implemented yet")))
    }
}
