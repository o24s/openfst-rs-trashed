use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::expectation_weight::*;
use crate::ffi::fst::FstFfi;
use crate::ffi::lexicographic_weight::*;
use crate::ffi::power_weight::{PowerWeightValueF32_3, PowerWeightValueF64_3};
use crate::ffi::sparse_power_weight::*;
use crate::ffi::weight::{PairWeightValueF32, PairWeightValueF64};
use openfst_rs_macros::template;

pub trait ShortestDistanceFfi: Arc<StateId = i32, Label = i32> + FstFfi
where
    Self::Weight: WeightFfi,
{
    fn fst_shortest_distance(
        fst: &Self::FstCxx,
        delta: f32,
    ) -> Result<<Self::Weight as WeightFfi>::ValueType, cxx::Exception>;

    fn fst_shortest_distance_vec(
        fst: &Self::FstCxx,
        distance_out: &mut Vec<<Self::Weight as WeightFfi>::ValueType>,
        reverse: bool,
        delta: f32,
    ) -> Result<(), cxx::Exception>;
}

#[template("shortest_distance")]
pub mod ffi_binding {
    unsafe extern "C++" {
        include!("cpp/shortest-distance.h");

        #[expand(for_each_arc_type)]
        type Fst___SFX__ = crate::ffi::fst::ffi_binding::Fst___SFX__;

        #[expand(for_each_arc_type)]
        fn fst_shortest_distance___SFX__(fst: &Fst___SFX__, delta: f32) -> Result<__WTYPE__>;
        #[expand(for_each_arc_type)]
        fn fst_shortest_distance_vec___SFX__(
            fst: &Fst___SFX__,
            distance_out: &mut Vec<__WTYPE__>,
            reverse: bool,
            delta: f32,
        ) -> Result<()>;
    }
}

macro_rules! bind_shortest_distance_ffi_impl {
    ($arc:ty, $wtype:ty, $sfx:ident) => {
        pastey::paste! {
            impl ShortestDistanceFfi for $arc {
                fn fst_shortest_distance(fst: &Self::FstCxx, delta: f32) -> Result<$wtype, cxx::Exception> {
                    ffi_binding::[<fst_shortest_distance_ $sfx>](fst, delta)
                }

                fn fst_shortest_distance_vec(
                    fst: &Self::FstCxx,
                    distance_out: &mut Vec<$wtype>,
                    reverse: bool,
                    delta: f32,
                ) -> Result<(), cxx::Exception> {
                    ffi_binding::[<fst_shortest_distance_vec_ $sfx>](fst, distance_out, reverse, delta)
                }
            }
        }
    };
}

crate::for_each_arc_type!(bind_shortest_distance_ffi_impl);
