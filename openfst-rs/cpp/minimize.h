#pragma once
#include "openfst/lib/minimize.h"
#include "cpp/fst.h"
#include "cpp/macros.h"
#include "rust/cxx.h"

namespace fst_rust {
namespace ffi {

#define DECLARE_MINIMIZE_FFI(SFX, ARC_TYPE, WTYPE) \
    inline void fst_minimize_##SFX(MutableFst_##SFX& fst, MutableFst_##SFX* sfst, float delta, bool allow_nondet) { \
        fst::Minimize(&fst, sfst, delta, allow_nondet); \
    }

FOR_EACH_ARC_TYPE(DECLARE_MINIMIZE_FFI)
#undef DECLARE_MINIMIZE_FFI

} // namespace ffi
} // namespace fst_rust
