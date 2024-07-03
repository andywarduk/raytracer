use crate::{
    float::*,
    perlin::PerlinNoise,
    triple::{Colour, Point3},
};

use super::texture::Texture;

#[derive(Debug)]
pub struct Perlin {
    scale: Flt,
    perlin: PerlinNoise,
}

impl Perlin {
    pub fn new(scale: FltPrim) -> Self {
        Self {
            scale: flt(scale),
            perlin: PerlinNoise::new(),
        }
    }
}

impl Texture for Perlin {
    fn value(&self, _u: Flt, _v: Flt, p: &Point3) -> Colour {
        // Convert from (-1..1) to (0..1)
        let noise = flt(0.5) * (flt(1.0) + self.perlin.noise(&(self.scale * p)));
        Colour::new_white() * noise
    }
}
