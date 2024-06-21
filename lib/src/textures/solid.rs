use crate::colour::Colour;

use super::texture::Texture;

#[derive(Debug)]
pub struct Solid {
    albedo: Colour,
}

impl Solid {
    pub fn new(albedo: Colour) -> Self {
        Self { albedo }
    }
}

impl Texture for Solid {
    fn value(&self, _u: f64, _v: f64, _p: &crate::vec3::Point3) -> crate::colour::Colour {
        self.albedo.clone()
    }
}
