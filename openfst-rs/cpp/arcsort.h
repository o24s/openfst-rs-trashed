#pragma once
#include "openfst/lib/arcsort.h"
#include "cpp/fst.h"
#include "cpp/macros.h"
#include "rust/cxx.h"
#include <memory>

namespace fst_rust {
namespace ffi {

#define DECLARE_ARCSORT_FFI(SFX, ARC_TYPE, WTYPE) \
    using ILabelArcSortFst_##SFX = fst::ArcSortFst<ARC_TYPE, fst::ILabelCompare<ARC_TYPE>>; \
    using OLabelArcSortFst_##SFX = fst::ArcSortFst<ARC_TYPE, fst::OLabelCompare<ARC_TYPE>>; \
    \
    inline void arcsort_ilabel_##SFX(MutableFst_##SFX& f) { fst::ArcSort(&f, fst::ILabelCompare<ARC_TYPE>()); } \
    inline void arcsort_olabel_##SFX(MutableFst_##SFX& f) { fst::ArcSort(&f, fst::OLabelCompare<ARC_TYPE>()); } \
    inline std::unique_ptr<ILabelArcSortFst_##SFX> create_ilabel_arcsort_fst_##SFX(const Fst_##SFX& f) { return std::make_unique<ILabelArcSortFst_##SFX>(f, fst::ILabelCompare<ARC_TYPE>()); } \
    inline std::unique_ptr<OLabelArcSortFst_##SFX> create_olabel_arcsort_fst_##SFX(const Fst_##SFX& f) { return std::make_unique<OLabelArcSortFst_##SFX>(f, fst::OLabelCompare<ARC_TYPE>()); } \
    inline const Fst_##SFX& ilabel_arcsort_fst_as_fst_##SFX(const ILabelArcSortFst_##SFX& f) { return f; } \
    inline const Fst_##SFX& olabel_arcsort_fst_as_fst_##SFX(const OLabelArcSortFst_##SFX& f) { return f; }

FOR_EACH_ARC_TYPE(DECLARE_ARCSORT_FFI)
#undef DECLARE_ARCSORT_FFI

} // namespace ffi
} // namespace fst_rust
