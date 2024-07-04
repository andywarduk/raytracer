//! Solid colour texture

use crate::{
    float::*,
    triple::{Colour, Point3},
};

use super::texture::Texture;

/// Solid colour details
#[derive(Debug)]
pub struct Solid {
    albedo: Colour,
}

impl Solid {
    /// Create a new solid colour texture
    pub fn new(albedo: Colour) -> Self {
        Self { albedo }
    }
}

impl Texture for Solid {
    fn value(&self, _u: Flt, _v: Flt, _p: &Point3) -> Colour {
        self.albedo.clone()
    }
}
