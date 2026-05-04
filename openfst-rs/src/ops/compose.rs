use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::compose::ComposeFfi;
use crate::ffi::fst::FstFfi;
use crate::fst::{AsFstCxx, AsMutFstCxx, Fst, MutableFst};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposeFilter {
    AutoFilter = 0,
    NullFilter = 1,
    TrivialFilter = 2,
    SequenceFilter = 3,
    AltSequenceFilter = 4,
    MatchFilter = 5,
    NoMatchFilter = 6,
}

#[derive(Debug, Clone)]
pub struct ComposeOptions {
    pub connect: bool,
    pub filter_type: ComposeFilter,
}

impl Default for ComposeOptions {
    fn default() -> Self {
        Self {
            connect: true,
            filter_type: ComposeFilter::AutoFilter,
        }
    }
}

pub trait Compose<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + ComposeFfi,
    A::Weight: WeightFfi,
{
    /// Computes the eager composition of two transducers.
    /// The composed result is written into this mutable FST.
    fn compose<F1, F2>(&mut self, fst1: &F1, fst2: &F2, opts: &ComposeOptions)
    where
        F1: Fst<A> + AsFstCxx<A>,
        F2: Fst<A> + AsFstCxx<A>;
}

impl<M, A> Compose<A> for M
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + ComposeFfi,
    A::Weight: WeightFfi,
    M: MutableFst<A> + AsMutFstCxx<A>,
{
    fn compose<F1, F2>(&mut self, fst1: &F1, fst2: &F2, opts: &ComposeOptions)
    where
        F1: Fst<A> + AsFstCxx<A>,
        F2: Fst<A> + AsFstCxx<A>,
    {
        A::fst_compose(
            unsafe { fst1.as_fst_cxx() },
            unsafe { fst2.as_fst_cxx() },
            unsafe { self.as_mut_fst_cxx() },
            opts.connect,
            opts.filter_type as i32,
        );
    }
}
