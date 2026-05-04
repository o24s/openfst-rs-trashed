use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use openfst_rs_macros::template;
use std::pin::Pin;

pub trait RmFinalEpsilonFfi: Arc<StateId = i32, Label = i32> + FstFfi
where
    Self::Weight: WeightFfi,
{
    fn fst_rmfinalepsilon(fst: Pin<&mut Self::MutableFstCxx>);
}

#[template("rmfinalepsilon")]
pub mod ffi_binding {
    unsafe extern "C++" {
        include!("cpp/rmfinalepsilon.h");

        #[expand(for_each_arc_type)]
        type MutableFst___SFX__ = crate::ffi::fst::ffi_binding::MutableFst___SFX__;

        #[expand(for_each_arc_type)]
        fn fst_rmfinalepsilon___SFX__(fst: Pin<&mut MutableFst___SFX__>);
    }
}

macro_rules! bind_rmfinalepsilon_ffi_impl {
    ($arc:ty, $wtype:ty, $sfx:ident) => {
        pastey::paste! {
            impl RmFinalEpsilonFfi for $arc {
                fn fst_rmfinalepsilon(fst: Pin<&mut Self::MutableFstCxx>) {
                    ffi_binding::[<fst_rmfinalepsilon_ $sfx>](fst)
                }
            }
        }
    };
}

crate::for_each_arc_type!(bind_rmfinalepsilon_ffi_impl);
