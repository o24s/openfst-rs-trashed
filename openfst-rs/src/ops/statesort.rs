use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use crate::ffi::statesort::StateSortFfi;
use crate::fst::{AsMutFstCxx, ExpandedFst, MutableFst};

pub trait StateSort<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + StateSortFfi,
    A::Weight: WeightFfi,
{
    /// Sorts the input states of an FST in-place.
    ///
    /// `order[i]` gives the state ID after sorting that corresponds to the state ID `i` before sorting;
    /// it must therefore be a permutation of the input FST's state IDs.
    ///
    /// This is typically used to improve cache locality during FST traversal.
    fn statesort(&mut self, order: &[A::StateId]);
}

impl<M, A> StateSort<A> for M
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + StateSortFfi,
    A::Weight: WeightFfi,
    M: MutableFst<A> + ExpandedFst<A> + AsMutFstCxx<A>,
{
    fn statesort(&mut self, order: &[A::StateId]) {
        // Panic gracefully if the user provided an incorrectly sized order array.
        let num_states = self.num_states() as usize;
        assert_eq!(
            order.len(),
            num_states,
            "StateSort: order array length must match the number of states in the FST"
        );
        A::fst_statesort(unsafe { self.as_mut_fst_cxx() }, order);
    }
}
