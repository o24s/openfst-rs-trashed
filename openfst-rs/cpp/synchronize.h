#pragma once
#include "openfst/lib/synchronize.h"
#include "cpp/fst.h"
#include "cpp/macros.h"
#include "rust/cxx.h"
#include <memory>

namespace fst_rust {
namespace ffi {

#define DECLARE_SYNCHRONIZE_FFI(SFX, ARC_TYPE, WTYPE) \
    using SynchronizeFst_##SFX = fst::SynchronizeFst<ARC_TYPE>; \
    \
    inline std::unique_ptr<SynchronizeFst_##SFX> create_synchronize_fst_##SFX(const Fst_##SFX& ifst, bool gc, size_t gc_limit) { \
        fst::SynchronizeFstOptions opts(gc, gc_limit); \
        return std::make_unique<SynchronizeFst_##SFX>(ifst, opts); \
    } \
    inline const Fst_##SFX& synchronize_fst_as_fst_##SFX(const SynchronizeFst_##SFX& f) { \
        return f; \
    } \
    inline void fst_synchronize_##SFX(const Fst_##SFX& ifst, MutableFst_##SFX& ofst) { \
        fst::Synchronize(ifst, &ofst); \
    }

FOR_EACH_ARC_TYPE(DECLARE_SYNCHRONIZE_FFI)
#undef DECLARE_SYNCHRONIZE_FFI

} // namespace ffi
} // namespace fst_rust
