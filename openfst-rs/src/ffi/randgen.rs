use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use openfst_rs_macros::template;
use std::pin::Pin;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArcSelectorType {
    Uniform = 0,
    LogProb = 1,
    FastLogProb = 2,
}

pub trait RandGenFfi: Arc<StateId = i32, Label = i32> + FstFfi
where
    Self::Weight: WeightFfi,
{
    fn fst_randgen(
        ifst: &Self::FstCxx,
        ofst: Pin<&mut Self::MutableFstCxx>,
        selector_type: ArcSelectorType,
        seed: u64,
        max_length: i32,
        npath: i32,
        weighted: bool,
        remove_total_weight: bool,
    ) -> Result<(), cxx::Exception>;
}

#[template("randgen")]
pub mod ffi_binding {
    unsafe extern "C++" {
        include!("cpp/randgen.h");

        #[expand(for_each_arc_type)]
        type Fst___SFX__ = crate::ffi::fst::ffi_binding::Fst___SFX__;
        #[expand(for_each_arc_type)]
        type MutableFst___SFX__ = crate::ffi::fst::ffi_binding::MutableFst___SFX__;

        #[expand(for_each_arc_type)]
        fn fst_randgen___SFX__(
            ifst: &Fst___SFX__,
            ofst: Pin<&mut MutableFst___SFX__>,
            selector_type: i32,
            seed: u64,
            max_length: i32,
            npath: i32,
            weighted: bool,
            remove_total_weight: bool,
        ) -> Result<()>;
    }
}

macro_rules! bind_randgen_ffi_impl {
    ($arc:ty, $wtype:ty, $sfx:ident) => {
        pastey::paste! {
            impl RandGenFfi for $arc {
                fn fst_randgen(
                    ifst: &Self::FstCxx,
                    ofst: Pin<&mut Self::MutableFstCxx>,
                    selector_type: ArcSelectorType,
                    seed: u64,
                    max_length: i32,
                    npath: i32,
                    weighted: bool,
                    remove_total_weight: bool,
                ) -> Result<(), cxx::Exception> {
                    ffi_binding::[<fst_randgen_ $sfx>](
                        ifst,
                        ofst,
                        selector_type as i32,
                        seed,
                        max_length,
                        npath,
                        weighted,
                        remove_total_weight,
                    )
                }
            }
        }
    };
}

crate::for_each_arc_type!(bind_randgen_ffi_impl);
