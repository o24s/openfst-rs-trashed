#pragma once
#include "openfst/lib/encode.h"
#include "cpp/fst.h"
#include "cpp/macros.h"
#include "rust/cxx.h"
#include <memory>
#include <cstdint>
#include <string>

namespace fst_rust {
namespace ffi {

#define DECLARE_ENCODE_FFI(SFX, ARC_TYPE, WTYPE) \
    using EncodeMapper_##SFX = fst::EncodeMapper<ARC_TYPE>; \
    \
    inline std::unique_ptr<EncodeMapper_##SFX> create_encode_mapper_##SFX(uint8_t flags) { \
        return std::make_unique<EncodeMapper_##SFX>(flags, fst::ENCODE); \
    } \
    inline void fst_encode_##SFX(MutableFst_##SFX& fst, EncodeMapper_##SFX& mapper) { \
        fst::Encode(&fst, &mapper); \
    } \
    inline void fst_decode_##SFX(MutableFst_##SFX& fst, const EncodeMapper_##SFX& mapper) { \
        fst::Decode(&fst, mapper); \
    } \
    inline bool write_encode_mapper_##SFX(const EncodeMapper_##SFX& mapper, rust::Str src) { \
        return mapper.Write(std::string(src.data(), src.size())); \
    } \
    inline std::unique_ptr<EncodeMapper_##SFX> read_encode_mapper_##SFX(rust::Str src) { \
        auto* mapper = EncodeMapper_##SFX::Read(std::string(src.data(), src.size())); \
        return std::unique_ptr<EncodeMapper_##SFX>(mapper); \
    } \
    inline uint8_t encode_mapper_flags_##SFX(const EncodeMapper_##SFX& mapper) { \
        return mapper.Flags(); \
    }

FOR_EACH_ARC_TYPE(DECLARE_ENCODE_FFI)
#undef DECLARE_ENCODE_FFI

} // namespace ffi
} // namespace fst_rust
