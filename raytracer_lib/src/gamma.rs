//! Gamma correction

use crate::float::*;

/// Gamma correction
pub enum Gamma {
    /// No gamma correction
    None,
    /// Power factor
    Power(Flt),
}

impl Gamma {
    /// Create new gamma correction def
    pub fn new(factor: FltPrim) -> Self {
        if factor == 0.0 {
            Self::None
        } else {
            Self::Power(flt(1.0 / factor))
        }
    }
}
