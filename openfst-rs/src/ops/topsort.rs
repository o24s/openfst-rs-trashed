use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use crate::ffi::topsort::TopSortFfi;
use crate::fst::{AsMutFstCxx, MutableFst};

pub trait TopSort<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + TopSortFfi,
    A::Weight: WeightFfi,
{
    /// Topologically sorts the FST if it is acyclic, modifying it in-place.
    /// When sorted, all transitions are from lower to higher state IDs.
    /// Returns `true` if the FST was successfully sorted (i.e. it is acyclic).
    /// Returns `false` if the FST has cycles and cannot be topologically sorted.
    fn topsort(&mut self) -> bool;
}

impl<M, A> TopSort<A> for M
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + TopSortFfi,
    A::Weight: WeightFfi,
    M: MutableFst<A> + AsMutFstCxx<A>,
{
    fn topsort(&mut self) -> bool {
        A::fst_topsort(unsafe { self.as_mut_fst_cxx() })
    }
}
