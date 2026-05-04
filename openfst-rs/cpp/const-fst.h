#pragma once
#include "openfst/lib/const-fst.h"
#include "openfst/lib/register.h"
#include "cpp/fst.h"
#include "cpp/macros.h"
#include "rust/cxx.h"
#include <memory>
#include <string>

namespace fst_rust {
namespace ffi {

#define DECLARE_CONST_FST_FFI(SFX, ARC_TYPE, WTYPE) \
    using ConstFst_##SFX = fst::ConstFst<ARC_TYPE>; \
    \
    inline std::unique_ptr<ConstFst_##SFX> read_const_fst_##SFX(rust::Str src) { \
        return std::unique_ptr<ConstFst_##SFX>(ConstFst_##SFX::Read(std::string(src.data(), src.size()))); \
    } \
    \
    inline std::unique_ptr<ConstFst_##SFX> create_const_fst_from_fst_##SFX(const Fst_##SFX& fst) { \
        return std::make_unique<ConstFst_##SFX>(fst); \
    } \
    \
    inline bool write_const_fst_##SFX(const ConstFst_##SFX& f, rust::Str src) { \
        return f.Write(std::string(src.data(), src.size())); \
    } \
    \
    inline const Fst_##SFX& const_fst_as_fst_##SFX(const ConstFst_##SFX& f) { \
        return f; \
    }

FOR_EACH_ARC_TYPE(DECLARE_CONST_FST_FFI)
#undef DECLARE_CONST_FST_FFI

#define REGISTER_CONST_FST(SFX, ARC_TYPE, WTYPE) \
    static fst::FstRegisterer<ConstFst_##SFX> reg_##SFX;

inline void init_const_fst_registry() {
    static bool initialized = false;
    if (!initialized) {
        FOR_EACH_ARC_TYPE(REGISTER_CONST_FST)
        initialized = true;
    }
}

#undef REGISTER_CONST_FST

} // namespace ffi
} // namespace fst_rust
