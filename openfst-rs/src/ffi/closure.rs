use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use openfst_rs_macros::template;
use std::pin::Pin;

pub trait ClosureFfi: Arc<StateId = i32, Label = i32> + FstFfi
where
    Self::Weight: WeightFfi,
{
    fn fst_closure(fst: Pin<&mut Self::MutableFstCxx>, closure_type: i32);
}

#[template("closure")]
pub mod ffi_binding {
    unsafe extern "C++" {
        include!("cpp/closure.h");

        #[expand(for_each_arc_type)]
        type MutableFst___SFX__ = crate::ffi::fst::ffi_binding::MutableFst___SFX__;

        #[expand(for_each_arc_type)]
        fn fst_closure___SFX__(fst: Pin<&mut MutableFst___SFX__>, closure_type: i32);
    }
}

macro_rules! bind_closure_ffi_impl {
    ($arc:ty, $wtype:ty, $sfx:ident) => {
        pastey::paste! {
            impl ClosureFfi for $arc {
                fn fst_closure(fst: Pin<&mut Self::MutableFstCxx>, closure_type: i32) {
                    ffi_binding::[<fst_closure_ $sfx>](fst, closure_type);
                }
            }
        }
    };
}

crate::for_each_arc_type!(bind_closure_ffi_impl);
