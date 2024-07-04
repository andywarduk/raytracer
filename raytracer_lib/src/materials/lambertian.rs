//! Lambertian material

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

/// Lambertian material details
#[derive(Debug)]
pub struct Lambertian<'a> {
    texture: TexRef<'a>,
}

impl<'a> Lambertian<'a> {
    /// Create a new lambertian material with a given colour
    pub fn new_with_colour(albedo: Colour) -> Self {
        Self::new_with_texref(TexRef::boxed(Solid::new(albedo)))
    }

    /// Create a new lambertian material with a given texture
    pub fn new_with_texture(texture: &'a dyn Texture) -> Self {
        Self::new_with_texref(TexRef::Borrow(texture))
    }

    /// Create a new lambertian material with a given texture reference
    pub fn new_with_texref(texture: TexRef<'a>) -> Self {
        Self { texture }
    }
}

impl<'a> Material for Lambertian<'a> {
    fn scatter(&self, rng: &mut ThreadRng, ray: &Ray, hit: &Hit) -> Scattered {
        let mut scatter_direction = &hit.normal + Vec3::new_random_unit_vector(rng);

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = hit.normal.clone();
        }

        let scattered = Ray::new(hit.p.clone(), scatter_direction, ray.time());

        (
            self.texture.value(hit.u, hit.v, &hit.p),
            None,
            Some(scattered),
        )
    }
}
