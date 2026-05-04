use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use crate::ffi::push::PushFfi;
use crate::fst::{AsFstCxx, AsMutFstCxx, Fst, MutableFst};
use crate::weight::{LeftSemiring, RightSemiring};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReweightType {
    ToInitial = 0,
    ToFinal = 1,
}

pub struct PushType;
impl PushType {
    pub const PUSH_WEIGHTS: u8 = 0x01;
    pub const PUSH_LABELS: u8 = 0x02;
    pub const PUSH_REMOVE_TOTAL_WEIGHT: u8 = 0x04;
    pub const PUSH_REMOVE_COMMON_AFFIX: u8 = 0x08;
}

pub trait Push<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + PushFfi,
    A::Weight: WeightFfi + LeftSemiring + RightSemiring,
{
    fn push(&mut self, rtype: ReweightType, delta: f32, remove_total_weight: bool);

    fn push_of<F>(&mut self, ifst: &F, ptype: u8, rtype: ReweightType, delta: f32)
    where
        F: Fst<A> + AsFstCxx<A>;
}

impl<M, A> Push<A> for M
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + PushFfi,
    A::Weight: WeightFfi + LeftSemiring + RightSemiring,
    M: MutableFst<A> + AsMutFstCxx<A>,
{
    fn push(&mut self, rtype: ReweightType, delta: f32, remove_total_weight: bool) {
        A::fst_push(
            unsafe { self.as_mut_fst_cxx() },
            rtype as i32,
            delta,
            remove_total_weight,
        );
    }

    fn push_of<F>(&mut self, ifst: &F, ptype: u8, rtype: ReweightType, delta: f32)
    where
        F: Fst<A> + AsFstCxx<A>,
    {
        A::fst_push_into(
            unsafe { ifst.as_fst_cxx() },
            unsafe { self.as_mut_fst_cxx() },
            ptype,
            rtype as i32,
            delta,
        );
    }
}
