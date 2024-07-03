use std::fmt::Display;

use crate::{
    float::*,
    triple::{Point3, Vec3},
};

/// A light ray
#[derive(Debug, Default)]
pub struct Ray {
    /// The ray origin
    orig: Point3,
    /// The ray direction
    dir: Vec3,
    /// The inverse ray direction (used in intersection calculation)
    inv_dir: Vec3,
    /// The ray time
    time: Flt,
    /// The ray depth
    depth: u64,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3, time: Flt) -> Self {
        let inv_dir = dir.recip();

        Self {
            orig,
            dir,
            inv_dir,
            time,
            depth: 0,
        }
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn inv_direction(&self) -> &Vec3 {
        &self.inv_dir
    }

    pub fn time(&self) -> Flt {
        self.time
    }

    pub fn at(&self, t: Flt) -> Point3 {
        &self.orig + (&self.dir * t)
    }

    pub fn depth(&self) -> u64 {
        self.depth
    }

    pub fn set_depth(&mut self, depth: u64) {
        self.depth = depth;
    }
}

impl Display for Ray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "[origin: {}, direction: {}, time {}]",
            self.orig, self.dir, self.time
        ))
    }
}
