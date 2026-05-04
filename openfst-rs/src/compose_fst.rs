use crate::arc::Arc;
use crate::cache::CacheOptions;
use crate::ffi::WeightFfi;
use crate::ffi::compose::ComposeFfi;
use crate::ffi::fst::FstFfi;
use crate::fst::{AsFstCxx, Fst, GenericFstArcIter, GenericFstStateIter};
use crate::symbol_table::SymbolTable;
use cxx::UniquePtr;

pub type StdComposeFst = ComposeFst<crate::arc::StdArc>;
pub type Log64ComposeFst = ComposeFst<crate::arc::Log64Arc>;

/// A lazy FST that computes the composition of two FSTs on-the-fly.
pub struct ComposeFst<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + ComposeFfi,
    A::Weight: WeightFfi,
{
    inner: UniquePtr<A::ComposeFstCxx>,
}

impl<A> ComposeFst<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + ComposeFfi,
    A::Weight: WeightFfi,
{
    /// Creates a delayed ComposeFst from two existing FSTs with default cache settings.
    pub fn new<F1, F2>(fst1: &F1, fst2: &F2) -> Self
    where
        F1: Fst<A> + AsFstCxx<A>,
        F2: Fst<A> + AsFstCxx<A>,
    {
        Self::with_cache_options(fst1, fst2, &CacheOptions::default())
    }

    /// Creates a delayed ComposeFst with explicit cache control (GC settings).
    pub fn with_cache_options<F1, F2>(fst1: &F1, fst2: &F2, cache: &CacheOptions) -> Self
    where
        F1: Fst<A> + AsFstCxx<A>,
        F2: Fst<A> + AsFstCxx<A>,
    {
        Self {
            inner: A::create_compose_fst(
                unsafe { fst1.as_fst_cxx() },
                unsafe { fst2.as_fst_cxx() },
                cache.gc,
                cache.gc_limit,
            ),
        }
    }
}

unsafe impl<A> AsFstCxx<A> for ComposeFst<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + ComposeFfi,
    A::Weight: WeightFfi,
{
    #[inline]
    unsafe fn as_fst_cxx(&self) -> &A::FstCxx {
        A::compose_fst_as_fst(&*self.inner)
    }
}

impl<A> Fst<A> for ComposeFst<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + ComposeFfi,
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
