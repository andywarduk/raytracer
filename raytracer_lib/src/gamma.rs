pub enum Gamma {
    None,
    Power(f64),
}

impl Gamma {
    pub fn new(factor: f64) -> Self {
        if factor == 0.0 {
            Self::None
        } else {
            Self::Power(1.0 / factor)
        }
    }
}
