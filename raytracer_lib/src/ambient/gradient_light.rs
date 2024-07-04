//! Gradient ambient light in the y axis

use crate::{float::*, ray::Ray, triple::Colour};

use super::ambience::Ambience;

/// Gradient light properties
#[derive(Debug)]
pub struct GradientLight {
    colour1: Colour,
    colour2: Colour,
}

impl GradientLight {
    /// Creates a new gradient ambient light
    pub fn new(colour1: Colour, colour2: Colour) -> Self {
        Self { colour1, colour2 }
    }
}

impl Ambience for GradientLight {
    fn value(&self, ray: &Ray) -> Colour {
        let unit_direction = ray.direction().unit_vector();

        // Convert y component from (-1..1) to (0..1)
        let a = flt(0.5) * (unit_direction.y() + 1.0);

        // Blend white with light blue
        (flt(1.0) - a) * &self.colour1 + a * &self.colour2
    }
}
