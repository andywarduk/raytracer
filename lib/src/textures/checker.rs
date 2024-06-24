use crate::{colour::Colour, vec3::Point3};

use super::{
    solid::Solid,
    texture::{TexRef, Texture},
};

#[derive(Debug)]
pub struct Checker<'a> {
    inv_scale: f64,
    even: TexRef<'a>,
    odd: TexRef<'a>,
}

impl<'a> Checker<'a> {
    pub fn new_with_textures(scale: f64, even: &'a dyn Texture, odd: &'a dyn Texture) -> Self {
        Self::new_with_texref(scale, TexRef::Borrow(even), TexRef::Borrow(odd))
    }

    pub fn new_with_colours(scale: f64, even: Colour, odd: Colour) -> Self {
        let even = TexRef::boxed(Solid::new(even));
        let odd = TexRef::boxed(Solid::new(odd));

        Self::new_with_texref(scale, even, odd)
    }

    pub fn new_with_texref(scale: f64, even: TexRef<'a>, odd: TexRef<'a>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }
}

impl<'a> Texture for Checker<'a> {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Colour {
        let x = (self.inv_scale * p.x()).floor() as i64;
        let y = (self.inv_scale * p.y()).floor() as i64;
        let z = (self.inv_scale * p.z()).floor() as i64;

        if (x + y + z) & 0x1 == 0 {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}
