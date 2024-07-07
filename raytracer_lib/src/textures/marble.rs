//! Marble texture

use crate::{
    float::*,
    perlin::PerlinNoise,
    triple::{Colour, Point3},
};

use super::texture::Texture;

/// Marble texture details
#[derive(Debug)]
pub struct Marble {
    scale: Flt,
    depth: usize,
    axis: usize,
    perlin: PerlinNoise,
}

impl Marble {
    /// Create a new marble texture
    pub fn new(scale: FltPrim, depth: usize, axis: usize) -> Self {
        Self {
            scale: flt(scale),
            depth,
            axis,
            perlin: PerlinNoise::new(),
        }
    }
}

impl Texture for Marble {
    fn value(&self, _u: Flt, _v: Flt, p: &Point3) -> Colour {
        // Get noise value
        let noise = self.perlin.turbulence(p, self.depth);

        // Calculate grey level between -1 and 1
        let mut level = (self.scale * p[self.axis] + flt(10.0) * noise).sin();

        // Convert to 0..1
        level += flt(1.0);
        level *= flt(0.5);

        // Return grey level
        Colour::new_grey(flt_prim(level))
    }
}
