use std::fmt::Debug;
use std::ops::Range;

use crate::hittable::aabb::Aabb;
use crate::ray::Ray;

use super::hit::Hit;

pub const T_MIN: f64 = 0.001;

pub trait Hittable: Debug + Send + Sync {
    fn hit(&self, ray: &Ray, t_range: Range<f64>) -> Option<Hit>;
    fn bounding_box(&self) -> &Aabb;
}
