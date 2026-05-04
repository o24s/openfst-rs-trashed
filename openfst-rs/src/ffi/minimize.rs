use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use std::pin::Pin;

use openfst_rs_macros::template;

pub trait MinimizeFfi: Arc<StateId = i32, Label = i32> + FstFfi
where
    Self::Weight: WeightFfi,
{
    unsafe fn fst_minimize(
        fst: Pin<&mut Self::MutableFstCxx>,
        sfst: *mut Self::MutableFstCxx,
        delta: f32,
        allow_nondet: bool,
    );
}

#[template("minimize")]
pub mod ffi_binding {
    unsafe extern "C++" {
        include!("cpp/minimize.h");

        #[expand(for_each_arc_type)]
        type MutableFst___SFX__ = crate::ffi::fst::ffi_binding::MutableFst___SFX__;

        #[expand(for_each_arc_type)]
        unsafe fn fst_minimize___SFX__(
            fst: Pin<&mut MutableFst___SFX__>,
            sfst: *mut MutableFst___SFX__,
            delta: f32,
            allow_nondet: bool,
        );
    }
}

macro_rules! bind_minimize_ffi_impl {
    ($arc:ty, $wtype:ty, $sfx:ident) => {
        pastey::paste! {
            impl MinimizeFfi for $arc {
                unsafe fn fst_minimize(
                    fst: Pin<&mut Self::MutableFstCxx>,
                    sfst: *mut Self::MutableFstCxx,
                    delta: f32,
                    allow_nondet: bool,
                ) {
                    unsafe {
                        ffi_binding::[<fst_minimize_ $sfx>](fst, sfst, delta, allow_nondet);
                    }
                }
            }
        }
    };
}

crate::for_each_arc_type!(bind_minimize_ffi_impl);
