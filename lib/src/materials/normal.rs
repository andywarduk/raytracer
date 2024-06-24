use rand::rngs::ThreadRng;

use crate::{hits::hit::Hit, ray::Ray, triple::Colour};

use super::material::{Material, Scattered};

#[derive(Debug, Default, Clone)]
pub struct Normal {}

impl Normal {
    pub fn new() -> Self {
        Self {}
    }
}

impl Material for Normal {
    fn scatter(&self, _rng: &mut ThreadRng, _ray: &Ray, hit: &Hit) -> Scattered {
        let colour = 0.5 * (&hit.normal + Colour::new(1.0, 1.0, 1.0));

        (Some(colour), None, None)
    }
}
