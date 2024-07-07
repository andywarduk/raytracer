//! Perlin noise texture

use crate::{
    float::*,
    perlin::PerlinNoise,
    triple::{Colour, Point3},
};

use super::texture::Texture;

/// Perlin noise details
#[derive(Debug)]
pub struct Perlin {
    scale: Flt,
    perlin: PerlinNoise,
}

impl Perlin {
    /// Creates a new perlin noise texture
    pub fn new(scale: FltPrim) -> Self {
        Self {
            scale: flt(scale),
            perlin: PerlinNoise::new(),
        }
    }
}

impl Texture for Perlin {
    fn value(&self, _u: Flt, _v: Flt, p: &Point3) -> Colour {
        // Get noise value between -1 and 1
        let mut noise = self.perlin.noise(&(self.scale * p));

        // Convert from (-1..1) to (0..1)
        noise += flt(1.0);
        noise *= flt(0.5);

        // Return colour (greyscale)
        Colour::new_grey(flt_prim(noise))
    }
}
