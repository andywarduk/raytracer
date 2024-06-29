use crate::{ray::Ray, triple::Colour};

use super::ambience::Ambience;

#[derive(Debug)]
pub struct AmbientLight {
    colour: Colour,
}

impl AmbientLight {
    pub fn new(colour: Colour) -> Self {
        Self { colour }
    }
}

impl Ambience for AmbientLight {
    fn value(&self, _ray: &Ray) -> Colour {
        self.colour.clone()
    }
}
