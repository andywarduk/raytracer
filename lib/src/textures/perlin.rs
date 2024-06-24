use crate::{
    perlin::PerlinNoise,
    triple::{Colour, Point3},
};

use super::texture::Texture;

#[derive(Debug)]
pub struct Perlin {
    scale: f64,
    perlin: PerlinNoise,
}

impl Perlin {
    pub fn new(scale: f64) -> Self {
        Self {
            scale,
            perlin: PerlinNoise::new(),
        }
    }
}

impl Texture for Perlin {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Colour {
        // Convert from (-1..1) to (0..1)
        let noise = 0.5 * (1.0 + self.perlin.noise(&(self.scale * p)));
        Colour::new(1.0, 1.0, 1.0) * noise
    }
}
