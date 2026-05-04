#pragma once
#include "openfst/lib/randgen.h"
#include "cpp/fst.h"
#include "cpp/macros.h"
#include "rust/cxx.h"
#include <cstdint>
#include <stdexcept>

namespace fst_rust {
namespace ffi {

enum class ArcSelectorType : int32_t {
    Uniform = 0,
    LogProb = 1,
    FastLogProb = 2
};

#define DECLARE_RANDGEN_FFI(SFX, ARC_TYPE, WTYPE) \
    inline void fst_randgen_##SFX( \
        const Fst_##SFX& ifst, \
        MutableFst_##SFX& ofst, \
        int32_t selector_type, \
        uint64_t seed, \
        int32_t max_length, \
        int32_t npath, \
        bool weighted, \
        bool remove_total_weight \
    ) { \
        switch (static_cast<ArcSelectorType>(selector_type)) { \
            case ArcSelectorType::Uniform: { \
                fst::UniformArcSelector<ARC_TYPE> selector(seed); \
                fst::RandGenOptions<fst::UniformArcSelector<ARC_TYPE>> opts( \
                    selector, max_length, npath, weighted, remove_total_weight); \
                fst::RandGen(ifst, &ofst, opts); \
                break; \
            } \
            case ArcSelectorType::LogProb: { \
                fst::LogProbArcSelector<ARC_TYPE> selector(seed); \
                fst::RandGenOptions<fst::LogProbArcSelector<ARC_TYPE>> opts( \
                    selector, max_length, npath, weighted, remove_total_weight); \
                fst::RandGen(ifst, &ofst, opts); \
                break; \
            } \
            case ArcSelectorType::FastLogProb: { \
                fst::FastLogProbArcSelector<ARC_TYPE> selector(seed); \
                fst::RandGenOptions<fst::FastLogProbArcSelector<ARC_TYPE>> opts( \
                    selector, max_length, npath, weighted, remove_total_weight); \
                fst::RandGen(ifst, &ofst, opts); \
                break; \
            } \
            default: \
                throw std::invalid_argument("Unknown ArcSelectorType"); \
        } \
    }

FOR_EACH_ARC_TYPE(DECLARE_RANDGEN_FFI)
#undef DECLARE_RANDGEN_FFI

} // namespace ffi
} // namespace fst_rust
