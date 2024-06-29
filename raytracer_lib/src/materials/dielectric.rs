use rand::{rngs::ThreadRng, Rng};

use crate::{hits::hit::Hit, ray::Ray, triple::Colour};

use super::material::{Material, Scattered};

#[derive(Debug)]
pub struct Dielectric {
    refraction_index: f64,
    inv_refraction_index: f64,
    r0_sq: f64,
    inv_r0_sq: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        let inv_refraction_index = 1.0 / refraction_index;

        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0_sq = r0 * r0;

        let inv_r0 = (1.0 - inv_refraction_index) / (1.0 + inv_refraction_index);
        let inv_r0_sq = inv_r0 * inv_r0;

        Self {
            refraction_index,
            inv_refraction_index,
            r0_sq,
            inv_r0_sq,
        }
    }

    fn reflectance(&self, cosine: f64, r0_sq: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        r0_sq + ((1.0 - r0_sq) * (1.0 - cosine).powf(5.0))
    }
}

impl Material for Dielectric {
    fn scatter(&self, rng: &mut ThreadRng, ray: &Ray, hit: &Hit) -> Scattered {
        let (ri, r0_sq) = if hit.front_face {
            (self.inv_refraction_index, self.inv_r0_sq)
        } else {
            (self.refraction_index, self.r0_sq)
        };

        let unit_direction = ray.direction().unit_vector();

        let cos_theta = -unit_direction.dot(&hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;

        let direction =
            if cannot_refract || self.reflectance(cos_theta, r0_sq) > rng.gen_range(0.0..1.0) {
                unit_direction.reflect(&hit.normal)
            } else {
                unit_direction.refract(&hit.normal, ri)
            };

        let scattered = Ray::new(hit.p.clone(), direction, ray.time());

        (Some(Colour::new(1.0, 1.0, 1.0)), None, Some(scattered))
    }
}
