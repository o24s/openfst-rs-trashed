use crate::ffi::WeightFfi;
use crate::ffi::lexicographic_weight::{LexicographicWeightValueF32, LexicographicWeightValueF64};
use crate::weight::{
    COMMUTATIVE, DivideType, IDEMPOTENT, LEFT_SEMIRING, PATH, RIGHT_SEMIRING, Weight,
};
use crate::weight::{CommutativeWeight, IdempotentWeight, LeftSemiring, PathWeight, RightSemiring};

macro_rules! define_lexicographic {
    ($name:ident, $t:ident, $val_type:ident, $type_name:expr) => {
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $name(pub $val_type);

        impl $name {
            #[inline(always)]
            pub const fn value(&self) -> $val_type {
                self.0
            }

            /// Creates a new lexicographic weight from two constituent weights (W1, W2).
            #[inline(always)]
            pub const fn new(w1: $t, w2: $t) -> Self {
                Self($val_type { w1, w2 })
            }
        }

        impl Eq for $name {}

        impl LeftSemiring for $name {}
        impl RightSemiring for $name {}
        impl CommutativeWeight for $name {}
        impl PathWeight for $name {}
        impl IdempotentWeight for $name {}

        #[allow(clippy::derived_hash_with_manual_eq)]
        impl std::hash::Hash for $name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.w1.to_bits().hash(state);
                self.0.w2.to_bits().hash(state);
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let w1_str = if self.0.w1 == $t::INFINITY {
                    "Infinity".to_string()
                } else if self.0.w1 == $t::NEG_INFINITY {
                    "-Infinity".to_string()
                } else if self.0.w1.is_nan() {
                    "BadNumber".to_string()
                } else {
                    self.0.w1.to_string()
                };
                let w2_str = if self.0.w2 == $t::INFINITY {
                    "Infinity".to_string()
                } else if self.0.w2 == $t::NEG_INFINITY {
                    "-Infinity".to_string()
                } else if self.0.w2.is_nan() {
                    "BadNumber".to_string()
                } else {
                    self.0.w2.to_string()
                };
                write!(f, "{},{}", w1_str, w2_str)
            }
        }

        impl std::str::FromStr for $name {
            type Err = std::num::ParseFloatError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let parts: Vec<&str> = s.split(',').collect();
                if parts.len() != 2 {
                    return "invalid".parse::<$t>().map(|_| Self::zero());
                }

                let parse_t = |s: &str| -> Result<$t, std::num::ParseFloatError> {
                    match s {
                        "Infinity" => Ok($t::INFINITY),
                        "-Infinity" => Ok($t::NEG_INFINITY),
                        "BadNumber" => Ok($t::NAN),
                        _ => s.parse::<$t>(),
                    }
                };

                let w1 = parse_t(parts[0])?;
                let w2 = parse_t(parts[1])?;
                Ok(Self($val_type { w1, w2 }))
            }
        }

        impl Weight for $name {
            type ReverseWeight = Self;

            #[inline(always)]
            fn zero() -> Self {
                Self($val_type {
                    w1: $t::INFINITY,
                    w2: $t::INFINITY,
                })
            }
            #[inline(always)]
            fn one() -> Self {
                Self($val_type { w1: 0.0, w2: 0.0 })
            }
            #[inline(always)]
            fn no_weight() -> Self {
                Self($val_type {
                    w1: $t::NAN,
                    w2: $t::NAN,
                })
            }
            fn type_name() -> &'static str {
                $type_name
            }
            fn properties() -> u64 {
                LEFT_SEMIRING | RIGHT_SEMIRING | COMMUTATIVE | PATH | IDEMPOTENT
            }
            #[inline(always)]
            fn is_member(&self) -> bool {
                // Lexicographic weights cannot mix zeroes and non-zeroes.
                let w1_member = self.0.w1 > $t::NEG_INFINITY && !self.0.w1.is_nan();
                let w2_member = self.0.w2 > $t::NEG_INFINITY && !self.0.w2.is_nan();
                if !w1_member || !w2_member {
                    return false;
                }
                let w1_zero = self.0.w1 == $t::INFINITY;
                let w2_zero = self.0.w2 == $t::INFINITY;
                if w1_zero && w2_zero {
                    return true;
                }
                if !w1_zero && !w2_zero {
                    return true;
                }
                false
            }
            #[inline(always)]
            fn approx_equal(w1: &Self, w2: &Self, delta: f32) -> bool {
                let w1_1 = w1.0.w1;
                let w1_2 = w1.0.w2;
                let w2_1 = w2.0.w1;
                let w2_2 = w2.0.w2;
                (w1_1 <= w2_1 + (delta as $t) && w2_1 <= w1_1 + (delta as $t))
                    && (w1_2 <= w2_2 + (delta as $t) && w2_2 <= w1_2 + (delta as $t))
            }
            fn quantize(w: &Self, delta: f32) -> Self {
                if !w.is_member() || w.0.w1 == $t::INFINITY {
                    *w
                } else {
                    Self($val_type {
                        w1: (((w.0.w1 as f32) / delta) + 0.5).floor() as $t * (delta as $t),
                        w2: (((w.0.w2 as f32) / delta) + 0.5).floor() as $t * (delta as $t),
                    })
                }
            }
            #[inline(always)]
            fn reverse(w: &Self) -> Self::ReverseWeight {
                *w
            }
            #[inline(always)]
            fn plus(w1: &Self, w2: &Self) -> Self {
                if !w1.is_member() || !w2.is_member() {
                    return Self::no_weight();
                }
                if w1.0.w1 < w2.0.w1 {
                    *w1
                } else if w2.0.w1 < w1.0.w1 {
                    *w2
                } else if w1.0.w2 < w2.0.w2 {
                    *w1
                } else if w2.0.w2 < w1.0.w2 {
                    *w2
                } else {
                    *w1
                }
            }
            #[inline(always)]
            fn times(w1: &Self, w2: &Self) -> Self {
                Self($val_type {
                    w1: w1.0.w1 + w2.0.w1,
                    w2: w1.0.w2 + w2.0.w2,
                })
            }
            #[inline(always)]
            fn divide(w1: &Self, w2: &Self, _typ: DivideType) -> Self {
                if w2.is_member() {
                    Self($val_type {
                        w1: w1.0.w1 - w2.0.w1,
                        w2: w1.0.w2 - w2.0.w2,
                    })
                } else {
                    Self::no_weight()
                }
            }
        }

        impl WeightFfi for $name {
            type ValueType = $val_type;
            #[inline(always)]
            fn from_ffi(val: Self::ValueType) -> Self {
                Self(val)
            }
            #[inline(always)]
            fn as_ffi(&self) -> Self::ValueType {
                self.0
            }
        }
    };
}

define_lexicographic!(
    LexicographicWeight,
    f32,
    LexicographicWeightValueF32,
    "tropical_LT_tropical"
);
define_lexicographic!(
    Lexicographic64Weight,
    f64,
    LexicographicWeightValueF64,
    "tropical64_LT_tropical64"
);

macro_rules! define_lexicographic_minmax_tropical {
    ($name:ident, $t:ident, $val_type:ident, $type_name:expr) => {
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $name(pub $val_type);

        impl $name {
            #[inline(always)]
            pub const fn value(&self) -> $val_type {
                self.0
            }

            #[inline(always)]
            pub const fn new(w1: $t, w2: $t) -> Self {
                Self($val_type { w1, w2 })
            }
        }

        impl Eq for $name {}
        impl LeftSemiring for $name {}
        impl RightSemiring for $name {}
        impl CommutativeWeight for $name {}
        impl PathWeight for $name {}
        impl IdempotentWeight for $name {}

        #[allow(clippy::derived_hash_with_manual_eq)]
        impl std::hash::Hash for $name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.w1.to_bits().hash(state);
                self.0.w2.to_bits().hash(state);
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let w1_str = if self.0.w1 == $t::INFINITY {
                    "Infinity".to_string()
                } else if self.0.w1 == $t::NEG_INFINITY {
                    "-Infinity".to_string()
                } else if self.0.w1.is_nan() {
                    "BadNumber".to_string()
                } else {
                    self.0.w1.to_string()
                };
                let w2_str = if self.0.w2 == $t::INFINITY {
                    "Infinity".to_string()
                } else if self.0.w2 == $t::NEG_INFINITY {
                    "-Infinity".to_string()
                } else if self.0.w2.is_nan() {
                    "BadNumber".to_string()
                } else {
                    self.0.w2.to_string()
                };
                write!(f, "{},{}", w1_str, w2_str)
            }
        }

        impl std::str::FromStr for $name {
            type Err = std::num::ParseFloatError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let parts: Vec<&str> = s.split(',').collect();
                if parts.len() != 2 {
                    return "invalid".parse::<$t>().map(|_| Self::zero());
                }
                let parse_t = |s: &str| -> Result<$t, std::num::ParseFloatError> {
                    match s {
                        "Infinity" => Ok($t::INFINITY),
                        "-Infinity" => Ok($t::NEG_INFINITY),
                        "BadNumber" => Ok($t::NAN),
                        _ => s.parse::<$t>(),
                    }
                };
                Ok(Self($val_type {
                    w1: parse_t(parts[0])?,
                    w2: parse_t(parts[1])?,
                }))
            }
        }

        impl Weight for $name {
            type ReverseWeight = Self;

            #[inline(always)]
            fn zero() -> Self {
                Self($val_type {
                    w1: $t::INFINITY,
                    w2: $t::INFINITY,
                })
            }
            #[inline(always)]
            fn one() -> Self {
                Self($val_type {
                    w1: $t::NEG_INFINITY,
                    w2: 0.0,
                })
            }
            #[inline(always)]
            fn no_weight() -> Self {
                Self($val_type {
                    w1: $t::NAN,
                    w2: $t::NAN,
                })
            }
            fn type_name() -> &'static str {
                $type_name
            }
            fn properties() -> u64 {
                LEFT_SEMIRING | RIGHT_SEMIRING | COMMUTATIVE | PATH | IDEMPOTENT
            }

            #[inline(always)]
            fn is_member(&self) -> bool {
                let w1_member = !self.0.w1.is_nan();
                let w2_member = self.0.w2 > $t::NEG_INFINITY && !self.0.w2.is_nan();
                if !w1_member || !w2_member {
                    return false;
                }
                let w1_zero = self.0.w1 == $t::INFINITY;
                let w2_zero = self.0.w2 == $t::INFINITY;
                if w1_zero && w2_zero {
                    return true;
                }
                if !w1_zero && !w2_zero {
                    return true;
                }
                false
            }
            #[inline(always)]
            fn approx_equal(w1: &Self, w2: &Self, delta: f32) -> bool {
                let w1_1 = w1.0.w1;
                let w1_2 = w1.0.w2;
                let w2_1 = w2.0.w1;
                let w2_2 = w2.0.w2;
                (w1_1 <= w2_1 + (delta as $t) && w2_1 <= w1_1 + (delta as $t))
                    && (w1_2 <= w2_2 + (delta as $t) && w2_2 <= w1_2 + (delta as $t))
            }
            fn quantize(w: &Self, delta: f32) -> Self {
                if !w.is_member() || w.0.w1 == $t::INFINITY || w.0.w1 == $t::NEG_INFINITY {
                    *w
                } else {
                    Self($val_type {
                        w1: (((w.0.w1 as f32) / delta) + 0.5).floor() as $t * (delta as $t),
                        w2: (((w.0.w2 as f32) / delta) + 0.5).floor() as $t * (delta as $t),
                    })
                }
            }
            #[inline(always)]
            fn reverse(w: &Self) -> Self::ReverseWeight {
                *w
            }

            #[inline(always)]
            fn plus(w1: &Self, w2: &Self) -> Self {
                if !w1.is_member() || !w2.is_member() {
                    return Self::no_weight();
                }
                if w1.0.w1 < w2.0.w1 {
                    *w1
                } else if w2.0.w1 < w1.0.w1 {
                    *w2
                } else if w1.0.w2 < w2.0.w2 {
                    *w1
                } else if w2.0.w2 < w1.0.w2 {
                    *w2
                } else {
                    *w1
                }
            }

            #[inline(always)]
            fn times(w1: &Self, w2: &Self) -> Self {
                if !w1.is_member() || !w2.is_member() {
                    return Self::no_weight();
                }
                Self($val_type {
                    w1: if w1.0.w1 >= w2.0.w1 { w1.0.w1 } else { w2.0.w1 },
                    w2: w1.0.w2 + w2.0.w2,
                })
            }

            #[inline(always)]
            fn divide(w1: &Self, w2: &Self, _typ: DivideType) -> Self {
                if !w1.is_member() || !w2.is_member() {
                    return Self::no_weight();
                }
                if w1.0.w1 >= w2.0.w1 {
                    Self($val_type {
                        w1: w1.0.w1,
                        w2: w1.0.w2 - w2.0.w2,
                    })
                } else {
                    Self::no_weight()
                }
            }
        }

        impl WeightFfi for $name {
            type ValueType = $val_type;
            #[inline(always)]
            fn from_ffi(val: Self::ValueType) -> Self {
                Self(val)
            }
            #[inline(always)]
            fn as_ffi(&self) -> Self::ValueType {
                self.0
            }
        }
    };
}

define_lexicographic_minmax_tropical!(
    LexicographicMinMaxTropicalWeight,
    f32,
    LexicographicWeightValueF32,
    "minmax_LT_tropical"
);

define_lexicographic_minmax_tropical!(
    LexicographicMinMaxTropical64Weight,
    f64,
    LexicographicWeightValueF64,
    "minmax64_LT_tropical64"
);

macro_rules! define_lexicographic_minmax_minmax {
    ($name:ident, $t:ident, $val_type:ident, $type_name:expr) => {
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $name(pub $val_type);

        impl $name {
            #[inline(always)]
            pub const fn value(&self) -> $val_type {
                self.0
            }
            #[inline(always)]
            pub const fn new(w1: $t, w2: $t) -> Self {
                Self($val_type { w1, w2 })
            }
        }

        impl Eq for $name {}
        impl LeftSemiring for $name {}
        impl RightSemiring for $name {}
        impl CommutativeWeight for $name {}
        impl PathWeight for $name {}
        impl IdempotentWeight for $name {}

        #[allow(clippy::derived_hash_with_manual_eq)]
        impl std::hash::Hash for $name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.w1.to_bits().hash(state);
                self.0.w2.to_bits().hash(state);
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let w1_str = if self.0.w1 == $t::INFINITY {
                    "Infinity".to_string()
                } else if self.0.w1 == $t::NEG_INFINITY {
                    "-Infinity".to_string()
                } else if self.0.w1.is_nan() {
                    "BadNumber".to_string()
                } else {
                    self.0.w1.to_string()
                };
                let w2_str = if self.0.w2 == $t::INFINITY {
                    "Infinity".to_string()
                } else if self.0.w2 == $t::NEG_INFINITY {
                    "-Infinity".to_string()
                } else if self.0.w2.is_nan() {
                    "BadNumber".to_string()
                } else {
                    self.0.w2.to_string()
                };
                write!(f, "{},{}", w1_str, w2_str)
            }
        }

        impl std::str::FromStr for $name {
            type Err = std::num::ParseFloatError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let parts: Vec<&str> = s.split(',').collect();
                if parts.len() != 2 {
                    return "invalid".parse::<$t>().map(|_| Self::zero());
                }
                let parse_t = |s: &str| -> Result<$t, std::num::ParseFloatError> {
                    match s {
                        "Infinity" => Ok($t::INFINITY),
                        "-Infinity" => Ok($t::NEG_INFINITY),
                        "BadNumber" => Ok($t::NAN),
                        _ => s.parse::<$t>(),
                    }
                };
                Ok(Self($val_type {
                    w1: parse_t(parts[0])?,
                    w2: parse_t(parts[1])?,
                }))
            }
        }

        impl Weight for $name {
            type ReverseWeight = Self;
            #[inline(always)]
            fn zero() -> Self {
                Self($val_type {
                    w1: $t::INFINITY,
                    w2: $t::INFINITY,
                })
            }
            #[inline(always)]
            fn one() -> Self {
                Self($val_type {
                    w1: $t::NEG_INFINITY,
                    w2: $t::NEG_INFINITY,
                })
            }
            #[inline(always)]
            fn no_weight() -> Self {
                Self($val_type {
                    w1: $t::NAN,
                    w2: $t::NAN,
                })
            }
            fn type_name() -> &'static str {
                $type_name
            }
            fn properties() -> u64 {
                LEFT_SEMIRING | RIGHT_SEMIRING | COMMUTATIVE | PATH | IDEMPOTENT
            }

            #[inline(always)]
            fn is_member(&self) -> bool {
                let w1_member = !self.0.w1.is_nan();
                let w2_member = !self.0.w2.is_nan();
                if !w1_member || !w2_member {
                    return false;
                }
                let w1_zero = self.0.w1 == $t::INFINITY;
                let w2_zero = self.0.w2 == $t::INFINITY;
                if w1_zero && w2_zero {
                    return true;
                }
                if !w1_zero && !w2_zero {
                    return true;
                }
                false
            }
            #[inline(always)]
            fn approx_equal(w1: &Self, w2: &Self, delta: f32) -> bool {
                let w1_1 = w1.0.w1;
                let w1_2 = w1.0.w2;
                let w2_1 = w2.0.w1;
                let w2_2 = w2.0.w2;
                (w1_1 <= w2_1 + (delta as $t) && w2_1 <= w1_1 + (delta as $t))
                    && (w1_2 <= w2_2 + (delta as $t) && w2_2 <= w1_2 + (delta as $t))
            }
            fn quantize(w: &Self, delta: f32) -> Self {
                if !w.is_member() || w.0.w1 == $t::INFINITY || w.0.w1 == $t::NEG_INFINITY {
                    *w
                } else {
                    Self($val_type {
                        w1: (((w.0.w1 as f32) / delta) + 0.5).floor() as $t * (delta as $t),
                        w2: (((w.0.w2 as f32) / delta) + 0.5).floor() as $t * (delta as $t),
                    })
                }
            }
            #[inline(always)]
            fn reverse(w: &Self) -> Self::ReverseWeight {
                *w
            }

            #[inline(always)]
            fn plus(w1: &Self, w2: &Self) -> Self {
                if !w1.is_member() || !w2.is_member() {
                    return Self::no_weight();
                }
                if w1.0.w1 < w2.0.w1 {
                    *w1
                } else if w2.0.w1 < w1.0.w1 {
                    *w2
                } else if w1.0.w2 < w2.0.w2 {
                    *w1
                } else if w2.0.w2 < w1.0.w2 {
                    *w2
                } else {
                    *w1
                }
            }

            #[inline(always)]
            fn times(w1: &Self, w2: &Self) -> Self {
                if !w1.is_member() || !w2.is_member() {
                    return Self::no_weight();
                }
                Self($val_type {
                    w1: if w1.0.w1 >= w2.0.w1 { w1.0.w1 } else { w2.0.w1 },
                    w2: if w1.0.w2 >= w2.0.w2 { w1.0.w2 } else { w2.0.w2 },
                })
            }

            #[inline(always)]
            fn divide(w1: &Self, w2: &Self, _typ: DivideType) -> Self {
                if !w1.is_member() || !w2.is_member() {
                    return Self::no_weight();
                }
                if w1.0.w1 >= w2.0.w1 && w1.0.w2 >= w2.0.w2 {
                    Self($val_type {
                        w1: w1.0.w1,
                        w2: w1.0.w2,
                    })
                } else {
                    Self::no_weight()
                }
            }
        }

        impl WeightFfi for $name {
            type ValueType = $val_type;
            #[inline(always)]
            fn from_ffi(val: Self::ValueType) -> Self {
                Self(val)
            }
            #[inline(always)]
            fn as_ffi(&self) -> Self::ValueType {
                self.0
            }
        }
    };
}

define_lexicographic_minmax_minmax!(
    LexicographicMinMaxWeight,
    f32,
    LexicographicWeightValueF32,
    "minmax_LT_minmax"
);

define_lexicographic_minmax_minmax!(
    LexicographicMinMax64Weight,
    f64,
    LexicographicWeightValueF64,
    "minmax64_LT_minmax64"
);
