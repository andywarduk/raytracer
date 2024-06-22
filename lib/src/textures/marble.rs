use crate::{colour::Colour, perlin::PerlinNoise, vec3::Point3};

use super::texture::Texture;

#[derive(Debug)]
pub struct Marble {
    scale: f64,
    depth: usize,
    axis: usize,
    perlin: PerlinNoise,
}

impl Marble {
    pub fn new(scale: f64, depth: usize, axis: usize) -> Self {
        Self {
            scale,
            depth,
            axis,
            perlin: PerlinNoise::new(),
        }
    }
}

impl Texture for Marble {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Colour {
        Colour::new(0.5, 0.5, 0.5)
            * (1.0
                + (self.scale * p.e[self.axis] + 10.0 * self.perlin.turbulence(p, self.depth))
                    .sin())
    }
}
