use std::{ops::Range, sync::Arc};

use crate::{
    hits::{aabb::Aabb, hit::Hit, hittable::Hittable},
    ray::Ray,
    vec3::Vec3,
};

#[derive(Debug)]
pub struct Translate {
    offset: Vec3,
    object: Arc<dyn Hittable>,
    bbox: Aabb,
}

impl Translate {
    pub fn new(offset: Vec3, object: Arc<dyn Hittable>) -> Self {
        let bbox = object.bounding_box() + &offset;

        Self {
            offset,
            object,
            bbox,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, t_range: Range<f64>) -> Option<Hit> {
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
