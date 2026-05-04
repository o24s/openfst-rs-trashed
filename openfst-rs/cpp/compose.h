#pragma once
#include "openfst/lib/cache.h"
#include "openfst/lib/compose.h"
#include "cpp/fst.h"
#include "cpp/macros.h"
#include "rust/cxx.h"
#include <memory>
#include <cstdint>

namespace fst_rust {
namespace ffi {

#define DECLARE_COMPOSE_FFI(SFX, ARC_TYPE, WTYPE) \
    using ComposeFst_##SFX = fst::ComposeFst<ARC_TYPE>; \
    \
    inline void fst_compose_##SFX(const Fst_##SFX& f1, const Fst_##SFX& f2, MutableFst_##SFX& ofst, bool conn, int32_t ft) { \
        fst::ComposeOptions opts(conn, static_cast<fst::ComposeFilter>(ft)); fst::Compose(f1, f2, &ofst, opts); \
    } \
    inline std::unique_ptr<ComposeFst_##SFX> create_compose_fst_##SFX(const Fst_##SFX& f1, const Fst_##SFX& f2, bool gc, size_t gc_limit) { \
        fst::CacheOptions cache_opts(gc, gc_limit); \
        fst::ComposeFstOptions<ARC_TYPE> opts(cache_opts); \
        return std::make_unique<ComposeFst_##SFX>(f1, f2, opts); \
    } \
    inline const Fst_##SFX& compose_fst_as_fst_##SFX(const ComposeFst_##SFX& f) { return f; }

FOR_EACH_ARC_TYPE(DECLARE_COMPOSE_FFI)
#undef DECLARE_COMPOSE_FFI

} // namespace ffi
} // namespace fst_rust
