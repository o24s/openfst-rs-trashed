use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use openfst_rs_macros::template;
use std::pin::Pin;

pub trait ProjectFfi: Arc<StateId = i32, Label = i32> + FstFfi
where
    Self::Weight: WeightFfi,
{
    fn fst_project_destructive(fst: Pin<&mut Self::MutableFstCxx>, project_type: i32);
    fn fst_project_non_destructive(
        ifst: &Self::FstCxx,
        ofst: Pin<&mut Self::MutableFstCxx>,
        project_type: i32,
    );
}

#[template("project")]
pub mod ffi_binding {
    unsafe extern "C++" {
        include!("cpp/project.h");

        #[expand(for_each_arc_type)]
        type Fst___SFX__ = crate::ffi::fst::ffi_binding::Fst___SFX__;
        #[expand(for_each_arc_type)]
        type MutableFst___SFX__ = crate::ffi::fst::ffi_binding::MutableFst___SFX__;

        #[expand(for_each_arc_type)]
        fn fst_project_destructive___SFX__(fst: Pin<&mut MutableFst___SFX__>, project_type: i32);
        #[expand(for_each_arc_type)]
        fn fst_project_non_destructive___SFX__(
            ifst: &Fst___SFX__,
            ofst: Pin<&mut MutableFst___SFX__>,
            project_type: i32,
        );
    }
}

macro_rules! bind_project_ffi_impl {
    ($arc:ty, $wtype:ty, $sfx:ident) => {
        pastey::paste! {
            impl ProjectFfi for $arc {
                fn fst_project_destructive(fst: Pin<&mut Self::MutableFstCxx>, project_type: i32) {
                    ffi_binding::[<fst_project_destructive_ $sfx>](fst, project_type);
                }
                fn fst_project_non_destructive(
                    ifst: &Self::FstCxx,
                    ofst: Pin<&mut Self::MutableFstCxx>,
                    project_type: i32,
                ) {
                    ffi_binding::[<fst_project_non_destructive_ $sfx>](ifst, ofst, project_type);
                }
            }
        }
    };
}

crate::for_each_arc_type!(bind_project_ffi_impl);
