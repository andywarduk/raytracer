use crate::{colour::Colour, ray::Ray};
use std::fmt::Debug;

pub trait Ambience: Debug + Sync + Send {
    fn value(&self, ray: &Ray) -> Colour;
}
