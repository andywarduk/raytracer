use crate::{colour::Colour, vec3::Point3};
use std::fmt::Debug;

pub trait Texture: Debug + Sync + Send {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Colour;
}
