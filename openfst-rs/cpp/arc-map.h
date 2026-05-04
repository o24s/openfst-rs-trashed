#pragma once
#include "openfst/lib/arc-map.h"
#include "openfst/lib/vector-fst.h"
#include "cpp/fst.h"
#include "cpp/macros.h"
#include "rust/cxx.h"
#include <memory>
#include <cstdint>
#include <stdexcept>

namespace fst_rust {
namespace ffi {

enum class MapType : int32_t {
    Plus = 1,
    Times = 2,
    Power = 3,
    InvertWeight = 4,
    RmWeight = 5,
    Quantize = 6,
    Identity = 7,
    InputEpsilon = 8,
    OutputEpsilon = 9,
    SuperFinal = 10,
    ReverseWeight = 11,
};

#define DECLARE_ARC_MAP_FFI(SFX, ARC_TYPE, WTYPE) \
    inline void fst_arc_map_inplace_##SFX(MutableFst_##SFX& fst, int32_t map_type, WTYPE weight, double power, float delta, int32_t superfinal_label) { \
        switch (static_cast<MapType>(map_type)) { \
            case MapType::Plus: fst::ArcMap(&fst, fst::PlusMapper<ARC_TYPE>(fst_rust::ffi::WeightConverter<ARC_TYPE, WTYPE>::ToFst(weight))); break; \
            case MapType::Times: fst::ArcMap(&fst, fst::TimesMapper<ARC_TYPE>(fst_rust::ffi::WeightConverter<ARC_TYPE, WTYPE>::ToFst(weight))); break; \
            case MapType::Power: fst::ArcMap(&fst, fst::PowerMapper<ARC_TYPE>(power)); break; \
            case MapType::InvertWeight: fst::ArcMap(&fst, fst::InvertWeightMapper<ARC_TYPE>()); break; \
            case MapType::RmWeight: fst::ArcMap(&fst, fst::RmWeightMapper<ARC_TYPE>()); break; \
            case MapType::Quantize: fst::ArcMap(&fst, fst::QuantizeMapper<ARC_TYPE>(delta)); break; \
            case MapType::Identity: fst::ArcMap(&fst, fst::IdentityArcMapper<ARC_TYPE>()); break; \
            case MapType::InputEpsilon: fst::ArcMap(&fst, fst::InputEpsilonMapper<ARC_TYPE>()); break; \
            case MapType::OutputEpsilon: fst::ArcMap(&fst, fst::OutputEpsilonMapper<ARC_TYPE>()); break; \
            case MapType::SuperFinal: fst::ArcMap(&fst, fst::SuperFinalMapper<ARC_TYPE>(superfinal_label)); break; \
            case MapType::ReverseWeight: fst::ArcMap(&fst, fst::ReverseWeightMapper<ARC_TYPE, ARC_TYPE>()); break; \
            default: throw std::invalid_argument("Unknown map_type"); \
        } \
    } \
    inline void fst_arc_map_into_##SFX(const Fst_##SFX& ifst, MutableFst_##SFX& ofst, int32_t map_type, WTYPE weight, double power, float delta, int32_t superfinal_label) { \
        switch (static_cast<MapType>(map_type)) { \
            case MapType::Plus: fst::ArcMap(ifst, &ofst, fst::PlusMapper<ARC_TYPE>(fst_rust::ffi::WeightConverter<ARC_TYPE, WTYPE>::ToFst(weight))); break; \
            case MapType::Times: fst::ArcMap(ifst, &ofst, fst::TimesMapper<ARC_TYPE>(fst_rust::ffi::WeightConverter<ARC_TYPE, WTYPE>::ToFst(weight))); break; \
            case MapType::Power: fst::ArcMap(ifst, &ofst, fst::PowerMapper<ARC_TYPE>(power)); break; \
            case MapType::InvertWeight: fst::ArcMap(ifst, &ofst, fst::InvertWeightMapper<ARC_TYPE>()); break; \
            case MapType::RmWeight: fst::ArcMap(ifst, &ofst, fst::RmWeightMapper<ARC_TYPE>()); break; \
            case MapType::Quantize: fst::ArcMap(ifst, &ofst, fst::QuantizeMapper<ARC_TYPE>(delta)); break; \
            case MapType::Identity: fst::ArcMap(ifst, &ofst, fst::IdentityArcMapper<ARC_TYPE>()); break; \
            case MapType::InputEpsilon: fst::ArcMap(ifst, &ofst, fst::InputEpsilonMapper<ARC_TYPE>()); break; \
            case MapType::OutputEpsilon: fst::ArcMap(ifst, &ofst, fst::OutputEpsilonMapper<ARC_TYPE>()); break; \
            case MapType::SuperFinal: fst::ArcMap(ifst, &ofst, fst::SuperFinalMapper<ARC_TYPE>(superfinal_label)); break; \
            case MapType::ReverseWeight: fst::ArcMap(ifst, &ofst, fst::ReverseWeightMapper<ARC_TYPE, ARC_TYPE>()); break; \
            default: throw std::invalid_argument("Unknown map_type"); \
        } \
    } \
    inline std::unique_ptr<Fst_##SFX> create_arc_map_fst_##SFX(const Fst_##SFX& ifst, int32_t map_type, WTYPE weight, double power, float delta, int32_t superfinal_label, bool gc, size_t gc_limit) { \
        fst::ArcMapFstOptions opts; \
        opts.gc = gc; \
        opts.gc_limit = gc_limit; \
        switch (static_cast<MapType>(map_type)) { \
            case MapType::Plus: return std::make_unique<fst::ArcMapFst<ARC_TYPE, ARC_TYPE, fst::PlusMapper<ARC_TYPE>>>(ifst, fst::PlusMapper<ARC_TYPE>(fst_rust::ffi::WeightConverter<ARC_TYPE, WTYPE>::ToFst(weight)), opts); \
            case MapType::Times: return std::make_unique<fst::ArcMapFst<ARC_TYPE, ARC_TYPE, fst::TimesMapper<ARC_TYPE>>>(ifst, fst::TimesMapper<ARC_TYPE>(fst_rust::ffi::WeightConverter<ARC_TYPE, WTYPE>::ToFst(weight)), opts); \
            case MapType::Power: return std::make_unique<fst::ArcMapFst<ARC_TYPE, ARC_TYPE, fst::PowerMapper<ARC_TYPE>>>(ifst, fst::PowerMapper<ARC_TYPE>(power), opts); \
            case MapType::InvertWeight: return std::make_unique<fst::ArcMapFst<ARC_TYPE, ARC_TYPE, fst::InvertWeightMapper<ARC_TYPE>>>(ifst, fst::InvertWeightMapper<ARC_TYPE>(), opts); \
            case MapType::RmWeight: return std::make_unique<fst::ArcMapFst<ARC_TYPE, ARC_TYPE, fst::RmWeightMapper<ARC_TYPE>>>(ifst, fst::RmWeightMapper<ARC_TYPE>(), opts); \
            case MapType::Quantize: return std::make_unique<fst::ArcMapFst<ARC_TYPE, ARC_TYPE, fst::QuantizeMapper<ARC_TYPE>>>(ifst, fst::QuantizeMapper<ARC_TYPE>(delta), opts); \
            case MapType::Identity: return std::make_unique<fst::ArcMapFst<ARC_TYPE, ARC_TYPE, fst::IdentityArcMapper<ARC_TYPE>>>(ifst, fst::IdentityArcMapper<ARC_TYPE>(), opts); \
            case MapType::InputEpsilon: return std::make_unique<fst::ArcMapFst<ARC_TYPE, ARC_TYPE, fst::InputEpsilonMapper<ARC_TYPE>>>(ifst, fst::InputEpsilonMapper<ARC_TYPE>(), opts); \
            case MapType::OutputEpsilon: return std::make_unique<fst::ArcMapFst<ARC_TYPE, ARC_TYPE, fst::OutputEpsilonMapper<ARC_TYPE>>>(ifst, fst::OutputEpsilonMapper<ARC_TYPE>(), opts); \
            case MapType::SuperFinal: return std::make_unique<fst::ArcMapFst<ARC_TYPE, ARC_TYPE, fst::SuperFinalMapper<ARC_TYPE>>>(ifst, fst::SuperFinalMapper<ARC_TYPE>(superfinal_label), opts); \
            case MapType::ReverseWeight: return std::make_unique<fst::ArcMapFst<ARC_TYPE, ARC_TYPE, fst::ReverseWeightMapper<ARC_TYPE, ARC_TYPE>>>(ifst, fst::ReverseWeightMapper<ARC_TYPE, ARC_TYPE>(), opts); \
            default: throw std::invalid_argument("Unknown map_type"); \
        } \
    }

FOR_EACH_ARC_TYPE(DECLARE_ARC_MAP_FFI)
#undef DECLARE_ARC_MAP_FFI

#define DECLARE_ARC_MAP_CONVERT_FFI(SFX_IN, ARC_IN, SFX_OUT, ARC_OUT) \
    inline void fst_arc_map_convert_##SFX_IN##_to_##SFX_OUT(const Fst_##SFX_IN& ifst, MutableFst_##SFX_OUT& ofst) { \
        fst::ArcMap(ifst, &ofst, fst::WeightConvertMapper<ARC_IN, ARC_OUT>()); \
    } \
    inline std::unique_ptr<Fst_##SFX_OUT> create_arc_map_convert_fst_##SFX_IN##_to_##SFX_OUT(const Fst_##SFX_IN& ifst, bool gc, size_t gc_limit) { \
        fst::ArcMapFstOptions opts; \
        opts.gc = gc; \
        opts.gc_limit = gc_limit; \
        return std::make_unique<fst::ArcMapFst<ARC_IN, ARC_OUT, fst::WeightConvertMapper<ARC_IN, ARC_OUT>>>(ifst, fst::WeightConvertMapper<ARC_IN, ARC_OUT>(), opts); \
    }

FOR_EACH_CONVERTIBLE_ARC_PAIR(DECLARE_ARC_MAP_CONVERT_FFI)
#undef DECLARE_ARC_MAP_CONVERT_FFI

#define DECLARE_GALLIC_MAP_FFI(SFX, ARC_TYPE, WTYPE) \
    struct OpaqueGallicFst_##SFX { \
        fst::VectorFst<fst::GallicArc<ARC_TYPE>> inner; \
    }; \
    \
    inline std::unique_ptr<OpaqueGallicFst_##SFX> fst_map_to_gallic_##SFX(const Fst_##SFX& ifst) { \
        auto wrapper = std::make_unique<OpaqueGallicFst_##SFX>(); \
        fst::ArcMap(ifst, &wrapper->inner, fst::ToGallicMapper<ARC_TYPE>()); \
        return wrapper; \
    } \
    inline void fst_map_from_gallic_##SFX(const OpaqueGallicFst_##SFX& ifst, MutableFst_##SFX& ofst, int32_t superfinal_label) { \
        fst::ArcMap(ifst.inner, &ofst, fst::FromGallicMapper<ARC_TYPE>(superfinal_label)); \
    } \
    inline void fst_gallic_to_new_symbols_##SFX(const OpaqueGallicFst_##SFX& ifst, MutableFst_##SFX& ofst) { \
        fst::ArcMap(ifst.inner, &ofst, fst::GallicToNewSymbolsMapper<ARC_TYPE>(&ofst)); \
    }

FOR_EACH_ARC_TYPE(DECLARE_GALLIC_MAP_FFI)
#undef DECLARE_GALLIC_MAP_FFI

} // namespace ffi
} // namespace fst_rust
