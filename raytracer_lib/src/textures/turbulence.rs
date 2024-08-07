//! Turbulence texture

use crate::{
    float::*,
    perlin::PerlinNoise,
    triple::{Colour, Point3},
};

use super::texture::Texture;

/// Turbulence details
#[derive(Debug)]
pub struct Turbulence {
    scale: Flt,
    depth: usize,
    perlin: PerlinNoise,
}

impl Turbulence {
    /// Creates a new turbulence texture
    pub fn new(scale: FltPrim, depth: usize) -> Self {
        Self {
            scale: flt(scale),
            depth,
            perlin: PerlinNoise::new(),
        }
    }
}

impl Texture for Turbulence {
    fn value(&self, _u: Flt, _v: Flt, p: &Point3) -> Colour {
        // Get noise value
        let noise = self.perlin.turbulence(&(self.scale * p), self.depth);

        // Return colour (greyscale)
        Colour::new_grey(flt_prim(noise))
    }
}
