use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use openfst_rs_macros::template;
use std::pin::Pin;

pub trait InvertFfi: Arc<StateId = i32, Label = i32> + FstFfi
where
    Self::Weight: WeightFfi,
{
    fn fst_invert_destructive(fst: Pin<&mut Self::MutableFstCxx>);
    fn fst_invert_non_destructive(ifst: &Self::FstCxx, ofst: Pin<&mut Self::MutableFstCxx>);
}

#[template("invert")]
pub mod ffi_binding {
    unsafe extern "C++" {
        include!("cpp/invert.h");

        #[expand(for_each_arc_type)]
        type Fst___SFX__ = crate::ffi::fst::ffi_binding::Fst___SFX__;
        #[expand(for_each_arc_type)]
        type MutableFst___SFX__ = crate::ffi::fst::ffi_binding::MutableFst___SFX__;

        #[expand(for_each_arc_type)]
        fn fst_invert_destructive___SFX__(fst: Pin<&mut MutableFst___SFX__>);
        #[expand(for_each_arc_type)]
        fn fst_invert_non_destructive___SFX__(
            ifst: &Fst___SFX__,
            ofst: Pin<&mut MutableFst___SFX__>,
        );
    }
}

macro_rules! bind_invert_ffi_impl {
    ($arc:ty, $wtype:ty, $sfx:ident) => {
        pastey::paste! {
            impl InvertFfi for $arc {
                fn fst_invert_destructive(fst: Pin<&mut Self::MutableFstCxx>) {
                    ffi_binding::[<fst_invert_destructive_ $sfx>](fst);
                }
                fn fst_invert_non_destructive(ifst: &Self::FstCxx, ofst: Pin<&mut Self::MutableFstCxx>) {
                    ffi_binding::[<fst_invert_non_destructive_ $sfx>](ifst, ofst);
                }
            }
        }
    };
}

crate::for_each_arc_type!(bind_invert_ffi_impl);
