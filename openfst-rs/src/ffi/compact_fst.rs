use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use cxx::{UniquePtr, memory::UniquePtrTarget};
use openfst_rs_macros::template;

pub trait CompactFstFfi<Compactor>: Arc<StateId = i32, Label = i32> + FstFfi
where
    Self::Weight: WeightFfi,
{
    type CompactFstCxx: UniquePtrTarget;

    fn read_compact_fst(source: &str) -> UniquePtr<Self::CompactFstCxx>;
    fn create_compact_fst_from_fst(fst: &Self::FstCxx) -> UniquePtr<Self::CompactFstCxx>;
    fn write_compact_fst(fst: &Self::CompactFstCxx, source: &str) -> bool;
    fn compact_fst_as_fst(fst: &Self::CompactFstCxx) -> &Self::FstCxx;
}

// Zero-sized marker types to distinguish compactor implementations at compile time.
pub struct StringCompactor;
pub struct WeightedStringCompactor;
pub struct AcceptorCompactor;
pub struct UnweightedCompactor;
pub struct UnweightedAcceptorCompactor;

#[template("compact_fst")]
pub mod ffi_binding {
    unsafe extern "C++" {
        include!("cpp/compact-fst.h");

        #[expand(for_each_arc_type)]
        type Fst___SFX__ = crate::ffi::fst::ffi_binding::Fst___SFX__;

        // String
        #[expand(for_each_arc_type)]
        type CompactStringFst___SFX__;
        #[expand(for_each_arc_type)]
        fn read_compact_String_fst___SFX__(s: &str) -> UniquePtr<CompactStringFst___SFX__>;
        #[expand(for_each_arc_type)]
        fn create_compact_String_fst_from_fst___SFX__(
            fst: &Fst___SFX__,
        ) -> UniquePtr<CompactStringFst___SFX__>;
        #[expand(for_each_arc_type)]
        fn write_compact_String_fst___SFX__(f: &CompactStringFst___SFX__, s: &str) -> bool;
        #[expand(for_each_arc_type)]
        fn compact_String_fst_as_fst___SFX__(f: &CompactStringFst___SFX__) -> &Fst___SFX__;

        // WeightedString
        #[expand(for_each_arc_type)]
        type CompactWeightedStringFst___SFX__;
        #[expand(for_each_arc_type)]
        fn read_compact_WeightedString_fst___SFX__(
            s: &str,
        ) -> UniquePtr<CompactWeightedStringFst___SFX__>;
        #[expand(for_each_arc_type)]
        fn create_compact_WeightedString_fst_from_fst___SFX__(
            fst: &Fst___SFX__,
        ) -> UniquePtr<CompactWeightedStringFst___SFX__>;
        #[expand(for_each_arc_type)]
        fn write_compact_WeightedString_fst___SFX__(
            f: &CompactWeightedStringFst___SFX__,
            s: &str,
        ) -> bool;
        #[expand(for_each_arc_type)]
        fn compact_WeightedString_fst_as_fst___SFX__(
            f: &CompactWeightedStringFst___SFX__,
        ) -> &Fst___SFX__;

        // Acceptor
        #[expand(for_each_arc_type)]
        type CompactAcceptorFst___SFX__;
        #[expand(for_each_arc_type)]
        fn read_compact_Acceptor_fst___SFX__(s: &str) -> UniquePtr<CompactAcceptorFst___SFX__>;
        #[expand(for_each_arc_type)]
        fn create_compact_Acceptor_fst_from_fst___SFX__(
            fst: &Fst___SFX__,
        ) -> UniquePtr<CompactAcceptorFst___SFX__>;
        #[expand(for_each_arc_type)]
        fn write_compact_Acceptor_fst___SFX__(f: &CompactAcceptorFst___SFX__, s: &str) -> bool;
        #[expand(for_each_arc_type)]
        fn compact_Acceptor_fst_as_fst___SFX__(f: &CompactAcceptorFst___SFX__) -> &Fst___SFX__;

        // Unweighted
        #[expand(for_each_arc_type)]
        type CompactUnweightedFst___SFX__;
        #[expand(for_each_arc_type)]
        fn read_compact_Unweighted_fst___SFX__(s: &str) -> UniquePtr<CompactUnweightedFst___SFX__>;
        #[expand(for_each_arc_type)]
        fn create_compact_Unweighted_fst_from_fst___SFX__(
            fst: &Fst___SFX__,
        ) -> UniquePtr<CompactUnweightedFst___SFX__>;
        #[expand(for_each_arc_type)]
        fn write_compact_Unweighted_fst___SFX__(f: &CompactUnweightedFst___SFX__, s: &str) -> bool;
        #[expand(for_each_arc_type)]
        fn compact_Unweighted_fst_as_fst___SFX__(f: &CompactUnweightedFst___SFX__) -> &Fst___SFX__;

        // UnweightedAcceptor
        #[expand(for_each_arc_type)]
        type CompactUnweightedAcceptorFst___SFX__;
        #[expand(for_each_arc_type)]
        fn read_compact_UnweightedAcceptor_fst___SFX__(
            s: &str,
        ) -> UniquePtr<CompactUnweightedAcceptorFst___SFX__>;
        #[expand(for_each_arc_type)]
        fn create_compact_UnweightedAcceptor_fst_from_fst___SFX__(
            fst: &Fst___SFX__,
        ) -> UniquePtr<CompactUnweightedAcceptorFst___SFX__>;
        #[expand(for_each_arc_type)]
        fn write_compact_UnweightedAcceptor_fst___SFX__(
            f: &CompactUnweightedAcceptorFst___SFX__,
            s: &str,
        ) -> bool;
        #[expand(for_each_arc_type)]
        fn compact_UnweightedAcceptor_fst_as_fst___SFX__(
            f: &CompactUnweightedAcceptorFst___SFX__,
        ) -> &Fst___SFX__;

        fn init_compact_fst_registry();
    }
}

macro_rules! bind_compact_fst_ffi_impl {
    ($compactor:ident, $compactor_str:ident) => {
        macro_rules! inner_impl {
            ($arc:ty, $wtype:ty, $sfx:ident) => {
                pastey::paste! {
                    impl CompactFstFfi<$compactor> for $arc {
                        type CompactFstCxx = ffi_binding::[<Compact $compactor_str Fst_ $sfx>];

                        fn read_compact_fst(s: &str) -> UniquePtr<Self::CompactFstCxx> {
                            ffi_binding::[<read_compact_ $compactor_str _fst_ $sfx>](s)
                        }
                        fn create_compact_fst_from_fst(fst: &Self::FstCxx) -> UniquePtr<Self::CompactFstCxx> {
                            ffi_binding::[<create_compact_ $compactor_str _fst_from_fst_ $sfx>](fst)
                        }
                        fn write_compact_fst(f: &Self::CompactFstCxx, s: &str) -> bool {
                            ffi_binding::[<write_compact_ $compactor_str _fst_ $sfx>](f, s)
                        }
                        fn compact_fst_as_fst(f: &Self::CompactFstCxx) -> &Self::FstCxx {
                            ffi_binding::[<compact_ $compactor_str _fst_as_fst_ $sfx>](f)
                        }
                    }
                }
            };
        }
        crate::for_each_arc_type!(inner_impl);
    };
}

bind_compact_fst_ffi_impl!(StringCompactor, String);
bind_compact_fst_ffi_impl!(WeightedStringCompactor, WeightedString);
bind_compact_fst_ffi_impl!(AcceptorCompactor, Acceptor);
bind_compact_fst_ffi_impl!(UnweightedCompactor, Unweighted);
bind_compact_fst_ffi_impl!(UnweightedAcceptorCompactor, UnweightedAcceptor);

pub fn init_compact_fst_registry() {
    static INIT: std::sync::OnceLock<()> = std::sync::OnceLock::new();
    INIT.get_or_init(|| {
        ffi_binding::init_compact_fst_registry();
    });
}
