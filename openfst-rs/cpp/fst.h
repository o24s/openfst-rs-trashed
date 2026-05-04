#pragma once
#include "openfst/lib/fst.h"
#include "openfst/lib/mutable-fst.h"
#include "cpp/symbol-table.h"
#include "cpp/macros.h"
#include "rust/cxx.h"
#include <memory>
#include <cstdint>
#include <string>

namespace fst_rust {
namespace ffi {

#define DECLARE_FST_FFI(SFX, ARC_TYPE, WTYPE) \
    using Fst_##SFX = fst::Fst<ARC_TYPE>; \
    using MutableFst_##SFX = fst::MutableFst<ARC_TYPE>; \
    using StateIterator_##SFX = fst::StateIterator<Fst_##SFX>; \
    using ArcIterator_##SFX = fst::ArcIterator<Fst_##SFX>; \
    \
    inline std::unique_ptr<Fst_##SFX> fst_read_##SFX(rust::Str src) { \
        return std::unique_ptr<Fst_##SFX>(Fst_##SFX::Read(std::string(src.data(), src.size()))); \
    } \
    inline bool fst_write_##SFX(const Fst_##SFX& f, rust::Str src) { \
        return f.Write(std::string(src.data(), src.size())); \
    } \
    \
    inline rust::String fst_type_##SFX(const Fst_##SFX& f) { return rust::String(f.Type()); } \
    inline int32_t fst_start_##SFX(const Fst_##SFX& f) { return f.Start(); } \
    inline WTYPE fst_final_weight_##SFX(const Fst_##SFX& f, int32_t s) { return fst_rust::ffi::WeightConverter<ARC_TYPE, WTYPE>::FromFst(f.Final(s)); } \
    inline size_t fst_num_arcs_##SFX(const Fst_##SFX& f, int32_t s) { return f.NumArcs(s); } \
    inline size_t fst_num_input_epsilons_##SFX(const Fst_##SFX& f, int32_t s) { return f.NumInputEpsilons(s); } \
    inline size_t fst_num_output_epsilons_##SFX(const Fst_##SFX& f, int32_t s) { return f.NumOutputEpsilons(s); } \
    inline int32_t fst_num_states_##SFX(const Fst_##SFX& f) { return f.Properties(fst::kExpanded, false) ? fst::CountStates(f) : -1; } \
    inline uint64_t fst_properties_##SFX(const Fst_##SFX& f, uint64_t m, bool t) { return f.Properties(m, t); } \
    \
    inline std::unique_ptr<fst_rust::SymbolTable> fst_input_symbols_##SFX(const Fst_##SFX& f) { auto* sym = f.InputSymbols(); return sym ? std::unique_ptr<fst_rust::SymbolTable>(sym->Copy()) : nullptr; } \
    inline std::unique_ptr<fst_rust::SymbolTable> fst_output_symbols_##SFX(const Fst_##SFX& f) { auto* sym = f.OutputSymbols(); return sym ? std::unique_ptr<fst_rust::SymbolTable>(sym->Copy()) : nullptr; } \
    \
    inline std::unique_ptr<StateIterator_##SFX> create_state_iter_##SFX(const Fst_##SFX& f) { return std::make_unique<StateIterator_##SFX>(f); } \
    inline bool state_iter_done_##SFX(const StateIterator_##SFX& i) { return i.Done(); } \
    inline int32_t state_iter_value_##SFX(const StateIterator_##SFX& i) { return i.Value(); } \
    inline void state_iter_next_##SFX(StateIterator_##SFX& i) { i.Next(); } \
    inline void state_iter_reset_##SFX(StateIterator_##SFX& i) { i.Reset(); } \
    \
    inline std::unique_ptr<ArcIterator_##SFX> create_arc_iter_##SFX(const Fst_##SFX& f, int32_t s) { return std::make_unique<ArcIterator_##SFX>(f, s); } \
    inline bool arc_iter_done_##SFX(const ArcIterator_##SFX& i) { return i.Done(); } \
    inline void arc_iter_next_##SFX(ArcIterator_##SFX& i) { i.Next(); } \
    inline void arc_iter_reset_##SFX(ArcIterator_##SFX& i) { i.Reset(); } \
    inline void arc_iter_seek_##SFX(ArcIterator_##SFX& i, size_t pos) { i.Seek(pos); } \
    inline size_t arc_iter_position_##SFX(const ArcIterator_##SFX& i) { return i.Position(); } \
    inline uint8_t arc_iter_flags_##SFX(const ArcIterator_##SFX& i) { return i.Flags(); } \
    inline void arc_iter_set_flags_##SFX(ArcIterator_##SFX& i, uint8_t f, uint8_t m) { i.SetFlags(f, m); } \
    inline bool arc_iter_advance_##SFX(ArcIterator_##SFX& i, int32_t* il, int32_t* ol, WTYPE* w, int32_t* ns) { \
        if(i.Done()) return false; const auto& a = i.Value(); *il = a.ilabel; *ol = a.olabel; *w = fst_rust::ffi::WeightConverter<ARC_TYPE, WTYPE>::FromFst(a.weight); *ns = a.nextstate; i.Next(); return true; \
    } \
    \
    inline void fst_set_start_##SFX(MutableFst_##SFX& f, int32_t s) { f.SetStart(s); } \
    inline void fst_set_final_##SFX(MutableFst_##SFX& f, int32_t s, WTYPE w) { f.SetFinal(s, fst_rust::ffi::WeightConverter<ARC_TYPE, WTYPE>::ToFst(w)); } \
    inline void fst_set_properties_##SFX(MutableFst_##SFX& f, uint64_t p, uint64_t m) { f.SetProperties(p, m); } \
    inline int32_t fst_add_state_##SFX(MutableFst_##SFX& f) { return f.AddState(); } \
    inline void fst_add_states_##SFX(MutableFst_##SFX& f, size_t n) { f.AddStates(n); } \
    inline void fst_add_arc_##SFX(MutableFst_##SFX& f, int32_t s, int32_t il, int32_t ol, WTYPE w, int32_t ns) { f.AddArc(s, ARC_TYPE(il, ol, fst_rust::ffi::WeightConverter<ARC_TYPE, WTYPE>::ToFst(w), ns)); } \
    inline void fst_delete_arcs_##SFX(MutableFst_##SFX& f, int32_t s) { f.DeleteArcs(s); } \
    inline void fst_delete_arcs_n_##SFX(MutableFst_##SFX& f, int32_t s, size_t n) { f.DeleteArcs(s, n); } \
    inline void fst_delete_all_states_##SFX(MutableFst_##SFX& f) { f.DeleteStates(); } \
    inline void fst_delete_states_##SFX(MutableFst_##SFX& f, rust::Slice<const int32_t> states) { \
        std::vector<int32_t> s(states.begin(), states.end()); f.DeleteStates(s); \
    } \
    inline void fst_reserve_states_##SFX(MutableFst_##SFX& f, size_t n) { f.ReserveStates(n); } \
    inline void fst_reserve_arcs_##SFX(MutableFst_##SFX& f, int32_t s, size_t n) { f.ReserveArcs(s, n); } \
    inline void fst_set_input_symbols_##SFX(MutableFst_##SFX& f, const fst_rust::SymbolTable* syms) { f.SetInputSymbols(syms); } \
    inline void fst_set_output_symbols_##SFX(MutableFst_##SFX& f, const fst_rust::SymbolTable* syms) { f.SetOutputSymbols(syms); }

FOR_EACH_ARC_TYPE(DECLARE_FST_FFI)
#undef DECLARE_FST_FFI

} // namespace ffi
} // namespace fst_rust
