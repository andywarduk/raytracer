// -- R64 --

#[cfg(float = "r64")]
mod noisy64 {
    pub use noisy_float::prelude::*;

    pub const FLOAT_DESC: &str = "r64";

    pub type Flt = R64;
    pub type FltPrim = f64;

    pub const PI: FltPrim = std::f64::consts::PI;

    #[inline]
    pub fn flt(v: FltPrim) -> Flt {
        r64(v)
    }

    #[inline]
    pub fn flt_prim(v: Flt) -> FltPrim {
        FltPrim::from(v)
    }

    #[inline]
    pub fn flt_min() -> R64 {
        r64(FltPrim::MIN)
    }

    #[inline]
    pub fn flt_max() -> R64 {
        r64(FltPrim::MAX)
    }

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

    pub const FLOAT_DESC: &str = "r32";

    pub type Flt = R32;
    pub type FltPrim = f32;

    pub const PI: FltPrim = std::f32::consts::PI;

    #[inline]
    pub fn flt(v: FltPrim) -> Flt {
        r32(v)
    }

    #[inline]
    pub fn flt_prim(v: Flt) -> FltPrim {
        FltPrim::from(v)
    }

    #[inline]
    pub fn flt_min() -> R32 {
        r32(FltPrim::MIN)
    }

    #[inline]
    pub fn flt_max() -> R32 {
        r32(FltPrim::MAX)
    }

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
    pub const FLOAT_DESC: &str = "f64";

    pub type Flt = f64;
    pub type FltPrim = f64;

    pub const PI: FltPrim = std::f64::consts::PI;

    #[inline]
    pub fn flt(v: FltPrim) -> Flt {
        v
    }

    #[inline]
    pub fn flt_prim(v: Flt) -> FltPrim {
        v
    }

    #[inline]
    pub fn flt_min() -> Flt {
        FltPrim::MIN
    }

    #[inline]
    pub fn flt_max() -> Flt {
        FltPrim::MAX
    }

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
    pub const FLOAT_DESC: &str = "f32";

    pub type Flt = f32;
    pub type FltPrim = f32;

    pub const PI: FltPrim = std::f32::consts::PI;

    #[inline]
    pub fn flt(v: FltPrim) -> Flt {
        v
    }

    #[inline]
    pub fn flt_prim(v: Flt) -> FltPrim {
        v
    }

    #[inline]
    pub fn flt_min() -> Flt {
        FltPrim::MIN
    }

    #[inline]
    pub fn flt_max() -> Flt {
        FltPrim::MAX
    }

    #[inline]
    pub fn clamp(v: Flt, min: Flt, max: Flt) -> Flt {
        v.clamp(min, max)
    }
}

#[cfg(float = "f32")]
pub use quiet32::*;
