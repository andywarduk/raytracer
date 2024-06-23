use rand::rngs::ThreadRng;

use crate::{colour::Colour, hits::hit::Hit, ray::Ray, vec3::Vec3};

use super::material::{Material, Scattered};

#[derive(Debug)]
pub struct Metal {
    albedo: Colour,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Colour, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, rng: &mut ThreadRng, ray: &Ray, hit: &Hit) -> Scattered {
        let mut reflected = ray.direction().reflect(&hit.normal);
        reflected = reflected.unit_vector() + (self.fuzz * Vec3::new_random_unit_vector(rng));

        let scattered = Ray::new(hit.p.clone(), reflected, ray.time());

        if scattered.direction().dot(&hit.normal) > 0.0 {
            (Some(self.albedo.clone()), None, Some(scattered))
        } else {
            (None, None, None)
        }
    }
}
