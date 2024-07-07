//! Vector, point and colour classes

use std::marker::PhantomData;
use std::ops::Index;

use rand::{rngs::ThreadRng, Rng};

use crate::float::*;

mod colour;
mod ops;
mod point;
mod vec;

pub use colour::Colour;
pub use point::Point3;
pub use vec::Vec3;

/// Base class for triple
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Triple<SubClassMixin> {
    /// Values
    pub e: [Flt; 3],
    phantom: PhantomData<SubClassMixin>,
}

/// Common methods
impl<Mixin> Triple<Mixin> {
    /// Create new triple from values of float type
    #[inline]
    pub fn new_flt(e1: Flt, e2: Flt, e3: Flt) -> Self {
        Self::new_from_array([e1, e2, e3])
    }

    /// Create new triple from values of primary float type
    #[inline]
    pub fn new(e1: FltPrim, e2: FltPrim, e3: FltPrim) -> Self {
        Self::new_from_array([flt(e1), flt(e2), flt(e3)])
    }

    /// Create new triple from a fixed array
    #[inline]
    pub fn new_from_array(e: [Flt; 3]) -> Self {
        Self {
            e,
            phantom: PhantomData,
        }
    }

    /// Creates a new random triple with values in the range 0.0 to 1.0
    #[inline]
    pub fn new_random(rng: &mut ThreadRng) -> Self {
        Self::new_random_clamped(rng, 0.0, 1.0)
    }

    /// Creates a new random triple with values in the given range
    #[inline]
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
        self[0]
    }

    /// Returns the y value
    #[inline]
    pub fn y(&self) -> Flt {
        self[1]
    }

    /// Returns the z value
    #[inline]
    pub fn z(&self) -> Flt {
        self[2]
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
            self[0], self[1], self[2]
        ))
    }
}

// Indexing
impl<Mixin> Index<usize> for Triple<Mixin> {
    type Output = Flt;

    #[inline]
    fn index(&self, i: usize) -> &Flt {
        &self.e[i]
    }
}
