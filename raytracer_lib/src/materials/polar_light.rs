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
pub struct PolarLight<'a> {
    cos_angle: Flt,
    texture: TexRef<'a>,
}

impl<'a> PolarLight<'a> {
    /// Create a new directional light with a given colour
    pub fn new_with_colour(angle: FltPrim, albedo: Colour) -> Self {
        Self::new_with_texref(angle, TexRef::boxed(Solid::new(albedo)))
    }

    /// Create a new directional light with a given texture
    pub fn new_with_texture(angle: FltPrim, texture: &'a dyn Texture) -> Self {
        Self::new_with_texref(angle, TexRef::Borrow(texture))
    }

    fn new_with_texref(angle: FltPrim, texture: TexRef<'a>) -> Self {
        Self {
            cos_angle: flt(angle).to_radians().cos(),
            texture,
        }
    }
}

impl<'a> Material for PolarLight<'a> {
    fn hit(&self, rng: &mut ThreadRng, u: Flt, v: Flt, p: &Point3) -> bool {
        self.texture.hit(rng, u, v, p)
    }

    fn scatter(&self, _rng: &mut ThreadRng, ray: &Ray, hit: &Hit) -> Scattered {
        let angle_in = ray.direction().unit_vector().dot(&hit.normal).abs();

        let colour = if angle_in >= self.cos_angle {
            self.texture.value(hit.u, hit.v, &hit.p)
        } else {
            Colour::default()
        };

        (Colour::default(), Some(colour), None)
    }
}
