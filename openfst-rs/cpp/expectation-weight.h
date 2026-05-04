#pragma once

#include "openfst/lib/expectation-weight.h"
#include "openfst/lib/float-weight.h"
#include "openfst/lib/signed-log-weight.h"
#include "openfst/lib/arc.h"

#include "openfst-rs/src/ffi/expectation_weight.rs.h"
#include "cpp/weight-convert.h"

using ExpectationWeightValueF32 = fst_rust::ffi::ExpectationWeightValueF32;
using ExpectationWeightValueF64 = fst_rust::ffi::ExpectationWeightValueF64;
using ExpectationWeightValuePairF32 = fst_rust::ffi::ExpectationWeightValuePairF32;
using ExpectationWeightValuePairF64 = fst_rust::ffi::ExpectationWeightValuePairF64;

namespace fst {
    #ifndef MINMAX_WEIGHT_64_DEFINED
    #define MINMAX_WEIGHT_64_DEFINED
    using MinMaxWeight64 = MinMaxWeightTpl<double>;
    #endif

    #ifndef TROPICAL_WEIGHT_64_DEFINED
    #define TROPICAL_WEIGHT_64_DEFINED
    using TropicalWeight64 = TropicalWeightTpl<double>;
    #endif

    using ExpectationTropicalWeight = ExpectationWeight<TropicalWeight, TropicalWeight>;
    using ExpectationTropical64Weight = ExpectationWeight<TropicalWeight64, TropicalWeight64>;
    using ExpectationTropicalArc = ArcTpl<ExpectationTropicalWeight>;
    using ExpectationTropical64Arc = ArcTpl<ExpectationTropical64Weight>;

    using ExpectationLogWeight = ExpectationWeight<LogWeight, LogWeight>;
    using ExpectationLog64Weight = ExpectationWeight<Log64Weight, Log64Weight>;
    using ExpectationLogArc = ArcTpl<ExpectationLogWeight>;
    using ExpectationLog64Arc = ArcTpl<ExpectationLog64Weight>;

    using ExpectationRealWeight = ExpectationWeight<RealWeight, RealWeight>;
    using ExpectationReal64Weight = ExpectationWeight<Real64Weight, Real64Weight>;
    using ExpectationRealArc = ArcTpl<ExpectationRealWeight>;
    using ExpectationReal64Arc = ArcTpl<ExpectationReal64Weight>;

    using ExpectationMinMaxWeight = ExpectationWeight<MinMaxWeight, MinMaxWeight>;
    using ExpectationMinMax64Weight = ExpectationWeight<MinMaxWeight64, MinMaxWeight64>;
    using ExpectationMinMaxArc = ArcTpl<ExpectationMinMaxWeight>;
    using ExpectationMinMax64Arc = ArcTpl<ExpectationMinMax64Weight>;

    using ExpectationSignedLogWeight = ExpectationWeight<SignedLogWeight, SignedLogWeight>;
    using ExpectationSignedLog64Weight = ExpectationWeight<SignedLog64Weight, SignedLog64Weight>;
    using ExpectationSignedLogArc = ArcTpl<ExpectationSignedLogWeight>;
    using ExpectationSignedLog64Arc = ArcTpl<ExpectationSignedLog64Weight>;

    #define DECLARE_EXPECTATION_DUMMY_DIVIDE(WEIGHT_TYPE) \
        inline WEIGHT_TYPE Divide(const WEIGHT_TYPE& w1, const WEIGHT_TYPE& w2, DivideType typ = DIVIDE_ANY) { \
            return WEIGHT_TYPE::NoWeight(); \
        }

    DECLARE_EXPECTATION_DUMMY_DIVIDE(ExpectationTropicalWeight)
    DECLARE_EXPECTATION_DUMMY_DIVIDE(ExpectationTropical64Weight)
    DECLARE_EXPECTATION_DUMMY_DIVIDE(ExpectationLogWeight)
    DECLARE_EXPECTATION_DUMMY_DIVIDE(ExpectationLog64Weight)
    DECLARE_EXPECTATION_DUMMY_DIVIDE(ExpectationRealWeight)
    DECLARE_EXPECTATION_DUMMY_DIVIDE(ExpectationReal64Weight)
    DECLARE_EXPECTATION_DUMMY_DIVIDE(ExpectationMinMaxWeight)
    DECLARE_EXPECTATION_DUMMY_DIVIDE(ExpectationMinMax64Weight)
    DECLARE_EXPECTATION_DUMMY_DIVIDE(ExpectationSignedLogWeight)
    DECLARE_EXPECTATION_DUMMY_DIVIDE(ExpectationSignedLog64Weight)

    #undef DECLARE_EXPECTATION_DUMMY_DIVIDE
}

namespace fst_rust {
namespace ffi {

#define DECLARE_EXPECTATION_SCALAR_CONVERTER(ARC_TYPE, WTYPE, INNER_WTYPE) \
template <> struct WeightConverter<fst::ARC_TYPE, WTYPE> { \
    static fst::ARC_TYPE::Weight ToFst(const WTYPE& w) { return fst::ARC_TYPE::Weight(fst::INNER_WTYPE(w.w1), fst::INNER_WTYPE(w.w2)); } \
    static WTYPE FromFst(fst::ARC_TYPE::Weight w) { return {w.Value1().Value(), w.Value2().Value()}; } \
};

DECLARE_EXPECTATION_SCALAR_CONVERTER(ExpectationTropicalArc, ExpectationWeightValueF32, TropicalWeight)
DECLARE_EXPECTATION_SCALAR_CONVERTER(ExpectationTropical64Arc, ExpectationWeightValueF64, TropicalWeight64)
DECLARE_EXPECTATION_SCALAR_CONVERTER(ExpectationLogArc, ExpectationWeightValueF32, LogWeight)
DECLARE_EXPECTATION_SCALAR_CONVERTER(ExpectationLog64Arc, ExpectationWeightValueF64, Log64Weight)
DECLARE_EXPECTATION_SCALAR_CONVERTER(ExpectationRealArc, ExpectationWeightValueF32, RealWeight)
DECLARE_EXPECTATION_SCALAR_CONVERTER(ExpectationReal64Arc, ExpectationWeightValueF64, Real64Weight)
DECLARE_EXPECTATION_SCALAR_CONVERTER(ExpectationMinMaxArc, ExpectationWeightValueF32, MinMaxWeight)
DECLARE_EXPECTATION_SCALAR_CONVERTER(ExpectationMinMax64Arc, ExpectationWeightValueF64, MinMaxWeight64)

template <> struct WeightConverter<fst::ExpectationSignedLogArc, ExpectationWeightValuePairF32> {
    static fst::ExpectationSignedLogArc::Weight ToFst(const ExpectationWeightValuePairF32& w) {
        return fst::ExpectationSignedLogArc::Weight(
            fst::SignedLogWeight(fst::TropicalWeight(w.w1_1), fst::LogWeight(w.w1_2)),
            fst::SignedLogWeight(fst::TropicalWeight(w.w2_1), fst::LogWeight(w.w2_2))
        );
    }
    static ExpectationWeightValuePairF32 FromFst(fst::ExpectationSignedLogArc::Weight w) {
        return {w.Value1().Value1().Value(), w.Value1().Value2().Value(), w.Value2().Value1().Value(), w.Value2().Value2().Value()};
    }
};

template <> struct WeightConverter<fst::ExpectationSignedLog64Arc, ExpectationWeightValuePairF64> {
    static fst::ExpectationSignedLog64Arc::Weight ToFst(const ExpectationWeightValuePairF64& w) {
        return fst::ExpectationSignedLog64Arc::Weight(
            fst::SignedLog64Weight(fst::TropicalWeight(static_cast<float>(w.w1_1)), fst::Log64Weight(w.w1_2)),
            fst::SignedLog64Weight(fst::TropicalWeight(static_cast<float>(w.w2_1)), fst::Log64Weight(w.w2_2))
        );
    }
    static ExpectationWeightValuePairF64 FromFst(fst::ExpectationSignedLog64Arc::Weight w) {
        return {
            static_cast<double>(w.Value1().Value1().Value()), w.Value1().Value2().Value(),
            static_cast<double>(w.Value2().Value1().Value()), w.Value2().Value2().Value()
        };
    }
};

} // namespace ffi
} // namespace fst_rust
