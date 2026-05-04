use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use cxx::{UniquePtr, memory::UniquePtrTarget};
use openfst_rs_macros::template;
use std::pin::Pin;

pub trait ArcSortFfi: Arc<StateId = i32, Label = i32> + FstFfi
where
    Self::Weight: WeightFfi,
{
    type ILabelArcSortFstCxx: UniquePtrTarget;
    type OLabelArcSortFstCxx: UniquePtrTarget;

    fn arcsort_ilabel(fst: Pin<&mut Self::MutableFstCxx>);
    fn arcsort_olabel(fst: Pin<&mut Self::MutableFstCxx>);
    fn create_ilabel_arcsort_fst(fst: &Self::FstCxx) -> UniquePtr<Self::ILabelArcSortFstCxx>;
    fn create_olabel_arcsort_fst(fst: &Self::FstCxx) -> UniquePtr<Self::OLabelArcSortFstCxx>;
    fn ilabel_arcsort_fst_as_fst(fst: &Self::ILabelArcSortFstCxx) -> &Self::FstCxx;
    fn olabel_arcsort_fst_as_fst(fst: &Self::OLabelArcSortFstCxx) -> &Self::FstCxx;
}

#[template("arcsort")]
pub mod ffi_binding {
    unsafe extern "C++" {
        include!("cpp/arcsort.h");

        #[expand(for_each_arc_type)]
        type Fst___SFX__ = crate::ffi::fst::ffi_binding::Fst___SFX__;
        #[expand(for_each_arc_type)]
        type MutableFst___SFX__ = crate::ffi::fst::ffi_binding::MutableFst___SFX__;

        #[expand(for_each_arc_type)]
        type ILabelArcSortFst___SFX__;
        #[expand(for_each_arc_type)]
        type OLabelArcSortFst___SFX__;

        #[expand(for_each_arc_type)]
        fn arcsort_ilabel___SFX__(f: Pin<&mut MutableFst___SFX__>);
        #[expand(for_each_arc_type)]
        fn arcsort_olabel___SFX__(f: Pin<&mut MutableFst___SFX__>);
        #[expand(for_each_arc_type)]
        fn create_ilabel_arcsort_fst___SFX__(
            f: &Fst___SFX__,
        ) -> UniquePtr<ILabelArcSortFst___SFX__>;
        #[expand(for_each_arc_type)]
        fn create_olabel_arcsort_fst___SFX__(
            f: &Fst___SFX__,
        ) -> UniquePtr<OLabelArcSortFst___SFX__>;
        #[expand(for_each_arc_type)]
        fn ilabel_arcsort_fst_as_fst___SFX__(f: &ILabelArcSortFst___SFX__) -> &Fst___SFX__;
        #[expand(for_each_arc_type)]
        fn olabel_arcsort_fst_as_fst___SFX__(f: &OLabelArcSortFst___SFX__) -> &Fst___SFX__;
    }
}

macro_rules! bind_arcsort_ffi_impl {
    ($arc:ty, $wtype:ty, $sfx:ident) => {
        pastey::paste! {
            impl ArcSortFfi for $arc {
                type ILabelArcSortFstCxx = ffi_binding::[<ILabelArcSortFst_ $sfx>];
                type OLabelArcSortFstCxx = ffi_binding::[<OLabelArcSortFst_ $sfx>];

                fn arcsort_ilabel(f: Pin<&mut Self::MutableFstCxx>) { ffi_binding::[<arcsort_ilabel_ $sfx>](f) }
                fn arcsort_olabel(f: Pin<&mut Self::MutableFstCxx>) { ffi_binding::[<arcsort_olabel_ $sfx>](f) }
                fn create_ilabel_arcsort_fst(f: &Self::FstCxx) -> UniquePtr<Self::ILabelArcSortFstCxx> { ffi_binding::[<create_ilabel_arcsort_fst_ $sfx>](f) }
                fn create_olabel_arcsort_fst(f: &Self::FstCxx) -> UniquePtr<Self::OLabelArcSortFstCxx> { ffi_binding::[<create_olabel_arcsort_fst_ $sfx>](f) }
                fn ilabel_arcsort_fst_as_fst(f: &Self::ILabelArcSortFstCxx) -> &Self::FstCxx { ffi_binding::[<ilabel_arcsort_fst_as_fst_ $sfx>](f) }
                fn olabel_arcsort_fst_as_fst(f: &Self::OLabelArcSortFstCxx) -> &Self::FstCxx { ffi_binding::[<olabel_arcsort_fst_as_fst_ $sfx>](f) }
            }
        }
    };
}

crate::for_each_arc_type!(bind_arcsort_ffi_impl);
