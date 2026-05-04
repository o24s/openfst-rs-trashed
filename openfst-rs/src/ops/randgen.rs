use std::time::{SystemTime, UNIX_EPOCH};

use crate::arc::Arc;
use crate::error::OpenFstError;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
pub use crate::ffi::randgen::ArcSelectorType;
use crate::ffi::randgen::RandGenFfi;
use crate::fst::{AsFstCxx, AsMutFstCxx, Fst, MutableFst};

#[derive(Debug, Clone)]
pub struct RandGenOptions {
    /// Strategy to choose a random transition.
    pub selector_type: ArcSelectorType,
    /// Seed for the random number generator. If set to 0, a random seed derived from system time is used.
    pub seed: u64,
    /// Maximum length of generated paths.
    pub max_length: i32,
    /// Number of paths to generate.
    pub npath: i32,
    /// If true, the output tree is weighted by path count. Otherwise, it is an unweighted DAG.
    pub weighted: bool,
    /// Remove total weight when output is weighted?
    pub remove_total_weight: bool,
}

impl Default for RandGenOptions {
    fn default() -> Self {
        Self {
            selector_type: ArcSelectorType::Uniform,
            seed: 0, // Special value mapped to SystemTime locally to simulate true randomness.
            max_length: i32::MAX,
            npath: 1,
            weighted: false,
            remove_total_weight: false,
        }
    }
}

pub trait RandGen<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + RandGenFfi,
    A::Weight: WeightFfi,
{
    /// Randomly generates paths through the input FST and stores the result into this MutableFst.
    fn randgen_of<F>(&mut self, ifst: &F, opts: &RandGenOptions) -> Result<(), OpenFstError>
    where
        F: Fst<A> + AsFstCxx<A>;
}

impl<M, A> RandGen<A> for M
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + RandGenFfi,
    A::Weight: WeightFfi,
    M: MutableFst<A> + AsMutFstCxx<A>,
{
    fn randgen_of<F>(&mut self, ifst: &F, opts: &RandGenOptions) -> Result<(), OpenFstError>
    where
        F: Fst<A> + AsFstCxx<A>,
    {
        // Generate a true random seed if the user provided 0.
        let seed = if opts.seed == 0 {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos() as u64
        } else {
            opts.seed
        };

        A::fst_randgen(
            unsafe { ifst.as_fst_cxx() },
            unsafe { self.as_mut_fst_cxx() },
            opts.selector_type,
            seed,
            opts.max_length,
            opts.npath,
            opts.weighted,
            opts.remove_total_weight,
        )?;
        Ok(())
    }
}
