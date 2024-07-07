use super::ops::*;
use auto_ops::*;
use std::fmt::Display;

use rand::{rngs::ThreadRng, Rng};

use crate::float::*;

use super::Triple;

/// Vector mixin
#[derive(Default, Debug, PartialEq, Clone)]
pub struct VecMixin;

/// Vector type
pub type Vec3 = Triple<VecMixin>;

/// Methods for vectors
impl Vec3 {
    /// Creates a new random vector within a sphere if radius 1.0
    pub fn new_random_in_unit_sphere(rng: &mut ThreadRng) -> Self {
        loop {
            let p = Self::new_random_clamped(rng, -1.0, 1.0);

            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    /// Creates a new random vector within a disc of radius 1.0 on the xy plane
    pub fn new_random_in_unit_disk(rng: &mut ThreadRng) -> Self {
        loop {
            let p = Self::new(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), 0.0);

            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    /// Creates a new random unit vector for the surface of a sphere with radius 1.0
    #[inline]
    pub fn new_random_unit_vector(rng: &mut ThreadRng) -> Self {
        Self::new_random_in_unit_sphere(rng).unit_vector()
    }

    /// Creates a new random unit vector for the surface of a hemisphere with radius 1.0
    #[inline]
    pub fn new_random_on_hemisphere(rng: &mut ThreadRng, normal: &Vec3) -> Self {
        let on_unit_sphere = Self::new_random_unit_vector(rng);

        if on_unit_sphere.dot(normal) > 0.0 {
            // In the same hemisphere as the normal
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    /// Returns the length of the vector
    #[inline]
    pub fn length(&self) -> Flt {
        self.length_squared().sqrt()
    }

    /// Returns the length squared of the vector
    #[inline]
    pub fn length_squared(&self) -> Flt {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }

    /// Returns the dot product of this triple with another triple
    #[inline]
    pub fn dot<T>(&self, other: &Triple<T>) -> Flt {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }

    /// Returns the cross product of this vector with another vector
    #[inline]
    pub fn cross(&self, other: &Self) -> Vec3 {
        Vec3::new_flt(
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
        )
    }

    /// Returns the reciprocal vector
    #[inline]
    pub fn recip(&self) -> Self {
        Self::new_from_array([self[0].recip(), self[1].recip(), self[2].recip()])
    }

    /// Returns the unit vector for this vector
    #[inline]
    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }

    /// Reflects the vector around a normal vector
    #[inline]
    pub fn reflect(&self, n: &Vec3) -> Vec3 {
        self - flt(2.0) * self.dot(n) * n
    }

    /// Refracts the vector
    #[inline]
    pub fn refract(&self, n: &Vec3, etai_over_etat: Flt) -> Vec3 {
        let cos_theta = -self.dot(n).min(flt(1.0));
        let r_out_perp = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = -((flt(1.0) - r_out_perp.length_squared()).abs().sqrt()) * n;

        r_out_perp + r_out_parallel
    }

    /// Return true if the vector is close to zero in all dimensions.
    #[inline]
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self[0].abs() < s && self[1].abs() < s && self[2].abs() < s
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display("u", "v", "w", f)
    }
}

// Vector operators
impl_op_add!(Vec3);
impl_op_add_assign_float!(Vec3);

impl_op_sub!(Vec3);
impl_op_sub_assign_float!(Vec3);

impl_op_mul_float!(Vec3);
impl_op_mul_assign_float!(Vec3);

impl_op_div_float!(Vec3);
impl_op_div_assign_float!(Vec3);

impl_op_neg!(Vec3);

#[cfg(test)]
mod tests {
    use crate::float::*;

    use super::*;

    #[test]
    fn test_length() {
        assert_eq!(Vec3::new(3.0, 4.0, 5.0).length(), (flt(50.0)).sqrt())
    }

    #[test]
    fn test_length_squared() {
        assert_eq!(Vec3::new(3.0, 4.0, 5.0).length_squared(), 50.0)
    }

    #[test]
    fn test_dot() {
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0).dot(&Vec3::new(4.0, 5.0, 6.0)),
            flt(32.0)
        )
    }

    #[test]
    fn test_cross() {
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0).cross(&Vec3::new(4.0, 5.0, 6.0)),
            Vec3::new(-3.0, 6.0, -3.0)
        )
    }

    #[test]
    fn test_unit_vector() {
        let vec = Vec3::new(3.0, 4.0, 5.0);
        let vec_len = vec.length();
        assert_eq!(
            vec.unit_vector(),
            Vec3::new_flt(flt(3.0) / vec_len, flt(4.0) / vec_len, flt(5.0) / vec_len)
        )
    }

    #[test]
    fn test_add() {
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0) + Vec3::new(4.0, 5.0, 6.0),
            Vec3::new(5.0, 7.0, 9.0)
        )
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            Vec3::new(5.0, 7.0, 9.0) - Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(4.0, 5.0, 6.0)
        )
    }

    #[test]
    fn test_mul() {
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0) * flt(2.0),
            Vec3::new(2.0, 4.0, 6.0)
        )
    }

    #[test]
    fn test_div() {
        assert_eq!(
            Vec3::new(2.0, 4.0, 6.0) / flt(2.0),
            Vec3::new(1.0, 2.0, 3.0)
        )
    }
}
