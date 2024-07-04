//! Ray and object intersection details

use crate::{
    float::*,
    materials::material::Material,
    ray::Ray,
    triple::{Point3, Vec3},
};

/// Intersection details
#[derive(Debug)]
pub struct Hit<'a> {
    /// The point of intersecton
    pub p: Point3,
    /// The normal vector at point of intersection
    pub normal: Vec3,
    /// The distance to the intersection
    pub t: Flt,
    /// The x position of the intersection on the surface 0.0-1.0
    pub u: Flt,
    /// The y position of the intersection on the surface 0.0-1.0
    pub v: Flt,
    /// Object front face hit
    pub front_face: bool,
    /// The material of the object at intersection
    pub material: &'a dyn Material,
}

impl<'a> Hit<'a> {
    /// Creates a new hit detail object
    pub fn new(
        p: Point3,
        t: Flt,
        u: Flt,
        v: Flt,
        ray: &Ray,
        outward_normal: &Vec3,
        material: &'a dyn Material,
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
            u,
            v,
            front_face,
            material,
        }
    }
}
