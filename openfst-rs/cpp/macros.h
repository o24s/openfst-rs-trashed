#pragma once
#include "openfst/lib/arc.h"
#include "openfst-rs/src/ffi/weight.rs.h"
#include "cpp/weight-convert.h"
#include "cpp/expectation-weight.h"
#include "cpp/lexicographic-weight.h"
#include "cpp/power-weight.h"
#include "cpp/sparse-power-weight.h"

#include "generated_macros.h"

#define STRIP_SEMIRING(SFX, ARC, WTYPE, SEMIRING, M) M(SFX, ARC, WTYPE)
#define FOR_EACH_ARC_TYPE(M) FOR_EACH_ARC_WITH_SEMIRING(STRIP_SEMIRING, M)