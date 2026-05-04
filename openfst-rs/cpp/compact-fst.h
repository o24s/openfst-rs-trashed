#pragma once
#include "openfst/lib/compact-fst.h"
#include "openfst/lib/register.h"
#include "cpp/fst.h"
#include "cpp/macros.h"
#include "rust/cxx.h"
#include <memory>
#include <string>

namespace fst_rust {
namespace ffi {

#define DECLARE_COMPACT_FST_TYPE_FFI(SFX, ARC_TYPE, WTYPE, COMPTYPE) \
    using Compact##COMPTYPE##Fst_##SFX = fst::Compact##COMPTYPE##Fst<ARC_TYPE, uint32_t>; \
    inline std::unique_ptr<Compact##COMPTYPE##Fst_##SFX> read_compact_##COMPTYPE##_fst_##SFX(rust::Str src) { \
        return std::unique_ptr<Compact##COMPTYPE##Fst_##SFX>(Compact##COMPTYPE##Fst_##SFX::Read(std::string(src.data(), src.size()))); \
    } \
    inline std::unique_ptr<Compact##COMPTYPE##Fst_##SFX> create_compact_##COMPTYPE##_fst_from_fst_##SFX(const Fst_##SFX& fst) { \
        return std::make_unique<Compact##COMPTYPE##Fst_##SFX>(fst); \
    } \
    inline bool write_compact_##COMPTYPE##_fst_##SFX(const Compact##COMPTYPE##Fst_##SFX& f, rust::Str src) { \
        return f.Write(std::string(src.data(), src.size())); \
    } \
    inline const Fst_##SFX& compact_##COMPTYPE##_fst_as_fst_##SFX(const Compact##COMPTYPE##Fst_##SFX& f) { \
        return f; \
    }

#define DECLARE_COMPACT_FST_FFI(SFX, ARC_TYPE, WTYPE) \
    DECLARE_COMPACT_FST_TYPE_FFI(SFX, ARC_TYPE, WTYPE, String) \
    DECLARE_COMPACT_FST_TYPE_FFI(SFX, ARC_TYPE, WTYPE, WeightedString) \
    DECLARE_COMPACT_FST_TYPE_FFI(SFX, ARC_TYPE, WTYPE, Acceptor) \
    DECLARE_COMPACT_FST_TYPE_FFI(SFX, ARC_TYPE, WTYPE, Unweighted) \
    DECLARE_COMPACT_FST_TYPE_FFI(SFX, ARC_TYPE, WTYPE, UnweightedAcceptor)

FOR_EACH_ARC_TYPE(DECLARE_COMPACT_FST_FFI)

#undef DECLARE_COMPACT_FST_FFI
#undef DECLARE_COMPACT_FST_TYPE_FFI

#define REGISTER_COMPACT_FST(SFX, ARC_TYPE, WTYPE) \
    static fst::FstRegisterer<CompactStringFst_##SFX> reg_string_##SFX; \
    static fst::FstRegisterer<CompactWeightedStringFst_##SFX> reg_w_string_##SFX; \
    static fst::FstRegisterer<CompactAcceptorFst_##SFX> reg_acc_##SFX; \
    static fst::FstRegisterer<CompactUnweightedFst_##SFX> reg_unw_##SFX; \
    static fst::FstRegisterer<CompactUnweightedAcceptorFst_##SFX> reg_unw_acc_##SFX;

inline void init_compact_fst_registry() {
    static bool initialized = false;
    if (!initialized) {
        FOR_EACH_ARC_TYPE(REGISTER_COMPACT_FST)
        initialized = true;
    }
}

#undef REGISTER_COMPACT_FST

} // namespace ffi
} // namespace fst_rust
