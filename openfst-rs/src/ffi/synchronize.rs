use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use cxx::{UniquePtr, memory::UniquePtrTarget};
use openfst_rs_macros::template;
use std::pin::Pin;

pub trait SynchronizeFfi: Arc<StateId = i32, Label = i32> + FstFfi
where
    Self::Weight: WeightFfi,
{
    type SynchronizeFstCxx: UniquePtrTarget;

    fn create_synchronize_fst(
        ifst: &Self::FstCxx,
        gc: bool,
        gc_limit: usize,
    ) -> UniquePtr<Self::SynchronizeFstCxx>;

    fn synchronize_fst_as_fst(fst: &Self::SynchronizeFstCxx) -> &Self::FstCxx;

    fn fst_synchronize(ifst: &Self::FstCxx, ofst: Pin<&mut Self::MutableFstCxx>);
}

#[template("synchronize")]
pub mod ffi_binding {
    unsafe extern "C++" {
        include!("cpp/synchronize.h");

        #[expand(for_each_arc_type)]
        type Fst___SFX__ = crate::ffi::fst::ffi_binding::Fst___SFX__;
        #[expand(for_each_arc_type)]
        type MutableFst___SFX__ = crate::ffi::fst::ffi_binding::MutableFst___SFX__;

        #[expand(for_each_arc_type)]
        type SynchronizeFst___SFX__;

        #[expand(for_each_arc_type)]
        fn create_synchronize_fst___SFX__(
            ifst: &Fst___SFX__,
            gc: bool,
            gc_limit: usize,
        ) -> UniquePtr<SynchronizeFst___SFX__>;
        #[expand(for_each_arc_type)]
        fn synchronize_fst_as_fst___SFX__(fst: &SynchronizeFst___SFX__) -> &Fst___SFX__;
        #[expand(for_each_arc_type)]
        fn fst_synchronize___SFX__(ifst: &Fst___SFX__, ofst: Pin<&mut MutableFst___SFX__>);
    }
}

macro_rules! bind_synchronize_ffi_impl {
    ($arc:ty, $wtype:ty, $sfx:ident) => {
        pastey::paste! {
            impl SynchronizeFfi for $arc {
                type SynchronizeFstCxx = ffi_binding::[<SynchronizeFst_ $sfx>];

                fn create_synchronize_fst(ifst: &Self::FstCxx, gc: bool, gc_limit: usize) -> UniquePtr<Self::SynchronizeFstCxx> {
                    ffi_binding::[<create_synchronize_fst_ $sfx>](ifst, gc, gc_limit)
                }

                fn synchronize_fst_as_fst(fst: &Self::SynchronizeFstCxx) -> &Self::FstCxx {
                    ffi_binding::[<synchronize_fst_as_fst_ $sfx>](fst)
                }

                fn fst_synchronize(ifst: &Self::FstCxx, ofst: Pin<&mut Self::MutableFstCxx>) {
                    ffi_binding::[<fst_synchronize_ $sfx>](ifst, ofst)
                }
            }
        }
    };
}

crate::for_each_arc_type!(bind_synchronize_ffi_impl);
