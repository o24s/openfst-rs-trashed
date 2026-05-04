use crate::ffi::WeightFfi;
use crate::ffi::power_weight::{PowerWeightValueF32_3, PowerWeightValueF64_3};
use crate::float_weight::{
    Log64Weight, LogWeight, MinMaxWeight, MinMaxWeight64, Real64Weight, RealWeight, TropicalWeight,
    TropicalWeight64,
};
use crate::weight::{COMMUTATIVE, DivideType, IDEMPOTENT, LEFT_SEMIRING, RIGHT_SEMIRING, Weight};
use std::array;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PowerWeight<W, const N: usize>(pub [W; N]);

impl<W: Weight, const N: usize> Weight for PowerWeight<W, N> {
    type ReverseWeight = PowerWeight<W::ReverseWeight, N>;

    fn zero() -> Self {
        Self(array::from_fn(|_| W::zero()))
    }
    fn one() -> Self {
        Self(array::from_fn(|_| W::one()))
    }
    fn no_weight() -> Self {
        Self(array::from_fn(|_| W::no_weight()))
    }

    fn type_name() -> &'static str {
        Box::leak(format!("{}_{}", W::type_name(), N).into_boxed_str())
    }
    fn properties() -> u64 {
        W::properties() & (LEFT_SEMIRING | RIGHT_SEMIRING | COMMUTATIVE | IDEMPOTENT)
    }
    fn is_member(&self) -> bool {
        self.0.iter().all(|w| w.is_member())
    }
    fn approx_equal(w1: &Self, w2: &Self, delta: f32) -> bool {
        w1.0.iter()
            .zip(w2.0.iter())
            .all(|(a, b)| W::approx_equal(a, b, delta))
    }
    fn quantize(w: &Self, delta: f32) -> Self {
        Self(array::from_fn(|i| W::quantize(&w.0[i], delta)))
    }
    fn reverse(w: &Self) -> Self::ReverseWeight {
        PowerWeight(array::from_fn(|i| W::reverse(&w.0[i])))
    }
    fn plus(w1: &Self, w2: &Self) -> Self {
        Self(array::from_fn(|i| W::plus(&w1.0[i], &w2.0[i])))
    }
    fn times(w1: &Self, w2: &Self) -> Self {
        Self(array::from_fn(|i| W::times(&w1.0[i], &w2.0[i])))
    }
    fn divide(w1: &Self, w2: &Self, typ: DivideType) -> Self {
        Self(array::from_fn(|i| W::divide(&w1.0[i], &w2.0[i], typ)))
    }
}

impl<W: std::fmt::Display, const N: usize> std::fmt::Display for PowerWeight<W, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, w) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ",")?;
            }
            write!(f, "{}", w)?;
        }
        Ok(())
    }
}

impl<W: std::str::FromStr + Weight, const N: usize> std::str::FromStr for PowerWeight<W, N> {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != N {
            return Err(());
        }
        let mut arr = array::from_fn(|_| W::zero());
        for i in 0..N {
            arr[i] = parts[i].parse().map_err(|_| ())?;
        }
        Ok(Self(arr))
    }
}

macro_rules! impl_power_weight_ffi {
    ($wtype:ident, $inner:ident, $ffi_type:ident) => {
        impl WeightFfi for PowerWeight<$inner, 3> {
            type ValueType = $ffi_type;
            fn from_ffi(val: Self::ValueType) -> Self {
                Self([
                    $inner::from_ffi(val.w[0]),
                    $inner::from_ffi(val.w[1]),
                    $inner::from_ffi(val.w[2]),
                ])
            }
            fn as_ffi(&self) -> Self::ValueType {
                $ffi_type {
                    w: [self.0[0].as_ffi(), self.0[1].as_ffi(), self.0[2].as_ffi()],
                }
            }
        }
    };
}

pub type Power3TropicalWeight = PowerWeight<TropicalWeight, 3>;
pub type Power3Tropical64Weight = PowerWeight<TropicalWeight64, 3>;
pub type Power3LogWeight = PowerWeight<LogWeight, 3>;
pub type Power3Log64Weight = PowerWeight<Log64Weight, 3>;
pub type Power3RealWeight = PowerWeight<RealWeight, 3>;
pub type Power3Real64Weight = PowerWeight<Real64Weight, 3>;
pub type Power3MinMaxWeight = PowerWeight<MinMaxWeight, 3>;
pub type Power3MinMax64Weight = PowerWeight<MinMaxWeight64, 3>;

impl_power_weight_ffi!(Power3TropicalWeight, TropicalWeight, PowerWeightValueF32_3);
impl_power_weight_ffi!(
    Power3Tropical64Weight,
    TropicalWeight64,
    PowerWeightValueF64_3
);
impl_power_weight_ffi!(Power3LogWeight, LogWeight, PowerWeightValueF32_3);
impl_power_weight_ffi!(Power3Log64Weight, Log64Weight, PowerWeightValueF64_3);
impl_power_weight_ffi!(Power3RealWeight, RealWeight, PowerWeightValueF32_3);
impl_power_weight_ffi!(Power3Real64Weight, Real64Weight, PowerWeightValueF64_3);
impl_power_weight_ffi!(Power3MinMaxWeight, MinMaxWeight, PowerWeightValueF32_3);
impl_power_weight_ffi!(Power3MinMax64Weight, MinMaxWeight64, PowerWeightValueF64_3);
