//! Quadrilateral shape

use std::ops::Range;

use crate::{
    float::*,
    hits::{aabb::Aabb, hit::Hit, hittable::Hittable},
    materials::material::{MatRef, Material},
    ray::Ray,
    triple::{Point3, Vec3},
};

/// Quadrilateral details
#[derive(Debug)]
pub struct Quad<'a> {
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
    material: MatRef<'a>,
    /// Bounding box
    bbox: Aabb,
}

impl<'a> Quad<'a> {
    /// Creates a new quadrilateral at a given point and two edge vectors
    pub fn new(p: Point3, u: Vec3, v: Vec3, material: &'a dyn Material) -> Self {
        Self::new_moving(p.clone(), u.clone(), v.clone(), p, u, v, material)
    }

    /// Creates a new moving quadrilateral given two sets of points and two edge vectors
    pub fn new_moving(
        p0: Point3,
        u0: Vec3,
        v0: Vec3,
        p1: Point3,
        u1: Vec3,
        v1: Vec3,
        material: &'a dyn Material,
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
            material: MatRef::Borrow(material),
            bbox,
        }
    }

    fn calc_bbox(p: &Point3, u: &Vec3, v: &Vec3) -> Aabb {
        let bbox_diag1 = Aabb::new_from_points(p, &(p + u + v));
        let bbox_diag2 = Aabb::new_from_points(&(p + u), &(p + v));

        Aabb::new_from_bbox(&bbox_diag1, &bbox_diag2)
    }

    fn position_at_time(&self, time: Flt) -> (Point3, Vec3, Vec3, Vec3) {
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

impl<'a> Hittable<'a> for Quad<'a> {
    fn hit(&self, ray: &Ray, t_range: Range<Flt>) -> Option<Hit> {
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

        if !(flt(0.0)..flt(1.0)).contains(&alpha) {
            return None;
        }

        let beta = w.dot(&u.cross(&planar_hitpt_vector));

        if !(flt(0.0)..flt(1.0)).contains(&beta) {
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
            self.material.get_ref(),
        ))
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
