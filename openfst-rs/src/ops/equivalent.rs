use crate::arc::Arc;
use crate::error::OpenFstError;
use crate::ffi::WeightFfi;
use crate::ffi::equivalent::EquivalentFfi;
use crate::ffi::fst::FstFfi;
use crate::fst::{AsFstCxx, Fst};
use crate::properties::{DetEpsFreeAcceptor, FstPropertiesExt, Verified};
use crate::weight::DELTA;

#[derive(Debug, Clone)]
pub struct RandEquivalentOptions {
    pub npath: i32,
    pub delta: f32,
    pub seed: u64,
    pub max_length: i32,
}

impl Default for RandEquivalentOptions {
    fn default() -> Self {
        Self {
            npath: 1,
            delta: DELTA,
            seed: 0,
            max_length: i32::MAX,
        }
    }
}

pub struct EqualType;
impl EqualType {
    pub const EQUAL_FSTS: u8 = 0x01;
    pub const EQUAL_FST_TYPES: u8 = 0x02;
    pub const EQUAL_COMPAT_PROPERTIES: u8 = 0x04;
    pub const EQUAL_COMPAT_SYMBOLS: u8 = 0x08;
    pub const EQUAL_ALL: u8 = Self::EQUAL_FSTS
        | Self::EQUAL_FST_TYPES
        | Self::EQUAL_COMPAT_PROPERTIES
        | Self::EQUAL_COMPAT_SYMBOLS;
}

/// Core verified operations for equivalent checks.
/// Implemented directly on the `Verified` wrapper to ensure compile-time safety.
pub trait EquivalentVerified<A>
where
    A: crate::arc::Arc<StateId = i32, Label = i32> + crate::ffi::fst::FstFfi,
    A::Weight: crate::ffi::WeightFfi,
{
    /// Determines if the two FSAs accept exactly the same set of strings with the same weights.
    /// Both the receiver and the argument are statically guaranteed to be deterministic, epsilon-free Acceptors.
    /// Returns `Err` only if the symbol tables are incompatible.
    fn equivalent<F2>(
        &self,
        other: Verified<&F2, DetEpsFreeAcceptor>,
        delta: f32,
    ) -> Result<bool, OpenFstError>
    where
        F2: Fst<A> + AsFstCxx<A>;
}

impl<F1, A> EquivalentVerified<A> for Verified<&F1, DetEpsFreeAcceptor>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + EquivalentFfi,
    A::Weight: WeightFfi,
    F1: Fst<A> + AsFstCxx<A>,
{
    fn equivalent<F2>(
        &self,
        other: Verified<&F2, DetEpsFreeAcceptor>,
        delta: f32,
    ) -> Result<bool, OpenFstError>
    where
        F2: Fst<A> + AsFstCxx<A>,
    {
        let res = A::fst_equivalent(
            unsafe { self.value.as_fst_cxx() },
            unsafe { other.value.as_fst_cxx() },
            delta,
        )?;
        Ok(res)
    }
}

pub trait Equivalent<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + EquivalentFfi,
    A::Weight: WeightFfi,
{
    /// Tests if two FSTs are equivalent by randomly generating paths.
    /// Returns `Err` if the FSTs are invalid (e.g., incompatible symbol tables).
    fn randequivalent<F2>(
        &self,
        other: &F2,
        opts: &RandEquivalentOptions,
    ) -> Result<bool, OpenFstError>
    where
        F2: Fst<A> + AsFstCxx<A>;

    /// Tests if two FSTs are isomorphic (equal up to a state and arc reordering).
    fn isomorphic<F2>(&self, other: &F2, delta: f32) -> bool
    where
        F2: Fst<A> + AsFstCxx<A>;

    /// Tests if two FSTs are strictly equal in terms of exact state and arc orderings.
    fn equal<F2>(&self, other: &F2, delta: f32, etype: u8) -> bool
    where
        F2: Fst<A> + AsFstCxx<A>;

    /// Determines if the two deterministic, epsilon-free FSTs accept exactly the same
    /// set of strings with the same weights.
    ///
    /// Dynamically verifies properties for both FSTs and returns `Err` if they are not
    /// deterministic epsilon-free acceptors, or if their symbol tables are incompatible.
    fn equivalent<F2>(&self, other: &F2, delta: f32) -> Result<bool, OpenFstError>
    where
        Self: Sized + Fst<A> + AsFstCxx<A>,
        F2: Fst<A> + AsFstCxx<A>,
    {
        let v_self = self.verify::<DetEpsFreeAcceptor>()?;
        let v_other = other.verify::<DetEpsFreeAcceptor>()?;

        v_self.equivalent(v_other, delta)
    }
}

impl<F1, A> Equivalent<A> for F1
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + EquivalentFfi,
    A::Weight: WeightFfi,
    F1: Fst<A> + AsFstCxx<A>,
{
    fn randequivalent<F2>(
        &self,
        other: &F2,
        opts: &RandEquivalentOptions,
    ) -> Result<bool, OpenFstError>
    where
        F2: Fst<A> + AsFstCxx<A>,
    {
        let res = A::fst_randequivalent(
            unsafe { self.as_fst_cxx() },
            unsafe { other.as_fst_cxx() },
            opts.npath,
            opts.delta,
            opts.seed,
            opts.max_length,
        )?;
        Ok(res)
    }

    fn isomorphic<F2>(&self, other: &F2, delta: f32) -> bool
    where
        F2: Fst<A> + AsFstCxx<A>,
    {
        A::fst_isomorphic(
            unsafe { self.as_fst_cxx() },
            unsafe { other.as_fst_cxx() },
            delta,
        )
    }

    fn equal<F2>(&self, other: &F2, delta: f32, etype: u8) -> bool
    where
        F2: Fst<A> + AsFstCxx<A>,
    {
        A::fst_equal(
            unsafe { self.as_fst_cxx() },
            unsafe { other.as_fst_cxx() },
            delta,
            etype,
        )
    }
}
