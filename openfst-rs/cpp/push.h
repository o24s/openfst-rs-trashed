#pragma once
#include "openfst/lib/push.h"
#include "openfst/lib/reweight.h"
#include "cpp/fst.h"
#include "cpp/macros.h"
#include "rust/cxx.h"
#include <cstdint>

namespace fst_rust {
namespace ffi {

#define DECLARE_PUSH_FFI(SFX, ARC_TYPE, WTYPE) \
    inline void fst_push_##SFX(MutableFst_##SFX& fst, int32_t reweight_type, float delta, bool remove_total_weight) { \
        fst::Push(&fst, static_cast<fst::ReweightType>(reweight_type), delta, remove_total_weight); \
    } \
    inline void fst_push_into_##SFX(const Fst_##SFX& ifst, MutableFst_##SFX& ofst, uint8_t ptype, int32_t reweight_type, float delta) { \
        if (reweight_type == fst::REWEIGHT_TO_INITIAL) { \
            fst::Push<ARC_TYPE, fst::REWEIGHT_TO_INITIAL>(ifst, &ofst, ptype, delta); \
        } else { \
            fst::Push<ARC_TYPE, fst::REWEIGHT_TO_FINAL>(ifst, &ofst, ptype, delta); \
        } \
    }

FOR_EACH_ARC_TYPE(DECLARE_PUSH_FFI)
#undef DECLARE_PUSH_FFI

} // namespace ffi
} // namespace fst_rust
