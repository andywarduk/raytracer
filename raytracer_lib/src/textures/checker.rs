//! Checkered texture

use crate::{
    float::*,
    triple::{Colour, Point3},
};

use super::{
    solid::Solid,
    texture::{TexRef, Texture},
};

/// Checker details
#[derive(Debug)]
pub struct Checker<'a> {
    inv_scale: Flt,
    even: TexRef<'a>,
    odd: TexRef<'a>,
}

impl<'a> Checker<'a> {
    /// Create a new checker with textures
    pub fn new_with_textures(scale: FltPrim, even: &'a dyn Texture, odd: &'a dyn Texture) -> Self {
        Self::new_with_texref(scale, TexRef::Borrow(even), TexRef::Borrow(odd))
    }

    /// Create a new checker with colours
    pub fn new_with_colours(scale: FltPrim, even: Colour, odd: Colour) -> Self {
        let even = TexRef::boxed(Solid::new(even));
        let odd = TexRef::boxed(Solid::new(odd));

        Self::new_with_texref(scale, even, odd)
    }

    /// Create a new checker with texture references
    pub fn new_with_texref(scale: FltPrim, even: TexRef<'a>, odd: TexRef<'a>) -> Self {
        Self {
            inv_scale: flt(1.0 / scale),
            even,
            odd,
        }
    }
}

impl<'a> Texture for Checker<'a> {
    fn value(&self, u: Flt, v: Flt, p: &Point3) -> Colour {
        let x: i64 = FltPrim::from((self.inv_scale * p.x()).floor()) as i64;
        let y: i64 = FltPrim::from((self.inv_scale * p.y()).floor()) as i64;
        let z: i64 = FltPrim::from((self.inv_scale * p.z()).floor()) as i64;

        if (x + y + z) & 0x1 == 0 {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}
