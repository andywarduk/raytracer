//! List of hittable objects

use std::{mem, ops::Range};

use crate::{
    float::*,
    hits::{aabb::Aabb, hit::Hit, hittable::Hittable},
    ray::Ray,
};

use super::hittable::HittableRef;

/// Hittable object list
#[derive(Debug, Default)]
pub struct HittableList<'a> {
    objects: Vec<HittableRef<'a>>,
    bbox: Option<Aabb>,
}

impl<'a> HittableList<'a> {
    /// Create a new empty hittable list
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: None,
        }
    }

    /// Clears the hittable list
    pub fn clear(&mut self) {
        self.objects.clear();
        self.bbox = None;
    }

    /// Adds an object to the hittable list
    pub fn add<T>(&mut self, hittable: T)
    where
        T: Hittable<'a> + 'a,
    {
        self.bbox = Some(if let Some(bbox) = &self.bbox {
            Aabb::new_from_bbox(bbox, hittable.bounding_box())
        } else {
            hittable.bounding_box().clone()
        });

        self.objects.push(HittableRef::boxed(hittable));
    }

    /// Returns the number of objects in the hittable list
    pub fn length(&self) -> usize {
        self.objects.len()
    }

    /// Converts the hittable list to a vector
    pub fn into_objects(mut self) -> Vec<HittableRef<'a>> {
        let vec = mem::take(&mut self.objects);
        self.bbox = None;
        vec
    }
}

impl<'a> Hittable<'a> for HittableList<'a> {
    fn hit(&self, ray: &Ray, t_range: Range<Flt>) -> Option<Hit> {
        let mut closest = t_range.end;
        let mut closest_hit = None;

        for obj in &self.objects {
            match obj.hit(ray, t_range.start..closest) {
                None => (),
                Some(hit) => {
                    closest = hit.t;
                    closest_hit = Some(hit);
                }
            }
        }

        closest_hit
    }

    fn bounding_box(&self) -> &Aabb {
        self.bbox.as_ref().expect("No objects in hittable list")
    }
}
