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

pub trait RmEpsilonFfi: Arc<StateId = i32, Label = i32> + FstFfi
where
    Self::Weight: WeightFfi,
{
    fn fst_rmepsilon(
        fst: Pin<&mut Self::MutableFstCxx>,
        connect: bool,
        weight_threshold: <Self::Weight as WeightFfi>::ValueType,
        state_threshold: i32,
        delta: f32,
    );
}

#[template("rmepsilon")]
pub mod ffi_binding {
    unsafe extern "C++" {
        include!("cpp/rmepsilon.h");

        #[expand(for_each_arc_type)]
        type MutableFst___SFX__ = crate::ffi::fst::ffi_binding::MutableFst___SFX__;

        #[expand(for_each_arc_type)]
        fn fst_rmepsilon___SFX__(
            fst: Pin<&mut MutableFst___SFX__>,
            connect: bool,
            weight_threshold: __WTYPE__,
            state_threshold: i32,
            delta: f32,
        );
    }
}

macro_rules! bind_rmepsilon_ffi_impl {
    ($arc:ty, $wtype:ty, $sfx:ident) => {
        pastey::paste! {
            impl RmEpsilonFfi for $arc {
                fn fst_rmepsilon(
                    fst: Pin<&mut Self::MutableFstCxx>,
                    connect: bool,
                    weight_threshold: $wtype,
                    state_threshold: i32,
                    delta: f32,
                ) {
                    ffi_binding::[<fst_rmepsilon_ $sfx>](
                        fst,
                        connect,
                        weight_threshold,
                        state_threshold,
                        delta
                    )
                }
            }
        }
    };
}

crate::for_each_arc_type!(bind_rmepsilon_ffi_impl);
