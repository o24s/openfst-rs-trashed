use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use crate::ffi::union::UnionFfi;
use crate::fst::{AsFstCxx, AsMutFstCxx, Fst, MutableFst};

pub trait Union<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + UnionFfi,
    A::Weight: WeightFfi,
{
    /// Computes the union (sum) of two FSTs.
    /// This version modifies `self` in-place.
    fn union<F>(&mut self, other: &F)
    where
        F: Fst<A> + AsFstCxx<A>;
}

impl<M, A> Union<A> for M
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + UnionFfi,
    A::Weight: WeightFfi,
    M: MutableFst<A> + AsMutFstCxx<A>,
{
    #[rustfmt::skip]
    fn union<F>(&mut self, other: &F)
    where
        F: Fst<A> + AsFstCxx<A>,
    {
        A::fst_union(
            unsafe { self.as_mut_fst_cxx() },
            unsafe { other.as_fst_cxx() },
        );
    }
}
