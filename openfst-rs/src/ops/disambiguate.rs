use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::disambiguate::DisambiguateFfi;
use crate::ffi::fst::FstFfi;
use crate::fst::{AsFstCxx, AsMutFstCxx, Fst, MutableFst, NO_STATE_ID};
use crate::weight::{DELTA, LeftSemiring, Weight};

#[derive(Debug, Clone)]
pub struct DisambiguateOptions<W: Weight> {
    pub delta: f32,
    pub weight_threshold: W,
    pub state_threshold: i32,
    pub subsequential_label: i32,
}

impl<W: Weight> Default for DisambiguateOptions<W> {
    fn default() -> Self {
        Self {
            delta: DELTA,
            weight_threshold: W::zero(),
            state_threshold: NO_STATE_ID,
            subsequential_label: 0,
        }
    }
}

pub trait Disambiguate<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + DisambiguateFfi,
    A::Weight: WeightFfi + LeftSemiring,
{
    /// Disambiguates a weighted FST.
    /// This writes an equivalent FST to `self` that has the property that there are
    /// not two distinct paths from the initial state to a final state with the same input labeling.
    ///
    /// The weights must be (weakly) left divisible (valid for Tropical and LogWeight).
    fn disambiguate<F>(&mut self, ifst: &F, opts: &DisambiguateOptions<A::Weight>)
    where
        F: Fst<A> + AsFstCxx<A>;
}

impl<M, A> Disambiguate<A> for M
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + DisambiguateFfi,
    A::Weight: WeightFfi + LeftSemiring,
    M: MutableFst<A> + AsMutFstCxx<A>,
{
    fn disambiguate<F>(&mut self, ifst: &F, opts: &DisambiguateOptions<A::Weight>)
    where
        F: Fst<A> + AsFstCxx<A>,
    {
        A::fst_disambiguate(
            unsafe { ifst.as_fst_cxx() },
            unsafe { self.as_mut_fst_cxx() },
            opts.delta,
            opts.weight_threshold.as_ffi(),
            opts.state_threshold,
            opts.subsequential_label,
        );
    }
}
