//! Ray

use std::fmt::Display;

use crate::{
    float::*,
    triple::{Point3, Vec3},
};

/// Ray properties
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
    /// Create a new light ray with given origin, direction and time
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

    /// Returns the ray origin
    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    /// Returns the ray direction
    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    /// Returns the ray inverse direction
    pub fn inv_direction(&self) -> &Vec3 {
        &self.inv_dir
    }

    /// Returns the ray time
    pub fn time(&self) -> Flt {
        self.time
    }

    /// Returns the ray position at given distance
    pub fn at(&self, t: Flt) -> Point3 {
        &self.orig + (&self.dir * t)
    }

    /// Returns the ray depth (number of bounces)
    pub fn depth(&self) -> u64 {
        self.depth
    }

    /// Sets the ray depth
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
