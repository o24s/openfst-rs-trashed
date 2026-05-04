#pragma once
#include "openfst/lib/rmfinalepsilon.h"
#include "cpp/fst.h"
#include "cpp/macros.h"
#include "rust/cxx.h"

namespace fst_rust {
namespace ffi {

#define DECLARE_RMFINALEPSILON_FFI(SFX, ARC_TYPE, WTYPE) \
    inline void fst_rmfinalepsilon_##SFX(MutableFst_##SFX& fst) { \
        fst::RmFinalEpsilon(&fst); \
    }

FOR_EACH_ARC_TYPE(DECLARE_RMFINALEPSILON_FFI)
#undef DECLARE_RMFINALEPSILON_FFI

} // namespace ffi
} // namespace fst_rust
