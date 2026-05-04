#pragma once
#include "openfst/lib/verify.h"
#include "cpp/fst.h"
#include "cpp/macros.h"
#include "rust/cxx.h"
#include <string>

namespace fst_rust {
namespace ffi {

#define DECLARE_VERIFY_FFI(SFX, ARC_TYPE, WTYPE) \
    inline bool fst_verify_##SFX(const Fst_##SFX& fst, bool allow_negative_labels, rust::String& out_err_msg) { \
        absl::Status status = fst::VerifyWithStatus(fst, allow_negative_labels); \
        if (!status.ok()) { \
            out_err_msg = rust::String(std::string(status.message())); \
            return false; \
        } \
        return true; \
    }

FOR_EACH_ARC_TYPE(DECLARE_VERIFY_FFI)
#undef DECLARE_VERIFY_FFI

} // namespace ffi
} // namespace fst_rust
