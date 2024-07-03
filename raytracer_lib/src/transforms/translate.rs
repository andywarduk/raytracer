use std::ops::Range;

use crate::{
    float::*,
    hits::{
        aabb::Aabb,
        hit::Hit,
        hittable::{Hittable, HittableRef},
    },
    ray::Ray,
    triple::Vec3,
};

#[derive(Debug)]
pub struct Translate<'a> {
    offset: Vec3,
    object: HittableRef<'a>,
    bbox: Aabb,
}

impl<'a> Translate<'a> {
    pub fn new(offset: Vec3, object: impl Hittable<'a> + 'a) -> Self {
        let bbox = object.bounding_box() + &offset;

        Self {
            offset,
            object: HittableRef::boxed(object),
            bbox,
        }
    }
}

impl<'a> Hittable<'a> for Translate<'a> {
    fn hit(&self, ray: &Ray, t_range: Range<Flt>) -> Option<Hit> {
        // Move the ray backwards by the offset
        let offset_ray = Ray::new(
            ray.origin() - &self.offset,
            ray.direction().clone(),
            ray.time(),
        );

        // Determine whether an intersection exists along the offset ray (and if so, where)
        match self.object.hit(&offset_ray, t_range) {
            None => None,
            Some(mut hit) => {
                // Move the intersection point forwards by the offset
                hit.p += &self.offset;
                Some(hit)
            }
        }
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
