#pragma once
#include "openfst/lib/sparse-power-weight.h"
#include "openfst/lib/float-weight.h"
#include "openfst/lib/arc.h"

#include "openfst-rs/src/ffi/sparse_power_weight.rs.h"
#include "cpp/weight-convert.h"

#include <vector>

using SparsePowerWeightValueF32 = fst_rust::ffi::SparsePowerWeightValueF32;
using SparsePowerWeightValueF64 = fst_rust::ffi::SparsePowerWeightValueF64;
using SparseTupleElementF32 = fst_rust::ffi::SparseTupleElementF32;
using SparseTupleElementF64 = fst_rust::ffi::SparseTupleElementF64;

namespace fst {
    #ifndef TROPICAL_WEIGHT_64_DEFINED
    #define TROPICAL_WEIGHT_64_DEFINED
    using TropicalWeight64 = TropicalWeightTpl<double>;
    #endif

    #ifndef MINMAX_WEIGHT_64_DEFINED
    #define MINMAX_WEIGHT_64_DEFINED
    using MinMaxWeight64 = MinMaxWeightTpl<double>;
    #endif

    using SparsePowerTropicalWeight = SparsePowerWeight<TropicalWeight>;
    using SparsePowerTropical64Weight = SparsePowerWeight<TropicalWeight64>;
    using SparsePowerTropicalArc = ArcTpl<SparsePowerTropicalWeight>;
    using SparsePowerTropical64Arc = ArcTpl<SparsePowerTropical64Weight>;

    using SparsePowerLogWeight = SparsePowerWeight<LogWeight>;
    using SparsePowerLog64Weight = SparsePowerWeight<Log64Weight>;
    using SparsePowerLogArc = ArcTpl<SparsePowerLogWeight>;
    using SparsePowerLog64Arc = ArcTpl<SparsePowerLog64Weight>;

    using SparsePowerRealWeight = SparsePowerWeight<RealWeight>;
    using SparsePowerReal64Weight = SparsePowerWeight<Real64Weight>;
    using SparsePowerRealArc = ArcTpl<SparsePowerRealWeight>;
    using SparsePowerReal64Arc = ArcTpl<SparsePowerReal64Weight>;

    using SparsePowerMinMaxWeight = SparsePowerWeight<MinMaxWeight>;
    using SparsePowerMinMax64Weight = SparsePowerWeight<MinMaxWeight64>;
    using SparsePowerMinMaxArc = ArcTpl<SparsePowerMinMaxWeight>;
    using SparsePowerMinMax64Arc = ArcTpl<SparsePowerMinMax64Weight>;
}

namespace fst_rust {
namespace ffi {

// Thread-local buffers for zero-allocation C++ -> Rust FFI transfer
extern thread_local std::vector<SparseTupleElementF32> tls_sparse_buffer_f32;
extern thread_local std::vector<SparseTupleElementF64> tls_sparse_buffer_f64;

#define DECLARE_SPARSE_SCALAR_CONVERTER(ARC_TYPE, WTYPE, INNER_WTYPE, ELEM_TYPE, BUF, CAST_T) \
template <> struct WeightConverter<fst::ARC_TYPE, WTYPE> { \
    static fst::ARC_TYPE::Weight ToFst(const WTYPE& w) { \
        auto const* elems = reinterpret_cast<const ELEM_TYPE*>(w.elements_ptr); \
        fst::ARC_TYPE::Weight res; \
        res.SetDefaultValue(fst::INNER_WTYPE(w.default_weight)); \
        for(size_t i=0; i<w.elements_len; ++i) { \
            res.PushBack(elems[i].key, fst::INNER_WTYPE(elems[i].weight)); \
        } \
        return res; \
    } \
    static WTYPE FromFst(fst::ARC_TYPE::Weight w) { \
        BUF.clear(); \
        for (fst::SparseTupleWeightIterator<fst::INNER_WTYPE, int32_t> it(w); !it.Done(); it.Next()) { \
            BUF.push_back({it.Value().first, static_cast<CAST_T>(it.Value().second.Value())}); \
        } \
        WTYPE res; \
        res.default_weight = w.DefaultValue().Value(); \
        res.elements_ptr = BUF.data(); \
        res.elements_len = BUF.size(); \
        return res; \
    } \
};

DECLARE_SPARSE_SCALAR_CONVERTER(SparsePowerTropicalArc, SparsePowerWeightValueF32, TropicalWeight, SparseTupleElementF32, tls_sparse_buffer_f32, float)
DECLARE_SPARSE_SCALAR_CONVERTER(SparsePowerTropical64Arc, SparsePowerWeightValueF64, TropicalWeight64, SparseTupleElementF64, tls_sparse_buffer_f64, double)
DECLARE_SPARSE_SCALAR_CONVERTER(SparsePowerLogArc, SparsePowerWeightValueF32, LogWeight, SparseTupleElementF32, tls_sparse_buffer_f32, float)
DECLARE_SPARSE_SCALAR_CONVERTER(SparsePowerLog64Arc, SparsePowerWeightValueF64, Log64Weight, SparseTupleElementF64, tls_sparse_buffer_f64, double)
DECLARE_SPARSE_SCALAR_CONVERTER(SparsePowerRealArc, SparsePowerWeightValueF32, RealWeight, SparseTupleElementF32, tls_sparse_buffer_f32, float)
DECLARE_SPARSE_SCALAR_CONVERTER(SparsePowerReal64Arc, SparsePowerWeightValueF64, Real64Weight, SparseTupleElementF64, tls_sparse_buffer_f64, double)
DECLARE_SPARSE_SCALAR_CONVERTER(SparsePowerMinMaxArc, SparsePowerWeightValueF32, MinMaxWeight, SparseTupleElementF32, tls_sparse_buffer_f32, float)
DECLARE_SPARSE_SCALAR_CONVERTER(SparsePowerMinMax64Arc, SparsePowerWeightValueF64, MinMaxWeight64, SparseTupleElementF64, tls_sparse_buffer_f64, double)

#undef DECLARE_SPARSE_SCALAR_CONVERTER

} // namespace ffi
} // namespace fst_rust
