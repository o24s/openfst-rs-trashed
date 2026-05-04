use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use crate::ffi::shortest_path::ShortestPathFfi;
use crate::fst::{AsFstCxx, AsMutFstCxx, Fst, MutableFst, NO_STATE_ID};
use crate::weight::{DELTA, PathWeight, RightSemiring, Weight};

#[derive(Debug, Clone)]
pub struct ShortestPathOptions<W: Weight> {
    pub nshortest: i32,
    pub unique: bool,
    pub first_path: bool,
    pub weight_threshold: W,
    pub state_threshold: i32,
    pub delta: f32,
}

impl<W: Weight> Default for ShortestPathOptions<W> {
    fn default() -> Self {
        Self {
            nshortest: 1,
            unique: false,
            first_path: false,
            weight_threshold: W::zero(),
            state_threshold: NO_STATE_ID,
            delta: DELTA,
        }
    }
}

pub trait ShortestPath<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + ShortestPathFfi,
    A::Weight: WeightFfi + PathWeight + RightSemiring,
{
    fn shortest_path<F>(&mut self, ifst: &F, opts: &ShortestPathOptions<A::Weight>)
    where
        F: Fst<A> + AsFstCxx<A>;
}

impl<M, A> ShortestPath<A> for M
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + ShortestPathFfi,
    A::Weight: WeightFfi + PathWeight + RightSemiring,
    M: MutableFst<A> + AsMutFstCxx<A>,
{
    fn shortest_path<F>(&mut self, ifst: &F, opts: &ShortestPathOptions<A::Weight>)
    where
        F: Fst<A> + AsFstCxx<A>,
    {
        A::fst_shortest_path(
            unsafe { ifst.as_fst_cxx() },
            unsafe { self.as_mut_fst_cxx() },
            opts.nshortest,
            opts.unique,
            opts.first_path,
            opts.weight_threshold.as_ffi(),
            opts.state_threshold,
            opts.delta,
        );
    }
}
