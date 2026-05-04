#pragma once
#include "openfst/lib/randequivalent.h"
#include "cpp/fst.h"
#include "cpp/macros.h"
#include "rust/cxx.h"
#include <cstdint>
#include <stdexcept>

namespace fst_rust {
namespace ffi {

#define DECLARE_RANDEQUIVALENT_FFI(SFX, ARC_TYPE, WTYPE) \
    inline bool fst_randequivalent_##SFX(const Fst_##SFX& fst1, const Fst_##SFX& fst2, \
                                         int32_t npath, float delta, uint64_t seed, \
                                         int32_t max_length) { \
        bool error = false; \
        bool result = fst::RandEquivalent(fst1, fst2, npath, delta, seed, max_length, &error); \
        if (error) { \
            throw std::runtime_error("fst::RandEquivalent failed: inputs may have mismatched symbol tables or other invalid properties."); \
        } \
        return result; \
    }

FOR_EACH_ARC_TYPE(DECLARE_RANDEQUIVALENT_FFI)
#undef DECLARE_RANDEQUIVALENT_FFI

} // namespace ffi
} // namespace fst_rust
