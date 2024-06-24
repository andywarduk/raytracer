use std::fmt::Debug;
use std::ops::{Deref, Range};

use crate::hits::aabb::Aabb;
use crate::ray::Ray;

use super::hit::Hit;

pub const T_MIN: f64 = 0.001;

pub trait Hittable<'a>: Debug + Send + Sync {
    fn hit(&self, ray: &Ray, t_range: Range<f64>) -> Option<Hit>;
    fn bounding_box(&self) -> &Aabb;
}

#[derive(Debug)]
pub enum HittableRef<'a> {
    Borrow(&'a dyn Hittable<'a>),
    Box(Box<dyn Hittable<'a> + 'a>),
}

impl<'a> HittableRef<'a> {
    pub fn boxed(hittable: impl Hittable<'a> + 'a) -> Self {
        Self::Box(Box::new(hittable))
    }

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
