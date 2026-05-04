use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use openfst_rs_macros::template;
use std::pin::Pin;

pub trait StateSortFfi: Arc<StateId = i32, Label = i32> + FstFfi
where
    Self::Weight: WeightFfi,
{
    fn fst_statesort(fst: Pin<&mut Self::MutableFstCxx>, order: &[i32]);
}

#[template("statesort")]
pub mod ffi_binding {
    unsafe extern "C++" {
        include!("cpp/statesort.h");

        #[expand(for_each_arc_type)]
        type MutableFst___SFX__ = crate::ffi::fst::ffi_binding::MutableFst___SFX__;

        #[expand(for_each_arc_type)]
        fn fst_statesort___SFX__(fst: Pin<&mut MutableFst___SFX__>, order: &[i32]);
    }
}

macro_rules! bind_statesort_ffi_impl {
    ($arc:ty, $wtype:ty, $sfx:ident) => {
        pastey::paste! {
            impl StateSortFfi for $arc {
                fn fst_statesort(fst: Pin<&mut Self::MutableFstCxx>, order: &[i32]) {
                    ffi_binding::[<fst_statesort_ $sfx>](fst, order)
                }
            }
        }
    };
}

crate::for_each_arc_type!(bind_statesort_ffi_impl);
