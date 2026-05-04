#pragma once
#include "openfst/lib/rmepsilon.h"
#include "cpp/fst.h"
#include "cpp/macros.h"
#include "rust/cxx.h"
#include <cstdint>

namespace fst_rust {
namespace ffi {

#define DECLARE_RMEPSILON_FFI(SFX, ARC_TYPE, WTYPE) \
    inline void fst_rmepsilon_##SFX(MutableFst_##SFX& fst, bool connect, \
                                    WTYPE weight_threshold, int32_t state_threshold, \
                                    float delta) { \
        fst::RmEpsilon(&fst, connect, fst_rust::ffi::WeightConverter<ARC_TYPE, WTYPE>::ToFst(weight_threshold), state_threshold, delta); \
    }

FOR_EACH_ARC_TYPE(DECLARE_RMEPSILON_FFI)
#undef DECLARE_RMEPSILON_FFI

} // namespace ffi
} // namespace fst_rust
