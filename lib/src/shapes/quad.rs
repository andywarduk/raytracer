use std::{ops::Range, sync::Arc};

use crate::{
    hittable::aabb::Aabb,
    hittable::hit::Hit,
    hittable::hittable::Hittable,
    materials::material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Debug, Clone)]
pub struct Quad {
    /// Anchor point at time 0
    p: Point3,
    /// Side 1 vector
    u: Vec3,
    /// Side 2 vector
    v: Vec3,
    /// Normal vector
    normal: Vec3,
    /// Is moving?
    moving: bool,
    /// p movement per time unit
    p_movement: Vec3,
    /// u movement per time unit
    u_movement: Vec3,
    /// v movement per time unit
    v_movement: Vec3,
    /// normal movement per time unit
    normal_movement: Vec3,
    /// Material to use
    material: Arc<dyn Material>,
    /// Bounding box
    bbox: Aabb,
}

impl Quad {
    pub fn new(p: Point3, u: Vec3, v: Vec3, material: Arc<dyn Material>) -> Self {
        Self::new_moving(p.clone(), p, u.clone(), u, v.clone(), v, material)
    }

    pub fn new_moving(
        p0: Point3,
        p1: Point3,
        u0: Vec3,
        u1: Vec3,
        v0: Vec3,
        v1: Vec3,
        material: Arc<dyn Material>,
    ) -> Self {
        // Calculate movement
        let p_movement = p0.vec_to(&p1);
        let u_movement = &u1 - &u0;
        let v_movement = &u1 - &u0;

        let moving =
            p_movement.length() > 0.0 || u_movement.length() > 0.0 || v_movement.length() > 0.0;

        // Calculate normals
        let n0 = u0.cross(&v0).unit_vector();
        let n1 = u1.cross(&v1).unit_vector();

        // Calculate normal movement
        let n_movement = &n1 - &n0;

        // Calculate bounding box
        let bbox = if moving {
            let box1 = Self::calc_bbox(&p0, &u0, &v0);
            let box2 = Self::calc_bbox(&p1, &u1, &v1);
            Aabb::new_from_bbox(&box1, &box2)
        } else {
            Self::calc_bbox(&p0, &u0, &v0)
        };

        Self {
            p: p0,
            u: u0,
            v: v0,
            normal: n0,
            moving,
            p_movement,
            u_movement,
            v_movement,
            normal_movement: n_movement,
            material,
            bbox,
        }
    }

    fn calc_bbox(p: &Point3, u: &Vec3, v: &Vec3) -> Aabb {
        let bbox_diag1 = Aabb::new_from_points(p, &(p + u + v));
        let bbox_diag2 = Aabb::new_from_points(&(p + u), &(p + v));

        Aabb::new_from_bbox(&bbox_diag1, &bbox_diag2)
    }

    fn position_at_time(&self, time: f64) -> (Point3, Vec3, Vec3, Vec3) {
        if self.moving {
            (
                &self.p + (time * &self.p_movement),
                &self.u + (time * &self.u_movement),
                &self.v + (time * &self.v_movement),
                &self.normal + (time * &self.normal_movement),
            )
        } else {
            (
                self.p.clone(),
                self.u.clone(),
                self.v.clone(),
                self.normal.clone(),
            )
        }
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, t_range: Range<f64>) -> Option<Hit> {
        let (p, u, v, normal) = self.position_at_time(ray.time());

        let dot = normal.dot(&p);

        let denom = normal.dot(ray.direction());

        // No hit if the ray is parallel to the plane.
        if denom.abs() < 1e-8 {
            return None;
        }

        // Return false if the hit point parameter t is outside the ray interval.
        let t = (dot - normal.dot(ray.origin())) / denom;

        if !t_range.contains(&t) {
            return None;
        }

        // Determine the hit point lies within the planar shape using its plane coordinates.
        let intersection = ray.at(t);

        let planar_hitpt_vector = p.vec_to(&intersection);

        let n = u.cross(&v);
        let w = &n / n.dot(&n);

        let alpha = w.dot(&planar_hitpt_vector.cross(&v));

        if !(0.0..1.0).contains(&alpha) {
            return None;
        }

        let beta = w.dot(&u.cross(&planar_hitpt_vector));

        if !(0.0..1.0).contains(&beta) {
            return None;
        }

        // Ray hits the 2D shape
        Some(Hit::new(
            intersection,
            t,
            alpha,
            beta,
            ray,
            &normal,
            self.material.clone(),
        ))
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
