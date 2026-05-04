#pragma once
#include "openfst/lib/vector-fst.h"
#include "cpp/fst.h"
#include "cpp/macros.h"
#include "rust/cxx.h"
#include <memory>
#include <cstdint>
#include <string>

namespace fst_rust {
namespace ffi {

#define DECLARE_VECTOR_FST_FFI(SFX, ARC_TYPE, WTYPE) \
    using VectorFst_##SFX = fst::VectorFst<ARC_TYPE>; \
    \
    inline std::unique_ptr<VectorFst_##SFX> create_vector_fst_##SFX() { return std::make_unique<VectorFst_##SFX>(); } \
    inline std::unique_ptr<VectorFst_##SFX> copy_vector_fst_##SFX(const VectorFst_##SFX& f) { return std::unique_ptr<VectorFst_##SFX>(f.Copy(true)); } \
    inline std::unique_ptr<VectorFst_##SFX> read_vector_fst_##SFX(rust::Str src) { return std::unique_ptr<VectorFst_##SFX>(VectorFst_##SFX::Read(std::string(src.data(), src.size()))); } \
    inline bool write_vector_fst_##SFX(const VectorFst_##SFX& f, rust::Str src) { return f.Write(std::string(src.data(), src.size())); } \
    \
    inline const Fst_##SFX& vector_fst_as_fst_##SFX(const VectorFst_##SFX& f) { return f; } \
    inline MutableFst_##SFX& vector_fst_as_mutable_fst_##SFX(VectorFst_##SFX& f) { return f; } \
    \
    inline void vector_fst_get_arc_##SFX(const VectorFst_##SFX& f, int32_t s, size_t n, int32_t* il, int32_t* ol, WTYPE* w, int32_t* ns) { \
        fst::ArcIterator<VectorFst_##SFX> i(f, s); i.Seek(n); const auto& a = i.Value(); \
        *il = a.ilabel; *ol = a.olabel; *w = fst_rust::ffi::WeightConverter<ARC_TYPE, WTYPE>::FromFst(a.weight); *ns = a.nextstate; \
    } \
    inline void vector_fst_set_arc_##SFX(VectorFst_##SFX& f, int32_t s, size_t n, int32_t il, int32_t ol, WTYPE w, int32_t ns) { \
        fst::MutableArcIterator<VectorFst_##SFX> i(&f, s); i.Seek(n); i.SetValue(ARC_TYPE(il, ol, fst_rust::ffi::WeightConverter<ARC_TYPE, WTYPE>::ToFst(w), ns)); \
    } \
    inline void vector_fst_arc_iter_set_flags_##SFX(const VectorFst_##SFX& f, int32_t s, uint8_t flags, uint8_t mask) { \
        fst::ArcIterator<VectorFst_##SFX> i(f, s); i.SetFlags(flags, mask); \
    }

FOR_EACH_ARC_TYPE(DECLARE_VECTOR_FST_FFI)
#undef DECLARE_VECTOR_FST_FFI

} // namespace ffi
} // namespace fst_rust
