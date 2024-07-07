#![allow(unused_imports)]
#![allow(unused_macros)]

macro_rules! impl_op_add {
    ($type:ident) => {
        // Addition
        impl_op_ex!(+ |a: &$type, b: &$type| -> $type { $type::new_from_array([a[0] + b[0], a[1] + b[1], a[2] + b[2]]) });
    }
}

macro_rules! impl_op_add_typed {
    ($type1:ident, $type2:ident, $rtype:ident) => {
        // Addition of different triple
        impl_op_ex!(+ |a: &$type1, b: &$type2| -> $rtype { $rtype::new_from_array([a[0] + b[0], a[1] + b[1], a[2] + b[2]]) });
    }
}

macro_rules! impl_op_add_assign {
    ($type:ident) => {
        // Add assign
        impl_op_ex!(+= |a: &mut $type, b: &$type| { a.e[0] += b[0]; a.e[1] += b[1]; a.e[2] += b[2]; });
    }
}

macro_rules! impl_op_add_assign_typed {
    ($type1:ident, $type2:ident) => {
        // Add assign of different triple
        impl_op_ex!(+= |a: &mut $type1, b: &$type2| { a.e[0] += b[0]; a.e[1] += b[1]; a.e[2] += b[2]; });
    }
}

macro_rules! impl_op_add_float {
    ($type:ident) => {
        // Add float to triple
        impl_op_ex_commutative!(+ |a: &$type, b: &Flt| -> $type { $type::new_from_array([a[0] + b, a[1] + b, a[2] + b]) });
    }
}

macro_rules! impl_op_add_assign_float {
    ($type:ident) => {
        // Add assign float to triple
        impl_op_ex!(+= |a: &mut $type, b: &Flt| { a.e[0] += b; a.e[1] += b; a.e[2] += b; });
    }
}

macro_rules! impl_op_sub {
    ($type:ident) => {
        // Subtraction
        impl_op_ex!(-|a: &$type, b: &$type| -> $type {
            $type::new_from_array([a[0] - b[0], a[1] - b[1], a[2] - b[2]])
        });
    };
}

macro_rules! impl_op_sub_typed {
    ($type1:ident, $type2:ident, $rtype:ident) => {
        // Subtraction of different triple
        impl_op_ex!(-|a: &$type1, b: &$type2| -> $rtype {
            $rtype::new_from_array([a[0] - b[0], a[1] - b[1], a[2] - b[2]])
        });
    };
}

macro_rules! impl_op_sub_assign {
    ($type:ident) => {
        // Subtraction assign
        impl_op_ex!(-= |a: &mut $type, b: &$type| { a.e[0] -= b[0]; a.e[1] -= b[1]; a.e[2] -= b[2]; });
    }
}

macro_rules! impl_op_sub_assign_typed {
    ($type1:ident, $type2:ident) => {
        // Sub assign of different triple
        impl_op_ex!(-= |a: &mut $type1, b: &$type2| { a.e[0] -= b[0]; a.e[1] -= b[1]; a.e[2] -= b[2]; });
    }
}

macro_rules! impl_op_sub_float {
    ($type:ident) => {
        // Subtraction of float
        impl_op_ex!(-|a: &$type, b: &Flt| -> $type {
            $type::new_from_array([a[0] - b, a[1] - b, a[2] - b])
        });
    };
}

macro_rules! impl_op_sub_assign_float {
    ($type:ident) => {
        // Subtraction assign of float
        impl_op_ex!(-= |a: &mut $type, b: &Flt| { a.e[0] -= b; a.e[1] -= b; a.e[2] -= b; });
    }
}

macro_rules! impl_op_mul {
    ($type:ident) => {
        // Multiplication
        impl_op_ex!(*|a: &$type, b: &$type| -> $type {
            $type::new_from_array([a[0] * b[0], a[1] * b[1], a[2] * b[2]])
        });
    };
}

macro_rules! impl_op_mul_typed {
    ($type1:ident, $type2:ident, $rtype:ident) => {
        // Multiplication of different triple
        impl_op_ex!(*|a: &$type1, b: &$type2| -> $rtype {
            $rtype::new_from_array([a[0] * b[0], a[1] * b[1], a[2] * b[2]])
        });
    };
}

macro_rules! impl_op_mul_assign {
    ($type:ident) => {
        // Multiplication assign
        impl_op_ex!(*= |a: &mut $type, b: &$type| { a.e[0] *= b[0]; a.e[1] *= b[1]; a.e[2] *= b[2]; });
    }
}

macro_rules! impl_op_mul_assign_typed {
    ($type1:ident, $type2:ident) => {
        // Multiplication assign of different triple
        impl_op_ex!(*= |a: &mut $type1, b: &$type2| { a.e[0] *= b[0]; a.e[1] *= b[1]; a.e[2] *= b[2]; });
    }
}

macro_rules! impl_op_mul_float {
    ($type:ident) => {
        // Multiplication with float
        impl_op_ex_commutative!(*|a: &$type, b: Flt| -> $type {
            $type::new_from_array([a[0] * b, a[1] * b, a[2] * b])
        });
    };
}

macro_rules! impl_op_mul_assign_float {
    ($type:ident) => {
        // Multiplication assign with float
        impl_op_ex!(*= |a: &mut $type, b: Flt| { a.e[0] *= b; a.e[1] *= b; a.e[2] *= b; });
    }
}

macro_rules! impl_op_div_float {
    ($type:ident) => {
        // Division with float
        impl_op_ex!(/ |a: &$type, b: Flt| -> $type { a * b.recip() } );
    }
}

macro_rules! impl_op_div_assign_float {
    ($type:ident) => {
        // Division assign with float
        impl_op_ex!(/= |a: &mut $type, b: Flt| { *a *= b.recip() });
    };
}

macro_rules! impl_op_neg {
    ($type:ident) => {
        // Negation
        impl_op_ex!(-|a: &$type| -> $type { $type::new_from_array([-a[0], -a[1], -a[2]]) });
    };
}

pub(crate) use impl_op_add;
pub(crate) use impl_op_add_assign;
pub(crate) use impl_op_add_assign_float;
pub(crate) use impl_op_add_assign_typed;
pub(crate) use impl_op_add_float;
pub(crate) use impl_op_add_typed;

pub(crate) use impl_op_sub;
pub(crate) use impl_op_sub_assign;
pub(crate) use impl_op_sub_assign_float;
pub(crate) use impl_op_sub_assign_typed;
pub(crate) use impl_op_sub_float;
pub(crate) use impl_op_sub_typed;

pub(crate) use impl_op_mul;
pub(crate) use impl_op_mul_assign;
pub(crate) use impl_op_mul_assign_float;
pub(crate) use impl_op_mul_assign_typed;
pub(crate) use impl_op_mul_float;
pub(crate) use impl_op_mul_typed;

pub(crate) use impl_op_div_assign_float;
pub(crate) use impl_op_div_float;

pub(crate) use impl_op_neg;
