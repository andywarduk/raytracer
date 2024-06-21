use std::{f64::consts::PI, ops::Range, sync::Arc};

use crate::{
    aabb::Aabb,
    hittable::{Hit, Hittable},
    materials::material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Debug)]
pub struct Sphere {
    /// Centre at time 0
    center0: Point3,
    // Is moving?
    moving: bool,
    /// Movement per time unit
    movement: Vec3,
    /// Radius
    radius: f64,
    /// Material to use
    material: Arc<dyn Material>,
    /// Bounding box
    bbox: Aabb,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        Self::new_moving(center.clone(), center, radius, material)
    }

    pub fn new_moving(
        center0: Point3,
        center1: Point3,
        radius: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        let movement = &center1 - &center0;
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
            radius,
            material,
            bbox,
        }
    }

    fn position_at_time(&self, time: f64) -> Point3 {
        if self.moving {
            &self.center0 + (time * &self.movement)
        } else {
            self.center0.clone()
        }
    }

    fn get_uv(p: &Point3) -> (f64, f64) {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

        let theta = -p.y().acos();
        let phi = -p.z().atan2(p.x()) + PI;

        let u = phi / (2.0 * PI);
        let v = theta / PI;

        (u, v)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_range: Range<f64>) -> Option<Hit> {
        let center = self.position_at_time(ray.time());
        let oc = &center - ray.origin();
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
        let outward_normal = (&p - &center) / self.radius;

        let (u, v) = Self::get_uv(&outward_normal);

        Some(Hit::new(p, t, u, v, ray, &outward_normal, self.material.clone()))
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
