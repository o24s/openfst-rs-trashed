use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use openfst_rs_macros::template;
use std::pin::Pin;

pub trait UnionFfi: Arc<StateId = i32, Label = i32> + FstFfi
where
    Self::Weight: WeightFfi,
{
    fn fst_union(fst1: Pin<&mut Self::MutableFstCxx>, fst2: &Self::FstCxx);
}

#[template("union")]
pub mod ffi_binding {
    unsafe extern "C++" {
        include!("cpp/union.h");

        #[expand(for_each_arc_type)]
        type Fst___SFX__ = crate::ffi::fst::ffi_binding::Fst___SFX__;
        #[expand(for_each_arc_type)]
        type MutableFst___SFX__ = crate::ffi::fst::ffi_binding::MutableFst___SFX__;

        #[expand(for_each_arc_type)]
        fn fst_union___SFX__(fst1: Pin<&mut MutableFst___SFX__>, fst2: &Fst___SFX__);
    }
}

macro_rules! bind_union_ffi_impl {
    ($arc:ty, $wtype:ty, $sfx:ident) => {
        pastey::paste! {
            impl UnionFfi for $arc {
                fn fst_union(fst1: Pin<&mut Self::MutableFstCxx>, fst2: &Self::FstCxx) {
                    ffi_binding::[<fst_union_ $sfx>](fst1, fst2);
                }
            }
        }
    };
}

crate::for_each_arc_type!(bind_union_ffi_impl);
