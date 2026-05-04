use crate::arc::Arc;
use crate::cache::CacheOptions;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use crate::ffi::synchronize::SynchronizeFfi;
use crate::fst::{AsFstCxx, AsMutFstCxx, Fst, GenericFstArcIter, GenericFstStateIter, MutableFst};
use crate::symbol_table::SymbolTable;
use cxx::UniquePtr;

/// A delayed FST that synchronizes a transducer.
///
/// The result is an equivalent FST that has the property that during the traversal
/// of a path, the delay is either zero or strictly increasing, where the delay is the
/// difference between the number of non-epsilon output labels and input labels along the path.
///
/// For the algorithm to terminate, the input transducer must have bounded delay,
/// i.e., the delay of every cycle must be zero.
pub struct SynchronizeFst<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + SynchronizeFfi,
    A::Weight: WeightFfi,
{
    inner: UniquePtr<A::SynchronizeFstCxx>,
}

impl<A> SynchronizeFst<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + SynchronizeFfi,
    A::Weight: WeightFfi,
{
    /// Creates a delayed SynchronizeFst from an existing FST with default cache settings.
    pub fn new<F>(ifst: &F) -> Self
    where
        F: Fst<A> + AsFstCxx<A>,
    {
        Self::with_cache_options(ifst, &CacheOptions::default())
    }

    /// Creates a delayed SynchronizeFst with explicit cache control (GC settings).
    pub fn with_cache_options<F>(ifst: &F, cache: &CacheOptions) -> Self
    where
        F: Fst<A> + AsFstCxx<A>,
    {
        Self {
            inner: A::create_synchronize_fst(
                unsafe { ifst.as_fst_cxx() },
                cache.gc,
                cache.gc_limit,
            ),
        }
    }
}

unsafe impl<A> AsFstCxx<A> for SynchronizeFst<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + SynchronizeFfi,
    A::Weight: WeightFfi,
{
    #[inline]
    unsafe fn as_fst_cxx(&self) -> &A::FstCxx {
        A::synchronize_fst_as_fst(&*self.inner)
    }
}

impl<A> Fst<A> for SynchronizeFst<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + SynchronizeFfi,
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

pub trait Synchronize<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + SynchronizeFfi,
    A::Weight: WeightFfi,
{
    /// Synchronizes a transducer. This version writes the synchronized result to this MutableFst.
    ///
    /// For the algorithm to terminate, the input transducer must have bounded
    /// delay, i.e., the delay of every cycle must be zero.
    fn synchronize_of<F>(&mut self, ifst: &F)
    where
        F: Fst<A> + AsFstCxx<A>;
}

impl<M, A> Synchronize<A> for M
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + SynchronizeFfi,
    A::Weight: WeightFfi,
    M: MutableFst<A> + AsMutFstCxx<A>,
{
    fn synchronize_of<F>(&mut self, ifst: &F)
    where
        F: Fst<A> + AsFstCxx<A>,
    {
        A::fst_synchronize(unsafe { ifst.as_fst_cxx() }, unsafe {
            self.as_mut_fst_cxx()
        });
    }
}
