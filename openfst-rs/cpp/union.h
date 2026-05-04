#pragma once
#include "openfst/lib/union.h"
#include "cpp/fst.h"
#include "cpp/macros.h"
#include "rust/cxx.h"

namespace fst_rust {
namespace ffi {

#define DECLARE_UNION_FFI(SFX, ARC_TYPE, WTYPE) \
    inline void fst_union_##SFX(MutableFst_##SFX& fst1, const Fst_##SFX& fst2) { \
        fst::Union(&fst1, fst2); \
    }

FOR_EACH_ARC_TYPE(DECLARE_UNION_FFI)
#undef DECLARE_UNION_FFI

} // namespace ffi
} // namespace fst_rust
