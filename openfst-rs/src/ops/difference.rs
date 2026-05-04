use crate::arc::Arc;
use crate::error::OpenFstError;
use crate::ffi::WeightFfi;
use crate::ffi::difference::DifferenceFfi;
use crate::ffi::fst::FstFfi;
use crate::fst::{AsFstCxx, AsMutFstCxx, Fst, MutableFst};
use crate::ops::compose::ComposeOptions;
use crate::properties::{Acceptor, FstPropertiesExt, UnweightedDetEpsFreeAcceptor, Verified};

pub trait Difference<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + DifferenceFfi,
    A::Weight: WeightFfi,
{
    /// Computes the difference (A - B) of two FSAs using statically verified properties.
    /// Does not panic or return an error.
    fn difference_verified<F1, F2>(
        &mut self,
        fst1: Verified<&F1, Acceptor>,
        fst2: Verified<&F2, UnweightedDetEpsFreeAcceptor>,
        opts: &ComposeOptions,
    ) where
        F1: Fst<A> + AsFstCxx<A>,
        F2: Fst<A> + AsFstCxx<A>;

    /// Computes the difference (A - B) of two FSAs.
    /// Dynamically verifies properties and returns an `Err` if the FSTs are invalid.
    fn difference<F1, F2>(
        &mut self,
        fst1: &F1,
        fst2: &F2,
        opts: &ComposeOptions,
    ) -> Result<(), OpenFstError>
    where
        F1: Fst<A> + AsFstCxx<A>,
        F2: Fst<A> + AsFstCxx<A>,
    {
        let v1 = fst1.verify::<Acceptor>()?;
        let v2 = fst2.verify::<UnweightedDetEpsFreeAcceptor>()?;
        self.difference_verified(v1, v2, opts);
        Ok(())
    }
}

impl<M, A> Difference<A> for M
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + DifferenceFfi,
    A::Weight: WeightFfi,
    M: MutableFst<A> + AsMutFstCxx<A>,
{
    fn difference_verified<F1, F2>(
        &mut self,
        fst1: Verified<&F1, Acceptor>,
        fst2: Verified<&F2, UnweightedDetEpsFreeAcceptor>,
        opts: &ComposeOptions,
    ) where
        F1: Fst<A> + AsFstCxx<A>,
        F2: Fst<A> + AsFstCxx<A>,
    {
        A::fst_difference(
            unsafe { fst1.value.as_fst_cxx() },
            unsafe { fst2.value.as_fst_cxx() },
            unsafe { self.as_mut_fst_cxx() },
            opts.connect,
            opts.filter_type as i32,
        )
        .expect("fst_difference failed despite verified properties");
    }
}
