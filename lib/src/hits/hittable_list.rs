use std::{mem, ops::Range};

use crate::{
    hits::{aabb::Aabb, hit::Hit, hittable::Hittable},
    ray::Ray,
};

use super::hittable::HittableRef;

#[derive(Debug, Default)]
pub struct HittableList<'a> {
    objects: Vec<HittableRef<'a>>,
    bbox: Option<Aabb>,
}

impl<'a> HittableList<'a> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: None,
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
        self.bbox = None;
    }

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

    pub fn length(&self) -> usize {
        self.objects.len()
    }

    pub fn into_objects(mut self) -> Vec<HittableRef<'a>> {
        let vec = mem::take(&mut self.objects);
        self.bbox = None;
        vec
    }
}

impl<'a> Hittable<'a> for HittableList<'a> {
    fn hit(&self, ray: &Ray, t_range: Range<f64>) -> Option<Hit> {
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
