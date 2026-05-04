#pragma once
#include "openfst/lib/disambiguate.h"
#include "cpp/fst.h"
#include "cpp/macros.h"
#include "rust/cxx.h"
#include <cstdint>

namespace fst_rust {
namespace ffi {

#define DECLARE_DISAMBIGUATE_FFI(SFX, ARC_TYPE, WTYPE) \
    inline void fst_disambiguate_##SFX(const Fst_##SFX& ifst, MutableFst_##SFX& ofst, \
                                       float delta, WTYPE weight_threshold, \
                                       int32_t state_threshold, int32_t subsequential_label) { \
        fst::DisambiguateOptions<ARC_TYPE> opts( \
            delta, fst_rust::ffi::WeightConverter<ARC_TYPE, WTYPE>::ToFst(weight_threshold), state_threshold, subsequential_label); \
        fst::Disambiguate(ifst, &ofst, opts); \
    }

FOR_EACH_ARC_TYPE(DECLARE_DISAMBIGUATE_FFI)
#undef DECLARE_DISAMBIGUATE_FFI

} // namespace ffi
} // namespace fst_rust
