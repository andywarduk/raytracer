use crate::{ray::Ray, triple::Colour};
use std::fmt::Debug;

pub trait Ambience: Debug + Sync + Send {
    fn value(&self, ray: &Ray) -> Colour;
}
