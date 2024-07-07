//! Materials

use rand::rngs::ThreadRng;
use std::{fmt::Debug, ops::Deref};

use crate::{
    float::*,
    hits::hit::Hit,
    ray::Ray,
    triple::{Colour, Point3},
};

/// Scattered light details
pub type Scattered = (Colour, Option<Colour>, Option<Ray>);

/// Material trait
pub trait Material: Debug + Send + Sync {
    /// Tests material for a hit
    fn hit(&self, _rng: &mut ThreadRng, _u: Flt, _v: Flt, _p: &Point3) -> bool {
        true
    }

    /// Returns details of scattered light
    fn scatter(&self, rng: &mut ThreadRng, ray: &Ray, hit: &Hit) -> Scattered;
}

/// A material reference, borrowed or owned
#[derive(Debug)]
pub enum MatRef<'a> {
    /// A borrowed material
    Borrow(&'a dyn Material),
    /// An owned material
    Box(Box<dyn Material + 'a>),
}

impl<'a> MatRef<'a> {
    /// Create a new owned material reference
    pub fn boxed(material: impl Material + 'a) -> Self {
        Self::Box(Box::new(material))
    }

    /// Gets a reference to the material
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
