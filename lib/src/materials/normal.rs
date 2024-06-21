use rand::rngs::ThreadRng;

use crate::{colour::Colour, hittable::Hit, ray::Ray};

use super::material::Material;

#[derive(Debug, Default, Clone)]
pub struct Normal {}

impl Normal {
    pub fn new() -> Self {
        Self {}
    }
}

impl Material for Normal {
    fn scatter(
        &self,
        _rng: &mut ThreadRng,
        _ray: &Ray,
        hit: &Hit,
    ) -> Option<(Colour, Option<Ray>)> {
        let colour = 0.5 * (&hit.normal + Colour::new(1.0, 1.0, 1.0));

        Some((colour, None))
    }
}
