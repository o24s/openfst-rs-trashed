use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use openfst_rs_macros::template;
use std::pin::Pin;

pub trait ConnectFfi: Arc<StateId = i32, Label = i32> + FstFfi
where
    Self::Weight: WeightFfi,
{
    fn fst_connect(fst: Pin<&mut Self::MutableFstCxx>);
}

#[template("connect")]
pub mod ffi_binding {
    unsafe extern "C++" {
        include!("cpp/connect.h");

        #[expand(for_each_arc_type)]
        type MutableFst___SFX__ = crate::ffi::fst::ffi_binding::MutableFst___SFX__;

        #[expand(for_each_arc_type)]
        fn fst_connect___SFX__(fst: Pin<&mut MutableFst___SFX__>);
    }
}

macro_rules! bind_connect_ffi_impl {
    ($arc:ty, $wtype:ty, $sfx:ident) => {
        pastey::paste! {
            impl ConnectFfi for $arc {
                fn fst_connect(fst: Pin<&mut Self::MutableFstCxx>) {
                    ffi_binding::[<fst_connect_ $sfx>](fst)
                }
            }
        }
    };
}

crate::for_each_arc_type!(bind_connect_ffi_impl);
