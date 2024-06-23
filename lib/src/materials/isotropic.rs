use std::sync::Arc;

use rand::rngs::ThreadRng;

use crate::{
    colour::Colour,
    hits::hit::Hit,
    ray::Ray,
    textures::{solid::Solid, texture::Texture},
    vec3::Vec3,
};

use super::material::{Material, Scattered};

#[derive(Debug)]
pub struct Isotropic {
    texture: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new_with_colour(albedo: Colour) -> Self {
        Self::new_with_texture(Arc::new(Solid::new(albedo)))
    }

    pub fn new_with_texture(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }
}

impl Material for Isotropic {
    fn scatter(&self, rng: &mut ThreadRng, ray: &Ray, hit: &Hit) -> Scattered {
        let scattered = Ray::new(hit.p.clone(), Vec3::new_random_unit_vector(rng), ray.time());

        (
            Some(self.texture.value(hit.u, hit.v, &hit.p)),
            None,
            Some(scattered),
        )
    }
}
