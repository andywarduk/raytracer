use rand::rngs::ThreadRng;

use crate::{colour::Colour, hittable::Hit, ray::Ray, vec3::Vec3};

use super::material::Material;

#[derive(Debug)]
pub struct Lambertian {
    albedo: Colour,
}

impl Lambertian {
    pub fn new(albedo: Colour) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, rng: &mut ThreadRng, ray: &Ray, hit: &Hit) -> Option<(Colour, Option<Ray>)> {
        let mut scatter_direction = &hit.normal + Vec3::new_random_unit_vector(rng);

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = hit.normal.clone();
        }

        let scattered = Ray::new(hit.p.clone(), scatter_direction, ray.time());

        Some((self.albedo.clone(), Some(scattered)))
    }
}
