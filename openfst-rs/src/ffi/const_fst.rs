use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use cxx::{UniquePtr, memory::UniquePtrTarget};
use openfst_rs_macros::template;

pub trait ConstFstFfi: Arc<StateId = i32, Label = i32> + FstFfi
where
    Self::Weight: WeightFfi,
{
    type ConstFstCxx: UniquePtrTarget;

    fn read_const_fst(source: &str) -> UniquePtr<Self::ConstFstCxx>;
    fn create_const_fst_from_fst(fst: &Self::FstCxx) -> UniquePtr<Self::ConstFstCxx>;
    fn write_const_fst(fst: &Self::ConstFstCxx, source: &str) -> bool;
    fn const_fst_as_fst(fst: &Self::ConstFstCxx) -> &Self::FstCxx;
}

#[template("const_fst")]
pub mod ffi_binding {
    unsafe extern "C++" {
        include!("cpp/const-fst.h");

        #[expand(for_each_arc_type)]
        type Fst___SFX__ = crate::ffi::fst::ffi_binding::Fst___SFX__;

        #[expand(for_each_arc_type)]
        type ConstFst___SFX__;

        #[expand(for_each_arc_type)]
        fn read_const_fst___SFX__(s: &str) -> UniquePtr<ConstFst___SFX__>;
        #[expand(for_each_arc_type)]
        fn create_const_fst_from_fst___SFX__(fst: &Fst___SFX__) -> UniquePtr<ConstFst___SFX__>;
        #[expand(for_each_arc_type)]
        fn write_const_fst___SFX__(f: &ConstFst___SFX__, s: &str) -> bool;
        #[expand(for_each_arc_type)]
        fn const_fst_as_fst___SFX__(f: &ConstFst___SFX__) -> &Fst___SFX__;

        fn init_const_fst_registry();
    }
}

macro_rules! bind_const_fst_ffi_impl {
    ($arc:ty, $wtype:ty, $sfx:ident) => {
        pastey::paste! {
            impl ConstFstFfi for $arc {
                type ConstFstCxx = ffi_binding::[<ConstFst_ $sfx>];

                fn read_const_fst(s: &str) -> UniquePtr<Self::ConstFstCxx> {
                    ffi_binding::[<read_const_fst_ $sfx>](s)
                }

                fn create_const_fst_from_fst(fst: &Self::FstCxx) -> UniquePtr<Self::ConstFstCxx> {
                    ffi_binding::[<create_const_fst_from_fst_ $sfx>](fst)
                }

                fn write_const_fst(f: &Self::ConstFstCxx, s: &str) -> bool {
                    ffi_binding::[<write_const_fst_ $sfx>](f, s)
                }

                fn const_fst_as_fst(f: &Self::ConstFstCxx) -> &Self::FstCxx {
                    ffi_binding::[<const_fst_as_fst_ $sfx>](f)
                }
            }
        }
    };
}

crate::for_each_arc_type!(bind_const_fst_ffi_impl);
