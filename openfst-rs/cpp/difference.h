#pragma once
#include "openfst/lib/difference.h"
#include "cpp/fst.h"
#include "cpp/macros.h"
#include "rust/cxx.h"
#include <cstdint>
#include <stdexcept>

namespace fst_rust {
namespace ffi {

#define DECLARE_DIFFERENCE_FFI(SFX, ARC_TYPE, WTYPE) \
    inline void fst_difference_##SFX(const Fst_##SFX& ifst1, const Fst_##SFX& ifst2, MutableFst_##SFX& ofst, bool connect, int32_t ft) { \
        if (!ifst1.Properties(fst::kAcceptor, true)) { \
            throw std::runtime_error("fst::Difference failed: The first argument must be an acceptor."); \
        } \
\
        constexpr uint64_t req_props2 = fst::kAcceptor | fst::kIDeterministic | fst::kNoEpsilons | fst::kUnweighted; \
        if (ifst2.Properties(req_props2, true) != req_props2) { \
            throw std::runtime_error("fst::Difference failed: The second argument must be an unweighted, epsilon-free, deterministic acceptor."); \
        } \
        fst::DifferenceOptions opts(connect, static_cast<fst::ComposeFilter>(ft)); \
        fst::Difference(ifst1, ifst2, &ofst, opts); \
    }

FOR_EACH_ARC_TYPE(DECLARE_DIFFERENCE_FFI)
#undef DECLARE_DIFFERENCE_FFI

} // namespace ffi
} // namespace fst_rust