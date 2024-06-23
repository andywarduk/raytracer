use rand::rngs::ThreadRng;
use std::fmt::Debug;

use crate::{colour::Colour, hits::hit::Hit, ray::Ray};

pub type Scattered = (Option<Colour>, Option<Colour>, Option<Ray>);

pub trait Material: Debug + Send + Sync {
    fn scatter(&self, rng: &mut ThreadRng, ray: &Ray, hit: &Hit) -> Scattered;
}
