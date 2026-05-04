use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use crate::ffi::invert::InvertFfi;
use crate::fst::{AsFstCxx, AsMutFstCxx, Fst, MutableFst};

pub trait Invert<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + InvertFfi,
    A::Weight: WeightFfi,
{
    /// Inverts the FST in-place by exchanging the input and output labels.
    fn invert(&mut self);

    /// Computes the inverse of the input FST and stores it in this FST.
    fn invert_of<F>(&mut self, ifst: &F)
    where
        F: Fst<A> + AsFstCxx<A>;
}

impl<M, A> Invert<A> for M
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + InvertFfi,
    A::Weight: WeightFfi,
    M: MutableFst<A> + AsMutFstCxx<A>,
{
    fn invert(&mut self) {
        A::fst_invert_destructive(unsafe { self.as_mut_fst_cxx() });
    }

    #[rustfmt::skip]
    fn invert_of<F>(&mut self, ifst: &F)
    where
        F: Fst<A> + AsFstCxx<A>,
    {
        A::fst_invert_non_destructive(
            unsafe { ifst.as_fst_cxx() },
            unsafe { self.as_mut_fst_cxx() },
        );
    }
}
