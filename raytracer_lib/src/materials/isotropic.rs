//! Isotropic material (random scattered light)

use rand::rngs::ThreadRng;

use crate::{
    hits::hit::Hit,
    ray::Ray,
    textures::{
        solid::Solid,
        texture::{TexRef, Texture},
    },
    triple::{Colour, Vec3},
};

use super::material::{Material, Scattered};

/// Isotropic material details
#[derive(Debug)]
pub struct Isotropic<'a> {
    texture: TexRef<'a>,
}

impl<'a> Isotropic<'a> {
    /// Create new isotropic materal with a given colour
    pub fn new_with_colour(albedo: Colour) -> Self {
        Self::new_with_texref(TexRef::boxed(Solid::new(albedo)))
    }

    /// Create new isotropic materal with a given texture
    pub fn new_with_texture(texture: &'a dyn Texture) -> Self {
        Self::new_with_texref(TexRef::Borrow(texture))
    }

    /// Create new isotropic materal with a given texture reference
    pub fn new_with_texref(texture: TexRef<'a>) -> Self {
        Self { texture }
    }
}

impl<'a> Material for Isotropic<'a> {
    fn scatter(&self, rng: &mut ThreadRng, ray: &Ray, hit: &Hit) -> Scattered {
        let scattered = Ray::new(hit.p.clone(), Vec3::new_random_unit_vector(rng), ray.time());

        (
            self.texture.value(hit.u, hit.v, &hit.p),
            None,
            Some(scattered),
        )
    }
}
