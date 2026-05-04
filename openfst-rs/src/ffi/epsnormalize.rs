use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use openfst_rs_macros::template;
use std::pin::Pin;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EpsNormalizeType {
    Input = 0,
    Output = 1,
}

pub trait EpsNormalizeFfi: Arc<StateId = i32, Label = i32> + FstFfi
where
    Self::Weight: WeightFfi,
{
    fn fst_epsnormalize(
        ifst: &Self::FstCxx,
        ofst: Pin<&mut Self::MutableFstCxx>,
        norm_type: EpsNormalizeType,
    );
}

#[template("epsnormalize")]
pub mod ffi_binding {
    unsafe extern "C++" {
        include!("cpp/epsnormalize.h");

        #[expand(for_each_arc_type)]
        type Fst___SFX__ = crate::ffi::fst::ffi_binding::Fst___SFX__;
        #[expand(for_each_arc_type)]
        type MutableFst___SFX__ = crate::ffi::fst::ffi_binding::MutableFst___SFX__;

        #[expand(for_each_arc_type)]
        fn fst_epsnormalize___SFX__(
            ifst: &Fst___SFX__,
            ofst: Pin<&mut MutableFst___SFX__>,
            norm_type: i32,
        );
    }
}

macro_rules! bind_epsnormalize_ffi_impl {
    ($arc:ty, $wtype:ty, $sfx:ident) => {
        pastey::paste! {
            impl EpsNormalizeFfi for $arc {
                fn fst_epsnormalize(
                    ifst: &Self::FstCxx,
                    ofst: Pin<&mut Self::MutableFstCxx>,
                    norm_type: EpsNormalizeType,
                ) {
                    ffi_binding::[<fst_epsnormalize_ $sfx>](ifst, ofst, norm_type as i32)
                }
            }
        }
    };
}

crate::for_each_arc_type!(bind_epsnormalize_ffi_impl);
