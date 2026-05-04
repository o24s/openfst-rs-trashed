#pragma once
#include "openfst/lib/equal.h"
#include "cpp/fst.h"
#include "cpp/macros.h"
#include "rust/cxx.h"
#include <cstdint>

namespace fst_rust {
namespace ffi {

#define DECLARE_EQUAL_FFI(SFX, ARC_TYPE, WTYPE) \
    inline bool fst_equal_##SFX(const Fst_##SFX& fst1, const Fst_##SFX& fst2, float delta, uint8_t etype) { \
        return fst::Equal(fst1, fst2, delta, etype); \
    }

FOR_EACH_ARC_TYPE(DECLARE_EQUAL_FFI)
#undef DECLARE_EQUAL_FFI

} // namespace ffi
} // namespace fst_rust
