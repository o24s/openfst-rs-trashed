use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use crate::ffi::reverse::ReverseFfi;
use crate::fst::{AsFstCxx, AsMutFstCxx, Fst, MutableFst};

pub trait Reverse<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + ReverseFfi,
    A::Weight: WeightFfi,
{
    /// Reverses an FST. The reversed result is written to this MutableFst.
    fn reverse_of<F>(&mut self, ifst: &F, require_superinitial: bool)
    where
        F: Fst<A> + AsFstCxx<A>;
}

impl<M, A> Reverse<A> for M
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + ReverseFfi,
    A::Weight: WeightFfi,
    M: MutableFst<A> + AsMutFstCxx<A>,
{
    fn reverse_of<F>(&mut self, ifst: &F, require_superinitial: bool)
    where
        F: Fst<A> + AsFstCxx<A>,
    {
        A::fst_reverse(
            unsafe { ifst.as_fst_cxx() },
            unsafe { self.as_mut_fst_cxx() },
            require_superinitial,
        );
    }
}
