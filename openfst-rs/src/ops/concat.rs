use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::concat::ConcatFfi;
use crate::ffi::fst::FstFfi;
use crate::fst::{AsFstCxx, AsMutFstCxx, Fst, MutableFst};

pub trait Concat<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + ConcatFfi,
    A::Weight: WeightFfi,
{
    /// Computes the concatenation (product) of two FSTs.
    /// This version modifies `self` in-place, appending `other` to the end of `self`.
    fn concat<F>(&mut self, other: &F)
    where
        F: Fst<A> + AsFstCxx<A>;
}

impl<M, A> Concat<A> for M
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + ConcatFfi,
    A::Weight: WeightFfi,
    M: MutableFst<A> + AsMutFstCxx<A>,
{
    #[rustfmt::skip]
    fn concat<F>(&mut self, other: &F)
    where
        F: Fst<A> + AsFstCxx<A>,
    {
        A::fst_concat(
            unsafe { self.as_mut_fst_cxx() },
            unsafe { other.as_fst_cxx() },
        );
    }
}
