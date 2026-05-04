#pragma once
#include "openfst/lib/prune.h"
#include "cpp/fst.h"
#include "cpp/macros.h"
#include "rust/cxx.h"
#include <cstdint>

namespace fst_rust {
namespace ffi {

#define DECLARE_PRUNE_FFI(SFX, ARC_TYPE, WTYPE) \
    inline void fst_prune_##SFX(MutableFst_##SFX& fst, WTYPE weight_threshold, int32_t state_threshold, float delta) { \
        fst::Prune(&fst, fst_rust::ffi::WeightConverter<ARC_TYPE, WTYPE>::ToFst(weight_threshold), state_threshold, delta); \
    } \
    inline void fst_prune_into_##SFX(const Fst_##SFX& ifst, MutableFst_##SFX& ofst, WTYPE weight_threshold, int32_t state_threshold, float delta) { \
        fst::Prune(ifst, &ofst, fst_rust::ffi::WeightConverter<ARC_TYPE, WTYPE>::ToFst(weight_threshold), state_threshold, delta); \
    }

FOR_EACH_PATH_ARC_TYPE(DECLARE_PRUNE_FFI)
#undef DECLARE_PRUNE_FFI

} // namespace ffi
} // namespace fst_rust
