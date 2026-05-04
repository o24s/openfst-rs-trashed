use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::arcsort::ArcSortFfi;
use crate::ffi::fst::FstFfi;
use crate::fst::{AsMutFstCxx, MutableFst};

pub trait ArcSort<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + ArcSortFfi,
    A::Weight: WeightFfi,
{
    fn arcsort_ilabel(&mut self);
    fn arcsort_olabel(&mut self);
}

impl<M, A> ArcSort<A> for M
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + ArcSortFfi,
    A::Weight: WeightFfi,
    M: MutableFst<A> + AsMutFstCxx<A>,
{
    fn arcsort_ilabel(&mut self) {
        A::arcsort_ilabel(unsafe { self.as_mut_fst_cxx() });
    }
    fn arcsort_olabel(&mut self) {
        A::arcsort_olabel(unsafe { self.as_mut_fst_cxx() });
    }
}
