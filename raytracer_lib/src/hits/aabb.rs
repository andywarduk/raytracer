use std::{fmt::Display, ops::Range};

use auto_ops::impl_op_ex_commutative;

use crate::{
    float::*,
    ray::Ray,
    triple::{Point3, Vec3},
};

#[derive(Debug, Clone)]
pub struct Aabb {
    pub ranges: [Range<Flt>; 3],
}

impl Aabb {
    pub fn new_from_ranges(x: Range<Flt>, y: Range<Flt>, z: Range<Flt>) -> Self {
        let mut res = Self { ranges: [x, y, z] };

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

        let mut res = Self { ranges: [x, y, z] };

        res.pad_to_minimums();

        res
    }

    pub fn new_from_bbox(a: &Aabb, b: &Aabb) -> Self {
        let mut res = Self {
            ranges: [
                (a.ranges[0].start.min(b.ranges[0].start))..(a.ranges[0].end.max(b.ranges[0].end)),
                (a.ranges[1].start.min(b.ranges[1].start))..(a.ranges[1].end.max(b.ranges[1].end)),
                (a.ranges[2].start.min(b.ranges[2].start))..(a.ranges[2].end.max(b.ranges[2].end)),
            ],
        };

        res.pad_to_minimums();

        res
    }

    pub fn hit(&self, ray: &Ray, t_range: &Range<Flt>) -> bool {
        let mut start = t_range.start;
        let mut end = t_range.end;

        let ray_orig = ray.origin();
        let ray_inv_dir = ray.inv_direction();

        for axis in 0..3 {
            let ax = &self.ranges[axis];
            let orig = ray_orig[axis];
            let adinv = ray_inv_dir[axis];

            let t0 = (ax.start - orig) * adinv;
            let t1 = (ax.end - orig) * adinv;

            start = start.max(t0.min(t1));
            end = end.min(t0.max(t1));

            if end <= start {
                return false;
            }
        }

        true
    }

    pub fn longest_axis(&self) -> usize {
        let mut largest = flt(0.0);
        let mut axis = 0;

        for (i, r) in self.ranges.iter().enumerate() {
            let len = r.end - r.start;

            if len > largest {
                largest = len;
                axis = i;
            }
        }

        axis
    }

    const DELTA: FltPrim = 0.0001;

    fn pad_to_minimums(&mut self) {
        // Adjust the AABB so that no side is narrower than some delta, padding if necessary.
        for i in 0..2 {
            if self.ranges[i].end - self.ranges[i].start < Self::DELTA {
                let half = Self::DELTA / 2.0;

                self.ranges[i].start -= half;
                self.ranges[i].end += half;
            }
        }
    }
}

// Operator implementations
impl_op_ex_commutative!(+ |a: &Aabb, b: &Vec3| -> Aabb {
    let ranges = [
        (a.ranges[0].start + b.e[0])..(a.ranges[0].end + b.e[0]),
        (a.ranges[1].start + b.e[1])..(a.ranges[1].end + b.e[1]),
        (a.ranges[2].start + b.e[2])..(a.ranges[2].end + b.e[2]),
    ];

    Aabb { ranges }
});

impl Display for Aabb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x0 = self.ranges[0].start;
        let x1 = self.ranges[0].end;
        let y0 = self.ranges[1].start;
        let y1 = self.ranges[1].end;
        let z0 = self.ranges[2].start;
        let z1 = self.ranges[2].end;

        f.write_fmt(format_args!(
            "({x0}, {y0}, {z0}) to ({x1}, {y1}, {z1}), size {} x {} x {}",
            (x1 - x0).abs(),
            (y1 - y0).abs(),
            (z1 - z0).abs()
        ))
    }
}
