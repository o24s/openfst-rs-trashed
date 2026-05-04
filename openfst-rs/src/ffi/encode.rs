use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use cxx::{UniquePtr, memory::UniquePtrTarget};
use openfst_rs_macros::template;
use std::pin::Pin;

pub trait EncodeFfi: Arc<StateId = i32, Label = i32> + FstFfi
where
    Self::Weight: WeightFfi,
{
    type EncodeMapperCxx: UniquePtrTarget;

    fn create_encode_mapper(flags: u8) -> UniquePtr<Self::EncodeMapperCxx>;
    fn fst_encode(fst: Pin<&mut Self::MutableFstCxx>, mapper: Pin<&mut Self::EncodeMapperCxx>);
    fn fst_decode(fst: Pin<&mut Self::MutableFstCxx>, mapper: &Self::EncodeMapperCxx);
    fn write_encode_mapper(mapper: &Self::EncodeMapperCxx, source: &str) -> bool;
    fn read_encode_mapper(source: &str) -> UniquePtr<Self::EncodeMapperCxx>;
    fn encode_mapper_flags(mapper: &Self::EncodeMapperCxx) -> u8;
}

#[template("encode")]
pub mod ffi_binding {
    unsafe extern "C++" {
        include!("cpp/encode.h");

        #[expand(for_each_arc_type)]
        type MutableFst___SFX__ = crate::ffi::fst::ffi_binding::MutableFst___SFX__;

        #[expand(for_each_arc_type)]
        type EncodeMapper___SFX__;

        #[expand(for_each_arc_type)]
        fn create_encode_mapper___SFX__(flags: u8) -> UniquePtr<EncodeMapper___SFX__>;
        #[expand(for_each_arc_type)]
        fn fst_encode___SFX__(
            fst: Pin<&mut MutableFst___SFX__>,
            mapper: Pin<&mut EncodeMapper___SFX__>,
        );
        #[expand(for_each_arc_type)]
        fn fst_decode___SFX__(fst: Pin<&mut MutableFst___SFX__>, mapper: &EncodeMapper___SFX__);
        #[expand(for_each_arc_type)]
        fn write_encode_mapper___SFX__(mapper: &EncodeMapper___SFX__, source: &str) -> bool;
        #[expand(for_each_arc_type)]
        fn read_encode_mapper___SFX__(source: &str) -> UniquePtr<EncodeMapper___SFX__>;
        #[expand(for_each_arc_type)]
        fn encode_mapper_flags___SFX__(mapper: &EncodeMapper___SFX__) -> u8;
    }
}

macro_rules! bind_encode_ffi_impl {
    ($arc:ty, $wtype:ty, $sfx:ident) => {
        pastey::paste! {
            impl EncodeFfi for $arc {
                type EncodeMapperCxx = ffi_binding::[<EncodeMapper_ $sfx>];

                fn create_encode_mapper(flags: u8) -> UniquePtr<Self::EncodeMapperCxx> {
                    ffi_binding::[<create_encode_mapper_ $sfx>](flags)
                }

                fn fst_encode(fst: Pin<&mut Self::MutableFstCxx>, mapper: Pin<&mut Self::EncodeMapperCxx>) {
                    ffi_binding::[<fst_encode_ $sfx>](fst, mapper);
                }

                fn fst_decode(fst: Pin<&mut Self::MutableFstCxx>, mapper: &Self::EncodeMapperCxx) {
                    ffi_binding::[<fst_decode_ $sfx>](fst, mapper);
                }

                fn write_encode_mapper(mapper: &Self::EncodeMapperCxx, source: &str) -> bool {
                    ffi_binding::[<write_encode_mapper_ $sfx>](mapper, source)
                }

                fn read_encode_mapper(source: &str) -> UniquePtr<Self::EncodeMapperCxx> {
                    ffi_binding::[<read_encode_mapper_ $sfx>](source)
                }

                fn encode_mapper_flags(mapper: &Self::EncodeMapperCxx) -> u8 {
                    ffi_binding::[<encode_mapper_flags_ $sfx>](mapper)
                }
            }
        }
    };
}

crate::for_each_arc_type!(bind_encode_ffi_impl);
