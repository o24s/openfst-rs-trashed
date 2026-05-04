use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use crate::ffi::rmfinalepsilon::RmFinalEpsilonFfi;
use crate::fst::{AsMutFstCxx, MutableFst};

pub trait RmFinalEpsilon<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + RmFinalEpsilonFfi,
    A::Weight: WeightFfi,
{
    /// Removes final states that have epsilon-only input arcs.
    /// This is a lighter alternative to `RmEpsilon` when you only want to clean up
    /// trailing epsilon transitions heading into final states.
    /// Modifies the FST in-place.
    fn rmfinalepsilon(&mut self);
}

impl<M, A> RmFinalEpsilon<A> for M
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + RmFinalEpsilonFfi,
    A::Weight: WeightFfi,
    M: MutableFst<A> + AsMutFstCxx<A>,
{
    fn rmfinalepsilon(&mut self) {
        A::fst_rmfinalepsilon(unsafe { self.as_mut_fst_cxx() });
    }
}
