use rand::rngs::ThreadRng;

use crate::{
    hits::hit::Hit,
    ray::Ray,
    textures::{
        solid::Solid,
        texture::{TexRef, Texture},
    },
    triple::Colour,
};

use super::material::{Material, Scattered};

#[derive(Debug)]
pub struct DiffuseLight<'a> {
    texture: TexRef<'a>,
}

impl<'a> DiffuseLight<'a> {
    pub fn new_with_colour(albedo: Colour) -> Self {
        Self::new_with_texref(TexRef::boxed(Solid::new(albedo)))
    }

    pub fn new_with_texture(texture: &'a dyn Texture) -> Self {
        Self::new_with_texref(TexRef::Borrow(texture))
    }

    fn new_with_texref(texture: TexRef<'a>) -> Self {
        Self { texture }
    }
}

impl<'a> Material for DiffuseLight<'a> {
    fn scatter(&self, _rng: &mut ThreadRng, _ray: &Ray, hit: &Hit) -> Scattered {
        (None, Some(self.texture.value(hit.u, hit.v, &hit.p)), None)
    }
}
