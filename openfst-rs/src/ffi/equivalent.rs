use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use openfst_rs_macros::template;

pub trait EquivalentFfi: Arc<StateId = i32, Label = i32> + FstFfi
where
    Self::Weight: WeightFfi,
{
    fn fst_randequivalent(
        fst1: &Self::FstCxx,
        fst2: &Self::FstCxx,
        npath: i32,
        delta: f32,
        seed: u64,
        max_length: i32,
    ) -> Result<bool, cxx::Exception>;

    fn fst_isomorphic(fst1: &Self::FstCxx, fst2: &Self::FstCxx, delta: f32) -> bool;

    fn fst_equal(fst1: &Self::FstCxx, fst2: &Self::FstCxx, delta: f32, etype: u8) -> bool;

    fn fst_equivalent(
        fst1: &Self::FstCxx,
        fst2: &Self::FstCxx,
        delta: f32,
    ) -> Result<bool, cxx::Exception>;
}

#[template("equivalent")]
pub mod ffi_binding {
    unsafe extern "C++" {
        include!("cpp/randequivalent.h");
        include!("cpp/isomorphic.h");
        include!("cpp/equal.h");
        include!("cpp/equivalent.h");

        #[expand(for_each_arc_type)]
        type Fst___SFX__ = crate::ffi::fst::ffi_binding::Fst___SFX__;

        #[expand(for_each_arc_type)]
        fn fst_randequivalent___SFX__(
            fst1: &Fst___SFX__,
            fst2: &Fst___SFX__,
            npath: i32,
            delta: f32,
            seed: u64,
            max_length: i32,
        ) -> Result<bool>;

        #[expand(for_each_arc_type)]
        fn fst_isomorphic___SFX__(fst1: &Fst___SFX__, fst2: &Fst___SFX__, delta: f32) -> bool;

        #[expand(for_each_arc_type)]
        fn fst_equal___SFX__(fst1: &Fst___SFX__, fst2: &Fst___SFX__, delta: f32, etype: u8)
        -> bool;

        #[expand(for_each_arc_type)]
        fn fst_equivalent___SFX__(
            fst1: &Fst___SFX__,
            fst2: &Fst___SFX__,
            delta: f32,
        ) -> Result<bool>;
    }
}

macro_rules! bind_equivalent_ffi_impl {
    ($arc:ty, $wtype:ty, $sfx:ident) => {
        pastey::paste! {
            impl EquivalentFfi for $arc {
                fn fst_randequivalent(
                    fst1: &Self::FstCxx, fst2: &Self::FstCxx, npath: i32, delta: f32, seed: u64, max_length: i32,
                ) -> Result<bool, cxx::Exception> {
                    ffi_binding::[<fst_randequivalent_ $sfx>](fst1, fst2, npath, delta, seed, max_length)
                }

                fn fst_isomorphic(fst1: &Self::FstCxx, fst2: &Self::FstCxx, delta: f32) -> bool {
                    ffi_binding::[<fst_isomorphic_ $sfx>](fst1, fst2, delta)
                }

                fn fst_equal(fst1: &Self::FstCxx, fst2: &Self::FstCxx, delta: f32, etype: u8) -> bool {
                    ffi_binding::[<fst_equal_ $sfx>](fst1, fst2, delta, etype)
                }

                fn fst_equivalent(fst1: &Self::FstCxx, fst2: &Self::FstCxx, delta: f32) -> Result<bool, cxx::Exception> {
                    ffi_binding::[<fst_equivalent_ $sfx>](fst1, fst2, delta)
                }
            }
        }
    };
}

crate::for_each_arc_type!(bind_equivalent_ffi_impl);
