use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use openfst_rs_macros::template;
use std::pin::Pin;

pub trait RelabelFfi: Arc<StateId = i32, Label = i32> + FstFfi
where
    Self::Weight: WeightFfi,
{
    fn fst_relabel(
        fst: Pin<&mut Self::MutableFstCxx>,
        ipairs_flat: &[i32],
        opairs_flat: &[i32],
    ) -> Result<(), cxx::Exception>;
}

#[template("relabel")]
pub mod ffi_binding {
    unsafe extern "C++" {
        include!("cpp/relabel.h");

        #[expand(for_each_arc_type)]
        type MutableFst___SFX__ = crate::ffi::fst::ffi_binding::MutableFst___SFX__;

        #[expand(for_each_arc_type)]
        fn fst_relabel___SFX__(
            fst: Pin<&mut MutableFst___SFX__>,
            ipairs_flat: &[i32],
            opairs_flat: &[i32],
        ) -> Result<()>;
    }
}

macro_rules! bind_relabel_ffi_impl {
    ($arc:ty, $wtype:ty, $sfx:ident) => {
        pastey::paste! {
            impl RelabelFfi for $arc {
                fn fst_relabel(fst: Pin<&mut Self::MutableFstCxx>, ipairs_flat: &[i32], opairs_flat: &[i32]) -> Result<(), cxx::Exception> {
                    ffi_binding::[<fst_relabel_ $sfx>](fst, ipairs_flat, opairs_flat)
                }
            }
        }
    };
}

crate::for_each_arc_type!(bind_relabel_ffi_impl);
