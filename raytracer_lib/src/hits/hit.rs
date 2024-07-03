use crate::{
    float::*,
    materials::material::Material,
    ray::Ray,
    triple::{Point3, Vec3},
};

#[derive(Debug)]
pub struct Hit<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub t: Flt,
    pub u: Flt,
    pub v: Flt,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> Hit<'a> {
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
