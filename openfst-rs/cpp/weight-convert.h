#pragma once

#include "openfst/lib/weight.h"
#include "openfst/lib/float-weight.h"
#include "openfst/lib/signed-log-weight.h"

#include "openfst-rs/src/ffi/weight.rs.h"

namespace fst_rust {
namespace ffi {

template <class Arc, class WTYPE>
struct WeightConverter;

template <class Arc>
struct WeightConverter<Arc, float> {
    static typename Arc::Weight ToFst(float w) { return typename Arc::Weight(w); }
    static float FromFst(typename Arc::Weight w) { return w.Value(); }
};

template <class Arc>
struct WeightConverter<Arc, double> {
    static typename Arc::Weight ToFst(double w) { return typename Arc::Weight(w); }
    static double FromFst(typename Arc::Weight w) { return w.Value(); }
};

template <>
struct WeightConverter<fst::SignedLogArc, fst_rust::ffi::PairWeightValueF32> {
    static fst::SignedLogWeight ToFst(const fst_rust::ffi::PairWeightValueF32& w) {
        return fst::SignedLogWeight(fst::TropicalWeight(w.w1), fst::LogWeight(w.w2));
    }
    static fst_rust::ffi::PairWeightValueF32 FromFst(fst::SignedLogWeight w) {
        fst_rust::ffi::PairWeightValueF32 res;
        res.w1 = w.Value1().Value();
        res.w2 = w.Value2().Value();
        return res;
    }
};

template <>
struct WeightConverter<fst::SignedLog64Arc, fst_rust::ffi::PairWeightValueF64> {
    static fst::SignedLog64Weight ToFst(const fst_rust::ffi::PairWeightValueF64& w) {
        return fst::SignedLog64Weight(fst::TropicalWeight(w.w1), fst::Log64Weight(w.w2));
    }
    static fst_rust::ffi::PairWeightValueF64 FromFst(fst::SignedLog64Weight w) {
        fst_rust::ffi::PairWeightValueF64 res;
        res.w1 = w.Value1().Value();
        res.w2 = w.Value2().Value();
        return res;
    }
};

} // namespace ffi
} // namespace fst_rust
