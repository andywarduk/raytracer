//! Texture trait

use rand::rngs::ThreadRng;
use std::{fmt::Debug, ops::Deref};

use crate::{
    float::*,
    triple::{Colour, Point3},
};

/// Texture trait
pub trait Texture: Debug + Sync + Send {
    /// Tests texture for a hit
    fn hit(&self, _rng: &mut ThreadRng, _u: Flt, _v: Flt, _p: &Point3) -> bool {
        true
    }

    /// Return the colour of a texture at a given point
    fn value(&self, u: Flt, v: Flt, p: &Point3) -> Colour;
}

/// Texture reference, either borrowed or owned
#[derive(Debug)]
pub enum TexRef<'a> {
    /// Borrowed texture reference
    Borrow(&'a dyn Texture),
    /// Owned texture
    Box(Box<dyn Texture + 'a>),
}

impl<'a> TexRef<'a> {
    /// Create new owned texture reference
    pub fn boxed(tex: impl Texture + 'a) -> Self {
        Self::Box(Box::new(tex))
    }

    /// Gets a reference to the texture
    pub fn get_ref(&'a self) -> &dyn Texture {
        match self {
            Self::Borrow(refer) => *refer,
            Self::Box(boxed) => boxed.as_ref(),
        }
    }
}

impl<'a> Deref for TexRef<'a> {
    type Target = dyn Texture + 'a;

    fn deref(&self) -> &Self::Target {
        match self {
            TexRef::Borrow(refer) => *refer,
            TexRef::Box(boxed) => boxed.as_ref(),
        }
    }
}
