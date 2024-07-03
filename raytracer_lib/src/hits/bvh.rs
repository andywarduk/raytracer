use std::{cmp::Ordering, ops::Range};

use crate::{
    float::*,
    hits::{aabb::Aabb, hit::Hit, hittable::Hittable},
    ray::Ray,
};

use super::{hittable::HittableRef, hittable_list::HittableList};

#[derive(Debug)]
pub struct BvhNode<'a> {
    bbox: Aabb,
    left: HittableRef<'a>,
    right: Option<HittableRef<'a>>,
}

impl<'a> BvhNode<'a> {
    pub fn new(hittable_list: HittableList<'a>) -> Self {
        let objects = hittable_list.into_objects();

        Self::new_from_vec(objects)
    }

    fn new_from_vec(mut objects: Vec<HittableRef<'a>>) -> Self {
        // Create bounding box for the object array
        let bbox = objects
            .iter()
            .map(|o| o.bounding_box())
            .fold(None, |bbox, next| {
                if let Some(bbox) = bbox {
                    Some(Aabb::new_from_bbox(&bbox, next))
                } else {
                    Some(next.clone())
                }
            })
            .expect("No objects for BvhNode");

        // Calculate longest axis
        let axis = bbox.longest_axis();

        let vec_len = objects.len();

        let (left, right) = match vec_len {
            0 => panic!("Zero length hittable vec"),
            1 => {
                let object = objects.pop().unwrap();
                (object, None)
            }
            2 => {
                let object1 = objects.pop().unwrap();
                let object0 = objects.pop().unwrap();
                (object0, Some(object1))
            }
            _ => {
                // Sort objects in to order by start point on the chosen axis
                objects.sort_by(|a, b| Self::box_compare(&**a, &**b, axis));

                // Select mid point
                let mid = vec_len / 2;

                // Split the vector
                let split = objects.split_off(mid);

                (
                    HittableRef::boxed(BvhNode::new_from_vec(objects)),
                    Some(HittableRef::boxed(BvhNode::new_from_vec(split))),
                )
            }
        };

        Self { bbox, left, right }
    }

    fn box_compare(a: &dyn Hittable<'a>, b: &dyn Hittable<'a>, axis: usize) -> Ordering {
        let a_axis_interval = &a.bounding_box().ranges[axis];
        let b_axis_interval = &b.bounding_box().ranges[axis];

        a_axis_interval
            .start
            .partial_cmp(&b_axis_interval.start)
            .expect("Invalid float in sort")
    }
}

impl<'a> Hittable<'a> for BvhNode<'a> {
    fn hit(&self, ray: &Ray, t_range: Range<Flt>) -> Option<Hit> {
        // Any hit at all?
        if !self.bbox.hit(ray, &t_range) {
            return None;
        }

        // Check for left hit
        match self.left.hit(ray, t_range.clone()) {
            None => {
                // No left hit - check right
                if let Some(right) = &self.right {
                    right.hit(ray, t_range)
                } else {
                    // No right hit either
                    None
                }
            }
            Some(lhit) => {
                // Got left hit - check right
                if let Some(right) = &self.right {
                    // Got right - check it
                    match right.hit(ray, t_range.start..lhit.t) {
                        None => Some(lhit),
                        rhit => rhit,
                    }
                } else {
                    Some(lhit)
                }
            }
        }
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
