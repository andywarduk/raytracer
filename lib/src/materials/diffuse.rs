use rand::rngs::ThreadRng;

use crate::{colour::Colour, hits::hit::Hit, ray::Ray, vec3::Vec3};

use super::material::{Material, Scattered};

#[derive(Debug)]
pub struct Diffuse {
    albedo: Colour,
}

impl Diffuse {
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
