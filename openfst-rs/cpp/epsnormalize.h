#pragma once
#include "openfst/lib/epsnormalize.h"
#include "cpp/fst.h"
#include "cpp/macros.h"
#include "rust/cxx.h"
#include <cstdint>

namespace fst_rust {
namespace ffi {

#define DECLARE_EPSNORMALIZE_FFI(SFX, ARC_TYPE, WTYPE) \
    inline void fst_epsnormalize_##SFX(const Fst_##SFX& ifst, MutableFst_##SFX& ofst, int32_t type) { \
        fst::EpsNormalize(ifst, &ofst, static_cast<fst::EpsNormalizeType>(type)); \
    }

FOR_EACH_ARC_TYPE(DECLARE_EPSNORMALIZE_FFI)
#undef DECLARE_EPSNORMALIZE_FFI

} // namespace ffi
} // namespace fst_rust
