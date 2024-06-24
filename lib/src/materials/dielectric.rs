use rand::{rngs::ThreadRng, Rng};

use crate::{hits::hit::Hit, ray::Ray, triple::Colour};

use super::material::{Material, Scattered};

#[derive(Debug)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + ((1.0 - r0) * (1.0 - cosine).powf(5.0))
    }
}

impl Material for Dielectric {
    fn scatter(&self, rng: &mut ThreadRng, ray: &Ray, hit: &Hit) -> Scattered {
        let ri = if hit.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray.direction().unit_vector();

        let cos_theta = -unit_direction.dot(&hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;

        let direction =
            if cannot_refract || Self::reflectance(cos_theta, ri) > rng.gen_range(0.0..1.0) {
                unit_direction.reflect(&hit.normal)
            } else {
                unit_direction.refract(&hit.normal, ri)
            };

        let scattered = Ray::new(hit.p.clone(), direction, ray.time());

        (Some(Colour::new(1.0, 1.0, 1.0)), None, Some(scattered))
    }
}
