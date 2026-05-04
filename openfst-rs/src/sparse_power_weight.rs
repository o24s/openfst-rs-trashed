use crate::ffi::WeightFfi;
use crate::ffi::sparse_power_weight::*;
use crate::float_weight::{
    Log64Weight, LogWeight, MinMaxWeight, MinMaxWeight64, Real64Weight, RealWeight, TropicalWeight,
    TropicalWeight64,
};
use crate::weight::{COMMUTATIVE, DivideType, IDEMPOTENT, LEFT_SEMIRING, RIGHT_SEMIRING, Weight};

#[derive(Debug, Clone, PartialEq)]
pub struct SparsePowerWeight<W> {
    pub default_weight: W,
    /// Must be sorted by key in ascending order.
    pub elements: Vec<(i32, W)>,
}

impl<W: Weight> SparsePowerWeight<W> {
    pub fn new(default_weight: W, mut elements: Vec<(i32, W)>) -> Self {
        elements.sort_by_key(|e| e.0);
        elements.retain(|e| e.1 != default_weight);
        Self {
            default_weight,
            elements,
        }
    }

    fn map<F>(w1: &Self, w2: &Self, mut op: F) -> Self
    where
        F: FnMut(&W, &W) -> W,
    {
        let def = op(&w1.default_weight, &w2.default_weight);
        let mut elems = Vec::new();
        let mut i = 0;
        let mut j = 0;

        while i < w1.elements.len() && j < w2.elements.len() {
            let (k1, v1) = &w1.elements[i];
            let (k2, v2) = &w2.elements[j];

            if k1 < k2 {
                let res = op(v1, &w2.default_weight);
                if res != def {
                    elems.push((*k1, res));
                }
                i += 1;
            } else if k1 > k2 {
                let res = op(&w1.default_weight, v2);
                if res != def {
                    elems.push((*k2, res));
                }
                j += 1;
            } else {
                let res = op(v1, v2);
                if res != def {
                    elems.push((*k1, res));
                }
                i += 1;
                j += 1;
            }
        }
        while i < w1.elements.len() {
            let (k1, v1) = &w1.elements[i];
            let res = op(v1, &w2.default_weight);
            if res != def {
                elems.push((*k1, res));
            }
            i += 1;
        }
        while j < w2.elements.len() {
            let (k2, v2) = &w2.elements[j];
            let res = op(&w1.default_weight, v2);
            if res != def {
                elems.push((*k2, res));
            }
            j += 1;
        }
        Self {
            default_weight: def,
            elements: elems,
        }
    }
}

impl<W: Weight> Weight for SparsePowerWeight<W> {
    type ReverseWeight = SparsePowerWeight<W::ReverseWeight>;

    fn zero() -> Self {
        Self {
            default_weight: W::zero(),
            elements: Vec::new(),
        }
    }
    fn one() -> Self {
        Self {
            default_weight: W::one(),
            elements: Vec::new(),
        }
    }
    fn no_weight() -> Self {
        Self {
            default_weight: W::no_weight(),
            elements: Vec::new(),
        }
    }
    fn type_name() -> &'static str {
        Box::leak(format!("{}_^n", W::type_name()).into_boxed_str())
    }
    fn properties() -> u64 {
        W::properties() & (LEFT_SEMIRING | RIGHT_SEMIRING | COMMUTATIVE | IDEMPOTENT)
    }

    fn is_member(&self) -> bool {
        if !self.default_weight.is_member() {
            return false;
        }
        self.elements.iter().all(|(_, w)| w.is_member())
    }

    fn approx_equal(w1: &Self, w2: &Self, delta: f32) -> bool {
        let diff = Self::map(w1, w2, |a, b| {
            if W::approx_equal(a, b, delta) {
                W::one()
            } else {
                W::zero()
            }
        });
        diff == Self::one()
    }

    fn quantize(w: &Self, delta: f32) -> Self {
        let def = W::quantize(&w.default_weight, delta);
        let mut elems = Vec::with_capacity(w.elements.len());
        for (k, v) in &w.elements {
            let q = W::quantize(v, delta);
            if q != def {
                elems.push((*k, q));
            }
        }
        Self {
            default_weight: def,
            elements: elems,
        }
    }

    fn reverse(w: &Self) -> Self::ReverseWeight {
        let def = W::reverse(&w.default_weight);
        let elems = w
            .elements
            .iter()
            .map(|(k, v)| (*k, W::reverse(v)))
            .collect();
        SparsePowerWeight {
            default_weight: def,
            elements: elems,
        }
    }

    fn plus(w1: &Self, w2: &Self) -> Self {
        Self::map(w1, w2, W::plus)
    }
    fn times(w1: &Self, w2: &Self) -> Self {
        Self::map(w1, w2, W::times)
    }
    fn divide(w1: &Self, w2: &Self, typ: DivideType) -> Self {
        Self::map(w1, w2, |a, b| W::divide(a, b, typ))
    }
}

impl<W: std::fmt::Display> std::fmt::Display for SparsePowerWeight<W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.default_weight)?;
        for (k, v) in &self.elements {
            write!(f, ",{}:{}", k, v)?;
        }
        Ok(())
    }
}

impl<W: std::str::FromStr + Weight> std::str::FromStr for SparsePowerWeight<W> {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.is_empty() {
            return Err(());
        }
        let default_weight = parts[0].parse().map_err(|_| ())?;
        let mut elements = Vec::new();
        for p in parts.into_iter().skip(1) {
            let kv: Vec<&str> = p.split(':').collect();
            if kv.len() != 2 {
                return Err(());
            }
            let key = kv[0].parse().map_err(|_| ())?;
            let val = kv[1].parse().map_err(|_| ())?;
            elements.push((key, val));
        }
        Ok(Self::new(default_weight, elements))
    }
}

macro_rules! impl_sparse_power_weight_ffi {
    ($wtype:ident, $inner:ident, $ffi_type:ident, $elem_type:ident) => {
        impl WeightFfi for SparsePowerWeight<$inner> {
            type ValueType = $ffi_type;
            fn from_ffi(val: Self::ValueType) -> Self {
                let slice =
                    unsafe { std::slice::from_raw_parts(val.elements_ptr, val.elements_len) };
                let mut elements = Vec::with_capacity(val.elements_len);
                for e in slice {
                    elements.push((e.key, $inner::from_ffi(e.weight.into())));
                }
                Self {
                    default_weight: $inner::from_ffi(val.default_weight.into()),
                    elements,
                }
            }
            fn as_ffi(&self) -> Self::ValueType {
                // To FFI: C++ takes deep copy immediately
                $ffi_type {
                    default_weight: self.default_weight.as_ffi().into(),
                    elements_ptr: self.elements.as_ptr() as *const _,
                    elements_len: self.elements.len(),
                }
            }
        }
    };
}

pub type SparsePowerTropicalWeight = SparsePowerWeight<TropicalWeight>;
pub type SparsePowerTropical64Weight = SparsePowerWeight<TropicalWeight64>;
pub type SparsePowerLogWeight = SparsePowerWeight<LogWeight>;
pub type SparsePowerLog64Weight = SparsePowerWeight<Log64Weight>;
pub type SparsePowerRealWeight = SparsePowerWeight<RealWeight>;
pub type SparsePowerReal64Weight = SparsePowerWeight<Real64Weight>;
pub type SparsePowerMinMaxWeight = SparsePowerWeight<MinMaxWeight>;
pub type SparsePowerMinMax64Weight = SparsePowerWeight<MinMaxWeight64>;

impl_sparse_power_weight_ffi!(
    SparsePowerTropicalWeight,
    TropicalWeight,
    SparsePowerWeightValueF32,
    SparseTupleElementF32
);
impl_sparse_power_weight_ffi!(
    SparsePowerTropical64Weight,
    TropicalWeight64,
    SparsePowerWeightValueF64,
    SparseTupleElementF64
);
impl_sparse_power_weight_ffi!(
    SparsePowerLogWeight,
    LogWeight,
    SparsePowerWeightValueF32,
    SparseTupleElementF32
);
impl_sparse_power_weight_ffi!(
    SparsePowerLog64Weight,
    Log64Weight,
    SparsePowerWeightValueF64,
    SparseTupleElementF64
);
impl_sparse_power_weight_ffi!(
    SparsePowerRealWeight,
    RealWeight,
    SparsePowerWeightValueF32,
    SparseTupleElementF32
);
impl_sparse_power_weight_ffi!(
    SparsePowerReal64Weight,
    Real64Weight,
    SparsePowerWeightValueF64,
    SparseTupleElementF64
);
impl_sparse_power_weight_ffi!(
    SparsePowerMinMaxWeight,
    MinMaxWeight,
    SparsePowerWeightValueF32,
    SparseTupleElementF32
);
impl_sparse_power_weight_ffi!(
    SparsePowerMinMax64Weight,
    MinMaxWeight64,
    SparsePowerWeightValueF64,
    SparseTupleElementF64
);
