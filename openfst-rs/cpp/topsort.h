#pragma once
#include "openfst/lib/topsort.h"
#include "cpp/fst.h"
#include "cpp/macros.h"
#include "rust/cxx.h"

namespace fst_rust {
namespace ffi {

#define DECLARE_TOPSORT_FFI(SFX, ARC_TYPE, WTYPE) \
    inline bool fst_topsort_##SFX(MutableFst_##SFX& fst) { \
        return fst::TopSort(&fst); \
    }

FOR_EACH_ARC_TYPE(DECLARE_TOPSORT_FFI)
#undef DECLARE_TOPSORT_FFI

} // namespace ffi
} // namespace fst_rust
