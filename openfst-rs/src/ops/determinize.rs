use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::determinize::DeterminizeFfi;
use crate::ffi::fst::FstFfi;
use crate::fst::{AsFstCxx, AsMutFstCxx, Fst, MutableFst, NO_STATE_ID};
use crate::weight::{DELTA, LeftSemiring, Weight};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeterminizeType {
    Functional = 0,
    NonFunctional = 1,
    Disambiguate = 2,
}

#[derive(Debug, Clone)]
pub struct DeterminizeOptions<W: Weight> {
    pub delta: f32,
    pub weight_threshold: W,
    pub state_threshold: i32,
    pub subsequential_label: i32,
    pub det_type: DeterminizeType,
    pub increment_subsequential_label: bool,
}

impl<W: Weight> Default for DeterminizeOptions<W> {
    fn default() -> Self {
        Self {
            delta: DELTA,
            weight_threshold: W::zero(),
            state_threshold: NO_STATE_ID,
            subsequential_label: 0,
            det_type: DeterminizeType::Functional,
            increment_subsequential_label: false,
        }
    }
}

pub trait Determinize<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + DeterminizeFfi,
    A::Weight: WeightFfi + LeftSemiring,
{
    fn determinize<F>(&mut self, ifst: &F, opts: &DeterminizeOptions<A::Weight>)
    where
        F: Fst<A> + AsFstCxx<A>;
}

impl<M, A> Determinize<A> for M
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + DeterminizeFfi,
    A::Weight: WeightFfi + LeftSemiring,
    M: MutableFst<A> + AsMutFstCxx<A>,
{
    fn determinize<F>(&mut self, ifst: &F, opts: &DeterminizeOptions<A::Weight>)
    where
        F: Fst<A> + AsFstCxx<A>,
    {
        A::fst_determinize(
            unsafe { ifst.as_fst_cxx() },
            unsafe { self.as_mut_fst_cxx() },
            opts.delta,
            opts.weight_threshold.as_ffi(),
            opts.state_threshold,
            opts.subsequential_label,
            opts.det_type as i32,
            opts.increment_subsequential_label,
        );
    }
}
