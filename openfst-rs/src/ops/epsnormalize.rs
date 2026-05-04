use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::epsnormalize::EpsNormalizeFfi;
pub use crate::ffi::epsnormalize::EpsNormalizeType;
use crate::ffi::fst::FstFfi;
use crate::fst::{AsFstCxx, AsMutFstCxx, Fst, MutableFst};

pub trait EpsNormalize<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + EpsNormalizeFfi,
    A::Weight: WeightFfi,
{
    /// Returns an equivalent FST that is epsilon-normalized.
    ///
    /// An acceptor is epsilon-normalized if it is epsilon-removed.
    /// A transducer is input epsilon-normalized if, additionally, on each path
    /// any epsilon input label follows all non-epsilon input labels.
    /// Output epsilon-normalized is defined similarly.
    ///
    /// For more information, see:
    ///
    /// Mohri, M. 2002. Generic epsilon-removal and input epsilon-normalization
    /// algorithms for weighted transducers. International Journal of Computer
    /// Science, 13(1): 129-143, 2002.
    fn epsnormalize_of<F>(&mut self, ifst: &F, norm_type: EpsNormalizeType)
    where
        F: Fst<A> + AsFstCxx<A>;
}

impl<M, A> EpsNormalize<A> for M
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + EpsNormalizeFfi,
    A::Weight: WeightFfi,
    M: MutableFst<A> + AsMutFstCxx<A>,
{
    fn epsnormalize_of<F>(&mut self, ifst: &F, norm_type: EpsNormalizeType)
    where
        F: Fst<A> + AsFstCxx<A>,
    {
        A::fst_epsnormalize(
            unsafe { ifst.as_fst_cxx() },
            unsafe { self.as_mut_fst_cxx() },
            norm_type,
        );
    }
}
