use std::{cmp::Ordering, ops::Range};

use crate::{hittable::aabb::Aabb, hittable::hit::Hit, hittable::hittable::Hittable, ray::Ray};

#[derive(Debug)]
pub struct BvhNode {
    bbox: Aabb,
    left: Box<dyn Hittable>,
    right: Option<Box<dyn Hittable>>,
}

impl BvhNode {
    pub fn new(mut objects: Vec<Box<dyn Hittable>>) -> Self {
        let bbox = objects.iter().fold(Aabb::default(), |bbox, object| {
            Aabb::new_from_bbox(&bbox, object.bounding_box())
        });

        let axis = bbox.longest_axis();

        let comparator = match axis {
            0 => |a: &Box<dyn Hittable>, b: &Box<dyn Hittable>| Self::box_compare(&**a, &**b, 0),
            1 => |a: &Box<dyn Hittable>, b: &Box<dyn Hittable>| Self::box_compare(&**a, &**b, 1),
            2 => |a: &Box<dyn Hittable>, b: &Box<dyn Hittable>| Self::box_compare(&**a, &**b, 2),
            _ => panic!("Invalid axis"),
        };

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
                objects.sort_by(comparator);

                let mid = vec_len / 2;
                let split = objects.split_off(mid);

                (
                    Box::new(BvhNode::new(objects)) as Box<dyn Hittable>,
                    Some(Box::new(BvhNode::new(split)) as Box<dyn Hittable>),
                )
            }
        };

        Self { bbox, left, right }
    }

    fn box_compare<'a>(a: &'a dyn Hittable, b: &'a dyn Hittable, axis: usize) -> Ordering {
        let a_axis_interval = a.bounding_box().axis_interval(axis);
        let b_axis_interval = b.bounding_box().axis_interval(axis);

        a_axis_interval
            .start
            .partial_cmp(&b_axis_interval.start)
            .expect("Invalid f64 in sort")
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, t_range: Range<f64>) -> Option<Hit> {
        if !self.bbox.hit(ray, t_range.clone()) {
            return None;
        }

        let left_hit = self.left.hit(ray, t_range.clone());

        let right_hit = if let Some(right) = &self.right {
            right.hit(ray, t_range)
        } else {
            None
        };

        match (left_hit, right_hit) {
            (_, Some(hit)) => Some(hit),
            (Some(hit), _) => Some(hit),
            _ => None,
        }
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
