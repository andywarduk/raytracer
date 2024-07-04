//! Vector, point and colour classes

use std::{fmt::Display, iter::Sum, ops::Index};

use auto_ops::*;
use rand::{rngs::ThreadRng, Rng};

use std::marker::PhantomData;

use crate::{float::*, gamma::Gamma};

/// Base class for triple
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Triple<SubClassMixin> {
    /// Values
    pub e: [Flt; 3],
    phantom: PhantomData<SubClassMixin>,
}

/// Vector mixin
#[derive(Default, Debug, PartialEq, Clone)]
pub struct VecMixin;

/// Point mixin
#[derive(Default, Debug, PartialEq, Clone)]
pub struct PointMixin;

/// Colour mixin
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ColourMixin;

/// Vector type
pub type Vec3 = Triple<VecMixin>;

/// Point tyoe
pub type Point3 = Triple<PointMixin>;

/// Colour type
pub type Colour = Triple<ColourMixin>;

/// Common methods
impl<Mixin> Triple<Mixin> {
    /// Create new triple from values of float type
    pub fn new_flt(e1: Flt, e2: Flt, e3: Flt) -> Self {
        Self::new_from_array([e1, e2, e3])
    }

    /// Create new triple from values of primary float type
    pub fn new(e1: FltPrim, e2: FltPrim, e3: FltPrim) -> Self {
        Self::new_from_array([flt(e1), flt(e2), flt(e3)])
    }

    /// Create new triple from a fixed array
    pub fn new_from_array(e: [Flt; 3]) -> Self {
        Self {
            e,
            phantom: PhantomData,
        }
    }

    /// Creates a new random triple with values in the range 0.0 to 1.0
    pub fn new_random(rng: &mut ThreadRng) -> Self {
        Self::new(
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
        )
    }

    /// Creates a new random triple with values in the given range
    pub fn new_random_clamped(rng: &mut ThreadRng, min: FltPrim, max: FltPrim) -> Self {
        Self::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    /// Returns the x value
    #[inline]
    pub fn x(&self) -> Flt {
        self.e[0]
    }

    /// Returns the y value
    #[inline]
    pub fn y(&self) -> Flt {
        self.e[1]
    }

    /// Returns the z value
    #[inline]
    pub fn z(&self) -> Flt {
        self.e[2]
    }

    /// Displays the triple
    fn display(
        &self,
        d1: &str,
        d2: &str,
        d3: &str,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "({d1}: {}, {d2}: {}, {d3}: {})",
            self.e[0], self.e[1], self.e[2]
        ))
    }
}

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
    pub fn new_random_unit_vector(rng: &mut ThreadRng) -> Self {
        Self::new_random_in_unit_sphere(rng).unit_vector()
    }

    /// Creates a new random unit vector for the surface of a hemisphere with radius 1.0
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
    pub fn length(&self) -> Flt {
        self.length_squared().sqrt()
    }

    /// Returns the length squared of the vector
    pub fn length_squared(&self) -> Flt {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    /// Returns the dot product of this triple with another triple
    pub fn dot<T>(&self, other: &Triple<T>) -> Flt {
        self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
    }

    /// Returns the cross product of this vector with another vector
    pub fn cross(&self, other: &Self) -> Vec3 {
        Vec3::new_flt(
            self.e[1] * other.e[2] - self.e[2] * other.e[1],
            self.e[2] * other.e[0] - self.e[0] * other.e[2],
            self.e[0] * other.e[1] - self.e[1] * other.e[0],
        )
    }

    /// Returns the reciprocal vector
    pub fn recip(&self) -> Self {
        Self::new_from_array([self.e[0].recip(), self.e[1].recip(), self.e[2].recip()])
    }

    /// Returns the unit vector for this vector
    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }

    /// Reflects the vector around a normal vector
    pub fn reflect(&self, n: &Vec3) -> Vec3 {
        self - flt(2.0) * self.dot(n) * n
    }

    /// Refracts the vector
    pub fn refract(&self, n: &Vec3, etai_over_etat: Flt) -> Vec3 {
        let cos_theta = -self.dot(n).min(flt(1.0));
        let r_out_perp = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = -((flt(1.0) - r_out_perp.length_squared()).abs().sqrt()) * n;

        r_out_perp + r_out_parallel
    }

    /// Return true if the vector is close to zero in all dimensions.
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display("u", "v", "w", f)
    }
}

/// Methods for points
impl Point3 {
    /// Convert point to vector
    pub fn to_vec3(self) -> Vec3 {
        Vec3::new_from_array(self.e)
    }

    /// Returns the vector between two points
    pub fn vec_to(&self, to: &Point3) -> Vec3 {
        Vec3::new_flt(
            to.e[0] - self.e[0],
            to.e[1] - self.e[1],
            to.e[2] - self.e[2],
        )
    }
}

impl Display for Point3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display("x", "y", "z", f)
    }
}

/// Methods for colours
impl Colour {
    /// White constructor
    pub fn new_white() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    /// Grey constructor
    pub fn new_grey(level: FltPrim) -> Self {
        Self::new(level, level, level)
    }

    /// Convert to RGB with optional gamma correction
    pub fn to_rgb(&self, gamma: &Gamma) -> (u8, u8, u8) {
        // Translate the [0,1] component values to the byte range [0,255].
        let (r, g, b) = match gamma {
            Gamma::None => (self.e[0], self.e[1], self.e[2]),
            Gamma::Power(factor) => (
                Self::linear_to_gamma(self.e[0], *factor),
                Self::linear_to_gamma(self.e[1], *factor),
                Self::linear_to_gamma(self.e[2], *factor),
            ),
        };

        (
            FltPrim::from(clamp(flt(256.0) * FltPrim::from(r), flt(0.0), flt(255.0))) as u8,
            FltPrim::from(clamp(flt(256.0) * FltPrim::from(g), flt(0.0), flt(255.0))) as u8,
            FltPrim::from(clamp(flt(256.0) * FltPrim::from(b), flt(0.0), flt(255.0))) as u8,
        )
    }

    fn linear_to_gamma(linear_component: Flt, power: Flt) -> Flt {
        if linear_component > 0.0 {
            linear_component.powf(power)
        } else {
            flt(0.0)
        }
    }
}

impl Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display("r", "g", "b", f)
    }
}

// -- Common Operator implementations --

// Indexing
impl<Mixin> Index<usize> for Triple<Mixin> {
    type Output = Flt;

    #[inline]
    fn index(&self, i: usize) -> &Flt {
        &self.e[i]
    }
}

// Vector Operator implementations

// Addition
impl_op_ex!(+ |a: &Vec3, b: &Vec3| -> Vec3 { Vec3::new_flt(a.e[0] + b.e[0], a.e[1] + b.e[1], a.e[2] + b.e[2]) });
impl_op_ex!(+= |a: &mut Vec3, b: &Vec3| { a.e[0] += b.e[0]; a.e[1] += b.e[1]; a.e[2] += b.e[2]; });

// Subtraction
impl_op_ex!(-|a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3::new_flt(a.e[0] - b.e[0], a.e[1] - b.e[1], a.e[2] - b.e[2])
});
impl_op_ex!(-= |a: &mut Vec3, b: &Vec3| { a.e[0] -= b.e[0]; a.e[1] -= b.e[1]; a.e[2] -= b.e[2]; });

// Negation
impl_op_ex!(-|a: &Vec3| -> Vec3 { Vec3::new_flt(-a.e[0], -a.e[1], -a.e[2]) });

// Multiplication
impl_op_ex_commutative!(*|a: &Vec3, b: Flt| -> Vec3 {
    Vec3::new_flt(a.e[0] * b, a.e[1] * b, a.e[2] * b)
});
impl_op_ex!(*|a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3::new_flt(a.e[0] * b.e[0], a.e[1] * b.e[1], a.e[2] * b.e[2])
});
impl_op_ex!(*= |a: &mut Vec3, b: &Vec3| {
    a.e[0] *= b.e[0];
    a.e[1] *= b.e[1];
    a.e[2] *= b.e[2];
});
impl_op_ex!(*= |a: &mut Vec3, b: Flt| { a.e[0] *= b; a.e[1] *= b; a.e[2] *= b; });

// Division
impl_op_ex!(/ |a: &Vec3, b: Flt| -> Vec3 { a * (flt(1.0) / b) } );
impl_op_ex!(/= |a: &mut Vec3, b: Flt| { *a *= flt(1.0) / b });

// -- Point Operator implementations

// Addition
impl_op_ex!(+ |a: &Point3, b: &Vec3| -> Point3 { Point3::new_flt(a.e[0] + b.e[0], a.e[1] + b.e[1], a.e[2] + b.e[2]) });
impl_op_ex!(+= |a: &mut Point3, b: &Vec3| { a.e[0] += b.e[0]; a.e[1] += b.e[1]; a.e[2] += b.e[2]; });

// Subtraction
impl_op_ex!(-|a: &Point3, b: &Vec3| -> Point3 {
    Point3::new_flt(a.e[0] - b.e[0], a.e[1] - b.e[1], a.e[2] - b.e[2])
});
impl_op_ex!(-= |a: &mut Point3, b: &Vec3| { a.e[0] -= b.e[0]; a.e[1] -= b.e[1]; a.e[2] -= b.e[2]; });

// Multiplication
impl_op_ex_commutative!(*|a: &Point3, b: Flt| -> Point3 {
    Point3::new_flt(a.e[0] * b, a.e[1] * b, a.e[2] * b)
});
impl_op_ex!(*|a: &Point3, b: &Point3| -> Point3 {
    Point3::new_flt(a.e[0] * b.e[0], a.e[1] * b.e[1], a.e[2] * b.e[2])
});
impl_op_ex!(*= |a: &mut Point3, b: &Point3| {
    a.e[0] *= b.e[0];
    a.e[1] *= b.e[1];
    a.e[2] *= b.e[2];
});
impl_op_ex!(*= |a: &mut Point3, b: Flt| { a.e[0] *= b; a.e[1] *= b; a.e[2] *= b; });

// Division
impl_op_ex!(/ |a: &Point3, b: Flt| -> Point3 { a * (flt(1.0) / b) } );
impl_op_ex!(/= |a: &mut Point3, b: Flt| { *a *= flt(1.0) / b });

// -- Colour Operator implementations

// Addition
impl_op_ex!(+ |a: &Colour, b: &Colour| -> Colour { Colour::new_flt(a.e[0] + b.e[0], a.e[1] + b.e[1], a.e[2] + b.e[2]) });
impl_op_ex!(+= |a: &mut Colour, b: &Colour| { a.e[0] += b.e[0]; a.e[1] += b.e[1]; a.e[2] += b.e[2]; });

impl_op_ex_commutative!(+ |a: &Colour, b: &Vec3| -> Colour { Colour::new_flt(a.e[0] + b.e[0], a.e[1] + b.e[1], a.e[2] + b.e[2]) });
impl_op_ex!(+= |a: &mut Colour, b: &Vec3| { a.e[0] += b.e[0]; a.e[1] += b.e[1]; a.e[2] += b.e[2]; });

// Subtraction
impl_op_ex!(-|a: &Colour, b: &Vec3| -> Colour {
    Colour::new_flt(a.e[0] - b.e[0], a.e[1] - b.e[1], a.e[2] - b.e[2])
});
impl_op_ex!(-= |a: &mut Colour, b: &Vec3| { a.e[0] -= b.e[0]; a.e[1] -= b.e[1]; a.e[2] -= b.e[2]; });

// Multiplication
impl_op_ex_commutative!(*|a: &Colour, b: Flt| -> Colour {
    Colour::new_flt(a.e[0] * b, a.e[1] * b, a.e[2] * b)
});
impl_op_ex!(*|a: &Colour, b: &Colour| -> Colour {
    Colour::new_flt(a.e[0] * b.e[0], a.e[1] * b.e[1], a.e[2] * b.e[2])
});
impl_op_ex!(*= |a: &mut Colour, b: &Colour| {
    a.e[0] *= b.e[0];
    a.e[1] *= b.e[1];
    a.e[2] *= b.e[2];
});
impl_op_ex!(*= |a: &mut Colour, b: Flt| { a.e[0] *= b; a.e[1] *= b; a.e[2] *= b; });

// Division
impl_op_ex!(/ |a: &Colour, b: Flt| -> Colour { a * (flt(1.0) / b) } );
impl_op_ex!(/= |a: &mut Colour, b: Flt| { *a *= flt(1.0) / b });

// Summing
impl Sum for Colour {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|acc, i| acc + i)
            .unwrap_or_else(Colour::default)
    }
}

#[cfg(test)]
mod tests {
    use crate::float::*;

    use super::Vec3;

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
