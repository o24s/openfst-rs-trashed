use std::path::Path;

use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::const_fst::ConstFstFfi;
use crate::ffi::fst::FstFfi;
use crate::fst::{AsFstCxx, ExpandedFst, Fst, GenericFstArcIter, GenericFstStateIter};
use crate::symbol_table::SymbolTable;
use cxx::UniquePtr;

pub struct ConstFst<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + ConstFstFfi,
    A::Weight: WeightFfi,
{
    inner: UniquePtr<A::ConstFstCxx>,
}

impl<A> ConstFst<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + ConstFstFfi,
    A::Weight: WeightFfi,
{
    /// Reads a ConstFst from a file.
    /// Because ConstFst supports memory mapping, loading even a multi-gigabyte
    /// FST can be nearly instantaneous if the file format matches exactly.
    pub fn read<P: AsRef<Path>>(path: P) -> Option<Self> {
        let ptr = A::read_const_fst(path.as_ref().to_str()?);
        if ptr.is_null() {
            None
        } else {
            Some(Self { inner: ptr })
        }
    }

    /// Creates a new ConstFst by copying an existing FST of any type.
    pub fn new_from_fst<F>(fst: &F) -> Self
    where
        F: Fst<A> + AsFstCxx<A>,
    {
        let ptr = A::create_const_fst_from_fst(unsafe { fst.as_fst_cxx() });
        Self { inner: ptr }
    }

    /// Writes the ConstFst to a file.
    pub fn write<P: AsRef<Path>>(&self, path: P) -> bool {
        if let Some(p) = path.as_ref().to_str() {
            A::write_const_fst(&self.inner, p)
        } else {
            false
        }
    }
}

pub type StdConstFst = ConstFst<crate::arc::StdArc>;
pub type Log64ConstFst = ConstFst<crate::arc::Log64Arc>;

unsafe impl<A> AsFstCxx<A> for ConstFst<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + ConstFstFfi,
    A::Weight: WeightFfi,
{
    #[inline]
    unsafe fn as_fst_cxx(&self) -> &A::FstCxx {
        A::const_fst_as_fst(&*self.inner)
    }
}

impl<A> Fst<A> for ConstFst<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + ConstFstFfi,
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

impl<A> ExpandedFst<A> for ConstFst<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + ConstFstFfi,
    A::Weight: WeightFfi,
{
    #[inline]
    fn num_states(&self) -> A::StateId {
        A::fst_num_states(unsafe { self.as_fst_cxx() })
    }
}
