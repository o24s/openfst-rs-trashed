#pragma once
#include "openfst/lib/isomorphic.h"
#include "cpp/fst.h"
#include "cpp/macros.h"
#include "rust/cxx.h"

namespace fst_rust {
namespace ffi {

#define DECLARE_ISOMORPHIC_FFI(SFX, ARC_TYPE, WTYPE) \
    inline bool fst_isomorphic_##SFX(const Fst_##SFX& fst1, const Fst_##SFX& fst2, float delta) { \
        return fst::Isomorphic(fst1, fst2, delta); \
    }

FOR_EACH_ARC_TYPE(DECLARE_ISOMORPHIC_FFI)
#undef DECLARE_ISOMORPHIC_FFI

} // namespace ffi
} // namespace fst_rust
