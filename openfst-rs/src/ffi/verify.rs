use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use openfst_rs_macros::template;

pub trait VerifyFfi: Arc<StateId = i32, Label = i32> + FstFfi
where
    Self::Weight: WeightFfi,
{
    fn fst_verify(
        fst: &Self::FstCxx,
        allow_negative_labels: bool,
        out_err_msg: &mut String,
    ) -> bool;
}

#[template("verify")]
pub mod ffi_binding {
    unsafe extern "C++" {
        include!("cpp/verify.h");

        #[expand(for_each_arc_type)]
        type Fst___SFX__ = crate::ffi::fst::ffi_binding::Fst___SFX__;

        #[expand(for_each_arc_type)]
        fn fst_verify___SFX__(
            fst: &Fst___SFX__,
            allow_negative_labels: bool,
            out_err_msg: &mut String,
        ) -> bool;
    }
}

macro_rules! bind_verify_ffi_impl {
    ($arc:ty, $wtype:ty, $sfx:ident) => {
        pastey::paste! {
            impl VerifyFfi for $arc {
                fn fst_verify(fst: &Self::FstCxx, allow_negative_labels: bool, out_err_msg: &mut String) -> bool {
                    ffi_binding::[<fst_verify_ $sfx>](fst, allow_negative_labels, out_err_msg)
                }
            }
        }
    };
}

crate::for_each_arc_type!(bind_verify_ffi_impl);
