use crate::arc::Arc;
use crate::error::OpenFstError;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use crate::ffi::intersect::IntersectFfi;
use crate::fst::{AsFstCxx, AsMutFstCxx, Fst, MutableFst};
use crate::ops::compose::ComposeOptions;
use crate::properties::{Acceptor, FstPropertiesExt, Verified};

pub trait Intersect<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + IntersectFfi,
    A::Weight: WeightFfi,
{
    /// Computes the intersection of two FSAs (Acceptors) using statically verified properties.
    fn intersect_verified<F1, F2>(
        &mut self,
        fst1: Verified<&F1, Acceptor>,
        fst2: Verified<&F2, Acceptor>,
        opts: &ComposeOptions,
    ) where
        F1: Fst<A> + AsFstCxx<A>,
        F2: Fst<A> + AsFstCxx<A>;

    /// Computes the intersection of two FSAs (Acceptors).
    /// Dynamically verifies properties and returns an `Err` if the FSTs are not Acceptors.
    fn intersect<F1, F2>(
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
        let v2 = fst2.verify::<Acceptor>()?;
        self.intersect_verified(v1, v2, opts);
        Ok(())
    }
}

impl<M, A> Intersect<A> for M
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + IntersectFfi,
    A::Weight: WeightFfi,
    M: MutableFst<A> + AsMutFstCxx<A>,
{
    fn intersect_verified<F1, F2>(
        &mut self,
        fst1: Verified<&F1, Acceptor>,
        fst2: Verified<&F2, Acceptor>,
        opts: &ComposeOptions,
    ) where
        F1: Fst<A> + AsFstCxx<A>,
        F2: Fst<A> + AsFstCxx<A>,
    {
        A::fst_intersect(
            unsafe { fst1.value.as_fst_cxx() },
            unsafe { fst2.value.as_fst_cxx() },
            unsafe { self.as_mut_fst_cxx() },
            opts.connect,
            opts.filter_type as i32,
        )
        .expect("fst_intersect failed despite verified properties");
    }
}
