use crate::arc::Arc;
use crate::ffi::fst::FstFfi;
use crate::ffi::power_map::{PowerMapFfi, PowerMapFromFfi, PowerProjectFfi};
use crate::ffi::vector_fst::VectorFstFfi;
use crate::fst::{AsFstCxx, AsMutFstCxx, Fst, MutableFst};
use crate::vector_fst::VectorFst;

pub trait MapPower<A> {
    /// Maps a 1D weight FST into an N-D PowerWeight FST, inserting the weight at `index` (0-based) and zeroes elsewhere.
    fn map_to_power<ToArc>(&self, index: usize) -> VectorFst<ToArc>
    where
        ToArc: Arc<StateId = i32, Label = i32> + FstFfi + VectorFstFfi,
        ToArc::Weight: crate::ffi::WeightFfi,
        A: PowerMapFfi<ToArc>,
        <A as Arc>::Weight: crate::ffi::WeightFfi;

    /// Extracts the weight at `index` (0-based) from an N-D PowerWeight FST and returns a 1D weight FST.
    fn map_from_power<FromArc>(&self, index: usize) -> VectorFst<FromArc>
    where
        FromArc: Arc<StateId = i32, Label = i32> + FstFfi + VectorFstFfi,
        FromArc::Weight: crate::ffi::WeightFfi,
        A: PowerMapFromFfi<FromArc>,
        <A as Arc>::Weight: crate::ffi::WeightFfi;
}

impl<F, A> MapPower<A> for F
where
    A: Arc<StateId = i32, Label = i32> + FstFfi,
    A::Weight: crate::ffi::WeightFfi,
    F: Fst<A> + AsFstCxx<A>,
{
    fn map_to_power<ToArc>(&self, index: usize) -> VectorFst<ToArc>
    where
        ToArc: Arc<StateId = i32, Label = i32> + FstFfi + VectorFstFfi,
        ToArc::Weight: crate::ffi::WeightFfi,
        A: PowerMapFfi<ToArc>,
        <A as Arc>::Weight: crate::ffi::WeightFfi,
    {
        let mut ofst = VectorFst::<ToArc>::new();
        A::fst_map_to_power(
            unsafe { self.as_fst_cxx() },
            unsafe { ofst.as_mut_fst_cxx() },
            index,
        );
        ofst
    }

    fn map_from_power<FromArc>(&self, index: usize) -> VectorFst<FromArc>
    where
        FromArc: Arc<StateId = i32, Label = i32> + FstFfi + VectorFstFfi,
        FromArc::Weight: crate::ffi::WeightFfi,
        A: PowerMapFromFfi<FromArc>,
        <A as Arc>::Weight: crate::ffi::WeightFfi,
    {
        let mut ofst = VectorFst::<FromArc>::new();
        A::fst_map_from_power(
            unsafe { self.as_fst_cxx() },
            unsafe { ofst.as_mut_fst_cxx() },
            index,
        );
        ofst
    }
}

pub trait ProjectPower<A> {
    /// Moves the weight at `from_index` to `to_index` in a PowerWeight FST, replacing the old `from_index` with Zero.
    fn project_power(&mut self, from_index: usize, to_index: usize);
}

impl<M, A> ProjectPower<A> for M
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + PowerProjectFfi,
    A::Weight: crate::ffi::WeightFfi,
    M: MutableFst<A> + AsMutFstCxx<A>,
{
    fn project_power(&mut self, from_index: usize, to_index: usize) {
        A::fst_project_power(unsafe { self.as_mut_fst_cxx() }, from_index, to_index);
    }
}
