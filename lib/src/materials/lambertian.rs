use std::sync::Arc;

use rand::rngs::ThreadRng;

use crate::{colour::Colour, hittable::Hit, ray::Ray, textures::{solid::Solid, texture::Texture}, vec3::Vec3};

use super::material::Material;

#[derive(Debug)]
pub struct Lambertian {
    texture: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new_with_colour(albedo: Colour) -> Self {
        Self::new_with_texture(Arc::new(Solid::new(albedo)))
    }

    pub fn new_with_texture(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
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

        Some((self.texture.value(hit.u, hit.v, &hit.p), Some(scattered)))
    }
}
