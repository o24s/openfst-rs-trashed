use crate::arc::{self, Arc};
use crate::ffi;
use crate::ffi::fst::FstFfi;
use openfst_rs_macros::template;
use std::pin::Pin;

pub trait PowerMapFfi<ToArc: Arc + FstFfi>: Arc + FstFfi
where
    <Self as arc::Arc>::Weight: ffi::WeightFfi,
    <ToArc as arc::Arc>::Weight: ffi::WeightFfi,
{
    fn fst_map_to_power(ifst: &Self::FstCxx, ofst: Pin<&mut ToArc::MutableFstCxx>, index: usize);
}

pub trait PowerMapFromFfi<ToArc: Arc + FstFfi>: Arc + FstFfi
where
    <Self as arc::Arc>::Weight: ffi::WeightFfi,
    <ToArc as arc::Arc>::Weight: ffi::WeightFfi,
{
    fn fst_map_from_power(ifst: &Self::FstCxx, ofst: Pin<&mut ToArc::MutableFstCxx>, index: usize);
}

pub trait PowerProjectFfi: Arc + FstFfi
where
    <Self as arc::Arc>::Weight: ffi::WeightFfi,
{
    fn fst_project_power(fst: Pin<&mut Self::MutableFstCxx>, from_index: usize, to_index: usize);
}

#[template("power_map")]
pub mod ffi_binding {
    unsafe extern "C++" {
        include!("cpp/power-map.h");

        type Fst_std = crate::ffi::fst::ffi_binding::Fst_std;
        type MutableFst_std = crate::ffi::fst::ffi_binding::MutableFst_std;

        type Fst_log = crate::ffi::fst::ffi_binding::Fst_log;
        type MutableFst_log = crate::ffi::fst::ffi_binding::MutableFst_log;
        type Fst_log64 = crate::ffi::fst::ffi_binding::Fst_log64;
        type MutableFst_log64 = crate::ffi::fst::ffi_binding::MutableFst_log64;

        type Fst_real = crate::ffi::fst::ffi_binding::Fst_real;
        type MutableFst_real = crate::ffi::fst::ffi_binding::MutableFst_real;
        type Fst_real64 = crate::ffi::fst::ffi_binding::Fst_real64;
        type MutableFst_real64 = crate::ffi::fst::ffi_binding::MutableFst_real64;

        type Fst_minmax = crate::ffi::fst::ffi_binding::Fst_minmax;
        type MutableFst_minmax = crate::ffi::fst::ffi_binding::MutableFst_minmax;

        #[expand(for_each_power_arc)]
        type Fst___SFX__ = crate::ffi::fst::ffi_binding::Fst___SFX__;
        #[expand(for_each_power_arc)]
        type MutableFst___SFX__ = crate::ffi::fst::ffi_binding::MutableFst___SFX__;

        #[expand(for_each_power_arc_pair)]
        fn fst_map_to_power___SFX1___to___SFX2__(
            ifst: &Fst___SFX1__,
            ofst: Pin<&mut MutableFst___SFX2__>,
            index: usize,
        );
        #[expand(for_each_power_arc_pair)]
        fn fst_map_from_power___SFX2___to___SFX1__(
            ifst: &Fst___SFX2__,
            ofst: Pin<&mut MutableFst___SFX1__>,
            index: usize,
        );

        #[expand(for_each_power_arc)]
        fn fst_project_power___SFX__(
            fst: Pin<&mut MutableFst___SFX__>,
            from_index: usize,
            to_index: usize,
        );
    }
}

macro_rules! bind_power_map_ffi_impl {
    ($arc_in:ty, $arc_out:ty, $sfx_in:ident, $sfx_out:ident) => {
        pastey::paste! {
            impl PowerMapFfi<$arc_out> for $arc_in {
                fn fst_map_to_power(ifst: &Self::FstCxx, ofst: Pin<&mut <$arc_out as FstFfi>::MutableFstCxx>, index: usize) {
                    ffi_binding::[<fst_map_to_power_ $sfx_in _to_ $sfx_out>](ifst, ofst, index)
                }
            }
            impl PowerMapFromFfi<$arc_in> for $arc_out {
                fn fst_map_from_power(ifst: &Self::FstCxx, ofst: Pin<&mut <$arc_in as FstFfi>::MutableFstCxx>, index: usize) {
                    ffi_binding::[<fst_map_from_power_ $sfx_out _to_ $sfx_in>](ifst, ofst, index)
                }
            }
        }
    }
}
crate::for_each_power_arc_pair!(bind_power_map_ffi_impl);

macro_rules! bind_power_project_ffi_impl {
    ($arc:ty, $sfx:ident) => {
        pastey::paste! {
            impl PowerProjectFfi for $arc {
                fn fst_project_power(fst: Pin<&mut Self::MutableFstCxx>, from_index: usize, to_index: usize) {
                    ffi_binding::[<fst_project_power_ $sfx>](fst, from_index, to_index)
                }
            }
        }
    }
}
crate::for_each_power_arc!(bind_power_project_ffi_impl);
