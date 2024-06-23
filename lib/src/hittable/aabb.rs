use std::ops::Range;

use auto_ops::impl_op_ex_commutative;

use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Debug, Clone)]
pub struct Aabb {
    pub x: Range<f64>,
    pub y: Range<f64>,
    pub z: Range<f64>,
}

impl Aabb {
    pub fn new_from_ranges(x: Range<f64>, y: Range<f64>, z: Range<f64>) -> Self {
        let mut res = Self { x, y, z };

        res.pad_to_minimums();

        res
    }

    pub fn new_from_points(a: &Point3, b: &Point3) -> Self {
        let x = if a.x() <= b.x() {
            a.x()..b.x()
        } else {
            b.x()..a.x()
        };
        let y = if a.y() <= b.y() {
            a.y()..b.y()
        } else {
            b.y()..a.y()
        };
        let z = if a.z() <= b.z() {
            a.z()..b.z()
        } else {
            b.z()..a.z()
        };

        let mut res = Self { x, y, z };

        res.pad_to_minimums();

        res
    }

    pub fn new_from_bbox(a: &Aabb, b: &Aabb) -> Self {
        let x = (a.x.start.min(b.x.start))..(a.x.end.max(b.x.end));
        let y = (a.y.start.min(b.y.start))..(a.y.end.max(b.y.end));
        let z = (a.z.start.min(b.z.start))..(a.z.end.max(b.z.end));

        let mut res = Self { x, y, z };

        res.pad_to_minimums();

        res
    }

    pub fn axis_interval(&self, axis: usize) -> &Range<f64> {
        match axis {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid axis {axis}"),
        }
    }

    pub fn hit(&self, ray: &Ray, t_range: Range<f64>) -> bool {
        let mut t_range = t_range;
        let ray_orig = ray.origin();
        let ray_dir = ray.direction();

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir[axis];

            let t0 = (ax.start - ray_orig[axis]) * adinv;
            let t1 = (ax.end - ray_orig[axis]) * adinv;

            if t0 < t1 {
                if t0 > t_range.start {
                    t_range.start = t0;
                }
                if t1 < t_range.end {
                    t_range.end = t1;
                }
            } else {
                if t1 > t_range.start {
                    t_range.start = t1;
                }
                if t0 < t_range.end {
                    t_range.end = t0;
                }
            }

            if t_range.end <= t_range.start {
                return false;
            }
        }

        true
    }

    pub fn longest_axis(&self) -> usize {
        let xn = self.x.end - self.x.start;
        let yn = self.y.end - self.y.start;
        let zn = self.z.end - self.z.start;

        if xn > yn {
            if xn > zn {
                0
            } else {
                2
            }
        } else if yn > zn {
            1
        } else {
            2
        }
    }

    const DELTA: f64 = 0.0001;

    fn pad_to_minimums(&mut self) {
        // Adjust the AABB so that no side is narrower than some delta, padding if necessary.

        if self.x.end - self.x.start < Self::DELTA {
            Self::expand_range(&mut self.x)
        }
        if self.y.end - self.y.start < Self::DELTA {
            Self::expand_range(&mut self.y)
        }
        if self.z.end - self.z.start < Self::DELTA {
            Self::expand_range(&mut self.z)
        }
    }

    fn expand_range(r: &mut Range<f64>) {
        let half = Self::DELTA / 2.0;

        r.start -= half;
        r.end += half;
    }
}

// Operator implementations
impl_op_ex_commutative!(+ |a: &Aabb, b: &Vec3| -> Aabb {
    let x = (a.x.start + b.x())..(a.x.end + b.x());
    let y = (a.y.start + b.y())..(a.y.end + b.y());
    let z = (a.z.start + b.z())..(a.z.end + b.z());

    Aabb::new_from_ranges(x, y, z)
});
