use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use crate::ffi::rmepsilon::RmEpsilonFfi;
use crate::fst::{AsMutFstCxx, MutableFst, NO_STATE_ID};
use crate::weight::{DELTA, Weight};

#[derive(Debug, Clone)]
pub struct RmEpsilonOptions<W: Weight> {
    /// If true, unconnected states will be removed.
    pub connect: bool,
    /// Pruning weight threshold (only allowed for weights with Path property like Tropical).
    pub weight_threshold: W,
    /// Pruning state threshold.
    pub state_threshold: i32,
    pub delta: f32,
}

impl<W: Weight> Default for RmEpsilonOptions<W> {
    fn default() -> Self {
        Self {
            connect: true,
            weight_threshold: W::zero(),
            state_threshold: NO_STATE_ID,
            delta: DELTA,
        }
    }
}

pub trait RmEpsilon<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + RmEpsilonFfi,
    A::Weight: WeightFfi,
{
    /// Removes epsilon-transitions (where both input and output labels are epsilon).
    /// This operation mutates the FST in-place.
    fn rmepsilon(&mut self, opts: &RmEpsilonOptions<A::Weight>);
}

impl<M, A> RmEpsilon<A> for M
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + RmEpsilonFfi,
    A::Weight: WeightFfi,
    M: MutableFst<A> + AsMutFstCxx<A>,
{
    fn rmepsilon(&mut self, opts: &RmEpsilonOptions<A::Weight>) {
        A::fst_rmepsilon(
            unsafe { self.as_mut_fst_cxx() },
            opts.connect,
            opts.weight_threshold.as_ffi(),
            opts.state_threshold,
            opts.delta,
        );
    }
}
