use crate::{
    float::*,
    triple::{Colour, Point3},
};

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
    fn value(&self, _u: Flt, _v: Flt, _p: &Point3) -> Colour {
        self.albedo.clone()
    }
}
