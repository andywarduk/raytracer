//! Material colour of the hit normal unit vector

use rand::rngs::ThreadRng;

use crate::{hits::hit::Hit, ray::Ray, triple::Colour};

use super::material::{Material, Scattered};

/// Normal material details
#[derive(Debug, Default, Clone)]
pub struct Normal {}

impl Normal {
    /// Create a new normal meterial
    pub fn new() -> Self {
        Self {}
    }
}

impl Material for Normal {
    fn scatter(&self, _rng: &mut ThreadRng, _ray: &Ray, hit: &Hit) -> Scattered {
        let colour = Colour::new_flt(hit.normal[0],hit.normal[1],hit.normal[2]);

        (Some(colour), None, None)
    }
}
