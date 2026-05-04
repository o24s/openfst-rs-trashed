#pragma once
#include "openfst/lib/closure.h"
#include "cpp/fst.h"
#include "cpp/macros.h"
#include "rust/cxx.h"
#include <cstdint>

namespace fst_rust {
namespace ffi {

#define DECLARE_CLOSURE_FFI(SFX, ARC_TYPE, WTYPE) \
    inline void fst_closure_##SFX(MutableFst_##SFX& fst, int32_t closure_type) { \
        fst::Closure(&fst, static_cast<fst::ClosureType>(closure_type)); \
    }

FOR_EACH_ARC_TYPE(DECLARE_CLOSURE_FFI)
#undef DECLARE_CLOSURE_FFI

} // namespace ffi
} // namespace fst_rust
