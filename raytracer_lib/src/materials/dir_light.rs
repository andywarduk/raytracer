//! Diffuse light material

use rand::rngs::ThreadRng;

use crate::{
    float::*,
    hits::hit::Hit,
    ray::Ray,
    textures::{
        solid::Solid,
        texture::{TexRef, Texture},
    },
    triple::{Colour, Point3},
};

use super::material::{Material, Scattered};

/// Directional light details
#[derive(Debug)]
pub struct DirLight<'a> {
    texture: TexRef<'a>,
}

impl<'a> DirLight<'a> {
    /// Create a new directional light with a given colour
    pub fn new_with_colour(albedo: Colour) -> Self {
        Self::new_with_texref(TexRef::boxed(Solid::new(albedo)))
    }

    /// Create a new directional light with a given texture
    pub fn new_with_texture(texture: &'a dyn Texture) -> Self {
        Self::new_with_texref(TexRef::Borrow(texture))
    }

    fn new_with_texref(texture: TexRef<'a>) -> Self {
        Self { texture }
    }
}

impl<'a> Material for DirLight<'a> {
    fn hit(&self, rng: &mut ThreadRng, u: Flt, v: Flt, p: &Point3) -> bool {
        self.texture.hit(rng, u, v, p)
    }

    fn scatter(&self, _rng: &mut ThreadRng, ray: &Ray, hit: &Hit) -> Scattered {
        let factor = ray.direction().unit_vector().dot(&hit.normal).abs();

        let colour = self.texture.value(hit.u, hit.v, &hit.p) * factor;

        (Colour::default(), Some(colour), None)
    }
}
