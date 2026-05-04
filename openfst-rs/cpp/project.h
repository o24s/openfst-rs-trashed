#pragma once
#include "openfst/lib/project.h"
#include "cpp/fst.h"
#include "cpp/macros.h"
#include "rust/cxx.h"
#include <cstdint>

namespace fst_rust {
namespace ffi {

#define DECLARE_PROJECT_FFI(SFX, ARC_TYPE, WTYPE) \
    inline void fst_project_destructive_##SFX(MutableFst_##SFX& fst, int32_t project_type) { \
        fst::Project(&fst, static_cast<fst::ProjectType>(project_type)); \
    } \
    inline void fst_project_non_destructive_##SFX(const Fst_##SFX& ifst, MutableFst_##SFX& ofst, int32_t project_type) { \
        fst::Project(ifst, &ofst, static_cast<fst::ProjectType>(project_type)); \
    }

FOR_EACH_ARC_TYPE(DECLARE_PROJECT_FFI)
#undef DECLARE_PROJECT_FFI

} // namespace ffi
} // namespace fst_rust
