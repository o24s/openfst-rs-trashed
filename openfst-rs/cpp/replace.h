#pragma once
#include "openfst/lib/replace.h"
#include "openfst/lib/replace-util.h"
#include "cpp/fst.h"
#include "cpp/macros.h"
#include "rust/cxx.h"
#include <cstdint>
#include <vector>
#include <utility>
#include <stdexcept>

namespace fst_rust {
namespace ffi {

#define DECLARE_REPLACE_FFI(SFX, ARC_TYPE, WTYPE) \
    inline void fst_replace_##SFX( \
        rust::Slice<const int32_t> labels, \
        rust::Slice<const size_t> fst_ptrs, \
        MutableFst_##SFX& ofst, \
        int64_t root, \
        int32_t call_label_type, \
        int32_t return_label_type, \
        int64_t return_label) { \
        std::vector<std::pair<int32_t, const Fst_##SFX*>> fst_array; \
        for (size_t i = 0; i < labels.size(); ++i) { \
            fst_array.emplace_back(labels[i], reinterpret_cast<const Fst_##SFX*>(fst_ptrs[i])); \
        } \
        fst::ReplaceUtilOptions opts(root, \
            static_cast<fst::ReplaceLabelType>(call_label_type), \
            static_cast<fst::ReplaceLabelType>(return_label_type), \
            return_label); \
        fst::Replace(fst_array, &ofst, opts); \
        if (ofst.Properties(fst::kError, false)) { \
            throw std::runtime_error("fst::Replace failed. There may be cyclic dependencies or a missing root FST."); \
        } \
    }

FOR_EACH_ARC_TYPE(DECLARE_REPLACE_FFI)
#undef DECLARE_REPLACE_FFI

} // namespace ffi
} // namespace fst_rust
