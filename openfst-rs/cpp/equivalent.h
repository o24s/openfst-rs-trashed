#pragma once
#include "openfst/lib/equivalent.h"
#include "cpp/fst.h"
#include "cpp/macros.h"
#include "rust/cxx.h"
#include <stdexcept>

namespace fst_rust {
namespace ffi {

#define DECLARE_EQUIVALENT_FFI(SFX, ARC_TYPE, WTYPE) \
    inline bool fst_equivalent_##SFX(const Fst_##SFX& fst1, const Fst_##SFX& fst2, float delta) { \
        bool error = false; \
        bool result = fst::Equivalent(fst1, fst2, delta, &error); \
        if (error) { \
            throw std::runtime_error("fst::Equivalent failed: inputs must be deterministic, epsilon-free acceptors, and symbol tables must match."); \
        } \
        return result; \
    }

FOR_EACH_ARC_TYPE(DECLARE_EQUIVALENT_FFI)
#undef DECLARE_EQUIVALENT_FFI

} // namespace ffi
} // namespace fst_rust
