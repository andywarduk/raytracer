use std::ops::Range;

use crate::{
    hits::{
        aabb::Aabb,
        hit::Hit,
        hittable::{Hittable, HittableRef},
    },
    ray::Ray,
};

#[derive(Debug)]
pub struct InvisibleFor<'a> {
    bounces: u64,
    object: HittableRef<'a>,
}

impl<'a> InvisibleFor<'a> {
    pub fn new(bounces: u64, object: impl Hittable<'a> + 'a) -> Self {
        Self {
            bounces,
            object: HittableRef::boxed(object),
        }
    }
}

impl<'a> Hittable<'a> for InvisibleFor<'a> {
    fn hit(&self, ray: &Ray, t_range: Range<f64>) -> Option<Hit> {
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
