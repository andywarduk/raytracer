use std::{mem, ops::Range};

use crate::{hittable::aabb::Aabb, hittable::hit::Hit, hittable::hittable::Hittable, ray::Ray};

#[derive(Debug, Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
    bbox: Aabb,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: Aabb::default(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
        self.bbox = Aabb::default();
    }

    pub fn add<T>(&mut self, hittable: T)
    where
        T: Hittable + 'static,
    {
        self.bbox = Aabb::new_from_bbox(&self.bbox, hittable.bounding_box());
        self.objects.push(Box::new(hittable));
    }

    pub fn into_objects(&mut self) -> Vec<Box<dyn Hittable>> {
        let vec = mem::take(&mut self.objects);
        self.bbox = Aabb::default();
        vec
    }
}

impl Hittable for HittableList {
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
        &self.bbox
    }
}
