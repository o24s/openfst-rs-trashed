use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use crate::ffi::lexicographic_weight::*;
use openfst_rs_macros::template;
use std::pin::Pin;

pub trait PruneFfi: Arc<StateId = i32, Label = i32> + FstFfi
where
    Self::Weight: WeightFfi,
{
    fn fst_prune(
        fst: Pin<&mut Self::MutableFstCxx>,
        weight_threshold: <Self::Weight as WeightFfi>::ValueType,
        state_threshold: i32,
        delta: f32,
    );

    fn fst_prune_into(
        ifst: &Self::FstCxx,
        ofst: Pin<&mut Self::MutableFstCxx>,
        weight_threshold: <Self::Weight as WeightFfi>::ValueType,
        state_threshold: i32,
        delta: f32,
    );
}

#[template("prune")]
pub mod ffi_binding {
    unsafe extern "C++" {
        include!("cpp/prune.h");

        #[expand(for_each_path_arc_type)]
        type Fst___SFX__ = crate::ffi::fst::ffi_binding::Fst___SFX__;
        #[expand(for_each_path_arc_type)]
        type MutableFst___SFX__ = crate::ffi::fst::ffi_binding::MutableFst___SFX__;

        #[expand(for_each_path_arc_type)]
        fn fst_prune___SFX__(
            fst: Pin<&mut MutableFst___SFX__>,
            weight_threshold: __WTYPE__,
            state_threshold: i32,
            delta: f32,
        );
        #[expand(for_each_path_arc_type)]
        fn fst_prune_into___SFX__(
            ifst: &Fst___SFX__,
            ofst: Pin<&mut MutableFst___SFX__>,
            weight_threshold: __WTYPE__,
            state_threshold: i32,
            delta: f32,
        );
    }
}

macro_rules! bind_prune_ffi_impl {
    ($arc:ty, $wtype:ty, $sfx:ident) => {
        pastey::paste! {
            impl PruneFfi for $arc {
                fn fst_prune(fst: Pin<&mut Self::MutableFstCxx>, weight_threshold: $wtype, state_threshold: i32, delta: f32) {
                    ffi_binding::[<fst_prune_ $sfx>](fst, weight_threshold, state_threshold, delta)
                }
                fn fst_prune_into(ifst: &Self::FstCxx, ofst: Pin<&mut Self::MutableFstCxx>, weight_threshold: $wtype, state_threshold: i32, delta: f32) {
                    ffi_binding::[<fst_prune_into_ $sfx>](ifst, ofst, weight_threshold, state_threshold, delta)
                }
            }
        }
    };
}

crate::for_each_path_arc_type!(bind_prune_ffi_impl);
