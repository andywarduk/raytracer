//! Rotates an object in the y axis

use std::ops::Range;

use crate::{
    float::*,
    hits::{
        aabb::Aabb,
        hit::Hit,
        hittable::{Hittable, HittableRef},
    },
    ray::Ray,
    triple::{Point3, Vec3},
};

/// Rotation details
#[derive(Debug)]
pub struct RotateY<'a> {
    cos_theta: Flt,
    sin_theta: Flt,
    object: HittableRef<'a>,
    bbox: Aabb,
}

impl<'a> RotateY<'a> {
    /// Creates a new rotation object
    pub fn new(angle: FltPrim, object: impl Hittable<'a> + 'a) -> Self {
        let angle = flt(angle);
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();

        let mut min = Point3::new_flt(flt_max(), flt_max(), flt_max());
        let mut max = Point3::new_flt(flt_min(), flt_min(), flt_min());

        for i in 0..2 {
            let fi = flt(i as FltPrim);

            for j in 0..2 {
                let fj = flt(j as FltPrim);

                for k in 0..2 {
                    let fk = flt(k as FltPrim);

                    let x = fi * bbox.ranges[0].end + (flt(1.0) - fi) * bbox.ranges[0].start;
                    let y = fj * bbox.ranges[1].end + (flt(1.0) - fj) * bbox.ranges[1].start;
                    let z = fk * bbox.ranges[2].end + (flt(1.0) - fk) * bbox.ranges[2].start;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new_flt(newx, y, newz);

                    for c in 0..3 {
                        min.e[c] = min.e[c].min(tester.e[c]);
                        max.e[c] = max.e[c].max(tester.e[c]);
                    }
                }
            }
        }

        let bbox = Aabb::new_from_points(&min, &max);

        Self {
            cos_theta,
            sin_theta,
            object: HittableRef::boxed(object),
            bbox,
        }
    }
}

impl<'a> Hittable<'a> for RotateY<'a> {
    fn hit(&self, ray: &Ray, t_range: Range<Flt>) -> Option<Hit> {
        // Change the ray from world space to object space
        let mut origin = ray.origin().clone();
        let mut direction = ray.direction().clone();

        origin.e[0] = self.cos_theta * ray.origin().e[0] - self.sin_theta * ray.origin().e[2];
        origin.e[2] = self.sin_theta * ray.origin().e[0] + self.cos_theta * ray.origin().e[2];

        direction.e[0] =
            self.cos_theta * ray.direction().e[0] - self.sin_theta * ray.direction().e[2];
        direction.e[2] =
            self.sin_theta * ray.direction().e[0] + self.cos_theta * ray.direction().e[2];

        let rotated = Ray::new(origin, direction, ray.time());

        // Determine whether an intersection exists in object space (and if so, where)
        match self.object.hit(&rotated, t_range) {
            None => None,
            Some(mut hit) => {
                // Change the intersection point from object space to world space
                let mut p = hit.p.clone();
                p.e[0] = self.cos_theta * hit.p.e[0] + self.sin_theta * hit.p.e[2];
                p.e[2] = -self.sin_theta * hit.p.e[0] + self.cos_theta * hit.p.e[2];

                // Change the normal from object space to world space
                let mut normal = hit.normal.clone();
                normal.e[0] = self.cos_theta * hit.normal.e[0] + self.sin_theta * hit.normal.e[2];
                normal.e[2] = -self.sin_theta * hit.normal.e[0] + self.cos_theta * hit.normal.e[2];

                hit.p = p;
                hit.normal = normal;

                Some(hit)
            }
        }
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
