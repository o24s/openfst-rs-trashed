#pragma once
#include "openfst/lib/shortest-distance.h"
#include "cpp/fst.h"
#include "cpp/macros.h"
#include "rust/cxx.h"
#include <cstdint>
#include <vector>
#include <stdexcept>

namespace fst_rust {
namespace ffi {

#define DECLARE_SHORTEST_DISTANCE_FFI(SFX, ARC_TYPE, WTYPE) \
    inline WTYPE fst_shortest_distance_##SFX(const Fst_##SFX& fst, float delta) { \
        auto w = fst::ShortestDistance(fst, delta); \
        if (!w.Member()) { \
            throw std::runtime_error("fst::ShortestDistance failed."); \
        } \
        return fst_rust::ffi::WeightConverter<ARC_TYPE, WTYPE>::FromFst(w); \
    } \
    \
    inline void fst_shortest_distance_vec_##SFX(const Fst_##SFX& fst, rust::Vec<WTYPE>& distance_out, bool reverse, float delta) { \
        std::vector<ARC_TYPE::Weight> dist; \
        fst::ShortestDistance(fst, &dist, reverse, delta); \
        if (dist.size() == 1 && !dist[0].Member()) { \
            throw std::runtime_error("fst::ShortestDistance vector computation failed."); \
        } \
        distance_out.reserve(dist.size()); \
        for (const auto& w : dist) { \
            distance_out.push_back(fst_rust::ffi::WeightConverter<ARC_TYPE, WTYPE>::FromFst(w)); \
        } \
    }

FOR_EACH_ARC_TYPE(DECLARE_SHORTEST_DISTANCE_FFI)
#undef DECLARE_SHORTEST_DISTANCE_FFI

} // namespace ffi
} // namespace fst_rust
