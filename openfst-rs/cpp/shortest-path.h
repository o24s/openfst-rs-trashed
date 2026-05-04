#pragma once
#include "openfst/lib/shortest-path.h"
#include "cpp/fst.h"
#include "cpp/macros.h"
#include "rust/cxx.h"
#include <cstdint>

namespace fst_rust {
namespace ffi {

#define DECLARE_SHORTEST_PATH_FFI(SFX, ARC_TYPE, WTYPE) \
    inline void fst_shortest_path_##SFX(const Fst_##SFX& ifst, MutableFst_##SFX& ofst, \
                                        int32_t nshortest, bool unique, bool first_path, \
                                        WTYPE weight_threshold, int32_t state_threshold, \
                                        float delta) { \
        fst::ShortestPath(ifst, &ofst, nshortest, unique, first_path, \
                          fst_rust::ffi::WeightConverter<ARC_TYPE, WTYPE>::ToFst(weight_threshold), state_threshold, delta); \
    }

FOR_EACH_PATH_ARC_TYPE(DECLARE_SHORTEST_PATH_FFI)
#undef DECLARE_SHORTEST_PATH_FFI

} // namespace ffi
} // namespace fst_rust
