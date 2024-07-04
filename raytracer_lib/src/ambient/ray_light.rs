//! Ambient light equal to the ray unit vector

use crate::{float::*, ray::Ray, triple::Colour};

use super::ambience::Ambience;

/// Ray light class
#[derive(Debug)]
pub struct RayLight {}

impl RayLight {
    /// Create a new ray unit vector light
    pub fn new() -> Self {
        Self {}
    }
}

impl Ambience for RayLight {
    fn value(&self, ray: &Ray) -> Colour {
        // Get ray unit vector
        let unit_direction = ray.direction().unit_vector();

        // Convert to colour
        let col = Colour::new_flt(unit_direction.x(), unit_direction.y(), unit_direction.z());

        // Map -1..1 to 0..1
        (col + Colour::new_white()) / flt(2.0)
    }
}

impl Default for RayLight {
    fn default() -> Self {
        Self::new()
    }
}
