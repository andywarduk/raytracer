use rand::rngs::ThreadRng;
use std::{fmt::Debug, ops::Deref};

use crate::{colour::Colour, hits::hit::Hit, ray::Ray};

pub type Scattered = (Option<Colour>, Option<Colour>, Option<Ray>);

pub trait Material: Debug + Send + Sync {
    fn scatter(&self, rng: &mut ThreadRng, ray: &Ray, hit: &Hit) -> Scattered;
}

#[derive(Debug)]
pub enum MatRef<'a> {
    Borrow(&'a dyn Material),
    Box(Box<dyn Material + 'a>),
}

impl<'a> MatRef<'a> {
    pub fn boxed(material: impl Material + 'a) -> Self {
        Self::Box(Box::new(material))
    }

    pub fn get_ref(&'a self) -> &dyn Material {
        match self {
            Self::Borrow(refer) => *refer,
            Self::Box(boxed) => boxed.as_ref(),
        }
    }
}

impl<'a> Deref for MatRef<'a> {
    type Target = dyn Material + 'a;

    fn deref(&self) -> &Self::Target {
        match self {
            MatRef::Borrow(refer) => *refer,
            MatRef::Box(boxed) => boxed.as_ref(),
        }
    }
}
