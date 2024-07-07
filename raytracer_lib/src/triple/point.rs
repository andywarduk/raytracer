use auto_ops::*;
use std::fmt::Display;

use super::ops::*;
use crate::float::*;

use super::{Triple, Vec3};

/// Point mixin
#[derive(Default, Debug, PartialEq, Clone)]
pub struct PointMixin;

/// Point tyoe
pub type Point3 = Triple<PointMixin>;

/// Methods for points
impl Point3 {
    /// Convert point to vector
    #[inline]
    pub fn to_vec3(self) -> Vec3 {
        Vec3::new_from_array(self.e)
    }

    /// Returns the vector between two points
    #[inline]
    pub fn vec_to(&self, to: &Point3) -> Vec3 {
        Vec3::new_flt(to[0] - self[0], to[1] - self[1], to[2] - self[2])
    }
}

impl Display for Point3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display("x", "y", "z", f)
    }
}

// Point operators
impl_op_add_typed!(Point3, Vec3, Point3);
impl_op_add_assign_typed!(Point3, Vec3);
impl_op_add_float!(Point3);
impl_op_add_assign_float!(Point3);

impl_op_sub_typed!(Point3, Vec3, Point3);
impl_op_sub_assign_typed!(Point3, Vec3);
impl_op_sub_float!(Point3);
impl_op_sub_assign_float!(Point3);

impl_op_mul_float!(Point3);
impl_op_mul_assign_float!(Point3);

impl_op_div_float!(Point3);
impl_op_div_assign_float!(Point3);
