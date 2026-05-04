use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use crate::ffi::lexicographic_weight::*;
use openfst_rs_macros::template;
use std::pin::Pin;

pub trait ShortestPathFfi: Arc<StateId = i32, Label = i32> + FstFfi
where
    Self::Weight: WeightFfi,
{
    fn fst_shortest_path(
        ifst: &Self::FstCxx,
        ofst: Pin<&mut Self::MutableFstCxx>,
        nshortest: i32,
        unique: bool,
        first_path: bool,
        weight_threshold: <Self::Weight as WeightFfi>::ValueType,
        state_threshold: i32,
        delta: f32,
    );
}

#[template("shortest_path")]
pub mod ffi_binding {
    unsafe extern "C++" {
        include!("cpp/shortest-path.h");

        #[expand(for_each_path_arc_type)]
        type Fst___SFX__ = crate::ffi::fst::ffi_binding::Fst___SFX__;
        #[expand(for_each_path_arc_type)]
        type MutableFst___SFX__ = crate::ffi::fst::ffi_binding::MutableFst___SFX__;

        #[expand(for_each_path_arc_type)]
        fn fst_shortest_path___SFX__(
            ifst: &Fst___SFX__,
            ofst: Pin<&mut MutableFst___SFX__>,
            nshortest: i32,
            unique: bool,
            first_path: bool,
            weight_threshold: __WTYPE__,
            state_threshold: i32,
            delta: f32,
        );
    }
}

macro_rules! bind_shortest_path_ffi_impl {
    ($arc:ty, $wtype:ty, $sfx:ident) => {
        pastey::paste! {
            impl ShortestPathFfi for $arc {
                fn fst_shortest_path(
                    ifst: &Self::FstCxx,
                    ofst: Pin<&mut Self::MutableFstCxx>,
                    nshortest: i32,
                    unique: bool,
                    first_path: bool,
                    weight_threshold: $wtype,
                    state_threshold: i32,
                    delta: f32,
                ) {
                    ffi_binding::[<fst_shortest_path_ $sfx>](
                        ifst,
                        ofst,
                        nshortest,
                        unique,
                        first_path,
                        weight_threshold,
                        state_threshold,
                        delta
                    )
                }
            }
        }
    };
}

crate::for_each_path_arc_type!(bind_shortest_path_ffi_impl);
