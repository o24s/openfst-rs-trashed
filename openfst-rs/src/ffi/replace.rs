use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use openfst_rs_macros::template;
use std::pin::Pin;

pub trait ReplaceFfi: Arc<StateId = i32, Label = i32> + FstFfi
where
    Self::Weight: WeightFfi,
{
    fn fst_replace(
        labels: &[i32],
        fst_ptrs: &[usize],
        ofst: Pin<&mut Self::MutableFstCxx>,
        root: i64,
        call_label_type: i32,
        return_label_type: i32,
        return_label: i64,
    ) -> Result<(), cxx::Exception>;
}

#[template("replace")]
pub mod ffi_binding {
    unsafe extern "C++" {
        include!("cpp/replace.h");

        #[expand(for_each_arc_type)]
        type MutableFst___SFX__ = crate::ffi::fst::ffi_binding::MutableFst___SFX__;

        #[expand(for_each_arc_type)]
        fn fst_replace___SFX__(
            labels: &[i32],
            fst_ptrs: &[usize],
            ofst: Pin<&mut MutableFst___SFX__>,
            root: i64,
            call_label_type: i32,
            return_label_type: i32,
            return_label: i64,
        ) -> Result<()>;
    }
}

macro_rules! bind_replace_ffi_impl {
    ($arc:ty, $wtype:ty, $sfx:ident) => {
        pastey::paste! {
            impl ReplaceFfi for $arc {
                fn fst_replace(
                    labels: &[i32],
                    fst_ptrs: &[usize],
                    ofst: Pin<&mut Self::MutableFstCxx>,
                    root: i64,
                    call_label_type: i32,
                    return_label_type: i32,
                    return_label: i64,
                ) -> Result<(), cxx::Exception> {
                    ffi_binding::[<fst_replace_ $sfx>](
                        labels, fst_ptrs, ofst, root, call_label_type, return_label_type, return_label,
                    )
                }
            }
        }
    };
}

crate::for_each_arc_type!(bind_replace_ffi_impl);
