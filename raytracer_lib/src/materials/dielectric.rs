use rand::{rngs::ThreadRng, Rng};

use crate::{float::*, hits::hit::Hit, ray::Ray, triple::Colour};

use super::material::{Material, Scattered};

#[derive(Debug)]
pub struct Dielectric {
    refraction_index: Flt,
    inv_refraction_index: Flt,
    r0_sq: Flt,
    inv_r0_sq: Flt,
}

impl Dielectric {
    pub fn new(refraction_index: FltPrim) -> Self {
        let inv_refraction_index = flt(1.0) / refraction_index;

        let r0 = (flt(1.0) - refraction_index) / (flt(1.0) + refraction_index);
        let r0_sq = r0 * r0;

        let inv_r0 = (flt(1.0) - inv_refraction_index) / (flt(1.0) + inv_refraction_index);
        let inv_r0_sq = inv_r0 * inv_r0;

        Self {
            refraction_index: flt(refraction_index),
            inv_refraction_index,
            r0_sq,
            inv_r0_sq,
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
            (self.inv_refraction_index, self.inv_r0_sq)
        } else {
            (self.refraction_index, self.r0_sq)
        };

        let unit_direction = ray.direction().unit_vector();

        let cos_theta = -unit_direction.dot(&hit.normal).min(flt(1.0));
        let sin_theta = (flt(1.0) - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;

        let direction =
            if cannot_refract || self.reflectance(cos_theta, r0_sq) > rng.gen_range(0.0..1.0) {
                unit_direction.reflect(&hit.normal)
            } else {
                unit_direction.refract(&hit.normal, ri)
            };

        let scattered = Ray::new(hit.p.clone(), direction, ray.time());

        (Some(Colour::new_white()), None, Some(scattered))
    }
}
