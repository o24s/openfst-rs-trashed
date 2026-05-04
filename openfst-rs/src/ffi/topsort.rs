use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use openfst_rs_macros::template;
use std::pin::Pin;

pub trait TopSortFfi: Arc<StateId = i32, Label = i32> + FstFfi
where
    Self::Weight: WeightFfi,
{
    fn fst_topsort(fst: Pin<&mut Self::MutableFstCxx>) -> bool;
}

#[template("topsort")]
pub mod ffi_binding {
    unsafe extern "C++" {
        include!("cpp/topsort.h");

        #[expand(for_each_arc_type)]
        type MutableFst___SFX__ = crate::ffi::fst::ffi_binding::MutableFst___SFX__;

        #[expand(for_each_arc_type)]
        fn fst_topsort___SFX__(fst: Pin<&mut MutableFst___SFX__>) -> bool;
    }
}

macro_rules! bind_topsort_ffi_impl {
    ($arc:ty, $wtype:ty, $sfx:ident) => {
        pastey::paste! {
            impl TopSortFfi for $arc {
                fn fst_topsort(fst: Pin<&mut Self::MutableFstCxx>) -> bool {
                    ffi_binding::[<fst_topsort_ $sfx>](fst)
                }
            }
        }
    };
}

crate::for_each_arc_type!(bind_topsort_ffi_impl);
