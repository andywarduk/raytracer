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
        Colour::new_grey(0.5)
            * (flt(1.0)
                + (self.scale * p.e[self.axis] + flt(10.0) * self.perlin.turbulence(p, self.depth))
                    .sin())
    }
}
