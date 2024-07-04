//! Solid colour ambient light

use crate::{ray::Ray, triple::Colour};

use super::ambience::Ambience;

/// Solid colour ambient light properties
#[derive(Debug)]
pub struct AmbientLight {
    colour: Colour,
}

impl AmbientLight {
    /// Creates a new solid colour ambient light
    pub fn new(colour: Colour) -> Self {
        Self { colour }
    }
}

impl Ambience for AmbientLight {
    fn value(&self, _ray: &Ray) -> Colour {
        self.colour.clone()
    }
}
