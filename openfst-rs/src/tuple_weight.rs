use crate::weight::Weight;
use std::array;

/// An n-tuple weight container.
///
/// Note: In OpenFst, `TupleWeight` is just a base container class and does NOT
/// form a semiring on its own (it lacks `Plus`, `Times`, and `Divide`).
///
/// Because it does not implement the `Weight` trait, the Rust compiler will
/// strictly prevent you from using it directly in `Arc` or FST algorithms.
/// Use `PowerWeight` if you need Cartesian power semiring operations.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TupleWeight<W, const N: usize>(pub [W; N]);

impl<W: Weight, const N: usize> TupleWeight<W, N> {
    #[inline(always)]
    pub fn zero() -> Self {
        Self(array::from_fn(|_| W::zero()))
    }

    #[inline(always)]
    pub fn one() -> Self {
        Self(array::from_fn(|_| W::one()))
    }

    #[inline(always)]
    pub fn no_weight() -> Self {
        Self(array::from_fn(|_| W::no_weight()))
    }

    #[inline(always)]
    pub fn is_member(&self) -> bool {
        self.0.iter().all(|w| w.is_member())
    }

    #[inline(always)]
    pub fn approx_equal(w1: &Self, w2: &Self, delta: f32) -> bool {
        w1.0.iter()
            .zip(w2.0.iter())
            .all(|(a, b)| W::approx_equal(a, b, delta))
    }

    pub fn quantize(&self, delta: f32) -> Self {
        Self(array::from_fn(|i| W::quantize(&self.0[i], delta)))
    }

    #[inline(always)]
    pub fn reverse(&self) -> TupleWeight<W::ReverseWeight, N> {
        TupleWeight(array::from_fn(|i| W::reverse(&self.0[i])))
    }

    #[inline(always)]
    pub fn value(&self, index: usize) -> &W {
        &self.0[index]
    }

    #[inline(always)]
    pub fn set_value(&mut self, index: usize, w: W) {
        self.0[index] = w;
    }
}

impl<W: std::fmt::Display, const N: usize> std::fmt::Display for TupleWeight<W, N> {
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

impl<W: std::str::FromStr + Weight, const N: usize> std::str::FromStr for TupleWeight<W, N> {
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
