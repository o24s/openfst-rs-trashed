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

pub trait ReweightFfi: Arc<StateId = i32, Label = i32> + FstFfi
where
    Self::Weight: WeightFfi,
{
    fn fst_reweight(
        fst: Pin<&mut Self::MutableFstCxx>,
        potentials: &[<Self::Weight as WeightFfi>::ValueType],
        reweight_type: i32,
    );
}

#[template("reweight")]
pub mod ffi_binding {
    unsafe extern "C++" {
        include!("cpp/reweight.h");

        #[expand(for_each_arc_type)]
        type MutableFst___SFX__ = crate::ffi::fst::ffi_binding::MutableFst___SFX__;

        #[expand(for_each_arc_type)]
        fn fst_reweight___SFX__(
            fst: Pin<&mut MutableFst___SFX__>,
            potentials: &[__WTYPE__],
            reweight_type: i32,
        );
    }
}

macro_rules! bind_reweight_ffi_impl {
    ($arc:ty, $wtype:ty, $sfx:ident) => {
        pastey::paste! {
            impl ReweightFfi for $arc {
                fn fst_reweight(
                    fst: Pin<&mut Self::MutableFstCxx>,
                    potentials: &[$wtype],
                    reweight_type: i32,
                ) {
                    ffi_binding::[<fst_reweight_ $sfx>](fst, potentials, reweight_type)
                }
            }
        }
    };
}

crate::for_each_arc_type!(bind_reweight_ffi_impl);
