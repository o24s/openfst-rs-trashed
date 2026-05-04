#pragma once
#include "openfst/lib/power-weight.h"
#include "openfst/lib/float-weight.h"
#include "openfst/lib/arc.h"

#include "openfst-rs/src/ffi/power_weight.rs.h"
#include "cpp/weight-convert.h"

using PowerWeightValueF32_3 = fst_rust::ffi::PowerWeightValueF32_3;
using PowerWeightValueF64_3 = fst_rust::ffi::PowerWeightValueF64_3;

namespace fst {
    #ifndef TROPICAL_WEIGHT_64_DEFINED
    #define TROPICAL_WEIGHT_64_DEFINED
    using TropicalWeight64 = TropicalWeightTpl<double>;
    #endif

    #ifndef MINMAX_WEIGHT_64_DEFINED
    #define MINMAX_WEIGHT_64_DEFINED
    using MinMaxWeight64 = MinMaxWeightTpl<double>;
    #endif

    using Power3TropicalWeight = PowerWeight<TropicalWeight, 3>;
    using Power3Tropical64Weight = PowerWeight<TropicalWeight64, 3>;
    using Power3TropicalArc = ArcTpl<Power3TropicalWeight>;
    using Power3Tropical64Arc = ArcTpl<Power3Tropical64Weight>;

    using Power3LogWeight = PowerWeight<LogWeight, 3>;
    using Power3Log64Weight = PowerWeight<Log64Weight, 3>;
    using Power3LogArc = ArcTpl<Power3LogWeight>;
    using Power3Log64Arc = ArcTpl<Power3Log64Weight>;

    using Power3RealWeight = PowerWeight<RealWeight, 3>;
    using Power3Real64Weight = PowerWeight<Real64Weight, 3>;
    using Power3RealArc = ArcTpl<Power3RealWeight>;
    using Power3Real64Arc = ArcTpl<Power3Real64Weight>;

    using Power3MinMaxWeight = PowerWeight<MinMaxWeight, 3>;
    using Power3MinMax64Weight = PowerWeight<MinMaxWeight64, 3>;
    using Power3MinMaxArc = ArcTpl<Power3MinMaxWeight>;
    using Power3MinMax64Arc = ArcTpl<Power3MinMax64Weight>;
}

namespace fst_rust {
namespace ffi {

#define DECLARE_POWER_SCALAR_CONVERTER(ARC_TYPE, WTYPE, INNER_WTYPE, N) \
template <> struct WeightConverter<fst::ARC_TYPE, WTYPE> { \
    static fst::ARC_TYPE::Weight ToFst(const WTYPE& w) { \
        fst::ARC_TYPE::Weight res; \
        for(size_t i=0; i<N; ++i) res.SetValue(i, fst::INNER_WTYPE(w.w[i])); \
        return res; \
    } \
    static WTYPE FromFst(fst::ARC_TYPE::Weight w) { \
        WTYPE res; \
        for(size_t i=0; i<N; ++i) res.w[i] = w.Value(i).Value(); \
        return res; \
    } \
};

DECLARE_POWER_SCALAR_CONVERTER(Power3TropicalArc, PowerWeightValueF32_3, TropicalWeight, 3)
DECLARE_POWER_SCALAR_CONVERTER(Power3Tropical64Arc, PowerWeightValueF64_3, TropicalWeight64, 3)

DECLARE_POWER_SCALAR_CONVERTER(Power3LogArc, PowerWeightValueF32_3, LogWeight, 3)
DECLARE_POWER_SCALAR_CONVERTER(Power3Log64Arc, PowerWeightValueF64_3, Log64Weight, 3)

DECLARE_POWER_SCALAR_CONVERTER(Power3RealArc, PowerWeightValueF32_3, RealWeight, 3)
DECLARE_POWER_SCALAR_CONVERTER(Power3Real64Arc, PowerWeightValueF64_3, Real64Weight, 3)

DECLARE_POWER_SCALAR_CONVERTER(Power3MinMaxArc, PowerWeightValueF32_3, MinMaxWeight, 3)
DECLARE_POWER_SCALAR_CONVERTER(Power3MinMax64Arc, PowerWeightValueF64_3, MinMaxWeight64, 3)

#undef DECLARE_POWER_SCALAR_CONVERTER

} // namespace ffi
} // namespace fst_rust
