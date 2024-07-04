//! Diffuse material

use rand::rngs::ThreadRng;

use crate::{
    hits::hit::Hit,
    ray::Ray,
    triple::{Colour, Vec3},
};

use super::material::{Material, Scattered};

/// Diffuse material details
#[derive(Debug)]
pub struct Diffuse {
    albedo: Colour,
}

impl Diffuse {
    /// Creates a new diffuse material with a given colour
    pub fn new(albedo: Colour) -> Self {
        Self { albedo }
    }
}

impl Material for Diffuse {
    fn scatter(&self, rng: &mut ThreadRng, ray: &Ray, hit: &Hit) -> Scattered {
        let direction = Vec3::new_random_on_hemisphere(rng, &hit.normal);

        let scattered = Ray::new(hit.p.clone(), direction, ray.time());

        (Some(self.albedo.clone()), None, Some(scattered))
    }
}
