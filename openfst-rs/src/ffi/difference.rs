use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use openfst_rs_macros::template;
use std::pin::Pin;

pub trait DifferenceFfi: Arc<StateId = i32, Label = i32> + FstFfi
where
    Self::Weight: WeightFfi,
{
    fn fst_difference(
        fst1: &Self::FstCxx,
        fst2: &Self::FstCxx,
        ofst: Pin<&mut Self::MutableFstCxx>,
        connect: bool,
        filter_type: i32,
    ) -> Result<(), cxx::Exception>;
}

#[template("difference")]
pub mod ffi_binding {
    unsafe extern "C++" {
        include!("cpp/difference.h");

        #[expand(for_each_arc_type)]
        type Fst___SFX__ = crate::ffi::fst::ffi_binding::Fst___SFX__;
        #[expand(for_each_arc_type)]
        type MutableFst___SFX__ = crate::ffi::fst::ffi_binding::MutableFst___SFX__;

        #[expand(for_each_arc_type)]
        fn fst_difference___SFX__(
            f1: &Fst___SFX__,
            f2: &Fst___SFX__,
            ofst: Pin<&mut MutableFst___SFX__>,
            conn: bool,
            ft: i32,
        ) -> Result<()>;
    }
}

macro_rules! bind_difference_ffi_impl {
    ($arc:ty, $wtype:ty, $sfx:ident) => {
        pastey::paste! {
            impl DifferenceFfi for $arc {
                fn fst_difference(f1: &Self::FstCxx, f2: &Self::FstCxx, ofst: Pin<&mut Self::MutableFstCxx>, conn: bool, ft: i32) -> Result<(), cxx::Exception> {
                    ffi_binding::[<fst_difference_ $sfx>](f1, f2, ofst, conn, ft)
                }
            }
        }
    };
}

crate::for_each_arc_type!(bind_difference_ffi_impl);
