use crate::arc::Arc;
use crate::error::OpenFstError;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use crate::ffi::verify::VerifyFfi;
use crate::fst::{AsFstCxx, Fst};

pub trait Verify<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + VerifyFfi,
    A::Weight: WeightFfi,
{
    /// Verifies that an FST's contents and properties are sane and consistent.
    /// Returns `Ok(())` if the FST is valid, or `Err(OpenFstError)` detailing the corruption.
    fn verify_fst(&self, allow_negative_labels: bool) -> Result<(), OpenFstError>;
}

impl<F, A> Verify<A> for F
where
    F: Fst<A> + AsFstCxx<A>,
    A: Arc<StateId = i32, Label = i32> + FstFfi + VerifyFfi,
    A::Weight: WeightFfi,
{
    fn verify_fst(&self, allow_negative_labels: bool) -> Result<(), OpenFstError> {
        let mut err_msg = String::new();
        let is_valid = A::fst_verify(
            unsafe { self.as_fst_cxx() },
            allow_negative_labels,
            &mut err_msg,
        );

        if is_valid {
            Ok(())
        } else {
            Err(OpenFstError::new(err_msg))
        }
    }
}
