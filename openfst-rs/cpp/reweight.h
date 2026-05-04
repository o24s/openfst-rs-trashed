#pragma once
#include "openfst/lib/reweight.h"
#include "cpp/fst.h"
#include "cpp/macros.h"
#include "rust/cxx.h"
#include <cstdint>
#include <vector>

namespace fst_rust {
namespace ffi {

#define DECLARE_REWEIGHT_FFI(SFX, ARC_TYPE, WTYPE) \
    inline void fst_reweight_##SFX(MutableFst_##SFX& fst, rust::Slice<const WTYPE> potentials, int32_t reweight_type) { \
        std::vector<ARC_TYPE::Weight> potential_vec; \
        potential_vec.reserve(potentials.size()); \
        for (auto w : potentials) { \
            potential_vec.emplace_back(fst_rust::ffi::WeightConverter<ARC_TYPE, WTYPE>::ToFst(w)); \
        } \
        fst::Reweight(&fst, absl::MakeSpan(potential_vec), static_cast<fst::ReweightType>(reweight_type)); \
    }

FOR_EACH_ARC_TYPE(DECLARE_REWEIGHT_FFI)
#undef DECLARE_REWEIGHT_FFI

} // namespace ffi
} // namespace fst_rust
