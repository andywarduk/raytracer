use std::sync::Arc;

use crate::{colour::Colour, vec3::Point3};

use super::{solid::Solid, texture::Texture};

#[derive(Debug)]
pub struct Checker {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl Checker {
    pub fn new_with_textures(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn new_with_colours(scale: f64, even: Colour, odd: Colour) -> Self {
        let even = Arc::new(Solid::new(even));
        let odd = Arc::new(Solid::new(odd));

        Self::new_with_textures(scale, even, odd)
    }
}

impl Texture for Checker {
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
