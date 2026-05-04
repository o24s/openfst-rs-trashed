use crate::arc::Arc;
use crate::error::OpenFstError;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use crate::ffi::relabel::RelabelFfi;
use crate::fst::{AsMutFstCxx, MutableFst};

pub trait Relabel<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + RelabelFfi,
    A::Weight: WeightFfi,
{
    /// Relabels the input and/or output labels of the FST in-place.
    /// Unspecified labels remain unchanged.
    fn relabel(&mut self, ipairs: &[(i32, i32)], opairs: &[(i32, i32)])
    -> Result<(), OpenFstError>;
}

impl<M, A> Relabel<A> for M
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + RelabelFfi,
    A::Weight: WeightFfi,
    M: MutableFst<A> + AsMutFstCxx<A>,
{
    fn relabel(
        &mut self,
        ipairs: &[(i32, i32)],
        opairs: &[(i32, i32)],
    ) -> Result<(), OpenFstError> {
        let ipairs_flat =
            unsafe { std::slice::from_raw_parts(ipairs.as_ptr() as *const i32, ipairs.len() * 2) };
        let opairs_flat =
            unsafe { std::slice::from_raw_parts(opairs.as_ptr() as *const i32, opairs.len() * 2) };

        unsafe {
            A::fst_relabel(self.as_mut_fst_cxx(), ipairs_flat, opairs_flat)?;
        }
        Ok(())
    }
}
