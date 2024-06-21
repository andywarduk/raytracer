use std::fmt::Debug;
use std::{ops::Range, sync::Arc};

use crate::aabb::Aabb;
use crate::{
    materials::material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Debug)]
pub struct Hit {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl Hit {
    pub fn new(
        p: Point3,
        t: f64,
        ray: &Ray,
        outward_normal: &Vec3,
        material: Arc<dyn Material>,
    ) -> Self {
        let front_face = ray.direction().dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal.clone()
        } else {
            -outward_normal
        };

        Self {
            p,
            normal,
            t,
            front_face,
            material,
        }
    }
}

pub trait Hittable: Debug + Send + Sync {
    fn hit(&self, ray: &Ray, t_range: Range<f64>) -> Option<Hit>;
    fn bounding_box(&self) -> &Aabb;
}
