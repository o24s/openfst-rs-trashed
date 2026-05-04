use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use crate::ffi::prune::PruneFfi;
use crate::fst::{AsFstCxx, AsMutFstCxx, Fst, MutableFst, NO_STATE_ID};
use crate::weight::{DELTA, Weight};

#[derive(Debug, Clone)]
pub struct PruneOptions<W: Weight> {
    pub weight_threshold: W,
    pub state_threshold: i32,
    pub delta: f32,
}

impl<W: Weight> Default for PruneOptions<W> {
    fn default() -> Self {
        Self {
            weight_threshold: W::zero(),
            state_threshold: NO_STATE_ID,
            delta: DELTA,
        }
    }
}

pub trait Prune<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + PruneFfi,
    A::Weight: WeightFfi,
{
    /// Prunes the FST in-place by removing states and arcs that do not belong to
    /// a successful path whose weight is <= (shortest path weight) * weight_threshold.
    fn prune(&mut self, opts: &PruneOptions<A::Weight>);

    /// Prunes the input FST and writes the result into this MutableFst.
    fn prune_of<F>(&mut self, ifst: &F, opts: &PruneOptions<A::Weight>)
    where
        F: Fst<A> + AsFstCxx<A>;
}

impl<M, A> Prune<A> for M
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + PruneFfi,
    A::Weight: WeightFfi,
    M: MutableFst<A> + AsMutFstCxx<A>,
{
    fn prune(&mut self, opts: &PruneOptions<A::Weight>) {
        A::fst_prune(
            unsafe { self.as_mut_fst_cxx() },
            opts.weight_threshold.as_ffi(),
            opts.state_threshold,
            opts.delta,
        );
    }

    fn prune_of<F>(&mut self, ifst: &F, opts: &PruneOptions<A::Weight>)
    where
        F: Fst<A> + AsFstCxx<A>,
    {
        A::fst_prune_into(
            unsafe { ifst.as_fst_cxx() },
            unsafe { self.as_mut_fst_cxx() },
            opts.weight_threshold.as_ffi(),
            opts.state_threshold,
            opts.delta,
        );
    }
}
