//! Make an object invisible for a given number of ray bounces

use std::ops::Range;

use crate::{
    float::*,
    hits::{
        aabb::Aabb,
        hit::Hit,
        hittable::{Hittable, HittableRef},
    },
    ray::Ray,
};

/// Invisibility details
#[derive(Debug)]
pub struct InvisibleFor<'a> {
    bounces: u64,
    object: HittableRef<'a>,
}

impl<'a> InvisibleFor<'a> {
    /// Creates a new invisibility object
    pub fn new(bounces: u64, object: impl Hittable<'a> + 'a) -> Self {
        Self {
            bounces,
            object: HittableRef::boxed(object),
        }
    }
}

impl<'a> Hittable<'a> for InvisibleFor<'a> {
    fn hit(&self, ray: &Ray, t_range: Range<Flt>) -> Option<Hit> {
        if ray.depth() < self.bounces {
            None
        } else {
            self.object.hit(ray, t_range)
        }
    }

    fn bounding_box(&self) -> &Aabb {
        self.object.bounding_box()
    }
}
