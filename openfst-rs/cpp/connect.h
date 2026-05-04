#pragma once
#include "openfst/lib/connect.h"
#include "cpp/fst.h"
#include "cpp/macros.h"
#include "rust/cxx.h"

namespace fst_rust {
namespace ffi {

#define DECLARE_CONNECT_FFI(SFX, ARC_TYPE, WTYPE) \
    inline void fst_connect_##SFX(MutableFst_##SFX& fst) { \
        fst::Connect(&fst); \
    }

FOR_EACH_ARC_TYPE(DECLARE_CONNECT_FFI)
#undef DECLARE_CONNECT_FFI

} // namespace ffi
} // namespace fst_rust
