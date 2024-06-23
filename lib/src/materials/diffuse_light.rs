use std::sync::Arc;

use rand::rngs::ThreadRng;

use crate::{
    colour::Colour,
    hits::hit::Hit,
    ray::Ray,
    textures::{solid::Solid, texture::Texture},
};

use super::material::{Material, Scattered};

#[derive(Debug)]
pub struct DiffuseLight {
    texture: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new_with_colour(albedo: Colour) -> Self {
        Self::new_with_texture(Arc::new(Solid::new(albedo)))
    }

    pub fn new_with_texture(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _rng: &mut ThreadRng, _ray: &Ray, hit: &Hit) -> Scattered {
        (None, Some(self.texture.value(hit.u, hit.v, &hit.p)), None)
    }
}
