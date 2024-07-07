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
        let mut unit_direction = ray.direction().unit_vector();

        // Map -1..1 to 0..1
        unit_direction += flt(1.0);
        unit_direction *= flt(0.5);

        // Convert to colour
        Colour::new_from_array(unit_direction.e)
    }
}

impl Default for RayLight {
    fn default() -> Self {
        Self::new()
    }
}
