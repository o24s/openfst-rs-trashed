use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use cxx::{UniquePtr, memory::UniquePtrTarget};
use openfst_rs_macros::template;
use std::pin::Pin;

pub trait ComposeFfi: Arc<StateId = i32, Label = i32> + FstFfi
where
    Self::Weight: WeightFfi,
{
    type ComposeFstCxx: UniquePtrTarget;
    fn fst_compose(
        f1: &Self::FstCxx,
        f2: &Self::FstCxx,
        ofst: Pin<&mut Self::MutableFstCxx>,
        conn: bool,
        ft: i32,
    );
    fn create_compose_fst(
        f1: &Self::FstCxx,
        f2: &Self::FstCxx,
        gc: bool,
        gc_limit: usize,
    ) -> UniquePtr<Self::ComposeFstCxx>;
    fn compose_fst_as_fst(fst: &Self::ComposeFstCxx) -> &Self::FstCxx;
}

#[template("compose")]
pub mod ffi_binding {
    unsafe extern "C++" {
        include!("cpp/compose.h");

        #[expand(for_each_arc_type)]
        type Fst___SFX__ = crate::ffi::fst::ffi_binding::Fst___SFX__;
        #[expand(for_each_arc_type)]
        type MutableFst___SFX__ = crate::ffi::fst::ffi_binding::MutableFst___SFX__;

        #[expand(for_each_arc_type)]
        type ComposeFst___SFX__;

        #[expand(for_each_arc_type)]
        fn fst_compose___SFX__(
            f1: &Fst___SFX__,
            f2: &Fst___SFX__,
            ofst: Pin<&mut MutableFst___SFX__>,
            conn: bool,
            ft: i32,
        );
        #[expand(for_each_arc_type)]
        fn create_compose_fst___SFX__(
            f1: &Fst___SFX__,
            f2: &Fst___SFX__,
            gc: bool,
            gc_limit: usize,
        ) -> UniquePtr<ComposeFst___SFX__>;
        #[expand(for_each_arc_type)]
        fn compose_fst_as_fst___SFX__(fst: &ComposeFst___SFX__) -> &Fst___SFX__;
    }
}

macro_rules! bind_compose_ffi_impl {
    ($arc:ty, $wtype:ty, $sfx:ident) => {
        pastey::paste! {
            impl ComposeFfi for $arc {
                type ComposeFstCxx = ffi_binding::[<ComposeFst_ $sfx>];

                fn fst_compose(f1: &Self::FstCxx, f2: &Self::FstCxx, ofst: Pin<&mut Self::MutableFstCxx>, conn: bool, ft: i32) {
                    ffi_binding::[<fst_compose_ $sfx>](f1, f2, ofst, conn, ft)
                }
                fn create_compose_fst(f1: &Self::FstCxx, f2: &Self::FstCxx, gc: bool, gc_limit: usize) -> UniquePtr<Self::ComposeFstCxx> {
                    ffi_binding::[<create_compose_fst_ $sfx>](f1, f2, gc, gc_limit)
                }
                fn compose_fst_as_fst(f: &Self::ComposeFstCxx) -> &Self::FstCxx {
                    ffi_binding::[<compose_fst_as_fst_ $sfx>](f)
                }
            }
        }
    };
}

crate::for_each_arc_type!(bind_compose_ffi_impl);
