use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::expectation_weight::*;
use crate::ffi::fst::FstFfi;
use crate::ffi::lexicographic_weight::*;
use crate::ffi::power_weight::{PowerWeightValueF32_3, PowerWeightValueF64_3};
use crate::ffi::sparse_power_weight::*;
use crate::ffi::weight::{PairWeightValueF32, PairWeightValueF64};
use openfst_rs_macros::template;
use std::pin::Pin;

pub trait DisambiguateFfi: Arc<StateId = i32, Label = i32> + FstFfi
where
    Self::Weight: WeightFfi,
{
    fn fst_disambiguate(
        ifst: &Self::FstCxx,
        ofst: Pin<&mut Self::MutableFstCxx>,
        delta: f32,
        weight_threshold: <Self::Weight as WeightFfi>::ValueType,
        state_threshold: i32,
        subsequential_label: i32,
    );
}

#[template("disambiguate")]
pub mod ffi_binding {
    unsafe extern "C++" {
        include!("cpp/disambiguate.h");

        #[expand(for_each_arc_type)]
        type Fst___SFX__ = crate::ffi::fst::ffi_binding::Fst___SFX__;
        #[expand(for_each_arc_type)]
        type MutableFst___SFX__ = crate::ffi::fst::ffi_binding::MutableFst___SFX__;

        #[expand(for_each_arc_type)]
        fn fst_disambiguate___SFX__(
            ifst: &Fst___SFX__,
            ofst: Pin<&mut MutableFst___SFX__>,
            delta: f32,
            weight_threshold: __WTYPE__,
            state_threshold: i32,
            subsequential_label: i32,
        );
    }
}

macro_rules! bind_disambiguate_ffi_impl {
    ($arc:ty, $wtype:ty, $sfx:ident) => {
        pastey::paste! {
            impl DisambiguateFfi for $arc {
                fn fst_disambiguate(
                    ifst: &Self::FstCxx,
                    ofst: Pin<&mut Self::MutableFstCxx>,
                    delta: f32,
                    weight_threshold: $wtype,
                    state_threshold: i32,
                    subsequential_label: i32,
                ) {
                    ffi_binding::[<fst_disambiguate_ $sfx>](
                        ifst,
                        ofst,
                        delta,
                        weight_threshold,
                        state_threshold,
                        subsequential_label,
                    );
                }
            }
        }
    };
}

crate::for_each_arc_type!(bind_disambiguate_ffi_impl);
