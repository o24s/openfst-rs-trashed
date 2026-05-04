use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use crate::arc::Arc;
use crate::error::OpenFstError;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use crate::fst::Fst;

// OpenFst property bitmasks (from properties.h)
pub const K_EXPANDED: u64 = 0x0000000000000001;
pub const K_MUTABLE: u64 = 0x0000000000000002;
pub const K_ERROR: u64 = 0x0000000000000004;
pub const K_ACCEPTOR: u64 = 0x0000000000010000;
pub const K_NOT_ACCEPTOR: u64 = 0x0000000000020000;
pub const K_I_DETERMINISTIC: u64 = 0x0000000000040000;
pub const K_NON_I_DETERMINISTIC: u64 = 0x0000000000080000;
pub const K_O_DETERMINISTIC: u64 = 0x0000000000100000;
pub const K_NON_O_DETERMINISTIC: u64 = 0x0000000000200000;
pub const K_EPSILONS: u64 = 0x0000000000400000;
pub const K_NO_EPSILONS: u64 = 0x0000000000800000;
pub const K_I_EPSILONS: u64 = 0x0000000001000000;
pub const K_NO_I_EPSILONS: u64 = 0x0000000002000000;
pub const K_O_EPSILONS: u64 = 0x0000000004000000;
pub const K_NO_O_EPSILONS: u64 = 0x0000000008000000;
pub const K_I_LABEL_SORTED: u64 = 0x0000000010000000;
pub const K_NOT_I_LABEL_SORTED: u64 = 0x0000000020000000;
pub const K_O_LABEL_SORTED: u64 = 0x0000000040000000;
pub const K_NOT_O_LABEL_SORTED: u64 = 0x0000000080000000;
pub const K_WEIGHTED: u64 = 0x0000000100000000;
pub const K_UNWEIGHTED: u64 = 0x0000000200000000;
pub const K_CYCLIC: u64 = 0x0000000400000000;
pub const K_ACYCLIC: u64 = 0x0000000800000000;
pub const K_INITIAL_CYCLIC: u64 = 0x0000001000000000;
pub const K_INITIAL_ACYCLIC: u64 = 0x0000002000000000;
pub const K_TOP_SORTED: u64 = 0x0000004000000000;
pub const K_NOT_TOP_SORTED: u64 = 0x0000008000000000;
pub const K_ACCESSIBLE: u64 = 0x0000010000000000;
pub const K_NOT_ACCESSIBLE: u64 = 0x0000020000000000;
pub const K_CO_ACCESSIBLE: u64 = 0x0000040000000000;
pub const K_NOT_CO_ACCESSIBLE: u64 = 0x0000080000000000;
pub const K_STRING: u64 = 0x0000100000000000;
pub const K_NOT_STRING: u64 = 0x0000200000000000;
pub const K_WEIGHTED_CYCLES: u64 = 0x0000400000000000;
pub const K_UNWEIGHTED_CYCLES: u64 = 0x0000800000000000;

/// A marker trait for expected FST properties.
pub trait FstProperty {
    const MASK: u64;
    const EXPECTED: u64;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Acceptor;
impl FstProperty for Acceptor {
    const MASK: u64 = K_ACCEPTOR;
    const EXPECTED: u64 = K_ACCEPTOR;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DetEpsFreeAcceptor;
impl FstProperty for DetEpsFreeAcceptor {
    const MASK: u64 = K_ACCEPTOR | K_I_DETERMINISTIC | K_NO_EPSILONS;
    const EXPECTED: u64 = K_ACCEPTOR | K_I_DETERMINISTIC | K_NO_EPSILONS;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UnweightedDetEpsFreeAcceptor;
impl FstProperty for UnweightedDetEpsFreeAcceptor {
    const MASK: u64 = K_ACCEPTOR | K_I_DETERMINISTIC | K_NO_EPSILONS | K_UNWEIGHTED;
    const EXPECTED: u64 = K_ACCEPTOR | K_I_DETERMINISTIC | K_NO_EPSILONS | K_UNWEIGHTED;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Acyclic;
impl FstProperty for Acyclic {
    const MASK: u64 = K_ACYCLIC;
    const EXPECTED: u64 = K_ACYCLIC;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StringFst;
impl FstProperty for StringFst {
    const MASK: u64 = K_STRING;
    const EXPECTED: u64 = K_STRING;
}

/// A generic wrapper that asserts the inner value `T` satisfies the property `P`.
/// Like rustc's `Binder`, this has exactly the same memory layout as `T`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Verified<T, P> {
    pub(crate) value: T,
    _marker: PhantomData<P>,
}

impl<T, P> Verified<T, P> {
    /// Wraps `value` in a `Verified` without performing any checks.
    /// The caller must guarantee that the property `P` is fully satisfied.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that the underlying FST exactly satisfies the properties
    /// defined by `P`. Specifically, the bitwise operation `fst.properties(P::MASK, false) & P::MASK`
    /// must rigorously equal `P::EXPECTED`.
    ///
    /// Passing an FST that does not meet these requirements into functions that expect
    /// a `Verified` wrapper violates internal invariants. This may result in incorrect
    /// algorithmic outputs, C++ exceptions (panics), or undefined behavior (UB) if the
    /// underlying OpenFst C++ library relies on these properties for memory safety.
    #[inline]
    pub unsafe fn assume(value: T) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }

    /// Extracts the inner value, dropping the property assertions.
    #[inline]
    pub fn skip_verification(self) -> T {
        self.value
    }
}

impl<T, P> Deref for Verified<T, P> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T, P> DerefMut for Verified<T, P> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

/// Extension trait to easily verify properties on any FST.
pub trait FstPropertiesExt<A>: Fst<A>
where
    A: Arc + FstFfi,
    A::Weight: WeightFfi,
{
    /// Verifies that the FST satisfies the given property `P` and returns a verified immutable reference.
    fn verify<P: FstProperty>(&self) -> Result<Verified<&Self, P>, OpenFstError> {
        if self.properties(P::MASK, true) & P::MASK == P::EXPECTED {
            Ok(unsafe { Verified::assume(self) })
        } else {
            Err(OpenFstError::new(format!(
                "FST does not satisfy the required property mask: {:016x}",
                P::MASK
            )))
        }
    }

    /// Verifies that the FST satisfies the given property `P` and returns a verified mutable reference.
    fn verify_mut<P: FstProperty>(&mut self) -> Result<Verified<&mut Self, P>, OpenFstError> {
        if self.properties(P::MASK, true) & P::MASK == P::EXPECTED {
            Ok(unsafe { Verified::assume(self) })
        } else {
            Err(OpenFstError::new(format!(
                "FST does not satisfy the required property mask: {:016x}",
                P::MASK
            )))
        }
    }
}

// Blanket implementation for any FST
impl<A, F> FstPropertiesExt<A> for F
where
    F: Fst<A>,
    A: Arc + FstFfi,
    A::Weight: WeightFfi,
{
}
