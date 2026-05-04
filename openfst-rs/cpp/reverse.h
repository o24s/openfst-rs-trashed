#pragma once
#include "openfst/lib/reverse.h"
#include "cpp/fst.h"
#include "cpp/macros.h"
#include "rust/cxx.h"

namespace fst_rust {
namespace ffi {

#define DECLARE_REVERSE_FFI(SFX, ARC_TYPE, WTYPE) \
    inline void fst_reverse_##SFX(const Fst_##SFX& ifst, MutableFst_##SFX& ofst, bool require_superinitial) { \
        fst::Reverse(ifst, &ofst, require_superinitial); \
    }

FOR_EACH_ARC_TYPE(DECLARE_REVERSE_FFI)
#undef DECLARE_REVERSE_FFI

} // namespace ffi
} // namespace fst_rust
