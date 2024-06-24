use crate::{colour::Colour, vec3::Point3};
use std::{fmt::Debug, ops::Deref};

pub trait Texture: Debug + Sync + Send {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Colour;
}

#[derive(Debug)]
pub enum TexRef<'a> {
    Borrow(&'a dyn Texture),
    Box(Box<dyn Texture + 'a>),
}

impl<'a> TexRef<'a> {
    pub fn boxed(tex: impl Texture + 'a) -> Self {
        Self::Box(Box::new(tex))
    }

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
