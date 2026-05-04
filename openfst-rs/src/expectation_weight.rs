use crate::ffi::WeightFfi;
use crate::ffi::expectation_weight::*;
use crate::ffi::weight::{PairWeightValueF32, PairWeightValueF64};
use crate::float_weight::{
    Log64Weight, LogWeight, MinMaxWeight, MinMaxWeight64, Real64Weight, RealWeight,
    SignedLog64Weight, SignedLogWeight, TropicalWeight, TropicalWeight64,
};
use crate::weight::{COMMUTATIVE, DivideType, IDEMPOTENT, LEFT_SEMIRING, RIGHT_SEMIRING, Weight};
use crate::weight::{CommutativeWeight, IdempotentWeight, LeftSemiring, RightSemiring};

macro_rules! impl_expectation_traits {
    ($name:ident, $inner_val_type:ident, $val_type:ident, $inner_weight:ident, $type_name:expr $(, $idempotent:ident)?) => {
        impl Eq for $name {}
        impl LeftSemiring for $name {}
        impl RightSemiring for $name {}
        impl CommutativeWeight for $name {}
        $(impl $idempotent for $name {})*

        #[allow(clippy::derived_hash_with_manual_eq)]
        impl std::hash::Hash for $name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                $inner_weight::from_ffi(self.get_w1()).hash(state);
                $inner_weight::from_ffi(self.get_w2()).hash(state);
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{},{}", $inner_weight::from_ffi(self.get_w1()), $inner_weight::from_ffi(self.get_w2()))
            }
        }

        impl std::str::FromStr for $name {
            type Err = std::num::ParseFloatError;
            fn from_str(_s: &str) -> Result<Self, Self::Err> {
                "invalid".parse::<f32>().map(|_| Self::zero())
            }
        }

        impl Weight for $name {
            type ReverseWeight = Self;

            #[inline(always)] fn zero() -> Self {
                let z = <$inner_weight as Weight>::zero();
                Self::from_inner(z.value(), z.value())
            }
            #[inline(always)] fn one() -> Self {
                let o = <$inner_weight as Weight>::one();
                let z = <$inner_weight as Weight>::zero();
                Self::from_inner(o.value(), z.value())
            }
            #[inline(always)] fn no_weight() -> Self {
                let nw = <$inner_weight as Weight>::no_weight();
                Self::from_inner(nw.value(), nw.value())
            }
            fn type_name() -> &'static str { $type_name }
            fn properties() -> u64 {
                <$inner_weight as Weight>::properties() & (LEFT_SEMIRING | RIGHT_SEMIRING | COMMUTATIVE | IDEMPOTENT)
            }

            #[inline(always)] fn is_member(&self) -> bool {
                $inner_weight::from_ffi(self.get_w1()).is_member() && $inner_weight::from_ffi(self.get_w2()).is_member()
            }
            #[inline(always)] fn approx_equal(w1: &Self, w2: &Self, delta: f32) -> bool {
                $inner_weight::approx_equal(&$inner_weight::from_ffi(w1.get_w1()), &$inner_weight::from_ffi(w2.get_w1()), delta) &&
                $inner_weight::approx_equal(&$inner_weight::from_ffi(w1.get_w2()), &$inner_weight::from_ffi(w2.get_w2()), delta)
            }
            fn quantize(w: &Self, delta: f32) -> Self {
                let q1 = $inner_weight::quantize(&$inner_weight::from_ffi(w.get_w1()), delta);
                let q2 = $inner_weight::quantize(&$inner_weight::from_ffi(w.get_w2()), delta);
                Self::from_inner(q1.value(), q2.value())
            }
            #[inline(always)] fn reverse(w: &Self) -> Self::ReverseWeight {
                let r1 = $inner_weight::reverse(&$inner_weight::from_ffi(w.get_w1()));
                let r2 = $inner_weight::reverse(&$inner_weight::from_ffi(w.get_w2()));
                Self::from_inner(r1.value(), r2.value())
            }

            #[inline(always)] fn plus(w1: &Self, w2: &Self) -> Self {
                let p1 = $inner_weight::plus(&$inner_weight::from_ffi(w1.get_w1()), &$inner_weight::from_ffi(w2.get_w1()));
                let p2 = $inner_weight::plus(&$inner_weight::from_ffi(w1.get_w2()), &$inner_weight::from_ffi(w2.get_w2()));
                Self::from_inner(p1.value(), p2.value())
            }

            #[inline(always)] fn times(w1: &Self, w2: &Self) -> Self {
                let a1 = $inner_weight::from_ffi(w1.get_w1()); let b1 = $inner_weight::from_ffi(w1.get_w2());
                let a2 = $inner_weight::from_ffi(w2.get_w1()); let b2 = $inner_weight::from_ffi(w2.get_w2());

                let t1 = $inner_weight::times(&a1, &a2);
                let cross1 = $inner_weight::times(&a1, &b2);
                let cross2 = $inner_weight::times(&a2, &b1);
                let t2 = $inner_weight::plus(&cross1, &cross2);

                Self::from_inner(t1.value(), t2.value())
            }

            #[inline(always)] fn divide(w1: &Self, w2: &Self, typ: DivideType) -> Self {
                let a1 = $inner_weight::from_ffi(w1.get_w1()); let b1 = $inner_weight::from_ffi(w1.get_w2());
                let a2 = $inner_weight::from_ffi(w2.get_w1()); let b2 = $inner_weight::from_ffi(w2.get_w2());
                let q1 = $inner_weight::divide(&a1, &a2, typ);

                if typ == DivideType::Left {
                    let q2 = $inner_weight::divide(&$inner_weight::minus(&b1, &$inner_weight::times(&b2, &q1)), &a2, typ);
                    Self::from_inner(q1.value(), q2.value())
                } else {
                    let q2 = $inner_weight::divide(&$inner_weight::minus(&b1, &$inner_weight::times(&q1, &b2)), &a2, typ);
                    Self::from_inner(q1.value(), q2.value())
                }
            }
        }

        impl WeightFfi for $name {
            type ValueType = $val_type;
            #[inline(always)] fn from_ffi(val: Self::ValueType) -> Self { Self(val) }
            #[inline(always)] fn as_ffi(&self) -> Self::ValueType { self.0 }
        }
    };
}

macro_rules! define_expectation {
    ($name:ident, $t:ident, $val_type:ident, $inner_weight:ident, $type_name:expr $(, $idempotent:ident)?) => {
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $name(pub $val_type);

        impl $name {
            #[inline(always)] pub const fn value(&self) -> $val_type { self.0 }
            #[inline(always)] pub const fn new(w1: $t, w2: $t) -> Self { Self($val_type { w1, w2 }) }
            #[inline(always)] fn from_inner(w1: $t, w2: $t) -> Self { Self($val_type { w1, w2 }) }
            #[inline(always)] fn get_w1(&self) -> $t { self.0.w1 }
            #[inline(always)] fn get_w2(&self) -> $t { self.0.w2 }
        }
        impl_expectation_traits!($name, $t, $val_type, $inner_weight, $type_name $(, $idempotent)?);
    };
}

macro_rules! define_expectation_pair {
    ($name:ident, $t:ident, $val_type:ident, $inner_weight:ident, $type_name:expr $(, $idempotent:ident)?) => {
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $name(pub $val_type);

        impl $name {
            #[inline(always)] pub const fn value(&self) -> $val_type { self.0 }
            #[inline(always)] pub const fn new(w1: $t, w2: $t) -> Self {
                Self($val_type {
                    w1_1: w1.w1 as _, w1_2: w1.w2,
                    w2_1: w2.w1 as _, w2_2: w2.w2
                })
            }
            #[inline(always)] fn from_inner(w1: $t, w2: $t) -> Self {
                Self($val_type { w1_1: w1.w1 as _, w1_2: w1.w2, w2_1: w2.w1 as _, w2_2: w2.w2 })
            }
            #[inline(always)] fn get_w1(&self) -> $t { $t { w1: self.0.w1_1 as _, w2: self.0.w1_2 } }
            #[inline(always)] fn get_w2(&self) -> $t { $t { w1: self.0.w2_1 as _, w2: self.0.w2_2 } }
        }
        impl_expectation_traits!($name, $t, $val_type, $inner_weight, $type_name $(, $idempotent)?);
    };
}

define_expectation!(
    ExpectationTropicalWeight,
    f32,
    ExpectationWeightValueF32,
    TropicalWeight,
    "expectation_tropical_tropical",
    IdempotentWeight
);
define_expectation!(
    ExpectationTropical64Weight,
    f64,
    ExpectationWeightValueF64,
    TropicalWeight64,
    "expectation_tropical64_tropical64",
    IdempotentWeight
);

define_expectation!(
    ExpectationLogWeight,
    f32,
    ExpectationWeightValueF32,
    LogWeight,
    "expectation_log_log"
);
define_expectation!(
    ExpectationLog64Weight,
    f64,
    ExpectationWeightValueF64,
    Log64Weight,
    "expectation_log64_log64"
);

define_expectation!(
    ExpectationRealWeight,
    f32,
    ExpectationWeightValueF32,
    RealWeight,
    "expectation_real_real"
);
define_expectation!(
    ExpectationReal64Weight,
    f64,
    ExpectationWeightValueF64,
    Real64Weight,
    "expectation_real64_real64"
);

define_expectation!(
    ExpectationMinMaxWeight,
    f32,
    ExpectationWeightValueF32,
    MinMaxWeight,
    "expectation_minmax_minmax",
    IdempotentWeight
);
define_expectation!(
    ExpectationMinMax64Weight,
    f64,
    ExpectationWeightValueF64,
    MinMaxWeight64,
    "expectation_minmax64_minmax64",
    IdempotentWeight
);

define_expectation_pair!(
    ExpectationSignedLogWeight,
    PairWeightValueF32,
    ExpectationWeightValuePairF32,
    SignedLogWeight,
    "expectation_signed_log_tropical_log_signed_log_tropical_log"
);
define_expectation_pair!(
    ExpectationSignedLog64Weight,
    PairWeightValueF64,
    ExpectationWeightValuePairF64,
    SignedLog64Weight,
    "expectation_signed_log_tropical_log64_signed_log_tropical_log64"
);
