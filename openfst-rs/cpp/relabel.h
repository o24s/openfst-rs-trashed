#pragma once
#include "openfst/lib/relabel.h"
#include "cpp/fst.h"
#include "cpp/macros.h"
#include "rust/cxx.h"
#include <cstdint>
#include <vector>
#include <utility>
#include <stdexcept>

namespace fst_rust {
namespace ffi {

#define DECLARE_RELABEL_FFI(SFX, ARC_TYPE, WTYPE) \
    inline void fst_relabel_##SFX(MutableFst_##SFX& fst, rust::Slice<const int32_t> ipairs_flat, rust::Slice<const int32_t> opairs_flat) { \
        std::vector<std::pair<int32_t, int32_t>> ipairs; \
        std::vector<std::pair<int32_t, int32_t>> opairs; \
        ipairs.reserve(ipairs_flat.size() / 2); \
        for (size_t i = 0; i + 1 < ipairs_flat.size(); i += 2) { \
            ipairs.emplace_back(ipairs_flat[i], ipairs_flat[i+1]); \
        } \
        opairs.reserve(opairs_flat.size() / 2); \
        for (size_t i = 0; i + 1 < opairs_flat.size(); i += 2) { \
            opairs.emplace_back(opairs_flat[i], opairs_flat[i+1]); \
        } \
        fst::Relabel(&fst, ipairs, opairs); \
        if (fst.Properties(fst::kError, false)) { \
            throw std::runtime_error("fst::Relabel failed: Invalid label mapping."); \
        } \
    }

FOR_EACH_ARC_TYPE(DECLARE_RELABEL_FFI)
#undef DECLARE_RELABEL_FFI

} // namespace ffi
} // namespace fst_rust
