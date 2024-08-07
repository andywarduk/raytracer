//! Sphere shape

use std::ops::Range;

use rand::rngs::ThreadRng;

use crate::{
    float::*,
    hits::{aabb::Aabb, hit::Hit, hittable::Hittable},
    materials::material::{MatRef, Material},
    ray::Ray,
    triple::{Point3, Vec3},
};

/// Sphere details
#[derive(Debug)]
pub struct Sphere<'a> {
    /// Centre at time 0
    center0: Point3,
    /// Is moving?
    moving: bool,
    /// Movement per time unit
    movement: Vec3,
    /// Radius
    radius: Flt,
    /// Material to use
    material: MatRef<'a>,
    /// Bounding box
    bbox: Aabb,
}

impl<'a> Sphere<'a> {
    /// Creates a new sphere with the center at a given point and a given radius. Material object
    pub fn new(center: Point3, radius: FltPrim, material: &'a dyn Material) -> Self {
        Self::new_moving(center.clone(), center, radius, material)
    }

    /// Creates a new sphere with the center at a given point and a given radius. Material reference
    pub fn new_with_matref(center: Point3, radius: FltPrim, matref: MatRef<'a>) -> Self {
        Self::new_moving_with_matref(center.clone(), center, radius, matref)
    }

    /// Creates a new moving sphere with given centres and radius. Material object
    pub fn new_moving(
        center0: Point3,
        center1: Point3,
        radius: FltPrim,
        material: &'a dyn Material,
    ) -> Self {
        Self::new_moving_with_matref(center0, center1, radius, MatRef::Borrow(material))
    }

    /// Creates a new moving sphere with given centres and radius. Material reference
    pub fn new_moving_with_matref(
        center0: Point3,
        center1: Point3,
        radius: FltPrim,
        material: MatRef<'a>,
    ) -> Self {
        let movement = center0.vec_to(&center1);
        let moving = movement.length() > 0.0;

        let rvec = Vec3::new(radius, radius, radius);

        let bbox = if moving {
            let box1 = Aabb::new_from_points(&(&center0 - &rvec), &(&center0 + &rvec));
            let box2 = Aabb::new_from_points(&(&center1 - &rvec), &(&center1 + rvec));
            Aabb::new_from_bbox(&box1, &box2)
        } else {
            Aabb::new_from_points(&(&center0 - &rvec), &(&center0 + &rvec))
        };

        Self {
            center0,
            moving,
            movement,
            radius: flt(radius),
            material,
            bbox,
        }
    }

    fn position_at_time(&self, time: Flt) -> Point3 {
        if self.moving {
            &self.center0 + (time * &self.movement)
        } else {
            self.center0.clone()
        }
    }

    fn get_uv(p: &Vec3) -> (Flt, Flt) {
        // p: a given vector from the centre of the sphere of length 1
        // u: returned value [0,1] of angle around the Y axis from X=-1
        // v: returned value [0,1] of angle from Y=-1 to Y=+1
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;

        let u = phi / (2.0 * PI);
        let v = theta / PI;

        (u, v)
    }
}

impl<'a> Hittable<'a> for Sphere<'a> {
    fn hit(&self, rng: &mut ThreadRng, ray: &Ray, t_range: Range<Flt>) -> Option<Hit> {
        let center = self.position_at_time(ray.time());
        let oc = ray.origin().vec_to(&center);
        let a = ray.direction().length_squared();
        let h = ray.direction().dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut t = (h - sqrtd) / a;

        if !t_range.contains(&t) {
            t = (h + sqrtd) / a;

            if !t_range.contains(&t) {
                return None;
            }
        }

        let p = ray.at(t);
        let outward_normal = center.vec_to(&p) / self.radius;

        let (u, v) = Self::get_uv(&outward_normal);

        // Check material registers a hit
        if !self.material.hit(rng, u, v, &p) {
            return None;
        }

        Some(Hit::new(
            p,
            t,
            u,
            v,
            ray,
            &outward_normal,
            self.material.get_ref(),
        ))
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
