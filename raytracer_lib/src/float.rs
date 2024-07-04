//! Floating point configuration

// -- R64 --

#[cfg(float = "r64")]
mod noisy64 {
    pub use noisy_float::prelude::*;

    /// Float type description
    pub const FLOAT_DESC: &str = "r64";

    /// Float wrapper type
    pub type Flt = R64;
    /// Float primitive type
    pub type FltPrim = f64;

    /// Pi for primitive float type
    pub const PI: FltPrim = std::f64::consts::PI;

    /// Convert primitive float to wrapped
    #[inline]
    pub fn flt(v: FltPrim) -> Flt {
        r64(v)
    }

    /// Convert wrapped float to primitive
    #[inline]
    pub fn flt_prim(v: Flt) -> FltPrim {
        FltPrim::from(v)
    }

    /// Return minimum wrapped float
    #[inline]
    pub fn flt_min() -> R64 {
        r64(FltPrim::MIN)
    }

    /// Return maximum wrapped float
    #[inline]
    pub fn flt_max() -> R64 {
        r64(FltPrim::MAX)
    }

    /// Clamp wrapped float
    #[inline]
    pub fn clamp(v: Flt, min: Flt, max: Flt) -> Flt {
        Ord::clamp(v, min, max)
    }
}

#[cfg(float = "r64")]
pub use noisy64::*;

// -- R32 --

#[cfg(float = "r32")]
mod noisy32 {
    pub use noisy_float::prelude::*;

    /// Float type description
    pub const FLOAT_DESC: &str = "r32";

    /// Float wrapper type
    pub type Flt = R32;
    /// Float primitive type
    pub type FltPrim = f32;

    /// Pi for primitive float type
    pub const PI: FltPrim = std::f32::consts::PI;

    /// Convert primitive float to wrapped
    #[inline]
    pub fn flt(v: FltPrim) -> Flt {
        r32(v)
    }

    /// Convert wrapped float to primitive
    #[inline]
    pub fn flt_prim(v: Flt) -> FltPrim {
        FltPrim::from(v)
    }

    /// Return minimum wrapped float
    #[inline]
    pub fn flt_min() -> R32 {
        r32(FltPrim::MIN)
    }

    /// Return maximum wrapped float
    #[inline]
    pub fn flt_max() -> R32 {
        r32(FltPrim::MAX)
    }

    /// Clamp wrapped float
    #[inline]
    pub fn clamp(v: Flt, min: Flt, max: Flt) -> Flt {
        Ord::clamp(v, min, max)
    }
}

#[cfg(float = "r32")]
pub use noisy32::*;

// -- f64 --

#[cfg(float = "f64")]
mod quiet64 {
    /// Float type description
    pub const FLOAT_DESC: &str = "f64";

    /// Float wrapper type
    pub type Flt = f64;
    /// Float primitive type
    pub type FltPrim = f64;

    /// Pi for primitive float type
    pub const PI: FltPrim = std::f64::consts::PI;

    /// Convert primitive float to wrapped
    #[inline]
    pub fn flt(v: FltPrim) -> Flt {
        v
    }

    /// Convert wrapped float to primitive
    #[inline]
    pub fn flt_prim(v: Flt) -> FltPrim {
        v
    }

    /// Return minimum wrapped float
    #[inline]
    pub fn flt_min() -> Flt {
        FltPrim::MIN
    }

    /// Return maximum wrapped float
    #[inline]
    pub fn flt_max() -> Flt {
        FltPrim::MAX
    }

    /// Clamp wrapped float
    #[inline]
    pub fn clamp(v: Flt, min: Flt, max: Flt) -> Flt {
        v.clamp(min, max)
    }
}

#[cfg(float = "f64")]
pub use quiet64::*;

// -- f32 --

#[cfg(float = "f32")]
mod quiet32 {
    /// Float type description
    pub const FLOAT_DESC: &str = "f32";

    /// Float wrapper type
    pub type Flt = f32;
    /// Float primitive type
    pub type FltPrim = f32;

    /// Pi for primitive float type
    pub const PI: FltPrim = std::f32::consts::PI;

    /// Convert primitive float to wrapped
    #[inline]
    pub fn flt(v: FltPrim) -> Flt {
        v
    }

    /// Convert wrapped float to primitive
    #[inline]
    pub fn flt_prim(v: Flt) -> FltPrim {
        v
    }

    /// Return minimum wrapped float
    #[inline]
    pub fn flt_min() -> Flt {
        FltPrim::MIN
    }

    /// Return maximum wrapped float
    #[inline]
    pub fn flt_max() -> Flt {
        FltPrim::MAX
    }

    /// Clamp wrapped float
    #[inline]
    pub fn clamp(v: Flt, min: Flt, max: Flt) -> Flt {
        v.clamp(min, max)
    }
}

#[cfg(float = "f32")]
pub use quiet32::*;
