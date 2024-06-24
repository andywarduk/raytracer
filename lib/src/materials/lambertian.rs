use rand::rngs::ThreadRng;

use crate::{
    colour::Colour,
    hits::hit::Hit,
    ray::Ray,
    textures::{
        solid::Solid,
        texture::{TexRef, Texture},
    },
    vec3::Vec3,
};

use super::material::{Material, Scattered};

#[derive(Debug)]
pub struct Lambertian<'a> {
    texture: TexRef<'a>,
}

impl<'a> Lambertian<'a> {
    pub fn new_with_colour(albedo: Colour) -> Self {
        Self::new_with_texref(TexRef::boxed(Solid::new(albedo)))
    }

    pub fn new_with_texture(texture: &'a dyn Texture) -> Self {
        Self::new_with_texref(TexRef::Borrow(texture))
    }

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
            Some(self.texture.value(hit.u, hit.v, &hit.p)),
            None,
            Some(scattered),
        )
    }
}
