use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::connect::ConnectFfi;
use crate::ffi::fst::FstFfi;
use crate::fst::{AsMutFstCxx, MutableFst};

pub trait Connect<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + ConnectFfi,
    A::Weight: WeightFfi,
{
    /// Removes states and arcs that are not on successful paths.
    /// (Paths connecting the start state to a final state).
    /// Modifies `self` in-place.
    fn connect(&mut self);
}

impl<M, A> Connect<A> for M
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + ConnectFfi,
    A::Weight: WeightFfi,
    M: MutableFst<A> + AsMutFstCxx<A>,
{
    fn connect(&mut self) {
        A::fst_connect(unsafe { self.as_mut_fst_cxx() });
    }
}
