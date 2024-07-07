//! Dielectric material

use rand::{rngs::ThreadRng, Rng};

use crate::{float::*, hits::hit::Hit, ray::Ray, triple::Colour};

use super::material::{Material, Scattered};

/// Dielectric material details
#[derive(Debug)]
pub struct Dielectric {
    refraction_index: Flt,
    inv_refraction_index: Flt,
    r0_sq: Flt,
    inv_r0_sq: Flt,
}

impl Dielectric {
    /// Create a new dielectric with a given refractive index
    pub fn new(refraction_index: FltPrim) -> Self {
        let refraction_index = flt(refraction_index);
        let inv_refraction_index = refraction_index.recip();

        let r0_sq = |v| -> Flt {
            let r0 = (flt(1.0) - v) / (flt(1.0) + v);
            r0 * r0
        };

        Self {
            refraction_index,
            inv_refraction_index,
            r0_sq: r0_sq(refraction_index),
            inv_r0_sq: r0_sq(inv_refraction_index),
        }
    }

    fn reflectance(&self, cosine: Flt, r0_sq: Flt) -> Flt {
        // Use Schlick's approximation for reflectance.
        r0_sq + ((flt(1.0) - r0_sq) * (flt(1.0) - cosine).powf(flt(5.0)))
    }
}

impl Material for Dielectric {
    fn scatter(&self, rng: &mut ThreadRng, ray: &Ray, hit: &Hit) -> Scattered {
        let (ri, r0_sq) = if hit.front_face {
            // Front face hit
            (self.inv_refraction_index, self.inv_r0_sq)
        } else {
            // Back face hit
            (self.refraction_index, self.r0_sq)
        };

        let unit_direction = ray.direction().unit_vector();

        let cos_theta = -unit_direction.dot(&hit.normal).min(flt(1.0));
        let sin_theta = (flt(1.0) - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;

        let direction =
            if cannot_refract || self.reflectance(cos_theta, r0_sq) > rng.gen_range(0.0..1.0) {
                // Ray is reflected
                unit_direction.reflect(&hit.normal)
            } else {
                // Ray is refracted
                unit_direction.refract(&hit.normal, ri)
            };

        let scattered = Ray::new(hit.p.clone(), direction, ray.time());

        (Colour::new_white(), None, Some(scattered))
    }
}
