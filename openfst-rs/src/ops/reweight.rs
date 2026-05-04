use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use crate::ffi::reweight::ReweightFfi;
use crate::fst::{AsMutFstCxx, MutableFst};
use crate::ops::push::ReweightType;
use crate::weight::{LeftSemiring, RightSemiring};

pub trait Reweight<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + ReweightFfi,
    A::Weight: WeightFfi + LeftSemiring + RightSemiring,
{
    /// Reweights an FST according to a slice of potentials in a given direction.
    /// The weight must be left distributive when reweighting towards the initial
    /// state and right distributive when reweighting towards the final states.
    ///
    /// Note: The `potentials` array must be indexed by state ID.
    fn reweight(&mut self, potentials: &[A::Weight], reweight_type: ReweightType);
}

impl<M, A> Reweight<A> for M
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + ReweightFfi,
    A::Weight: WeightFfi + LeftSemiring + RightSemiring,
    M: MutableFst<A> + AsMutFstCxx<A>,
{
    fn reweight(&mut self, potentials: &[A::Weight], reweight_type: ReweightType) {
        // Convert array of Weights to an array of raw FFI value types.
        let ffi_potentials: Vec<<A::Weight as WeightFfi>::ValueType> =
            potentials.iter().map(|w| w.as_ffi()).collect();

        A::fst_reweight(
            unsafe { self.as_mut_fst_cxx() },
            &ffi_potentials,
            reweight_type as i32,
        );
    }
}
