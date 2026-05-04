#![allow(non_camel_case_types)]

include!(concat!(env!("OUT_DIR"), "/generated_macros.rs"));

pub(crate) mod arc_map;
pub(crate) mod arcsort;
pub(crate) mod closure;
pub(crate) mod compact_fst;
pub(crate) mod compose;
pub(crate) mod concat;
pub(crate) mod connect;
pub(crate) mod const_fst;
pub(crate) mod determinize;
pub(crate) mod difference;
pub(crate) mod disambiguate;
pub(crate) mod encode;
pub(crate) mod epsnormalize;
pub(crate) mod equivalent;
pub(crate) mod expectation_weight;
pub(crate) mod fst;
pub(crate) mod intersect;
pub(crate) mod invert;
pub(crate) mod lexicographic_weight;
pub(crate) mod minimize;
pub(crate) mod power_map;
pub(crate) mod power_weight;
pub(crate) mod project;
pub(crate) mod prune;
pub(crate) mod push;
pub(crate) mod randgen;
pub(crate) mod relabel;
pub(crate) mod replace;
pub(crate) mod reverse;
pub(crate) mod reweight;
pub(crate) mod rmepsilon;
pub(crate) mod rmfinalepsilon;
pub(crate) mod shortest_distance;
pub(crate) mod shortest_path;
pub(crate) mod sparse_power_weight;
pub(crate) mod statesort;
pub(crate) mod symbol_table;
pub(crate) mod synchronize;
pub(crate) mod topsort;
pub(crate) mod union;
pub(crate) mod vector_fst;
pub(crate) mod verify;
pub(crate) mod weight;

#[allow(unused)]
use crate::arc::{Lexicographic64Arc, LexicographicStdArc};
use crate::weight::Weight;

pub trait WeightFfi: Weight {
    type ValueType: Copy + PartialEq;

    fn from_ffi(val: Self::ValueType) -> Self;
    fn as_ffi(&self) -> Self::ValueType;
}
#[macro_export]
macro_rules! for_each_arc_with_semiring {
    ($m:path $(, $args:tt)*) => {
        $m!($crate::arc::StdArc, f32, std, Tropical $(, $args)*);
        $m!($crate::arc::LogArc, f32, log, Log $(, $args)*);
        $m!($crate::arc::Log64Arc, f64, log64, Log $(, $args)*);
        $m!($crate::arc::RealArc, f32, real, Real $(, $args)*);
        $m!($crate::arc::Real64Arc, f64, real64, Real $(, $args)*);
        $m!($crate::arc::MinMaxArc, f32, minmax, MinMax $(, $args)*);
        $m!($crate::arc::SignedLogArc, PairWeightValueF32, signed_log, SignedLog $(, $args)*);
        $m!($crate::arc::SignedLog64Arc, PairWeightValueF64, signed_log64, SignedLog $(, $args)*);
        $m!($crate::arc::LexicographicStdArc, LexicographicWeightValueF32, lexicographic, Lexicographic $(, $args)*);
        $m!($crate::arc::Lexicographic64Arc, LexicographicWeightValueF64, lexicographic64, Lexicographic $(, $args)*);
        $m!($crate::arc::LexicographicMinMaxTropicalArc, LexicographicWeightValueF32, lexicographic_minmax_tropical, Lexicographic $(, $args)*);
        $m!($crate::arc::LexicographicMinMaxTropical64Arc, LexicographicWeightValueF64, lexicographic_minmax_tropical64, Lexicographic $(, $args)*);
        $m!($crate::arc::LexicographicMinMaxArc, LexicographicWeightValueF32, lexicographic_minmax, Lexicographic $(, $args)*);
        $m!($crate::arc::LexicographicMinMax64Arc, LexicographicWeightValueF64, lexicographic_minmax64, Lexicographic $(, $args)*);
        $m!($crate::arc::ExpectationTropicalArc, ExpectationWeightValueF32, expectation_tropical, Expectation $(, $args)*);
        $m!($crate::arc::ExpectationTropical64Arc, ExpectationWeightValueF64, expectation_tropical64, Expectation $(, $args)*);
        $m!($crate::arc::ExpectationLogArc, ExpectationWeightValueF32, expectation_log, Expectation $(, $args)*);
        $m!($crate::arc::ExpectationLog64Arc, ExpectationWeightValueF64, expectation_log64, Expectation $(, $args)*);
        $m!($crate::arc::ExpectationRealArc, ExpectationWeightValueF32, expectation_real, Expectation $(, $args)*);
        $m!($crate::arc::ExpectationReal64Arc, ExpectationWeightValueF64, expectation_real64, Expectation $(, $args)*);
        $m!($crate::arc::ExpectationMinMaxArc, ExpectationWeightValueF32, expectation_minmax, Expectation $(, $args)*);
        $m!($crate::arc::ExpectationMinMax64Arc, ExpectationWeightValueF64, expectation_minmax64, Expectation $(, $args)*);
        $m!($crate::arc::ExpectationSignedLogArc, ExpectationWeightValuePairF32, expectation_signedlog, Expectation $(, $args)*);
        $m!($crate::arc::ExpectationSignedLog64Arc, ExpectationWeightValuePairF64, expectation_signedlog64, Expectation $(, $args)*);
        $m!($crate::arc::Power3TropicalArc, PowerWeightValueF32_3, power3_tropical, Power $(, $args)*);
        $m!($crate::arc::Power3Tropical64Arc, PowerWeightValueF64_3, power3_tropical64, Power $(, $args)*);
        $m!($crate::arc::Power3LogArc, PowerWeightValueF32_3, power3_log, Power $(, $args)*);
        $m!($crate::arc::Power3Log64Arc, PowerWeightValueF64_3, power3_log64, Power $(, $args)*);
        $m!($crate::arc::Power3RealArc, PowerWeightValueF32_3, power3_real, Power $(, $args)*);
        $m!($crate::arc::Power3Real64Arc, PowerWeightValueF64_3, power3_real64, Power $(, $args)*);
        $m!($crate::arc::Power3MinMaxArc, PowerWeightValueF32_3, power3_minmax, Power $(, $args)*);
        $m!($crate::arc::Power3MinMax64Arc, PowerWeightValueF64_3, power3_minmax64, Power $(, $args)*);
        $m!($crate::arc::SparsePowerTropicalArc, SparsePowerWeightValueF32, sparse_power_tropical, SparsePower $(, $args)*);
        $m!($crate::arc::SparsePowerTropical64Arc, SparsePowerWeightValueF64, sparse_power_tropical64, SparsePower $(, $args)*);
        $m!($crate::arc::SparsePowerLogArc, SparsePowerWeightValueF32, sparse_power_log, SparsePower $(, $args)*);
        $m!($crate::arc::SparsePowerLog64Arc, SparsePowerWeightValueF64, sparse_power_log64, SparsePower $(, $args)*);
        $m!($crate::arc::SparsePowerRealArc, SparsePowerWeightValueF32, sparse_power_real, SparsePower $(, $args)*);
        $m!($crate::arc::SparsePowerReal64Arc, SparsePowerWeightValueF64, sparse_power_real64, SparsePower $(, $args)*);
        $m!($crate::arc::SparsePowerMinMaxArc, SparsePowerWeightValueF32, sparse_power_minmax, SparsePower $(, $args)*);
        $m!($crate::arc::SparsePowerMinMax64Arc, SparsePowerWeightValueF64, sparse_power_minmax64, SparsePower $(, $args)*);
    };
}

#[macro_export]
macro_rules! strip_semiring_for_arc_type {
    ($arc:ty, $wtype:ty, $sfx:ident, $semiring:ident, $m:path) => {
        $m!($arc, $wtype, $sfx);
    };
}

#[macro_export]
macro_rules! for_each_arc_type {
    ($m:path) => {
        $crate::for_each_arc_with_semiring!($crate::strip_semiring_for_arc_type, $m);
    };
}

#[macro_export]
macro_rules! for_each_path_arc_type {
    ($macro_name:path $(, $args:tt)*) => {
        $macro_name!($crate::arc::StdArc, f32, std $(, $args)*);
        $macro_name!($crate::arc::MinMaxArc, f32, minmax $(, $args)*);

        $macro_name!($crate::arc::LexicographicStdArc, LexicographicWeightValueF32, lexicographic $(, $args)*);
        $macro_name!($crate::arc::Lexicographic64Arc, LexicographicWeightValueF64, lexicographic64 $(, $args)*);
        $macro_name!($crate::arc::LexicographicMinMaxTropicalArc, LexicographicWeightValueF32, lexicographic_minmax_tropical $(, $args)*);
        $macro_name!($crate::arc::LexicographicMinMaxTropical64Arc, LexicographicWeightValueF64, lexicographic_minmax_tropical64 $(, $args)*);
        $macro_name!($crate::arc::LexicographicMinMaxArc, LexicographicWeightValueF32, lexicographic_minmax $(, $args)*);
        $macro_name!($crate::arc::LexicographicMinMax64Arc, LexicographicWeightValueF64, lexicographic_minmax64 $(, $args)*);
    };
}

#[macro_export]
macro_rules! for_each_power_arc_pair {
    ($m:path) => {
        $m!(
            $crate::arc::StdArc,
            $crate::arc::Power3TropicalArc,
            std,
            power3_tropical
        );
        $m!(
            $crate::arc::LogArc,
            $crate::arc::Power3LogArc,
            log,
            power3_log
        );
        $m!(
            $crate::arc::Log64Arc,
            $crate::arc::Power3Log64Arc,
            log64,
            power3_log64
        );
        $m!(
            $crate::arc::RealArc,
            $crate::arc::Power3RealArc,
            real,
            power3_real
        );
        $m!(
            $crate::arc::Real64Arc,
            $crate::arc::Power3Real64Arc,
            real64,
            power3_real64
        );
        $m!(
            $crate::arc::MinMaxArc,
            $crate::arc::Power3MinMaxArc,
            minmax,
            power3_minmax
        );
    };
}

#[macro_export]
macro_rules! for_each_power_arc {
    ($m:path) => {
        $m!($crate::arc::Power3TropicalArc, power3_tropical);
        $m!($crate::arc::Power3LogArc, power3_log);
        $m!($crate::arc::Power3Log64Arc, power3_log64);
        $m!($crate::arc::Power3RealArc, power3_real);
        $m!($crate::arc::Power3Real64Arc, power3_real64);
        $m!($crate::arc::Power3MinMaxArc, power3_minmax);
    };
}

#[macro_export]
macro_rules! convertible_semiring_rules {
    ($m:path) => {
        $m!(Tropical, Tropical);
        $m!(Tropical, Log);
        $m!(Tropical, SignedLog);

        $m!(Log, Tropical);
        $m!(Log, Log);
        $m!(Log, Real);
        $m!(Log, SignedLog);

        $m!(Real, Log);
        $m!(Real, Real);
        $m!(Real, SignedLog);

        $m!(SignedLog, Tropical);
        $m!(SignedLog, Log);
        $m!(SignedLog, Real);
        $m!(SignedLog, SignedLog);

        $m!(MinMax, MinMax);
        $m!(Lexicographic, Lexicographic);
    };
}
