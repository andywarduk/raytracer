use crate::{
    float::*,
    perlin::PerlinNoise,
    triple::{Colour, Point3},
};

use super::texture::Texture;

#[derive(Debug)]
pub struct Turbulence {
    scale: Flt,
    depth: usize,
    perlin: PerlinNoise,
}

impl Turbulence {
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
        Colour::new_white() * self.perlin.turbulence(&(self.scale * p), self.depth)
    }
}
