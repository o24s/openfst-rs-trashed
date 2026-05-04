use std::marker::PhantomData;
use std::path::Path;

use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::compact_fst::{
    AcceptorCompactor, CompactFstFfi, StringCompactor, UnweightedAcceptorCompactor,
    UnweightedCompactor, WeightedStringCompactor, init_compact_fst_registry,
};
use crate::ffi::fst::FstFfi;
use crate::fst::{AsFstCxx, ExpandedFst, Fst, GenericFstArcIter, GenericFstStateIter};
use crate::symbol_table::SymbolTable;
use cxx::UniquePtr;

/// A memory-efficient representation of common types of FSTs (linear automata, acceptors, etc).
pub struct CompactFst<A, C>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + CompactFstFfi<C>,
    A::Weight: WeightFfi,
{
    inner: UniquePtr<A::CompactFstCxx>,
    _compactor: PhantomData<C>,
}

impl<A, C> CompactFst<A, C>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + CompactFstFfi<C>,
    A::Weight: WeightFfi,
{
    /// Reads a CompactFst from a file.
    pub fn read<P: AsRef<Path>>(path: P) -> Option<Self> {
        init_compact_fst_registry();
        let ptr = A::read_compact_fst(path.as_ref().to_str()?);
        if ptr.is_null() {
            None
        } else {
            Some(Self {
                inner: ptr,
                _compactor: PhantomData,
            })
        }
    }

    /// Creates a new CompactFst by compacting an existing FST.
    /// Note: The input FST must be compatible with the selected compactor
    /// (e.g., you cannot compact a transducer using `AcceptorCompactor`).
    /// Incompatible FSTs will cause OpenFst to log an error and set the `kError` property.
    pub fn new_from_fst<F>(fst: &F) -> Self
    where
        F: Fst<A> + AsFstCxx<A>,
    {
        init_compact_fst_registry();
        let ptr = A::create_compact_fst_from_fst(unsafe { fst.as_fst_cxx() });
        Self {
            inner: ptr,
            _compactor: PhantomData,
        }
    }

    /// Writes the CompactFst to a file.
    pub fn write<P: AsRef<Path>>(&self, path: P) -> bool {
        if let Some(p) = path.as_ref().to_str() {
            A::write_compact_fst(&self.inner, p)
        } else {
            false
        }
    }
}

// Type aliases for convenience
pub type CompactStringFst<A> = CompactFst<A, StringCompactor>;
pub type CompactWeightedStringFst<A> = CompactFst<A, WeightedStringCompactor>;
pub type CompactAcceptorFst<A> = CompactFst<A, AcceptorCompactor>;
pub type CompactUnweightedFst<A> = CompactFst<A, UnweightedCompactor>;
pub type CompactUnweightedAcceptorFst<A> = CompactFst<A, UnweightedAcceptorCompactor>;

pub type StdCompactStringFst = CompactStringFst<crate::arc::StdArc>;
pub type StdCompactAcceptorFst = CompactAcceptorFst<crate::arc::StdArc>;

unsafe impl<A, C> AsFstCxx<A> for CompactFst<A, C>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + CompactFstFfi<C>,
    A::Weight: WeightFfi,
{
    #[inline]
    unsafe fn as_fst_cxx(&self) -> &A::FstCxx {
        A::compact_fst_as_fst(&*self.inner)
    }
}

impl<A, C> Fst<A> for CompactFst<A, C>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + CompactFstFfi<C>,
    A::Weight: WeightFfi,
{
    type StateIter<'a>
        = GenericFstStateIter<'a, A>
    where
        Self: 'a;
    type ArcIter<'a>
        = GenericFstArcIter<'a, A>
    where
        Self: 'a;

    #[inline]
    fn fst_type(&self) -> String {
        A::fst_type(unsafe { self.as_fst_cxx() })
    }
    #[inline]
    fn start(&self) -> A::StateId {
        A::fst_start(unsafe { self.as_fst_cxx() })
    }
    #[inline]
    fn final_weight(&self, state: A::StateId) -> A::Weight {
        A::Weight::from_ffi(A::fst_final_weight(unsafe { self.as_fst_cxx() }, state))
    }
    #[inline]
    fn num_arcs(&self, state: A::StateId) -> usize {
        A::fst_num_arcs(unsafe { self.as_fst_cxx() }, state)
    }
    #[inline]
    fn num_input_epsilons(&self, state: A::StateId) -> usize {
        A::fst_num_input_epsilons(unsafe { self.as_fst_cxx() }, state)
    }
    #[inline]
    fn num_output_epsilons(&self, state: A::StateId) -> usize {
        A::fst_num_output_epsilons(unsafe { self.as_fst_cxx() }, state)
    }
    #[inline]
    fn num_states_if_known(&self) -> Option<usize> {
        let n = A::fst_num_states(unsafe { self.as_fst_cxx() });
        if n < 0 { None } else { Some(n as usize) }
    }
    #[inline]
    fn properties(&self, mask: u64, test: bool) -> u64 {
        A::fst_properties(unsafe { self.as_fst_cxx() }, mask, test)
    }

    fn input_symbols(&self) -> Option<SymbolTable> {
        let ptr = A::fst_input_symbols(unsafe { self.as_fst_cxx() });
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { SymbolTable::from_raw_ptr(ptr) })
        }
    }
    fn output_symbols(&self) -> Option<SymbolTable> {
        let ptr = A::fst_output_symbols(unsafe { self.as_fst_cxx() });
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { SymbolTable::from_raw_ptr(ptr) })
        }
    }

    fn states<'a>(&'a self) -> Self::StateIter<'a> {
        GenericFstStateIter::new(unsafe { self.as_fst_cxx() })
    }
    fn arcs<'a>(&'a self, state: A::StateId) -> Self::ArcIter<'a> {
        GenericFstArcIter::new(unsafe { self.as_fst_cxx() }, state)
    }
}

impl<A, C> ExpandedFst<A> for CompactFst<A, C>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + CompactFstFfi<C>,
    A::Weight: WeightFfi,
{
    #[inline]
    fn num_states(&self) -> A::StateId {
        A::fst_num_states(unsafe { self.as_fst_cxx() })
    }
}
