#pragma once
#include "openfst/lib/concat.h"
#include "cpp/fst.h"
#include "cpp/macros.h"
#include "rust/cxx.h"

namespace fst_rust {
namespace ffi {

#define DECLARE_CONCAT_FFI(SFX, ARC_TYPE, WTYPE) \
    inline void fst_concat_##SFX(MutableFst_##SFX& fst1, const Fst_##SFX& fst2) { \
        fst::Concat(&fst1, fst2); \
    }

FOR_EACH_ARC_TYPE(DECLARE_CONCAT_FFI)
#undef DECLARE_CONCAT_FFI

} // namespace ffi
} // namespace fst_rust
