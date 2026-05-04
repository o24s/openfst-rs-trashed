use crate::arc::{
    self, Arc, Lexicographic64Arc, LexicographicMinMax64Arc, LexicographicMinMaxArc,
    LexicographicMinMaxTropical64Arc, LexicographicMinMaxTropicalArc, LexicographicStdArc,
    Log64Arc, LogArc, MinMaxArc, Real64Arc, RealArc, SignedLog64Arc, SignedLogArc, StdArc,
};
use crate::ffi::expectation_weight::*;
use crate::ffi::fst::FstFfi;
use crate::ffi::lexicographic_weight::*;
use crate::ffi::power_weight::{PowerWeightValueF32_3, PowerWeightValueF64_3};
use crate::ffi::sparse_power_weight::*;
use crate::ffi::weight::{PairWeightValueF32, PairWeightValueF64};
use crate::ffi::{self, WeightFfi};
use cxx::{UniquePtr, memory::UniquePtrTarget};
use openfst_rs_macros::template;
use std::pin::Pin;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapType {
    Plus = 1,
    Times = 2,
    Power = 3,
    InvertWeight = 4,
    RmWeight = 5,
    Quantize = 6,
    Identity = 7,
    InputEpsilon = 8,
    OutputEpsilon = 9,
    SuperFinal = 10,
    ReverseWeight = 11,
}

pub trait ArcMapInplaceFfi: Arc<StateId = i32, Label = i32> + FstFfi
where
    Self::Weight: WeightFfi,
{
    fn fst_arc_map_inplace(
        fst: Pin<&mut Self::MutableFstCxx>,
        map_type: MapType,
        weight: <Self::Weight as WeightFfi>::ValueType,
        power: f64,
        delta: f32,
        superfinal_label: i32,
    ) -> Result<(), cxx::Exception>;

    fn fst_arc_map_into(
        ifst: &Self::FstCxx,
        ofst: Pin<&mut Self::MutableFstCxx>,
        map_type: MapType,
        weight: <Self::Weight as WeightFfi>::ValueType,
        power: f64,
        delta: f32,
        superfinal_label: i32,
    ) -> Result<(), cxx::Exception>;

    fn create_arc_map_fst(
        ifst: &Self::FstCxx,
        map_type: MapType,
        weight: <Self::Weight as WeightFfi>::ValueType,
        power: f64,
        delta: f32,
        superfinal_label: i32,
        gc: bool,
        gc_limit: usize,
    ) -> Result<UniquePtr<Self::FstCxx>, cxx::Exception>;
}

pub trait GallicMapFfi: Arc<StateId = i32, Label = i32> + FstFfi
where
    <Self as arc::Arc>::Weight: ffi::WeightFfi,
{
    type GallicFstCxx: UniquePtrTarget;

    fn fst_map_to_gallic(ifst: &Self::FstCxx) -> UniquePtr<Self::GallicFstCxx>;
    fn fst_map_from_gallic(
        ifst: &Self::GallicFstCxx,
        ofst: Pin<&mut Self::MutableFstCxx>,
        superfinal_label: i32,
    );
    fn fst_gallic_to_new_symbols(ifst: &Self::GallicFstCxx, ofst: Pin<&mut Self::MutableFstCxx>);
}

pub trait ArcMapConvertFfi<ToArc: Arc + FstFfi>: Arc + FstFfi
where
    <Self as arc::Arc>::Weight: ffi::WeightFfi,
    <ToArc as arc::Arc>::Weight: ffi::WeightFfi,
{
    fn fst_arc_map_convert(ifst: &Self::FstCxx, ofst: Pin<&mut ToArc::MutableFstCxx>);
    fn create_arc_map_convert_fst(
        ifst: &Self::FstCxx,
        gc: bool,
        gc_limit: usize,
    ) -> UniquePtr<ToArc::FstCxx>;
}

#[template("arc_map")]
pub mod ffi_binding {
    unsafe extern "C++" {
        include!("cpp/arc-map.h");

        #[expand(for_each_arc_type)]
        type Fst___SFX__ = crate::ffi::fst::ffi_binding::Fst___SFX__;
        #[expand(for_each_arc_type)]
        type MutableFst___SFX__ = crate::ffi::fst::ffi_binding::MutableFst___SFX__;

        #[expand(for_each_arc_type)]
        type OpaqueGallicFst___SFX__;

        #[expand(for_each_arc_type)]
        fn fst_arc_map_inplace___SFX__(
            fst: Pin<&mut MutableFst___SFX__>,
            map_type: i32,
            weight: __WTYPE__,
            power: f64,
            delta: f32,
            sf_label: i32,
        ) -> Result<()>;

        #[expand(for_each_arc_type)]
        fn fst_arc_map_into___SFX__(
            ifst: &Fst___SFX__,
            ofst: Pin<&mut MutableFst___SFX__>,
            map_type: i32,
            weight: __WTYPE__,
            power: f64,
            delta: f32,
            sf_label: i32,
        ) -> Result<()>;

        #[expand(for_each_arc_type)]
        fn create_arc_map_fst___SFX__(
            ifst: &Fst___SFX__,
            map_type: i32,
            weight: __WTYPE__,
            power: f64,
            delta: f32,
            sf_label: i32,
            gc: bool,
            gc_limit: usize,
        ) -> Result<UniquePtr<Fst___SFX__>>;

        #[expand(for_each_arc_type)]
        fn fst_map_to_gallic___SFX__(ifst: &Fst___SFX__) -> UniquePtr<OpaqueGallicFst___SFX__>;

        #[expand(for_each_arc_type)]
        fn fst_map_from_gallic___SFX__(
            ifst: &OpaqueGallicFst___SFX__,
            ofst: Pin<&mut MutableFst___SFX__>,
            sf_label: i32,
        );

        #[expand(for_each_arc_type)]
        fn fst_gallic_to_new_symbols___SFX__(
            ifst: &OpaqueGallicFst___SFX__,
            ofst: Pin<&mut MutableFst___SFX__>,
        );

        #[expand(for_each_convertible_arc_pair)]
        fn fst_arc_map_convert___SFX1___to___SFX2__(
            ifst: &Fst___SFX1__,
            ofst: Pin<&mut MutableFst___SFX2__>,
        );

        #[expand(for_each_convertible_arc_pair)]
        fn create_arc_map_convert_fst___SFX1___to___SFX2__(
            ifst: &Fst___SFX1__,
            gc: bool,
            gc_limit: usize,
        ) -> UniquePtr<Fst___SFX2__>;
    }
}

macro_rules! bind_map_inplace_and_gallic_impl {
    ($arc:ty, $wtype:ty, $sfx:ident) => {
        pastey::paste! {
            impl ArcMapInplaceFfi for $arc {
                fn fst_arc_map_inplace(
                    fst: Pin<&mut Self::MutableFstCxx>, map_type: MapType, weight: $wtype, power: f64, delta: f32, sf_label: i32
                ) -> Result<(), cxx::Exception> {
                    ffi_binding::[<fst_arc_map_inplace_ $sfx>](fst, map_type as i32, weight, power, delta, sf_label)
                }

                fn fst_arc_map_into(
                    ifst: &Self::FstCxx, ofst: Pin<&mut Self::MutableFstCxx>, map_type: MapType, weight: $wtype, power: f64, delta: f32, sf_label: i32
                ) -> Result<(), cxx::Exception> {
                    ffi_binding::[<fst_arc_map_into_ $sfx>](ifst, ofst, map_type as i32, weight, power, delta, sf_label)
                }

                fn create_arc_map_fst(
                    ifst: &Self::FstCxx, map_type: MapType, weight: $wtype, power: f64, delta: f32, sf_label: i32, gc: bool, gc_limit: usize
                ) -> Result<UniquePtr<Self::FstCxx>, cxx::Exception> {
                    ffi_binding::[<create_arc_map_fst_ $sfx>](ifst, map_type as i32, weight, power, delta, sf_label, gc, gc_limit)
                }
            }

            impl GallicMapFfi for $arc {
                type GallicFstCxx = ffi_binding::[<OpaqueGallicFst_ $sfx>];

                fn fst_map_to_gallic(ifst: &Self::FstCxx) -> UniquePtr<Self::GallicFstCxx> {
                    ffi_binding::[<fst_map_to_gallic_ $sfx>](ifst)
                }
                fn fst_map_from_gallic(ifst: &Self::GallicFstCxx, ofst: Pin<&mut Self::MutableFstCxx>, sf_label: i32) {
                    ffi_binding::[<fst_map_from_gallic_ $sfx>](ifst, ofst, sf_label)
                }
                fn fst_gallic_to_new_symbols(ifst: &Self::GallicFstCxx, ofst: Pin<&mut Self::MutableFstCxx>) {
                    ffi_binding::[<fst_gallic_to_new_symbols_ $sfx>](ifst, ofst)
                }
            }
        }
    };
}
crate::for_each_arc_type!(bind_map_inplace_and_gallic_impl);

macro_rules! bind_map_convert_ffi_impl {
    ($from_arc:ty, $to_arc:ty, $from_sfx:ident, $to_sfx:ident) => {
        pastey::paste! {
            impl ArcMapConvertFfi<$to_arc> for $from_arc {
                fn fst_arc_map_convert(
                    ifst: &<$from_arc as crate::ffi::fst::FstFfi>::FstCxx,
                    ofst: Pin<&mut <$to_arc as crate::ffi::fst::FstFfi>::MutableFstCxx>,
                ) {
                    ffi_binding::[<fst_arc_map_convert_ $from_sfx _to_ $to_sfx>](ifst, ofst)
                }

                fn create_arc_map_convert_fst(
                    ifst: &<$from_arc as crate::ffi::fst::FstFfi>::FstCxx,
                    gc: bool,
                    gc_limit: usize,
                ) -> UniquePtr<<$to_arc as crate::ffi::fst::FstFfi>::FstCxx> {
                    ffi_binding::[<create_arc_map_convert_fst_ $from_sfx _to_ $to_sfx>](ifst, gc, gc_limit)
                }
            }
        }
    };
}

for_each_convertible_arc_pair!(bind_map_convert_ffi_impl);
