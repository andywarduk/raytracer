use std::{fmt::Display, iter::Sum};

use crate::{float::*, gamma::Gamma};

use super::Triple;

use super::ops::*;
use auto_ops::*;

/// Colour mixin
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ColourMixin;

/// Colour type
pub type Colour = Triple<ColourMixin>;

/// Methods for colours
impl Colour {
    /// White constructor
    #[inline]
    pub fn new_white() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    /// Grey constructor
    #[inline]
    pub fn new_grey(level: FltPrim) -> Self {
        Self::new(level, level, level)
    }

    /// Convert to RGB with optional gamma correction
    #[inline]
    pub fn to_rgb(&self, gamma: &Gamma) -> (u8, u8, u8) {
        // Translate the [0,1] component values to the byte range [0,255].
        let (r, g, b) = match gamma {
            Gamma::None => (self[0], self[1], self[2]),
            Gamma::Power(factor) => (
                Self::linear_to_gamma(self[0], *factor),
                Self::linear_to_gamma(self[1], *factor),
                Self::linear_to_gamma(self[2], *factor),
            ),
        };

        (
            FltPrim::from(clamp(flt(256.0) * FltPrim::from(r), flt(0.0), flt(255.0))) as u8,
            FltPrim::from(clamp(flt(256.0) * FltPrim::from(g), flt(0.0), flt(255.0))) as u8,
            FltPrim::from(clamp(flt(256.0) * FltPrim::from(b), flt(0.0), flt(255.0))) as u8,
        )
    }

    #[inline]
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

// Colour operators
impl_op_add!(Colour);
impl_op_add_assign!(Colour);

impl_op_sub!(Colour);
impl_op_sub_assign!(Colour);

impl_op_mul!(Colour);
impl_op_mul_assign!(Colour);
impl_op_mul_float!(Colour);
impl_op_mul_assign_float!(Colour);

impl_op_div_float!(Colour);
impl_op_div_assign_float!(Colour);

// Colour summing
impl Sum for Colour {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|acc, i| acc + i)
            .unwrap_or_else(Colour::default)
    }
}
