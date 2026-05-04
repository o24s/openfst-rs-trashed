use crate::ffi::WeightFfi;
use crate::ffi::weight::{PairWeightValueF32, PairWeightValueF64};
use crate::weight::{
    COMMUTATIVE, DivideType, IDEMPOTENT, LEFT_SEMIRING, PATH, RIGHT_SEMIRING, Weight,
};
use crate::weight::{CommutativeWeight, IdempotentWeight, LeftSemiring, PathWeight, RightSemiring};

macro_rules! define_tropical {
    ($name:ident, $t:ident, $type_name:expr) => {
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $name(pub $t);

        impl $name {
            #[inline(always)]
            pub const fn value(&self) -> $t {
                self.0
            }

            #[inline(always)]
            pub fn minus(_w1: &Self, _w2: &Self) -> Self {
                Self::no_weight()
            }

            /// Calculates the n-th power of w with respect to semiring Times.
            /// For Tropical weight, this is scaling (multiplication).
            #[inline(always)]
            pub fn powerf(&self, n: $t) -> Self {
                if !self.is_member() || n.is_nan() {
                    Self::no_weight()
                } else if n == 0.0 || *self == <Self as Weight>::one() {
                    <Self as Weight>::one()
                } else {
                    Self(self.0 * n)
                }
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
                self.0.to_bits().hash(state);
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if self.0 == $t::INFINITY {
                    write!(f, "Infinity")
                } else if self.0 == $t::NEG_INFINITY {
                    write!(f, "-Infinity")
                } else if self.0.is_nan() {
                    write!(f, "BadNumber")
                } else {
                    write!(f, "{}", self.0)
                }
            }
        }

        impl std::str::FromStr for $name {
            type Err = std::num::ParseFloatError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    "Infinity" => Ok(Self($t::INFINITY)),
                    "-Infinity" => Ok(Self($t::NEG_INFINITY)),
                    "BadNumber" => Ok(Self($t::NAN)),
                    _ => s.parse::<$t>().map(Self),
                }
            }
        }

        impl Weight for $name {
            type ReverseWeight = Self;
            #[inline(always)]
            fn zero() -> Self {
                Self($t::INFINITY)
            }
            #[inline(always)]
            fn one() -> Self {
                Self(0.0)
            }
            #[inline(always)]
            fn no_weight() -> Self {
                Self($t::NAN)
            }
            fn type_name() -> &'static str {
                $type_name
            }
            fn properties() -> u64 {
                LEFT_SEMIRING | RIGHT_SEMIRING | COMMUTATIVE | PATH | IDEMPOTENT
            }
            #[inline(always)]
            fn is_member(&self) -> bool {
                self.0 > $t::NEG_INFINITY
            }
            #[inline(always)]
            fn approx_equal(w1: &Self, w2: &Self, delta: f32) -> bool {
                w1.0 <= w2.0 + (delta as $t) && w2.0 <= w1.0 + (delta as $t)
            }
            fn quantize(w: &Self, delta: f32) -> Self {
                if !w.is_member() || w.0 == $t::INFINITY {
                    *w
                } else {
                    Self((((w.0 as f32) / delta) + 0.5).floor() as $t * (delta as $t))
                }
            }
            #[inline(always)]
            fn reverse(w: &Self) -> Self::ReverseWeight {
                *w
            }
            #[inline(always)]
            fn plus(w1: &Self, w2: &Self) -> Self {
                if !w1.is_member() || !w2.is_member() {
                    Self::no_weight()
                } else if w1.0 < w2.0 {
                    *w1
                } else {
                    *w2
                }
            }
            #[inline(always)]
            fn times(w1: &Self, w2: &Self) -> Self {
                Self(w1.0 + w2.0)
            }
            #[inline(always)]
            fn divide(w1: &Self, w2: &Self, _typ: DivideType) -> Self {
                if w2.is_member() {
                    Self(w1.0 - w2.0)
                } else {
                    Self::no_weight()
                }
            }
        }

        impl WeightFfi for $name {
            type ValueType = $t;
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
define_tropical!(TropicalWeight, f32, "tropical");
define_tropical!(TropicalWeight64, f64, "tropical64");

macro_rules! define_log {
    ($name:ident, $t:ident, $type_name:expr) => {
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $name(pub $t);

        impl $name {
            #[inline(always)]
            pub const fn value(&self) -> $t {
                self.0
            }

            #[inline(always)]
            pub fn minus(w1: &Self, w2: &Self) -> Self {
                let f1 = w1.0;
                let f2 = w2.0;
                if f1 > f2 {
                    return Self::no_weight();
                }
                if f2 == $t::INFINITY {
                    return *w1;
                }
                let d = f2 - f1;
                if d == $t::INFINITY {
                    return *w1;
                }
                // LogNegExp mapping
                Self(f1 - (-(d).exp()).ln_1p())
            }

            #[inline(always)]
            pub fn powerf(&self, n: $t) -> Self {
                if !self.is_member() || n.is_nan() {
                    Self::no_weight()
                } else if n == 0.0 || *self == <Self as Weight>::one() {
                    <Self as Weight>::one()
                } else {
                    Self(self.0 * n)
                }
            }
        }

        impl Eq for $name {}

        impl LeftSemiring for $name {}
        impl RightSemiring for $name {}
        impl CommutativeWeight for $name {}

        #[allow(clippy::derived_hash_with_manual_eq)]
        impl std::hash::Hash for $name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.to_bits().hash(state);
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if self.0 == $t::INFINITY {
                    write!(f, "Infinity")
                } else if self.0 == $t::NEG_INFINITY {
                    write!(f, "-Infinity")
                } else if self.0.is_nan() {
                    write!(f, "BadNumber")
                } else {
                    write!(f, "{}", self.0)
                }
            }
        }

        impl std::str::FromStr for $name {
            type Err = std::num::ParseFloatError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    "Infinity" => Ok(Self($t::INFINITY)),
                    "-Infinity" => Ok(Self($t::NEG_INFINITY)),
                    "BadNumber" => Ok(Self($t::NAN)),
                    _ => s.parse::<$t>().map(Self),
                }
            }
        }

        impl Weight for $name {
            type ReverseWeight = Self;
            #[inline(always)]
            fn zero() -> Self {
                Self($t::INFINITY)
            }
            #[inline(always)]
            fn one() -> Self {
                Self(0.0)
            }
            #[inline(always)]
            fn no_weight() -> Self {
                Self($t::NAN)
            }
            fn type_name() -> &'static str {
                $type_name
            }
            fn properties() -> u64 {
                LEFT_SEMIRING | RIGHT_SEMIRING | COMMUTATIVE
            }
            #[inline(always)]
            fn is_member(&self) -> bool {
                self.0 > $t::NEG_INFINITY
            }
            #[inline(always)]
            fn approx_equal(w1: &Self, w2: &Self, delta: f32) -> bool {
                w1.0 <= w2.0 + (delta as $t) && w2.0 <= w1.0 + (delta as $t)
            }
            fn quantize(w: &Self, delta: f32) -> Self {
                if !w.is_member() || w.0 == $t::INFINITY {
                    *w
                } else {
                    Self((((w.0 as f32) / delta) + 0.5).floor() as $t * (delta as $t))
                }
            }
            #[inline(always)]
            fn reverse(w: &Self) -> Self::ReverseWeight {
                *w
            }
            #[inline(always)]
            fn plus(w1: &Self, w2: &Self) -> Self {
                let f1 = w1.0;
                let f2 = w2.0;
                if f1 == $t::INFINITY {
                    *w2
                } else if f2 == $t::INFINITY {
                    *w1
                } else if f1 > f2 {
                    // LogPosExp implementation matching C++ OpenFst
                    Self(f2 - (-(f1 - f2)).exp().ln_1p())
                } else {
                    Self(f1 - (-(f2 - f1)).exp().ln_1p())
                }
            }
            #[inline(always)]
            fn times(w1: &Self, w2: &Self) -> Self {
                Self(w1.0 + w2.0)
            }
            #[inline(always)]
            fn divide(w1: &Self, w2: &Self, _typ: DivideType) -> Self {
                if w2.is_member() {
                    Self(w1.0 - w2.0)
                } else {
                    Self::no_weight()
                }
            }
        }

        impl WeightFfi for $name {
            type ValueType = $t;
            #[inline(always)]
            fn from_ffi(val: Self::ValueType) -> Self {
                Self(val)
            }
            #[inline(always)]
            fn as_ffi(&self) -> Self::ValueType {
                self.0
            }
        }

        pastey::paste! {
            /// Kahan compensated summation for LogWeight.
            /// Provides an error bound independent of the number of addends.
            pub struct [<KahanAdder $name>] {
                sum: $t,
                c: $t,
            }

            impl [<KahanAdder $name>] {
                pub fn new(w: $name) -> Self {
                    Self { sum: w.0, c: 0.0 }
                }

                #[inline]
                pub fn add(&mut self, w: &$name) -> $name {
                    let f = w.0;
                    if f == $t::INFINITY {
                        $name(self.sum)
                    } else if self.sum == $t::INFINITY {
                        self.sum = f;
                        self.c = 0.0;
                        $name(self.sum)
                    } else if f > self.sum {
                        // KahanLogSum algorithm
                        let y = -(-(f - self.sum)).exp().ln_1p() - self.c;
                        let t = self.sum + y;
                        self.c = (t - self.sum) - y;
                        self.sum = t;
                        $name(self.sum)
                    } else {
                        let y = -(-(self.sum - f)).exp().ln_1p() - self.c;
                        let t = f + y;
                        self.c = (t - f) - y;
                        self.sum = t;
                        $name(self.sum)
                    }
                }

                pub fn sum(&self) -> $name {
                    $name(self.sum)
                }

                pub fn reset(&mut self, w: $name) {
                    self.sum = w.0;
                    self.c = 0.0;
                }
            }

            impl Default for [<KahanAdder $name>] {
                fn default() -> Self {
                    Self::new(<$name as Weight>::zero())
                }
            }
        }
    };
}
define_log!(LogWeight, f32, "log");
define_log!(Log64Weight, f64, "log64");

macro_rules! define_real {
    ($name:ident, $t:ident, $type_name:expr) => {
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $name(pub $t);

        impl $name {
            #[inline(always)]
            pub const fn value(&self) -> $t {
                self.0
            }

            #[inline(always)]
            pub fn minus(w1: &Self, w2: &Self) -> Self {
                let f1 = w1.0;
                let f2 = w2.0;
                if f1 < f2 {
                    Self::no_weight()
                } else {
                    Self(f1 - f2)
                }
            }

            #[inline(always)]
            pub fn powerf(&self, n: $t) -> Self {
                if !self.is_member() || n.is_nan() {
                    Self::no_weight()
                } else if n == 0.0 || *self == <Self as Weight>::one() {
                    <Self as Weight>::one()
                } else {
                    Self(self.0.powf(n))
                }
            }
        }

        impl Eq for $name {}

        impl LeftSemiring for $name {}
        impl RightSemiring for $name {}
        impl CommutativeWeight for $name {}

        #[allow(clippy::derived_hash_with_manual_eq)]
        impl std::hash::Hash for $name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.to_bits().hash(state);
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if self.0 == $t::INFINITY {
                    write!(f, "Infinity")
                } else if self.0 == $t::NEG_INFINITY {
                    write!(f, "-Infinity")
                } else if self.0.is_nan() {
                    write!(f, "BadNumber")
                } else {
                    write!(f, "{}", self.0)
                }
            }
        }

        impl std::str::FromStr for $name {
            type Err = std::num::ParseFloatError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    "Infinity" => Ok(Self($t::INFINITY)),
                    "-Infinity" => Ok(Self($t::NEG_INFINITY)),
                    "BadNumber" => Ok(Self($t::NAN)),
                    _ => s.parse::<$t>().map(Self),
                }
            }
        }

        impl Weight for $name {
            type ReverseWeight = Self;
            #[inline(always)]
            fn zero() -> Self {
                Self(0.0)
            }
            #[inline(always)]
            fn one() -> Self {
                Self(1.0)
            }
            #[inline(always)]
            fn no_weight() -> Self {
                Self($t::NAN)
            }
            fn type_name() -> &'static str {
                $type_name
            }
            fn properties() -> u64 {
                LEFT_SEMIRING | RIGHT_SEMIRING | COMMUTATIVE
            }
            #[inline(always)]
            fn is_member(&self) -> bool {
                self.0 > $t::NEG_INFINITY
            }
            #[inline(always)]
            fn approx_equal(w1: &Self, w2: &Self, delta: f32) -> bool {
                w1.0 <= w2.0 + (delta as $t) && w2.0 <= w1.0 + (delta as $t)
            }
            fn quantize(w: &Self, delta: f32) -> Self {
                if !w.is_member() || w.0 == $t::INFINITY {
                    *w
                } else {
                    Self((((w.0 as f32) / delta) + 0.5).floor() as $t * (delta as $t))
                }
            }
            #[inline(always)]
            fn reverse(w: &Self) -> Self::ReverseWeight {
                *w
            }
            #[inline(always)]
            fn plus(w1: &Self, w2: &Self) -> Self {
                Self(w1.0 + w2.0)
            }
            #[inline(always)]
            fn times(w1: &Self, w2: &Self) -> Self {
                Self(w1.0 * w2.0)
            }
            #[inline(always)]
            fn divide(w1: &Self, w2: &Self, _typ: DivideType) -> Self {
                if w2.is_member() {
                    Self(w1.0 / w2.0)
                } else {
                    Self::no_weight()
                }
            }
        }

        impl WeightFfi for $name {
            type ValueType = $t;
            #[inline(always)]
            fn from_ffi(val: Self::ValueType) -> Self {
                Self(val)
            }
            #[inline(always)]
            fn as_ffi(&self) -> Self::ValueType {
                self.0
            }
        }

        pastey::paste! {
            /// Kahan compensated summation for RealWeight.
            pub struct [<KahanAdder $name>] {
                sum: $t,
                c: $t,
            }

            impl [<KahanAdder $name>] {
                pub fn new(w: $name) -> Self {
                    Self { sum: w.0, c: 0.0 }
                }

                #[inline]
                pub fn add(&mut self, w: &$name) -> $name {
                    let f = w.0;
                    if f == $t::INFINITY {
                        self.sum = f;
                        $name(self.sum)
                    } else if self.sum == $t::INFINITY {
                        $name(self.sum)
                    } else {
                        // KahanRealSum algorithm
                        let y = f - self.c;
                        let t = self.sum + y;
                        self.c = (t - self.sum) - y;
                        self.sum = t;
                        $name(self.sum)
                    }
                }

                pub fn sum(&self) -> $name {
                    $name(self.sum)
                }

                pub fn reset(&mut self, w: $name) {
                    self.sum = w.0;
                    self.c = 0.0;
                }
            }

            impl Default for [<KahanAdder $name>] {
                fn default() -> Self {
                    Self::new(<$name as Weight>::zero())
                }
            }
        }
    };
}
define_real!(RealWeight, f32, "real");
define_real!(Real64Weight, f64, "real64");

macro_rules! define_minmax {
    ($name:ident, $t:ident, $type_name:expr) => {
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $name(pub $t);

        impl $name {
            #[inline(always)]
            pub const fn value(&self) -> $t {
                self.0
            }

            #[inline(always)]
            pub fn minus(_w1: &Self, _w2: &Self) -> Self {
                Self::no_weight()
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
                self.0.to_bits().hash(state);
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if self.0 == $t::INFINITY {
                    write!(f, "Infinity")
                } else if self.0 == $t::NEG_INFINITY {
                    write!(f, "-Infinity")
                } else if self.0.is_nan() {
                    write!(f, "BadNumber")
                } else {
                    write!(f, "{}", self.0)
                }
            }
        }

        impl std::str::FromStr for $name {
            type Err = std::num::ParseFloatError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    "Infinity" => Ok(Self($t::INFINITY)),
                    "-Infinity" => Ok(Self($t::NEG_INFINITY)),
                    "BadNumber" => Ok(Self($t::NAN)),
                    _ => s.parse::<$t>().map(Self),
                }
            }
        }

        impl Weight for $name {
            type ReverseWeight = Self;
            #[inline(always)]
            fn zero() -> Self {
                Self($t::INFINITY)
            }
            #[inline(always)]
            fn one() -> Self {
                Self($t::NEG_INFINITY)
            }
            #[inline(always)]
            fn no_weight() -> Self {
                Self($t::NAN)
            }
            fn type_name() -> &'static str {
                $type_name
            }
            fn properties() -> u64 {
                LEFT_SEMIRING | RIGHT_SEMIRING | COMMUTATIVE | IDEMPOTENT | PATH
            }
            #[inline(always)]
            fn is_member(&self) -> bool {
                !self.0.is_nan()
            }
            #[inline(always)]
            fn approx_equal(w1: &Self, w2: &Self, delta: f32) -> bool {
                w1.0 <= w2.0 + (delta as $t) && w2.0 <= w1.0 + (delta as $t)
            }
            fn quantize(w: &Self, delta: f32) -> Self {
                if !w.is_member() || w.0 == $t::NEG_INFINITY || w.0 == $t::INFINITY {
                    *w
                } else {
                    Self((((w.0 as f32) / delta) + 0.5).floor() as $t * (delta as $t))
                }
            }
            #[inline(always)]
            fn reverse(w: &Self) -> Self::ReverseWeight {
                *w
            }
            #[inline(always)]
            fn plus(w1: &Self, w2: &Self) -> Self {
                if !w1.is_member() || !w2.is_member() {
                    Self::no_weight()
                } else if w1.0 < w2.0 {
                    *w1
                } else {
                    *w2
                }
            }
            #[inline(always)]
            fn times(w1: &Self, w2: &Self) -> Self {
                if !w1.is_member() || !w2.is_member() {
                    Self::no_weight()
                } else if w1.0 >= w2.0 {
                    *w1
                } else {
                    *w2
                }
            }
            #[inline(always)]
            fn divide(w1: &Self, w2: &Self, _typ: DivideType) -> Self {
                if w1.0 >= w2.0 { *w1 } else { Self::no_weight() }
            }
        }

        impl WeightFfi for $name {
            type ValueType = $t;
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
define_minmax!(MinMaxWeight, f32, "minmax");
define_minmax!(MinMaxWeight64, f64, "minmax64");

macro_rules! impl_weight_convert {
    ($from:ident, $to:ident, $closure:expr) => {
        impl From<$from> for $to {
            #[inline(always)]
            fn from(w: $from) -> Self {
                $closure(w)
            }
        }
    };
}

// Log <-> Tropical
impl_weight_convert!(LogWeight, TropicalWeight, |w: LogWeight| TropicalWeight(
    w.value()
));
impl_weight_convert!(Log64Weight, TropicalWeight64, |w: Log64Weight| {
    TropicalWeight64(w.value())
});
impl_weight_convert!(TropicalWeight, LogWeight, |w: TropicalWeight| LogWeight(
    w.value()
));
impl_weight_convert!(TropicalWeight64, Log64Weight, |w: TropicalWeight64| {
    Log64Weight(w.value())
});

// Real -> Log (-ln(x))
impl_weight_convert!(RealWeight, LogWeight, |w: RealWeight| LogWeight(
    -w.value().ln()
));
impl_weight_convert!(Real64Weight, Log64Weight, |w: Real64Weight| Log64Weight(
    -w.value().ln()
));

// Log -> Real (exp(-x))
impl_weight_convert!(LogWeight, RealWeight, |w: LogWeight| RealWeight(
    (-w.value()).exp()
));
impl_weight_convert!(Log64Weight, Real64Weight, |w: Log64Weight| Real64Weight(
    (-w.value()).exp()
));

macro_rules! define_signed_log {
    ($name:ident, $t:ident, $val_type:ident, $type_name:expr) => {
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $name(pub $val_type);

        impl $name {
            #[inline(always)]
            pub const fn value(&self) -> $val_type {
                self.0
            }

            /// Creates a new SignedLogWeight.
            /// `sign` determines the sign (positive > 0, negative <= 0).
            /// `neg_log_prob` is the negative natural logarithm of the absolute probability.
            #[inline(always)]
            pub const fn new(sign: $t, neg_log_prob: $t) -> Self {
                Self($val_type {
                    w1: sign as f32,
                    w2: neg_log_prob,
                })
            }

            #[inline(always)]
            pub fn is_positive(&self) -> bool {
                self.0.w1 > 0.0
            }

            #[inline(always)]
            pub fn minus(w1: &Self, w2: &Self) -> Self {
                let minus_w2 = Self($val_type {
                    w1: -w2.0.w1,
                    w2: w2.0.w2,
                });
                Self::plus(w1, &minus_w2)
            }
        }

        impl Eq for $name {}

        impl LeftSemiring for $name {}
        impl RightSemiring for $name {}
        impl CommutativeWeight for $name {}

        #[allow(clippy::derived_hash_with_manual_eq)]
        impl std::hash::Hash for $name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                let h1 = if self.0.w2 == $t::INFINITY || self.is_positive() {
                    1.0f32.to_bits()
                } else {
                    (-1.0f32).to_bits()
                };
                let h2 = self.0.w2.to_bits();
                let combined =
                    (h1 as u64).wrapping_shl(5) ^ (h1 as u64).wrapping_shr(64 - 5) ^ (h2 as u64);
                combined.hash(state);
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let w1_str = if self.0.w1 == f32::INFINITY {
                    "Infinity".to_string()
                } else if self.0.w1 == f32::NEG_INFINITY {
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

                let parse_f32 = |s: &str| -> Result<f32, std::num::ParseFloatError> {
                    match s {
                        "Infinity" => Ok(f32::INFINITY),
                        "-Infinity" => Ok(f32::NEG_INFINITY),
                        "BadNumber" => Ok(f32::NAN),
                        _ => s.parse::<f32>(),
                    }
                };

                let parse_t = |s: &str| -> Result<$t, std::num::ParseFloatError> {
                    match s {
                        "Infinity" => Ok($t::INFINITY),
                        "-Infinity" => Ok($t::NEG_INFINITY),
                        "BadNumber" => Ok($t::NAN),
                        _ => s.parse::<$t>(),
                    }
                };

                let w1 = parse_f32(parts[0])?;
                let w2 = parse_t(parts[1])?;
                Ok(Self($val_type { w1, w2 }))
            }
        }

        impl Weight for $name {
            type ReverseWeight = Self;

            #[inline(always)]
            fn zero() -> Self {
                Self($val_type {
                    w1: 1.0,
                    w2: $t::INFINITY,
                })
            }
            #[inline(always)]
            fn one() -> Self {
                Self($val_type { w1: 1.0, w2: 0.0 })
            }
            #[inline(always)]
            fn no_weight() -> Self {
                Self($val_type {
                    w1: 1.0,
                    w2: $t::NAN,
                })
            }
            fn type_name() -> &'static str {
                $type_name
            }
            fn properties() -> u64 {
                LEFT_SEMIRING | RIGHT_SEMIRING | COMMUTATIVE
            }
            #[inline(always)]
            fn is_member(&self) -> bool {
                self.0.w2 > $t::NEG_INFINITY && !self.0.w1.is_nan()
            }
            #[inline(always)]
            fn approx_equal(w1: &Self, w2: &Self, delta: f32) -> bool {
                if w1.is_positive() == w2.is_positive() {
                    w1.0.w2 <= w2.0.w2 + (delta as $t) && w2.0.w2 <= w1.0.w2 + (delta as $t)
                } else {
                    w1.0.w2 <= 0.0 + (delta as $t)
                        && 0.0 <= w1.0.w2 + (delta as $t)
                        && w2.0.w2 <= 0.0 + (delta as $t)
                        && 0.0 <= w2.0.w2 + (delta as $t)
                }
            }
            fn quantize(w: &Self, delta: f32) -> Self {
                if !w.is_member() || w.0.w2 == $t::INFINITY {
                    *w
                } else {
                    let qw2 = (((w.0.w2 as f32) / delta) + 0.5).floor() as $t * (delta as $t);
                    Self($val_type {
                        w1: w.0.w1,
                        w2: qw2,
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
                let s1 = w1.is_positive();
                let s2 = w2.is_positive();
                let equal = s1 == s2;
                let f1 = w1.0.w2;
                let f2 = w2.0.w2;

                if f1 == $t::INFINITY {
                    *w2
                } else if f2 == $t::INFINITY {
                    *w1
                } else if f1 == f2 {
                    if equal {
                        let ln2 = std::f64::consts::LN_2 as $t;
                        Self($val_type {
                            w1: w1.0.w1,
                            w2: f2 - ln2,
                        })
                    } else {
                        Self::zero()
                    }
                } else if f1 > f2 {
                    if equal {
                        Self($val_type {
                            w1: w1.0.w1,
                            w2: f2 - (-(f1 - f2)).exp().ln_1p(),
                        })
                    } else {
                        Self($val_type {
                            w1: w2.0.w1,
                            w2: f2 - (1.0 - (-(f1 - f2)).exp()).ln(),
                        })
                    }
                } else {
                    if equal {
                        Self($val_type {
                            w1: w2.0.w1,
                            w2: f1 - (-(f2 - f1)).exp().ln_1p(),
                        })
                    } else {
                        Self($val_type {
                            w1: w1.0.w1,
                            w2: f1 - (1.0 - (-(f2 - f1)).exp()).ln(),
                        })
                    }
                }
            }
            #[inline(always)]
            fn times(w1: &Self, w2: &Self) -> Self {
                if !w1.is_member() || !w2.is_member() {
                    return Self::no_weight();
                }
                let sign = if w1.is_positive() == w2.is_positive() {
                    1.0
                } else {
                    -1.0
                };
                Self($val_type {
                    w1: sign,
                    w2: w1.0.w2 + w2.0.w2,
                })
            }
            #[inline(always)]
            fn divide(w1: &Self, w2: &Self, _typ: DivideType) -> Self {
                if !w1.is_member() || !w2.is_member() {
                    return Self::no_weight();
                }
                let s1 = w1.is_positive();
                let s2 = w2.is_positive();
                let f1 = w1.0.w2;
                let f2 = w2.0.w2;

                if f2 == $t::INFINITY {
                    Self($val_type {
                        w1: 1.0,
                        w2: $t::NAN,
                    })
                } else if f1 == $t::INFINITY {
                    Self($val_type {
                        w1: 1.0,
                        w2: $t::INFINITY,
                    })
                } else {
                    let sign = if s1 == s2 { 1.0 } else { -1.0 };
                    Self($val_type {
                        w1: sign,
                        w2: f1 - f2,
                    })
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

define_signed_log!(
    SignedLogWeight,
    f32,
    PairWeightValueF32,
    "signed_log_tropical_log"
);
define_signed_log!(
    SignedLog64Weight,
    f64,
    PairWeightValueF64,
    "signed_log_tropical_log64"
);

// SignedLog -> Real
impl_weight_convert!(SignedLogWeight, RealWeight, |w: SignedLogWeight| {
    RealWeight(w.0.w1 * (-w.0.w2).exp())
});
impl_weight_convert!(SignedLog64Weight, Real64Weight, |w: SignedLog64Weight| {
    Real64Weight(w.0.w1 as f64 * (-w.0.w2).exp())
});

// Tropical -> SignedLog
impl_weight_convert!(TropicalWeight, SignedLogWeight, |w: TropicalWeight| {
    SignedLogWeight(PairWeightValueF32 { w1: 1.0, w2: w.0 })
});
impl_weight_convert!(
    TropicalWeight64,
    SignedLog64Weight,
    |w: TropicalWeight64| { SignedLog64Weight(PairWeightValueF64 { w1: 1.0, w2: w.0 }) }
);

// Log -> SignedLog
impl_weight_convert!(LogWeight, SignedLogWeight, |w: LogWeight| {
    SignedLogWeight(PairWeightValueF32 { w1: 1.0, w2: w.0 })
});
impl_weight_convert!(Log64Weight, SignedLog64Weight, |w: Log64Weight| {
    SignedLog64Weight(PairWeightValueF64 { w1: 1.0, w2: w.0 })
});

// Real -> SignedLog
impl_weight_convert!(RealWeight, SignedLogWeight, |w: RealWeight| {
    let sign = if w.0 >= 0.0 { 1.0 } else { -1.0 };
    SignedLogWeight(PairWeightValueF32 {
        w1: sign,
        w2: -w.0.abs().ln(),
    })
});
impl_weight_convert!(Real64Weight, SignedLog64Weight, |w: Real64Weight| {
    let sign = if w.0 >= 0.0 { 1.0 } else { -1.0 };
    SignedLog64Weight(PairWeightValueF64 {
        w1: sign,
        w2: -w.0.abs().ln(),
    })
});

// SignedLog64 <-> SignedLog
impl_weight_convert!(
    SignedLog64Weight,
    SignedLogWeight,
    |w: SignedLog64Weight| {
        SignedLogWeight(PairWeightValueF32 {
            w1: w.0.w1,
            w2: w.0.w2 as f32,
        })
    }
);
impl_weight_convert!(SignedLogWeight, SignedLog64Weight, |w: SignedLogWeight| {
    SignedLog64Weight(PairWeightValueF64 {
        w1: w.0.w1,
        w2: w.0.w2 as f64,
    })
});
