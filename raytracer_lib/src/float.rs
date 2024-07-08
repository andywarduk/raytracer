//! Floating point configuration

/// Float checker
#[cfg(any(float = "r64", float = "r32"))]
mod noisy_checker {
    pub use noisy_float::{prelude::*, FloatChecker};

    /// Float checker
    pub struct FltChecker;

    impl<F: Float> FloatChecker<F> for FltChecker {
        #[inline]
        fn assert(value: F) {
            assert!(Self::check(value), "unexpected NaN");
        }

        #[inline]
        fn check(value: F) -> bool {
            !value.is_nan()
        }
    }
}

// -- R64 --

#[cfg(float = "r64")]
mod noisy64 {
    pub use noisy_float::prelude::*;
    use noisy_float::NoisyFloat;

    use super::noisy_checker::FltChecker;

    /// Float type description
    pub const FLOAT_DESC: &str = "Real64";

    /// Float primitive type
    pub type FltPrim = f64;

    /// Float wrapper type
    pub type Flt = NoisyFloat<f64, FltChecker>;

    /// Pi for primitive float type
    pub const PI: FltPrim = std::f64::consts::PI;

    /// Convert primitive float to wrapped
    #[inline]
    pub fn flt(v: FltPrim) -> Flt {
        Flt::new(v)
    }

    /// Convert wrapped float to primitive
    #[inline]
    pub fn flt_prim(v: Flt) -> FltPrim {
        FltPrim::from(v)
    }

    /// Return minimum wrapped float
    #[inline]
    pub fn flt_min() -> Flt {
        Flt::new(FltPrim::MIN)
    }

    /// Return maximum wrapped float
    #[inline]
    pub fn flt_max() -> Flt {
        Flt::new(FltPrim::MAX)
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
    use noisy_float::NoisyFloat;

    use super::noisy_checker::FltChecker;

    /// Float type description
    pub const FLOAT_DESC: &str = "Real32";

    /// Float wrapper type
    pub type Flt = NoisyFloat<f32, FltChecker>;

    /// Float primitive type
    pub type FltPrim = f32;

    /// Pi for primitive float type
    pub const PI: FltPrim = std::f32::consts::PI;

    /// Convert primitive float to wrapped
    #[inline]
    pub fn flt(v: FltPrim) -> Flt {
        Flt::new(v)
    }

    /// Convert wrapped float to primitive
    #[inline]
    pub fn flt_prim(v: Flt) -> FltPrim {
        FltPrim::from(v)
    }

    /// Return minimum wrapped float
    #[inline]
    pub fn flt_min() -> Flt {
        Flt::new(FltPrim::MIN)
    }

    /// Return maximum wrapped float
    #[inline]
    pub fn flt_max() -> Flt {
        Flt::new(FltPrim::MAX)
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
