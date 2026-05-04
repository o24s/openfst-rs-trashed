use crate::arc::{self, Arc};
use crate::error::OpenFstError;
use crate::ffi::fst::FstFfi;
use crate::ffi::minimize::MinimizeFfi;
use crate::ffi::{self, WeightFfi};
use crate::fst::{AsMutFstCxx, MutableFst};
use crate::properties::{Acceptor, FstPropertiesExt, Verified};
use crate::weight::DELTA;

#[derive(Debug, Clone)]
pub struct MinimizeOptions {
    pub delta: f32,
    pub allow_nondet: bool,
}

impl Default for MinimizeOptions {
    fn default() -> Self {
        Self {
            delta: DELTA,
            allow_nondet: false,
        }
    }
}

pub trait MinimizeVerified<A>
where
    A: ffi::fst::FstFfi,
    <A as arc::Arc>::Weight: ffi::WeightFfi,
{
    fn minimize_verified(&mut self, opts: &MinimizeOptions);
    fn minimize_with_sfst_verified<M2>(&mut self, sfst: &mut M2, opts: &MinimizeOptions)
    where
        M2: MutableFst<A> + AsMutFstCxx<A>;
}

impl<M, A> MinimizeVerified<A> for Verified<&mut M, Acceptor>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + MinimizeFfi,
    A::Weight: WeightFfi,
    M: MutableFst<A> + AsMutFstCxx<A>,
{
    fn minimize_verified(&mut self, opts: &MinimizeOptions) {
        unsafe {
            A::fst_minimize(
                self.value.as_mut_fst_cxx(),
                std::ptr::null_mut(),
                opts.delta,
                opts.allow_nondet,
            );
        }
    }

    fn minimize_with_sfst_verified<M2>(&mut self, sfst: &mut M2, opts: &MinimizeOptions)
    where
        M2: MutableFst<A> + AsMutFstCxx<A>,
    {
        unsafe {
            let sfst_ptr = sfst.as_mut_fst_cxx().get_unchecked_mut() as *mut A::MutableFstCxx;
            A::fst_minimize(
                self.value.as_mut_fst_cxx(),
                sfst_ptr,
                opts.delta,
                opts.allow_nondet,
            );
        }
    }
}

pub trait Minimize<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + MinimizeFfi,
    A::Weight: WeightFfi,
{
    fn minimize(&mut self, opts: &MinimizeOptions) -> Result<(), OpenFstError>;
    fn minimize_with_sfst<M2>(
        &mut self,
        sfst: &mut M2,
        opts: &MinimizeOptions,
    ) -> Result<(), OpenFstError>
    where
        M2: MutableFst<A> + AsMutFstCxx<A>;
}

impl<M, A> Minimize<A> for M
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + MinimizeFfi,
    A::Weight: WeightFfi,
    M: MutableFst<A> + AsMutFstCxx<A>,
{
    fn minimize(&mut self, opts: &MinimizeOptions) -> Result<(), OpenFstError> {
        let mut v_self = self.verify_mut::<Acceptor>()?;
        v_self.minimize_verified(opts);
        Ok(())
    }

    fn minimize_with_sfst<M2>(
        &mut self,
        sfst: &mut M2,
        opts: &MinimizeOptions,
    ) -> Result<(), OpenFstError>
    where
        M2: MutableFst<A> + AsMutFstCxx<A>,
    {
        let mut v_self = self.verify_mut::<Acceptor>()?;
        v_self.minimize_with_sfst_verified(sfst, opts);
        Ok(())
    }
}
