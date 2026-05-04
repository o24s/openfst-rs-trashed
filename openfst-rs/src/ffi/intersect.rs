use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use openfst_rs_macros::template;
use std::pin::Pin;

pub trait IntersectFfi: Arc<StateId = i32, Label = i32> + FstFfi
where
    Self::Weight: WeightFfi,
{
    fn fst_intersect(
        fst1: &Self::FstCxx,
        fst2: &Self::FstCxx,
        ofst: Pin<&mut Self::MutableFstCxx>,
        connect: bool,
        filter_type: i32,
    ) -> Result<(), cxx::Exception>;
}

#[template("intersect")]
pub mod ffi_binding {
    unsafe extern "C++" {
        include!("cpp/intersect.h");

        #[expand(for_each_arc_type)]
        type Fst___SFX__ = crate::ffi::fst::ffi_binding::Fst___SFX__;
        #[expand(for_each_arc_type)]
        type MutableFst___SFX__ = crate::ffi::fst::ffi_binding::MutableFst___SFX__;

        #[expand(for_each_arc_type)]
        fn fst_intersect___SFX__(
            f1: &Fst___SFX__,
            f2: &Fst___SFX__,
            ofst: Pin<&mut MutableFst___SFX__>,
            conn: bool,
            ft: i32,
        ) -> Result<()>;
    }
}

macro_rules! bind_intersect_ffi_impl {
    ($arc:ty, $wtype:ty, $sfx:ident) => {
        pastey::paste! {
            impl IntersectFfi for $arc {
                fn fst_intersect(f1: &Self::FstCxx, f2: &Self::FstCxx, ofst: Pin<&mut Self::MutableFstCxx>, conn: bool, ft: i32) -> Result<(), cxx::Exception> {
                    ffi_binding::[<fst_intersect_ $sfx>](f1, f2, ofst, conn, ft)
                }
            }
        }
    };
}

crate::for_each_arc_type!(bind_intersect_ffi_impl);
