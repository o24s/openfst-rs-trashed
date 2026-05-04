#pragma once

#include "openfst/lib/lexicographic-weight.h"
#include "openfst/lib/float-weight.h"
#include "openfst/lib/arc.h"

#include "openfst-rs/src/ffi/lexicographic_weight.rs.h"
#include "cpp/weight-convert.h"

using LexicographicWeightValueF32 = fst_rust::ffi::LexicographicWeightValueF32;
using LexicographicWeightValueF64 = fst_rust::ffi::LexicographicWeightValueF64;

namespace fst {
    using TropicalWeight64 = TropicalWeightTpl<double>;
    using MinMaxWeight64 = MinMaxWeightTpl<double>;

    using LexicographicTropicalWeight = LexicographicWeight<TropicalWeight, TropicalWeight>;
    using LexicographicTropical64Weight = LexicographicWeight<TropicalWeight64, TropicalWeight64>;

    using LexicographicStdArc = ArcTpl<LexicographicTropicalWeight>;
    using Lexicographic64Arc = ArcTpl<LexicographicTropical64Weight>;

    using LexicographicMinMaxTropicalWeight = LexicographicWeight<MinMaxWeight, TropicalWeight>;
    using LexicographicMinMaxTropicalArc = ArcTpl<LexicographicMinMaxTropicalWeight>;
    using LexicographicMinMaxTropical64Weight = LexicographicWeight<MinMaxWeight64, TropicalWeight64>;
    using LexicographicMinMaxTropical64Arc = ArcTpl<LexicographicMinMaxTropical64Weight>;

    using LexicographicMinMaxWeight = LexicographicWeight<MinMaxWeight, MinMaxWeight>;
    using LexicographicMinMaxArc = ArcTpl<LexicographicMinMaxWeight>;
    using LexicographicMinMax64Weight = LexicographicWeight<MinMaxWeight64, MinMaxWeight64>;
    using LexicographicMinMax64Arc = ArcTpl<LexicographicMinMax64Weight>;
}

namespace fst_rust {
namespace ffi {

template <>
struct WeightConverter<fst::LexicographicStdArc, fst_rust::ffi::LexicographicWeightValueF32> {
    static fst::LexicographicStdArc::Weight ToFst(const fst_rust::ffi::LexicographicWeightValueF32& w) {
        return fst::LexicographicStdArc::Weight(fst::TropicalWeight(w.w1), fst::TropicalWeight(w.w2));
    }
    static fst_rust::ffi::LexicographicWeightValueF32 FromFst(fst::LexicographicStdArc::Weight w) {
        fst_rust::ffi::LexicographicWeightValueF32 res;
        res.w1 = w.Value1().Value();
        res.w2 = w.Value2().Value();
        return res;
    }
};

template <>
struct WeightConverter<fst::Lexicographic64Arc, fst_rust::ffi::LexicographicWeightValueF64> {
    static fst::Lexicographic64Arc::Weight ToFst(const fst_rust::ffi::LexicographicWeightValueF64& w) {
        return fst::Lexicographic64Arc::Weight(fst::TropicalWeight64(w.w1), fst::TropicalWeight64(w.w2));
    }
    static fst_rust::ffi::LexicographicWeightValueF64 FromFst(fst::Lexicographic64Arc::Weight w) {
        fst_rust::ffi::LexicographicWeightValueF64 res;
        res.w1 = w.Value1().Value();
        res.w2 = w.Value2().Value();
        return res;
    }
};

template <>
struct WeightConverter<fst::LexicographicMinMaxTropicalArc, fst_rust::ffi::LexicographicWeightValueF32> {
    static fst::LexicographicMinMaxTropicalArc::Weight ToFst(const fst_rust::ffi::LexicographicWeightValueF32& w) {
        return fst::LexicographicMinMaxTropicalArc::Weight(fst::MinMaxWeight(w.w1), fst::TropicalWeight(w.w2));
    }
    static fst_rust::ffi::LexicographicWeightValueF32 FromFst(fst::LexicographicMinMaxTropicalArc::Weight w) {
        fst_rust::ffi::LexicographicWeightValueF32 res;
        res.w1 = w.Value1().Value();
        res.w2 = w.Value2().Value();
        return res;
    }
};

template <>
struct WeightConverter<fst::LexicographicMinMaxArc, fst_rust::ffi::LexicographicWeightValueF32> {
    static fst::LexicographicMinMaxArc::Weight ToFst(const fst_rust::ffi::LexicographicWeightValueF32& w) {
        return fst::LexicographicMinMaxArc::Weight(fst::MinMaxWeight(w.w1), fst::MinMaxWeight(w.w2));
    }
    static fst_rust::ffi::LexicographicWeightValueF32 FromFst(fst::LexicographicMinMaxArc::Weight w) {
        fst_rust::ffi::LexicographicWeightValueF32 res;
        res.w1 = w.Value1().Value();
        res.w2 = w.Value2().Value();
        return res;
    }
};

template <>
struct WeightConverter<fst::LexicographicMinMaxTropical64Arc, fst_rust::ffi::LexicographicWeightValueF64> {
    static fst::LexicographicMinMaxTropical64Arc::Weight ToFst(const fst_rust::ffi::LexicographicWeightValueF64& w) {
        return fst::LexicographicMinMaxTropical64Arc::Weight(fst::MinMaxWeight64(w.w1), fst::TropicalWeight64(w.w2));
    }
    static fst_rust::ffi::LexicographicWeightValueF64 FromFst(fst::LexicographicMinMaxTropical64Arc::Weight w) {
        fst_rust::ffi::LexicographicWeightValueF64 res;
        res.w1 = w.Value1().Value();
        res.w2 = w.Value2().Value();
        return res;
    }
};

template <>
struct WeightConverter<fst::LexicographicMinMax64Arc, fst_rust::ffi::LexicographicWeightValueF64> {
    static fst::LexicographicMinMax64Arc::Weight ToFst(const fst_rust::ffi::LexicographicWeightValueF64& w) {
        return fst::LexicographicMinMax64Arc::Weight(fst::MinMaxWeight64(w.w1), fst::MinMaxWeight64(w.w2));
    }
    static fst_rust::ffi::LexicographicWeightValueF64 FromFst(fst::LexicographicMinMax64Arc::Weight w) {
        fst_rust::ffi::LexicographicWeightValueF64 res;
        res.w1 = w.Value1().Value();
        res.w2 = w.Value2().Value();
        return res;
    }
};

} // namespace ffi
} // namespace fst_rust
