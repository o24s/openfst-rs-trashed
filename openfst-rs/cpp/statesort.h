#pragma once
#include "openfst/lib/statesort.h"
#include "cpp/fst.h"
#include "cpp/macros.h"
#include "rust/cxx.h"
#include <vector>
#include <cstdint>

namespace fst_rust {
namespace ffi {

#define DECLARE_STATESORT_FFI(SFX, ARC_TYPE, WTYPE) \
    inline void fst_statesort_##SFX(MutableFst_##SFX& fst, rust::Slice<const int32_t> order) { \
        std::vector<int32_t> order_vec(order.begin(), order.end()); \
        fst::StateSort(&fst, absl::MakeSpan(order_vec)); \
    }

FOR_EACH_ARC_TYPE(DECLARE_STATESORT_FFI)
#undef DECLARE_STATESORT_FFI

} // namespace ffi
} // namespace fst_rust
