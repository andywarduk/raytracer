// -- R64 --

#[cfg(debug_assertions)]
mod noisy64 {
    pub use noisy_float::prelude::*;

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

#[cfg(debug_assertions)]
pub use noisy64::*;

// -- f64 --

#[cfg(not(debug_assertions))]
mod quiet64 {
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

#[cfg(not(debug_assertions))]
pub use quiet64::*;
