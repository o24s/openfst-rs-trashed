#pragma once
#include "openfst/lib/invert.h"
#include "cpp/fst.h"
#include "cpp/macros.h"
#include "rust/cxx.h"

namespace fst_rust {
namespace ffi {

#define DECLARE_INVERT_FFI(SFX, ARC_TYPE, WTYPE) \
    inline void fst_invert_destructive_##SFX(MutableFst_##SFX& fst) { \
        fst::Invert(&fst); \
    } \
    inline void fst_invert_non_destructive_##SFX(const Fst_##SFX& ifst, MutableFst_##SFX& ofst) { \
        fst::Invert(ifst, &ofst); \
    }

FOR_EACH_ARC_TYPE(DECLARE_INVERT_FFI)
#undef DECLARE_INVERT_FFI

} // namespace ffi
} // namespace fst_rust
