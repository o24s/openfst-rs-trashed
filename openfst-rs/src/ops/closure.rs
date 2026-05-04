use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::closure::ClosureFfi;
use crate::ffi::fst::FstFfi;
use crate::fst::{AsMutFstCxx, MutableFst};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClosureType {
    /// Kleene closure (A*): accepts the empty string as well.
    Star = 0,
    /// Positive closure (A+): does not implicitly accept the empty string.
    Plus = 1,
}

pub trait Closure<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + ClosureFfi,
    A::Weight: WeightFfi,
{
    /// Computes the concatenative closure of the FST.
    /// Modifies `self` in-place.
    fn closure(&mut self, closure_type: ClosureType);
}

impl<M, A> Closure<A> for M
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + ClosureFfi,
    A::Weight: WeightFfi,
    M: MutableFst<A> + AsMutFstCxx<A>,
{
    fn closure(&mut self, closure_type: ClosureType) {
        A::fst_closure(unsafe { self.as_mut_fst_cxx() }, closure_type as i32);
    }
}
