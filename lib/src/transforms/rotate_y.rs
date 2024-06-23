use std::{ops::Range, sync::Arc};

use crate::{
    hittable::{aabb::Aabb, hit::Hit, hittable::Hittable},
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Debug)]
pub struct RotateY {
    cos_theta: f64,
    sin_theta: f64,
    object: Arc<dyn Hittable>,
    bbox: Aabb,
}

impl RotateY {
    pub fn new(angle: f64, object: Arc<dyn Hittable>) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();

        let mut min = Point3::new(f64::MAX, f64::MAX, f64::MAX);
        let mut max = Point3::new(f64::MIN, f64::MIN, f64::MIN);

        for i in 0..2 {
            let fi = i as f64;

            for j in 0..2 {
                let fj = j as f64;

                for k in 0..2 {
                    let fk = k as f64;

                    let x = fi * bbox.ranges[0].end + (1.0 - fi) * bbox.ranges[0].start;
                    let y = fj * bbox.ranges[1].end + (1.0 - fj) * bbox.ranges[1].start;
                    let z = fk * bbox.ranges[2].end + (1.0 - fk) * bbox.ranges[2].start;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

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
            object,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, t_range: Range<f64>) -> Option<Hit> {
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
