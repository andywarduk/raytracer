use std::{iter::Sum, ops::Index};

use auto_ops::*;
use rand::{rngs::ThreadRng, Rng};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    pub const fn new(e1: f64, e2: f64, e3: f64) -> Self {
        Self { e: [e1, e2, e3] }
    }

    pub fn new_random(rng: &mut ThreadRng) -> Self {
        Self::new(
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
        )
    }

    pub fn new_random_clamped(rng: &mut ThreadRng, min: f64, max: f64) -> Self {
        Self::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    pub fn new_random_in_unit_sphere(rng: &mut ThreadRng) -> Self {
        loop {
            let p = Self::new_random_clamped(rng, -1.0, 1.0);

            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn new_random_in_unit_disk(rng: &mut ThreadRng) -> Self {
        loop {
            let p = Self::new(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), 0.0);

            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn new_random_unit_vector(rng: &mut ThreadRng) -> Self {
        Self::new_random_in_unit_sphere(rng).unit_vector()
    }

    pub fn new_random_on_hemisphere(rng: &mut ThreadRng, normal: &Vec3) -> Self {
        let on_unit_sphere = Self::new_random_unit_vector(rng);

        if on_unit_sphere.dot(normal) > 0.0 {
            // In the same hemisphere as the normal
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
    }

    pub fn cross(&self, other: &Self) -> Vec3 {
        Vec3::new(
            self.e[1] * other.e[2] - self.e[2] * other.e[1],
            self.e[2] * other.e[0] - self.e[0] * other.e[2],
            self.e[0] * other.e[1] - self.e[1] * other.e[0],
        )
    }

    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }

    pub fn reflect(&self, n: &Vec3) -> Vec3 {
        self - 2.0 * self.dot(n) * n
    }

    pub fn refract(&self, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = -self.dot(n).min(1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * n;

        r_out_perp + r_out_parallel
    }

    pub fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        let s = 1e-8;
        self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s
    }
}

// Operator implementations
impl_op_ex!(+ |a: &Vec3, b: &Vec3| -> Vec3 { Vec3 { e: [a.e[0] + b.e[0], a.e[1] + b.e[1], a.e[2] + b.e[2]] } });
impl_op_ex!(+= |a: &mut Vec3, b: &Vec3| { a.e[0] += b.e[0]; a.e[1] += b.e[1]; a.e[2] += b.e[2]; });
impl_op_ex!(-|a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3 {
        e: [a.e[0] - b.e[0], a.e[1] - b.e[1], a.e[2] - b.e[2]],
    }
});
impl_op_ex!(-|a: &Vec3| -> Vec3 {
    Vec3 {
        e: [-a.e[0], -a.e[1], -a.e[2]],
    }
});
impl_op_ex!(-= |a: &mut Vec3, b: &Vec3| { a.e[0] -= b.e[0]; a.e[1] -= b.e[1]; a.e[2] -= b.e[2]; });
impl_op_ex_commutative!(*|a: &Vec3, b: f64| -> Vec3 {
    Vec3 {
        e: [a.e[0] * b, a.e[1] * b, a.e[2] * b],
    }
});
impl_op_ex!(*|a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3 {
        e: [a.e[0] * b.e[0], a.e[1] * b.e[1], a.e[2] * b.e[2]],
    }
});
impl_op_ex!(*= |a: &mut Vec3, b: f64| { a.e[0] *= b; a.e[1] *= b; a.e[2] *= b; });
impl_op_ex!(/ |a: &Vec3, b: f64| -> Vec3 { a * (1f64 / b) } );
impl_op_ex!(/= |a: &mut Vec3, b: f64| { *a *= 1f64 / b });

impl Sum for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|acc, i| acc + i).unwrap_or_else(Vec3::default)
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &f64 {
        &self.e[i]
    }
}

pub type Point3 = Vec3;

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn test_length() {
        assert_eq!(Vec3::new(3.0, 4.0, 5.0).length(), (50f64).sqrt())
    }

    #[test]
    fn test_length_squared() {
        assert_eq!(Vec3::new(3.0, 4.0, 5.0).length_squared(), 50.0)
    }

    #[test]
    fn test_dot() {
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0).dot(&Vec3::new(4.0, 5.0, 6.0)),
            32f64
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
            Vec3::new(3.0 / vec_len, 4.0 / vec_len, 5.0 / vec_len)
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
        assert_eq!(Vec3::new(1.0, 2.0, 3.0) * 2.0, Vec3::new(2.0, 4.0, 6.0))
    }

    #[test]
    fn test_div() {
        assert_eq!(Vec3::new(2.0, 4.0, 6.0) / 2.0, Vec3::new(1.0, 2.0, 3.0))
    }
}
