use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use openfst_rs_macros::template;
use std::pin::Pin;

pub trait ConcatFfi: Arc<StateId = i32, Label = i32> + FstFfi
where
    Self::Weight: WeightFfi,
{
    fn fst_concat(fst1: Pin<&mut Self::MutableFstCxx>, fst2: &Self::FstCxx);
}

#[template("concat")]
pub mod ffi_binding {
    unsafe extern "C++" {
        include!("cpp/concat.h");

        #[expand(for_each_arc_type)]
        type Fst___SFX__ = crate::ffi::fst::ffi_binding::Fst___SFX__;
        #[expand(for_each_arc_type)]
        type MutableFst___SFX__ = crate::ffi::fst::ffi_binding::MutableFst___SFX__;

        #[expand(for_each_arc_type)]
        fn fst_concat___SFX__(fst1: Pin<&mut MutableFst___SFX__>, fst2: &Fst___SFX__);
    }
}

macro_rules! bind_concat_ffi_impl {
    ($arc:ty, $wtype:ty, $sfx:ident) => {
        pastey::paste! {
            impl ConcatFfi for $arc {
                fn fst_concat(fst1: Pin<&mut Self::MutableFstCxx>, fst2: &Self::FstCxx) {
                    ffi_binding::[<fst_concat_ $sfx>](fst1, fst2);
                }
            }
        }
    };
}

crate::for_each_arc_type!(bind_concat_ffi_impl);
