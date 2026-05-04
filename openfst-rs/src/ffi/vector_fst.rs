use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::expectation_weight::*;
use crate::ffi::fst::FstFfi;
use crate::ffi::lexicographic_weight::*;
use crate::ffi::power_weight::{PowerWeightValueF32_3, PowerWeightValueF64_3};
use crate::ffi::sparse_power_weight::*;
use crate::ffi::weight::{PairWeightValueF32, PairWeightValueF64};
use cxx::{UniquePtr, memory::UniquePtrTarget};
use openfst_rs_macros::template;
use std::pin::Pin;

pub trait VectorFstFfi: Arc<StateId = i32, Label = i32> + FstFfi
where
    Self::Weight: WeightFfi,
{
    type VectorFstCxx: UniquePtrTarget;

    fn create_vector_fst() -> UniquePtr<Self::VectorFstCxx>;
    fn copy_vector_fst(fst: &Self::VectorFstCxx) -> UniquePtr<Self::VectorFstCxx>;
    fn read_vector_fst(source: &str) -> UniquePtr<Self::VectorFstCxx>;
    fn write_vector_fst(fst: &Self::VectorFstCxx, source: &str) -> bool;

    fn vector_fst_as_fst(fst: &Self::VectorFstCxx) -> &Self::FstCxx;
    fn vector_fst_as_mutable_fst(
        fst: Pin<&mut Self::VectorFstCxx>,
    ) -> Pin<&mut Self::MutableFstCxx>;

    unsafe fn vector_fst_get_arc(
        fst: &Self::VectorFstCxx,
        s: i32,
        n: usize,
        il: *mut i32,
        ol: *mut i32,
        w: *mut <Self::Weight as WeightFfi>::ValueType,
        ns: *mut i32,
    );
    fn vector_fst_set_arc(
        fst: Pin<&mut Self::VectorFstCxx>,
        s: i32,
        n: usize,
        il: i32,
        ol: i32,
        w: <Self::Weight as WeightFfi>::ValueType,
        ns: i32,
    );
    fn vector_fst_arc_iter_set_flags(fst: &Self::VectorFstCxx, s: i32, flags: u8, mask: u8);
}

#[template("vector_fst")]
pub mod ffi_binding {
    unsafe extern "C++" {
        include!("cpp/vector-fst.h");

        #[expand(for_each_arc_type)]
        type Fst___SFX__ = crate::ffi::fst::ffi_binding::Fst___SFX__;
        #[expand(for_each_arc_type)]
        type MutableFst___SFX__ = crate::ffi::fst::ffi_binding::MutableFst___SFX__;

        #[expand(for_each_arc_type)]
        type VectorFst___SFX__;

        #[expand(for_each_arc_type)]
        fn create_vector_fst___SFX__() -> UniquePtr<VectorFst___SFX__>;
        #[expand(for_each_arc_type)]
        fn copy_vector_fst___SFX__(f: &VectorFst___SFX__) -> UniquePtr<VectorFst___SFX__>;
        #[expand(for_each_arc_type)]
        fn read_vector_fst___SFX__(s: &str) -> UniquePtr<VectorFst___SFX__>;
        #[expand(for_each_arc_type)]
        fn write_vector_fst___SFX__(f: &VectorFst___SFX__, s: &str) -> bool;
        #[expand(for_each_arc_type)]
        fn vector_fst_as_fst___SFX__(f: &VectorFst___SFX__) -> &Fst___SFX__;
        #[expand(for_each_arc_type)]
        fn vector_fst_as_mutable_fst___SFX__(
            f: Pin<&mut VectorFst___SFX__>,
        ) -> Pin<&mut MutableFst___SFX__>;

        #[expand(for_each_arc_type)]
        unsafe fn vector_fst_get_arc___SFX__(
            f: &VectorFst___SFX__,
            s: i32,
            n: usize,
            il: *mut i32,
            ol: *mut i32,
            w: *mut __WTYPE__,
            ns: *mut i32,
        );
        #[expand(for_each_arc_type)]
        fn vector_fst_set_arc___SFX__(
            f: Pin<&mut VectorFst___SFX__>,
            s: i32,
            n: usize,
            il: i32,
            ol: i32,
            w: __WTYPE__,
            ns: i32,
        );
        #[expand(for_each_arc_type)]
        fn vector_fst_arc_iter_set_flags___SFX__(
            f: &VectorFst___SFX__,
            s: i32,
            flags: u8,
            mask: u8,
        );
    }
}

macro_rules! bind_vector_fst_ffi_impl {
    ($arc:ty, $wtype:ty, $sfx:ident) => {
        pastey::paste! {
            impl VectorFstFfi for $arc {
                type VectorFstCxx = ffi_binding::[<VectorFst_ $sfx>];

                fn create_vector_fst() -> UniquePtr<Self::VectorFstCxx> { ffi_binding::[<create_vector_fst_ $sfx>]() }
                fn copy_vector_fst(f: &Self::VectorFstCxx) -> UniquePtr<Self::VectorFstCxx> { ffi_binding::[<copy_vector_fst_ $sfx>](f) }
                fn read_vector_fst(s: &str) -> UniquePtr<Self::VectorFstCxx> { ffi_binding::[<read_vector_fst_ $sfx>](s) }
                fn write_vector_fst(f: &Self::VectorFstCxx, s: &str) -> bool { ffi_binding::[<write_vector_fst_ $sfx>](f, s) }

                fn vector_fst_as_fst(f: &Self::VectorFstCxx) -> &Self::FstCxx { ffi_binding::[<vector_fst_as_fst_ $sfx>](f) }
                fn vector_fst_as_mutable_fst(f: Pin<&mut Self::VectorFstCxx>) -> Pin<&mut Self::MutableFstCxx> { ffi_binding::[<vector_fst_as_mutable_fst_ $sfx>](f) }

                unsafe fn vector_fst_get_arc(f: &Self::VectorFstCxx, s: i32, n: usize, il: *mut i32, ol: *mut i32, w: *mut $wtype, ns: *mut i32) {
                    unsafe { ffi_binding::[<vector_fst_get_arc_ $sfx>](f, s, n, il, ol, w, ns) }
                }
                fn vector_fst_set_arc(f: Pin<&mut Self::VectorFstCxx>, s: i32, n: usize, il: i32, ol: i32, w: $wtype, ns: i32) {
                    ffi_binding::[<vector_fst_set_arc_ $sfx>](f, s, n, il, ol, w, ns)
                }
                fn vector_fst_arc_iter_set_flags(f: &Self::VectorFstCxx, s: i32, flags: u8, mask: u8) {
                    ffi_binding::[<vector_fst_arc_iter_set_flags_ $sfx>](f, s, flags, mask)
                }
            }
        }
    };
}

crate::for_each_arc_type!(bind_vector_fst_ffi_impl);
