use crate::arc::Arc;
use crate::error::OpenFstError;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use crate::ffi::shortest_distance::ShortestDistanceFfi;
use crate::fst::{AsFstCxx, Fst};
use crate::weight::{DELTA, RightSemiring};

#[derive(Debug, Clone)]
pub struct ShortestDistanceOptions {
    /// If false, computes distance from the start state to each state.
    /// If true, computes distance from each state to the final states.
    pub reverse: bool,
    pub delta: f32,
}

impl Default for ShortestDistanceOptions {
    fn default() -> Self {
        Self {
            reverse: false,
            delta: DELTA,
        }
    }
}

pub trait ShortestDistance<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + ShortestDistanceFfi,
    A::Weight: WeightFfi + RightSemiring,
{
    /// Computes the sum of the weight of all successful paths in an FST.
    /// (e.g. For LogWeight, this is the negative log of the sum of the probabilities).
    fn shortest_distance(&self, delta: f32) -> Result<A::Weight, OpenFstError>;

    /// Computes the shortest distance to/from each state.
    /// Returns a vector where the index corresponds to the `StateId`.
    /// Unvisited states will have `Weight::zero()`.
    fn shortest_distance_vec(
        &self,
        opts: &ShortestDistanceOptions,
    ) -> Result<Vec<A::Weight>, OpenFstError>;
}

impl<F, A> ShortestDistance<A> for F
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + ShortestDistanceFfi,
    A::Weight: WeightFfi + RightSemiring,
    F: Fst<A> + AsFstCxx<A>,
{
    fn shortest_distance(&self, delta: f32) -> Result<A::Weight, OpenFstError> {
        let val = A::fst_shortest_distance(unsafe { self.as_fst_cxx() }, delta)?;
        Ok(A::Weight::from_ffi(val))
    }

    fn shortest_distance_vec(
        &self,
        opts: &ShortestDistanceOptions,
    ) -> Result<Vec<A::Weight>, OpenFstError> {
        let mut raw_vec = Vec::new();
        A::fst_shortest_distance_vec(
            unsafe { self.as_fst_cxx() },
            &mut raw_vec,
            opts.reverse,
            opts.delta,
        )?;
        Ok(raw_vec.into_iter().map(A::Weight::from_ffi).collect())
    }
}
