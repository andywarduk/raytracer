use crate::{colour::Colour, perlin::PerlinNoise, vec3::Point3};

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
        Colour::new(1.0,1.0,1.0) * 0.5 * (1.0 + self.perlin.noise(&(self.scale * p)))
    }
}
