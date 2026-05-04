use crate::arc::Arc;
use crate::error::OpenFstError;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use crate::ffi::replace::ReplaceFfi;
use crate::fst::{AsFstCxx, AsMutFstCxx, Fst, MutableFst};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplaceLabelType {
    Neither = 1,
    Input = 2,
    Output = 3,
    Both = 4,
}

#[derive(Debug, Clone)]
pub struct ReplaceOptions {
    /// The root non-terminal label for expansion.
    pub root: i64,
    /// How to label the call arc.
    pub call_label_type: ReplaceLabelType,
    /// How to label the return arc.
    pub return_label_type: ReplaceLabelType,
    /// Specifies the label to put on the return arc.
    pub return_label: i64,
}

impl Default for ReplaceOptions {
    fn default() -> Self {
        Self {
            root: -1, // Convention for kNoLabel, though valid roots must be specified
            call_label_type: ReplaceLabelType::Input,
            return_label_type: ReplaceLabelType::Neither,
            return_label: 0,
        }
    }
}

pub trait Replace<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + ReplaceFfi,
    A::Weight: WeightFfi,
{
    /// Recursively replaces arcs in the root FST with other FSTs.
    /// The input is a list of pairs of non-terminal labels and their corresponding FSTs.
    /// The root FST is identified by the `root` label in the `opts`.
    ///
    /// The resulting expanded FST is written into `self`.
    fn replace_of<F>(
        &mut self,
        fst_pairs: &[(A::Label, &F)],
        opts: &ReplaceOptions,
    ) -> Result<(), OpenFstError>
    where
        F: Fst<A> + AsFstCxx<A>;
}

impl<M, A> Replace<A> for M
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + ReplaceFfi,
    A::Weight: WeightFfi,
    M: MutableFst<A> + AsMutFstCxx<A>,
{
    fn replace_of<F>(
        &mut self,
        fst_pairs: &[(A::Label, &F)],
        opts: &ReplaceOptions,
    ) -> Result<(), OpenFstError>
    where
        F: Fst<A> + AsFstCxx<A>,
    {
        // Split pairs into arrays of labels and raw pointers
        let labels: Vec<i32> = fst_pairs.iter().map(|(l, _)| *l).collect();

        let fst_ptrs: Vec<usize> = fst_pairs
            .iter()
            .map(|(_, fst)| unsafe { fst.as_fst_cxx() as *const A::FstCxx as usize })
            .collect();

        let mut_ofst = unsafe { self.as_mut_fst_cxx() };

        A::fst_replace(
            &labels,
            &fst_ptrs,
            mut_ofst,
            opts.root,
            opts.call_label_type as i32,
            opts.return_label_type as i32,
            opts.return_label,
        )?;

        Ok(())
    }
}
