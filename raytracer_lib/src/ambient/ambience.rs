//! Ambient lighting trait

use crate::{ray::Ray, triple::Colour};
use std::fmt::Debug;

/// Ambient light trait
pub trait Ambience: Debug + Sync + Send {
    /// Returns the colour of the ambient light
    fn value(&self, ray: &Ray) -> Colour;
}
