use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::expectation_weight::*;
use crate::ffi::lexicographic_weight::*;
use crate::ffi::power_weight::{PowerWeightValueF32_3, PowerWeightValueF64_3};
use crate::ffi::sparse_power_weight::*;
use crate::ffi::symbol_table::SymbolTable as CxxSymbolTable;
use crate::ffi::weight::{PairWeightValueF32, PairWeightValueF64};
use cxx::{UniquePtr, memory::UniquePtrTarget};
use openfst_rs_macros::template;
use std::pin::Pin;

pub trait FstFfi: Arc<StateId = i32, Label = i32>
where
    Self::Weight: WeightFfi,
{
    type FstCxx: UniquePtrTarget;
    type MutableFstCxx;
    type StateIteratorCxx: UniquePtrTarget;
    type ArcIteratorCxx: UniquePtrTarget;

    fn fst_read(source: &str) -> UniquePtr<Self::FstCxx>;
    fn fst_write(fst: &Self::FstCxx, source: &str) -> bool;

    fn fst_type(fst: &Self::FstCxx) -> String;
    fn fst_start(fst: &Self::FstCxx) -> i32;
    fn fst_final_weight(fst: &Self::FstCxx, s: i32) -> <Self::Weight as WeightFfi>::ValueType;
    fn fst_num_arcs(fst: &Self::FstCxx, s: i32) -> usize;
    fn fst_num_input_epsilons(fst: &Self::FstCxx, s: i32) -> usize;
    fn fst_num_output_epsilons(fst: &Self::FstCxx, s: i32) -> usize;
    fn fst_num_states(fst: &Self::FstCxx) -> i32;
    fn fst_properties(fst: &Self::FstCxx, mask: u64, test: bool) -> u64;
    fn fst_input_symbols(fst: &Self::FstCxx) -> UniquePtr<CxxSymbolTable>;
    fn fst_output_symbols(fst: &Self::FstCxx) -> UniquePtr<CxxSymbolTable>;

    fn create_state_iter(fst: &Self::FstCxx) -> UniquePtr<Self::StateIteratorCxx>;
    fn state_iter_done(iter: &Self::StateIteratorCxx) -> bool;
    fn state_iter_value(iter: &Self::StateIteratorCxx) -> i32;
    fn state_iter_next(iter: Pin<&mut Self::StateIteratorCxx>);
    fn state_iter_reset(iter: Pin<&mut Self::StateIteratorCxx>);

    fn create_arc_iter(fst: &Self::FstCxx, s: i32) -> UniquePtr<Self::ArcIteratorCxx>;
    fn arc_iter_done(iter: &Self::ArcIteratorCxx) -> bool;
    fn arc_iter_next(iter: Pin<&mut Self::ArcIteratorCxx>);
    fn arc_iter_reset(iter: Pin<&mut Self::ArcIteratorCxx>);
    fn arc_iter_seek(iter: Pin<&mut Self::ArcIteratorCxx>, pos: usize);
    fn arc_iter_position(iter: &Self::ArcIteratorCxx) -> usize;
    fn arc_iter_flags(iter: &Self::ArcIteratorCxx) -> u8;
    fn arc_iter_set_flags(iter: Pin<&mut Self::ArcIteratorCxx>, flags: u8, mask: u8);
    unsafe fn arc_iter_advance(
        iter: Pin<&mut Self::ArcIteratorCxx>,
        il: *mut i32,
        ol: *mut i32,
        w: *mut <Self::Weight as WeightFfi>::ValueType,
        ns: *mut i32,
    ) -> bool;

    fn fst_set_start(fst: Pin<&mut Self::MutableFstCxx>, s: i32);
    fn fst_set_final(
        fst: Pin<&mut Self::MutableFstCxx>,
        s: i32,
        w: <Self::Weight as WeightFfi>::ValueType,
    );
    fn fst_set_properties(fst: Pin<&mut Self::MutableFstCxx>, props: u64, mask: u64);
    fn fst_add_state(fst: Pin<&mut Self::MutableFstCxx>) -> i32;
    fn fst_add_states(fst: Pin<&mut Self::MutableFstCxx>, n: usize);
    fn fst_add_arc(
        fst: Pin<&mut Self::MutableFstCxx>,
        s: i32,
        il: i32,
        ol: i32,
        w: <Self::Weight as WeightFfi>::ValueType,
        ns: i32,
    );
    fn fst_delete_arcs(fst: Pin<&mut Self::MutableFstCxx>, s: i32);
    fn fst_delete_arcs_n(fst: Pin<&mut Self::MutableFstCxx>, s: i32, n: usize);
    fn fst_delete_all_states(fst: Pin<&mut Self::MutableFstCxx>);
    fn fst_delete_states(fst: Pin<&mut Self::MutableFstCxx>, states: &[i32]);
    fn fst_reserve_states(fst: Pin<&mut Self::MutableFstCxx>, n: usize);
    fn fst_reserve_arcs(fst: Pin<&mut Self::MutableFstCxx>, s: i32, n: usize);

    unsafe fn fst_set_input_symbols(
        fst: Pin<&mut Self::MutableFstCxx>,
        syms: *const CxxSymbolTable,
    );
    unsafe fn fst_set_output_symbols(
        fst: Pin<&mut Self::MutableFstCxx>,
        syms: *const CxxSymbolTable,
    );
}

#[template("fst")]
pub mod ffi_binding {
    unsafe extern "C++" {
        include!("cpp/fst.h");

        #[namespace = "fst_rust"]
        type SymbolTable = crate::ffi::symbol_table::SymbolTable;

        #[expand(for_each_arc_type)]
        type Fst___SFX__;
        #[expand(for_each_arc_type)]
        type MutableFst___SFX__;
        #[expand(for_each_arc_type)]
        type StateIterator___SFX__;
        #[expand(for_each_arc_type)]
        type ArcIterator___SFX__;

        #[expand(for_each_arc_type)]
        fn fst_read___SFX__(s: &str) -> UniquePtr<Fst___SFX__>;
        #[expand(for_each_arc_type)]
        fn fst_write___SFX__(f: &Fst___SFX__, s: &str) -> bool;

        #[expand(for_each_arc_type)]
        fn fst_type___SFX__(f: &Fst___SFX__) -> String;
        #[expand(for_each_arc_type)]
        fn fst_start___SFX__(f: &Fst___SFX__) -> i32;
        #[expand(for_each_arc_type)]
        fn fst_final_weight___SFX__(f: &Fst___SFX__, s: i32) -> __WTYPE__;
        #[expand(for_each_arc_type)]
        fn fst_num_arcs___SFX__(f: &Fst___SFX__, s: i32) -> usize;
        #[expand(for_each_arc_type)]
        fn fst_num_input_epsilons___SFX__(f: &Fst___SFX__, s: i32) -> usize;
        #[expand(for_each_arc_type)]
        fn fst_num_output_epsilons___SFX__(f: &Fst___SFX__, s: i32) -> usize;
        #[expand(for_each_arc_type)]
        fn fst_num_states___SFX__(f: &Fst___SFX__) -> i32;
        #[expand(for_each_arc_type)]
        fn fst_properties___SFX__(f: &Fst___SFX__, m: u64, t: bool) -> u64;
        #[expand(for_each_arc_type)]
        fn fst_input_symbols___SFX__(f: &Fst___SFX__) -> UniquePtr<SymbolTable>;
        #[expand(for_each_arc_type)]
        fn fst_output_symbols___SFX__(f: &Fst___SFX__) -> UniquePtr<SymbolTable>;

        #[expand(for_each_arc_type)]
        fn create_state_iter___SFX__(f: &Fst___SFX__) -> UniquePtr<StateIterator___SFX__>;
        #[expand(for_each_arc_type)]
        fn state_iter_done___SFX__(i: &StateIterator___SFX__) -> bool;
        #[expand(for_each_arc_type)]
        fn state_iter_value___SFX__(i: &StateIterator___SFX__) -> i32;
        #[expand(for_each_arc_type)]
        fn state_iter_next___SFX__(i: Pin<&mut StateIterator___SFX__>);
        #[expand(for_each_arc_type)]
        fn state_iter_reset___SFX__(i: Pin<&mut StateIterator___SFX__>);

        #[expand(for_each_arc_type)]
        fn create_arc_iter___SFX__(f: &Fst___SFX__, s: i32) -> UniquePtr<ArcIterator___SFX__>;
        #[expand(for_each_arc_type)]
        fn arc_iter_done___SFX__(i: &ArcIterator___SFX__) -> bool;
        #[expand(for_each_arc_type)]
        fn arc_iter_next___SFX__(i: Pin<&mut ArcIterator___SFX__>);
        #[expand(for_each_arc_type)]
        fn arc_iter_reset___SFX__(i: Pin<&mut ArcIterator___SFX__>);
        #[expand(for_each_arc_type)]
        fn arc_iter_seek___SFX__(i: Pin<&mut ArcIterator___SFX__>, pos: usize);
        #[expand(for_each_arc_type)]
        fn arc_iter_position___SFX__(i: &ArcIterator___SFX__) -> usize;
        #[expand(for_each_arc_type)]
        fn arc_iter_flags___SFX__(i: &ArcIterator___SFX__) -> u8;
        #[expand(for_each_arc_type)]
        fn arc_iter_set_flags___SFX__(i: Pin<&mut ArcIterator___SFX__>, f: u8, m: u8);
        #[expand(for_each_arc_type)]
        unsafe fn arc_iter_advance___SFX__(
            i: Pin<&mut ArcIterator___SFX__>,
            il: *mut i32,
            ol: *mut i32,
            w: *mut __WTYPE__,
            ns: *mut i32,
        ) -> bool;

        #[expand(for_each_arc_type)]
        fn fst_set_start___SFX__(f: Pin<&mut MutableFst___SFX__>, s: i32);
        #[expand(for_each_arc_type)]
        fn fst_set_final___SFX__(f: Pin<&mut MutableFst___SFX__>, s: i32, w: __WTYPE__);
        #[expand(for_each_arc_type)]
        fn fst_set_properties___SFX__(f: Pin<&mut MutableFst___SFX__>, p: u64, m: u64);
        #[expand(for_each_arc_type)]
        fn fst_add_state___SFX__(f: Pin<&mut MutableFst___SFX__>) -> i32;
        #[expand(for_each_arc_type)]
        fn fst_add_states___SFX__(f: Pin<&mut MutableFst___SFX__>, n: usize);
        #[expand(for_each_arc_type)]
        fn fst_add_arc___SFX__(
            f: Pin<&mut MutableFst___SFX__>,
            s: i32,
            il: i32,
            ol: i32,
            w: __WTYPE__,
            ns: i32,
        );
        #[expand(for_each_arc_type)]
        fn fst_delete_arcs___SFX__(f: Pin<&mut MutableFst___SFX__>, s: i32);
        #[expand(for_each_arc_type)]
        fn fst_delete_arcs_n___SFX__(f: Pin<&mut MutableFst___SFX__>, s: i32, n: usize);
        #[expand(for_each_arc_type)]
        fn fst_delete_all_states___SFX__(f: Pin<&mut MutableFst___SFX__>);
        #[expand(for_each_arc_type)]
        fn fst_delete_states___SFX__(f: Pin<&mut MutableFst___SFX__>, states: &[i32]);
        #[expand(for_each_arc_type)]
        fn fst_reserve_states___SFX__(f: Pin<&mut MutableFst___SFX__>, n: usize);
        #[expand(for_each_arc_type)]
        fn fst_reserve_arcs___SFX__(f: Pin<&mut MutableFst___SFX__>, s: i32, n: usize);
        #[expand(for_each_arc_type)]
        unsafe fn fst_set_input_symbols___SFX__(
            f: Pin<&mut MutableFst___SFX__>,
            syms: *const SymbolTable,
        );
        #[expand(for_each_arc_type)]
        unsafe fn fst_set_output_symbols___SFX__(
            f: Pin<&mut MutableFst___SFX__>,
            syms: *const SymbolTable,
        );
    }
}

macro_rules! bind_fst_ffi_impl {
    ($arc:ty, $wtype:ty, $sfx:ident) => {
        pastey::paste! {
            impl FstFfi for $arc {
                type FstCxx = ffi_binding::[<Fst_ $sfx>];
                type MutableFstCxx = ffi_binding::[<MutableFst_ $sfx>];
                type StateIteratorCxx = ffi_binding::[<StateIterator_ $sfx>];
                type ArcIteratorCxx = ffi_binding::[<ArcIterator_ $sfx>];

                fn fst_read(s: &str) -> UniquePtr<Self::FstCxx> { ffi_binding::[<fst_read_ $sfx>](s) }
                fn fst_write(f: &Self::FstCxx, s: &str) -> bool { ffi_binding::[<fst_write_ $sfx>](f, s) }

                fn fst_type(f: &Self::FstCxx) -> String { ffi_binding::[<fst_type_ $sfx>](f) }
                fn fst_start(f: &Self::FstCxx) -> i32 { ffi_binding::[<fst_start_ $sfx>](f) }
                fn fst_final_weight(f: &Self::FstCxx, s: i32) -> $wtype { ffi_binding::[<fst_final_weight_ $sfx>](f, s) }
                fn fst_num_arcs(f: &Self::FstCxx, s: i32) -> usize { ffi_binding::[<fst_num_arcs_ $sfx>](f, s) }
                fn fst_num_input_epsilons(f: &Self::FstCxx, s: i32) -> usize { ffi_binding::[<fst_num_input_epsilons_ $sfx>](f, s) }
                fn fst_num_output_epsilons(f: &Self::FstCxx, s: i32) -> usize { ffi_binding::[<fst_num_output_epsilons_ $sfx>](f, s) }
                fn fst_num_states(f: &Self::FstCxx) -> i32 { ffi_binding::[<fst_num_states_ $sfx>](f) }
                fn fst_properties(f: &Self::FstCxx, m: u64, t: bool) -> u64 { ffi_binding::[<fst_properties_ $sfx>](f, m, t) }
                fn fst_input_symbols(f: &Self::FstCxx) -> UniquePtr<CxxSymbolTable> { ffi_binding::[<fst_input_symbols_ $sfx>](f) }
                fn fst_output_symbols(f: &Self::FstCxx) -> UniquePtr<CxxSymbolTable> { ffi_binding::[<fst_output_symbols_ $sfx>](f) }

                fn create_state_iter(f: &Self::FstCxx) -> UniquePtr<Self::StateIteratorCxx> { ffi_binding::[<create_state_iter_ $sfx>](f) }
                fn state_iter_done(i: &Self::StateIteratorCxx) -> bool { ffi_binding::[<state_iter_done_ $sfx>](i) }
                fn state_iter_value(i: &Self::StateIteratorCxx) -> i32 { ffi_binding::[<state_iter_value_ $sfx>](i) }
                fn state_iter_next(i: Pin<&mut Self::StateIteratorCxx>) { ffi_binding::[<state_iter_next_ $sfx>](i) }
                fn state_iter_reset(i: Pin<&mut Self::StateIteratorCxx>) { ffi_binding::[<state_iter_reset_ $sfx>](i) }

                fn create_arc_iter(f: &Self::FstCxx, s: i32) -> UniquePtr<Self::ArcIteratorCxx> { ffi_binding::[<create_arc_iter_ $sfx>](f, s) }
                fn arc_iter_done(i: &Self::ArcIteratorCxx) -> bool { ffi_binding::[<arc_iter_done_ $sfx>](i) }
                fn arc_iter_next(i: Pin<&mut Self::ArcIteratorCxx>) { ffi_binding::[<arc_iter_next_ $sfx>](i) }
                fn arc_iter_reset(i: Pin<&mut Self::ArcIteratorCxx>) { ffi_binding::[<arc_iter_reset_ $sfx>](i) }
                fn arc_iter_seek(i: Pin<&mut Self::ArcIteratorCxx>, pos: usize) { ffi_binding::[<arc_iter_seek_ $sfx>](i, pos) }
                fn arc_iter_position(i: &Self::ArcIteratorCxx) -> usize { ffi_binding::[<arc_iter_position_ $sfx>](i) }
                fn arc_iter_flags(i: &Self::ArcIteratorCxx) -> u8 { ffi_binding::[<arc_iter_flags_ $sfx>](i) }
                fn arc_iter_set_flags(i: Pin<&mut Self::ArcIteratorCxx>, f: u8, m: u8) { ffi_binding::[<arc_iter_set_flags_ $sfx>](i, f, m) }
                unsafe fn arc_iter_advance(i: Pin<&mut Self::ArcIteratorCxx>, il: *mut i32, ol: *mut i32, w: *mut $wtype, ns: *mut i32) -> bool { unsafe { ffi_binding::[<arc_iter_advance_ $sfx>](i, il, ol, w, ns) }}

                fn fst_set_start(f: Pin<&mut Self::MutableFstCxx>, s: i32) { ffi_binding::[<fst_set_start_ $sfx>](f, s) }
                fn fst_set_final(f: Pin<&mut Self::MutableFstCxx>, s: i32, w: $wtype) { ffi_binding::[<fst_set_final_ $sfx>](f, s, w) }
                fn fst_set_properties(f: Pin<&mut Self::MutableFstCxx>, p: u64, m: u64) { ffi_binding::[<fst_set_properties_ $sfx>](f, p, m) }
                fn fst_add_state(f: Pin<&mut Self::MutableFstCxx>) -> i32 { ffi_binding::[<fst_add_state_ $sfx>](f) }
                fn fst_add_states(f: Pin<&mut Self::MutableFstCxx>, n: usize) { ffi_binding::[<fst_add_states_ $sfx>](f, n) }
                fn fst_add_arc(f: Pin<&mut Self::MutableFstCxx>, s: i32, il: i32, ol: i32, w: $wtype, ns: i32) { ffi_binding::[<fst_add_arc_ $sfx>](f, s, il, ol, w, ns) }
                fn fst_delete_arcs(f: Pin<&mut Self::MutableFstCxx>, s: i32) { ffi_binding::[<fst_delete_arcs_ $sfx>](f, s) }
                fn fst_delete_arcs_n(f: Pin<&mut Self::MutableFstCxx>, s: i32, n: usize) { ffi_binding::[<fst_delete_arcs_n_ $sfx>](f, s, n) }
                fn fst_delete_all_states(f: Pin<&mut Self::MutableFstCxx>) { ffi_binding::[<fst_delete_all_states_ $sfx>](f) }
                fn fst_delete_states(f: Pin<&mut Self::MutableFstCxx>, states: &[i32]) { ffi_binding::[<fst_delete_states_ $sfx>](f, states) }
                fn fst_reserve_states(f: Pin<&mut Self::MutableFstCxx>, n: usize) { ffi_binding::[<fst_reserve_states_ $sfx>](f, n) }
                fn fst_reserve_arcs(f: Pin<&mut Self::MutableFstCxx>, s: i32, n: usize) { ffi_binding::[<fst_reserve_arcs_ $sfx>](f, s, n) }
                unsafe fn fst_set_input_symbols(f: Pin<&mut Self::MutableFstCxx>, syms: *const CxxSymbolTable) { unsafe { ffi_binding::[<fst_set_input_symbols_ $sfx>](f, syms) } }
                unsafe fn fst_set_output_symbols(f: Pin<&mut Self::MutableFstCxx>, syms: *const CxxSymbolTable) { unsafe { ffi_binding::[<fst_set_output_symbols_ $sfx>](f, syms) } }
            }
        }
    };
}

crate::for_each_arc_type!(bind_fst_ffi_impl);
