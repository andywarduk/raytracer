//! Texture trait

use crate::{
    float::*,
    triple::{Colour, Point3},
};
use std::{fmt::Debug, ops::Deref};

/// Texture trait
pub trait Texture: Debug + Sync + Send {
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
