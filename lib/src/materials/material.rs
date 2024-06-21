use rand::rngs::ThreadRng;
use std::fmt::Debug;

use crate::{colour::Colour, hittable::Hit, ray::Ray};

pub trait Material: Debug + Send + Sync {
    fn scatter(&self, rng: &mut ThreadRng, ray: &Ray, hit: &Hit) -> Option<(Colour, Option<Ray>)>;
}
