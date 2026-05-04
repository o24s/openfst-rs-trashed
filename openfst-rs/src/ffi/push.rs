use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use openfst_rs_macros::template;
use std::pin::Pin;

pub trait PushFfi: Arc<StateId = i32, Label = i32> + FstFfi
where
    Self::Weight: WeightFfi,
{
    fn fst_push(
        fst: Pin<&mut Self::MutableFstCxx>,
        reweight_type: i32,
        delta: f32,
        remove_total_weight: bool,
    );

    fn fst_push_into(
        ifst: &Self::FstCxx,
        ofst: Pin<&mut Self::MutableFstCxx>,
        ptype: u8,
        reweight_type: i32,
        delta: f32,
    );
}

#[template("push")]
pub mod ffi_binding {
    unsafe extern "C++" {
        include!("cpp/push.h");

        #[expand(for_each_arc_type)]
        type Fst___SFX__ = crate::ffi::fst::ffi_binding::Fst___SFX__;
        #[expand(for_each_arc_type)]
        type MutableFst___SFX__ = crate::ffi::fst::ffi_binding::MutableFst___SFX__;

        #[expand(for_each_arc_type)]
        fn fst_push___SFX__(
            fst: Pin<&mut MutableFst___SFX__>,
            reweight_type: i32,
            delta: f32,
            remove_total_weight: bool,
        );
        #[expand(for_each_arc_type)]
        fn fst_push_into___SFX__(
            ifst: &Fst___SFX__,
            ofst: Pin<&mut MutableFst___SFX__>,
            ptype: u8,
            reweight_type: i32,
            delta: f32,
        );
    }
}

macro_rules! bind_push_ffi_impl {
    ($arc:ty, $wtype:ty, $sfx:ident) => {
        pastey::paste! {
            impl PushFfi for $arc {
                fn fst_push(fst: Pin<&mut Self::MutableFstCxx>, reweight_type: i32, delta: f32, remove_total_weight: bool) {
                    ffi_binding::[<fst_push_ $sfx>](fst, reweight_type, delta, remove_total_weight)
                }
                fn fst_push_into(ifst: &Self::FstCxx, ofst: Pin<&mut Self::MutableFstCxx>, ptype: u8, reweight_type: i32, delta: f32) {
                    ffi_binding::[<fst_push_into_ $sfx>](ifst, ofst, ptype, reweight_type, delta)
                }
            }
        }
    };
}

crate::for_each_arc_type!(bind_push_ffi_impl);
