use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use crate::ffi::project::ProjectFfi;
use crate::fst::{AsFstCxx, AsMutFstCxx, Fst, MutableFst};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectType {
    Input = 1,
    Output = 2,
}

pub trait Project<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + ProjectFfi,
    A::Weight: WeightFfi,
{
    /// Projects an FST onto its domain (Input) or range (Output) in-place.
    /// This converts the transducer to an acceptor.
    fn project(&mut self, project_type: ProjectType);

    /// Computes the projection of the input FST and stores it in this FST.
    fn project_of<F>(&mut self, ifst: &F, project_type: ProjectType)
    where
        F: Fst<A> + AsFstCxx<A>;
}

impl<M, A> Project<A> for M
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + ProjectFfi,
    A::Weight: WeightFfi,
    M: MutableFst<A> + AsMutFstCxx<A>,
{
    fn project(&mut self, project_type: ProjectType) {
        A::fst_project_destructive(unsafe { self.as_mut_fst_cxx() }, project_type as i32);
    }

    fn project_of<F>(&mut self, ifst: &F, project_type: ProjectType)
    where
        F: Fst<A> + AsFstCxx<A>,
    {
        A::fst_project_non_destructive(
            unsafe { ifst.as_fst_cxx() },
            unsafe { self.as_mut_fst_cxx() },
            project_type as i32,
        );
    }
}
