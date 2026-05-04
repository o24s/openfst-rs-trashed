use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use openfst_rs_macros::template;
use std::pin::Pin;

pub trait ReverseFfi: Arc<StateId = i32, Label = i32> + FstFfi
where
    Self::Weight: WeightFfi,
{
    fn fst_reverse(
        ifst: &Self::FstCxx,
        ofst: Pin<&mut Self::MutableFstCxx>,
        require_superinitial: bool,
    );
}

#[template("reverse")]
pub mod ffi_binding {
    unsafe extern "C++" {
        include!("cpp/reverse.h");

        #[expand(for_each_arc_type)]
        type Fst___SFX__ = crate::ffi::fst::ffi_binding::Fst___SFX__;
        #[expand(for_each_arc_type)]
        type MutableFst___SFX__ = crate::ffi::fst::ffi_binding::MutableFst___SFX__;

        #[expand(for_each_arc_type)]
        fn fst_reverse___SFX__(ifst: &Fst___SFX__, ofst: Pin<&mut MutableFst___SFX__>, req: bool);
    }
}

macro_rules! bind_reverse_ffi_impl {
    ($arc:ty, $wtype:ty, $sfx:ident) => {
        pastey::paste! {
            impl ReverseFfi for $arc {
                fn fst_reverse(ifst: &Self::FstCxx, ofst: Pin<&mut Self::MutableFstCxx>, req: bool) {
                    ffi_binding::[<fst_reverse_ $sfx>](ifst, ofst, req)
                }
            }
        }
    };
}

crate::for_each_arc_type!(bind_reverse_ffi_impl);
