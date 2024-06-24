use crate::{
    perlin::PerlinNoise,
    triple::{Colour, Point3},
};

use super::texture::Texture;

#[derive(Debug)]
pub struct Turbulence {
    scale: f64,
    depth: usize,
    perlin: PerlinNoise,
}

impl Turbulence {
    pub fn new(scale: f64, depth: usize) -> Self {
        Self {
            scale,
            depth,
            perlin: PerlinNoise::new(),
        }
    }
}

impl Texture for Turbulence {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Colour {
        Colour::new(1.0, 1.0, 1.0) * self.perlin.turbulence(&(self.scale * p), self.depth)
    }
}
