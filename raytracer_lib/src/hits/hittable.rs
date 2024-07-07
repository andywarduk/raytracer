//! Hittable trait

use std::{
    fmt::Debug,
    ops::{Deref, Range},
};

use rand::rngs::ThreadRng;

use crate::{float::*, hits::aabb::Aabb, ray::Ray};

use super::hit::Hit;

/// Ignore hits closer than distance
pub const T_MIN: FltPrim = 0.00001;
// TODO this is better at 0.001 for f32

/// Hittable object trait
pub trait Hittable<'a>: Debug + Send + Sync {
    /// Tests whether the object intersects a given ray
    fn hit(&self, rng: &mut ThreadRng, ray: &Ray, t_range: Range<Flt>) -> Option<Hit>;

    /// Returns the bounding box of the object
    fn bounding_box(&self) -> &Aabb;
}

/// Reference to a hittable object, either borrowed or owned
#[derive(Debug)]
pub enum HittableRef<'a> {
    /// A borrowed object
    Borrow(&'a dyn Hittable<'a>),
    /// An owned object
    Box(Box<dyn Hittable<'a> + 'a>),
}

impl<'a> HittableRef<'a> {
    /// Creates a new owned hittable reference
    pub fn boxed(hittable: impl Hittable<'a> + 'a) -> Self {
        Self::Box(Box::new(hittable))
    }

    /// Gets a reference to the hittable
    pub fn get_ref(&'a self) -> &dyn Hittable {
        match self {
            Self::Borrow(refer) => *refer,
            Self::Box(boxed) => boxed.as_ref(),
        }
    }
}

impl<'a> Deref for HittableRef<'a> {
    type Target = dyn Hittable<'a> + 'a;

    fn deref(&self) -> &Self::Target {
        match self {
            HittableRef::Borrow(refer) => *refer,
            HittableRef::Box(boxed) => boxed.as_ref(),
        }
    }
}
