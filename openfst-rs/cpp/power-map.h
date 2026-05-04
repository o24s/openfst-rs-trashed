#pragma once
#include "openfst/lib/arc-map.h"
#include "openfst/lib/power-weight-mappers.h"
#include "cpp/fst.h"
#include "cpp/macros.h"
#include "rust/cxx.h"
#include "cpp/power-weight.h"

namespace fst_rust {
namespace ffi {

template <class FromArc, class ToArc, class Mapper>
struct CustomWeightToArcMapper {
    Mapper mapper_;
    explicit CustomWeightToArcMapper(const Mapper& mapper) : mapper_(mapper) {}
    
    ToArc operator()(const FromArc& arc) const {
        return ToArc(arc.ilabel, arc.olabel, mapper_(arc.weight), arc.nextstate);
    }
    
    fst::MapFinalAction FinalAction() const { return fst::MAP_NO_SUPERFINAL; }
    fst::MapSymbolsAction InputSymbolsAction() const { return fst::MAP_COPY_SYMBOLS; }
    fst::MapSymbolsAction OutputSymbolsAction() const { return fst::MAP_COPY_SYMBOLS; }
    uint64_t Properties(uint64_t props) const { return props & fst::kWeightInvariantProperties; }
};

#define DECLARE_MAP_TO_POWER(ARC1, ARC2, SFX1, SFX2) \
    inline void fst_map_to_power_##SFX1##_to_##SFX2(const Fst_##SFX1& ifst, MutableFst_##SFX2& ofst, size_t index) { \
        fst::ToPowerWeightMapper<typename fst::ARC1::Weight, typename fst::ARC2::Weight> wmapper(index); \
        fst_rust::ffi::CustomWeightToArcMapper<fst::ARC1, fst::ARC2, decltype(wmapper)> amapper(wmapper); \
        fst::ArcMap(ifst, &ofst, amapper); \
    } \
    inline void fst_map_from_power_##SFX2##_to_##SFX1(const Fst_##SFX2& ifst, MutableFst_##SFX1& ofst, size_t index) { \
        fst::FromPowerWeightMapper<typename fst::ARC2::Weight, typename fst::ARC1::Weight> wmapper(index); \
        fst_rust::ffi::CustomWeightToArcMapper<fst::ARC2, fst::ARC1, decltype(wmapper)> amapper(wmapper); \
        fst::ArcMap(ifst, &ofst, amapper); \
    }

#define FOR_EACH_POWER_ARC_PAIR_C(M) \
    M(StdArc, Power3TropicalArc, std, power3_tropical) \
    M(LogArc, Power3LogArc, log, power3_log) \
    M(Log64Arc, Power3Log64Arc, log64, power3_log64) \
    M(RealArc, Power3RealArc, real, power3_real) \
    M(Real64Arc, Power3Real64Arc, real64, power3_real64) \
    M(MinMaxArc, Power3MinMaxArc, minmax, power3_minmax)

FOR_EACH_POWER_ARC_PAIR_C(DECLARE_MAP_TO_POWER)

#define DECLARE_PROJECT_POWER(ARC, SFX) \
    inline void fst_project_power_##SFX(MutableFst_##SFX& fst, size_t from_index, size_t to_index) { \
        fst::ProjectPowerWeightMapper<typename fst::ARC::Weight> wmapper(from_index, to_index); \
        fst_rust::ffi::CustomWeightToArcMapper<fst::ARC, fst::ARC, decltype(wmapper)> amapper(wmapper); \
        fst::ArcMap(&fst, amapper); \
    }

#define FOR_EACH_POWER_ARC_C(M) \
    M(Power3TropicalArc, power3_tropical) \
    M(Power3LogArc, power3_log) \
    M(Power3Log64Arc, power3_log64) \
    M(Power3RealArc, power3_real) \
    M(Power3Real64Arc, power3_real64) \
    M(Power3MinMaxArc, power3_minmax)

FOR_EACH_POWER_ARC_C(DECLARE_PROJECT_POWER)

} // namespace ffi
} // namespace fst_rust
