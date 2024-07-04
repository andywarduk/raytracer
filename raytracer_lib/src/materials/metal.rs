//! Metal material

use rand::rngs::ThreadRng;

use crate::{
    float::*,
    hits::hit::Hit,
    ray::Ray,
    triple::{Colour, Vec3},
};

use super::material::{Material, Scattered};

/// Metal material details
#[derive(Debug)]
pub struct Metal {
    albedo: Colour,
    fuzz: Flt,
}

impl Metal {
    /// Creates a new metal material
    pub fn new(albedo: Colour, fuzz: FltPrim) -> Self {
        Self {
            albedo,
            fuzz: flt(fuzz.clamp(0.0, 1.0)),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, rng: &mut ThreadRng, ray: &Ray, hit: &Hit) -> Scattered {
        let mut reflected = ray.direction().reflect(&hit.normal);

        if self.fuzz != 0.0 {
            reflected = reflected.unit_vector() + (self.fuzz * Vec3::new_random_unit_vector(rng));
        }

        let scattered = Ray::new(hit.p.clone(), reflected, ray.time());

        if scattered.direction().dot(&hit.normal) > 0.0 {
            (Some(self.albedo.clone()), None, Some(scattered))
        } else {
            (None, None, None)
        }
    }
}
