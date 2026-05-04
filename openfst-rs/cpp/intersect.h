#pragma once
#include "openfst/lib/intersect.h"
#include "cpp/fst.h"
#include "cpp/macros.h"
#include "rust/cxx.h"
#include <cstdint>
#include <stdexcept>

namespace fst_rust {
namespace ffi {

#define DECLARE_INTERSECT_FFI(SFX, ARC_TYPE, WTYPE) \
    inline void fst_intersect_##SFX(const Fst_##SFX& ifst1, const Fst_##SFX& ifst2, MutableFst_##SFX& ofst, bool connect, int32_t ft) { \
        if (!ifst1.Properties(fst::kAcceptor, true) || !ifst2.Properties(fst::kAcceptor, true)) { \
            throw std::runtime_error("fst::Intersect failed: Both input FSTs must be acceptors."); \
        } \
        fst::IntersectOptions opts(connect, static_cast<fst::ComposeFilter>(ft)); \
        fst::Intersect(ifst1, ifst2, &ofst, opts); \
    }

FOR_EACH_ARC_TYPE(DECLARE_INTERSECT_FFI)
#undef DECLARE_INTERSECT_FFI

} // namespace ffi
} // namespace fst_rust
