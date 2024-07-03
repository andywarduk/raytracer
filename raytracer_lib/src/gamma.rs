use crate::float::*;

pub enum Gamma {
    None,
    Power(Flt),
}

impl Gamma {
    pub fn new(factor: FltPrim) -> Self {
        if factor == 0.0 {
            Self::None
        } else {
            Self::Power(flt(1.0 / factor))
        }
    }
}
