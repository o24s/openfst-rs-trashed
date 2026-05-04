use crate::expectation_weight::*;
use crate::float_weight::*;
use crate::lexicographic_weight::*;
use crate::power_weight::*;
use crate::sparse_power_weight::*;
use crate::weight::Weight;

pub trait Arc: Clone + PartialEq {
    type Weight: Weight;
    type Label: Copy + PartialEq;
    type StateId: Copy + PartialEq;

    fn new(
        ilabel: Self::Label,
        olabel: Self::Label,
        weight: Self::Weight,
        nextstate: Self::StateId,
    ) -> Self;
    fn ilabel(&self) -> Self::Label;
    fn olabel(&self) -> Self::Label;
    fn weight(&self) -> &Self::Weight;
    fn nextstate(&self) -> Self::StateId;
    fn type_name() -> String;
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ArcTpl<W, L = i32, S = i32> {
    pub ilabel: L,
    pub olabel: L,
    pub weight: W,
    pub nextstate: S,
}

impl<W: Weight, L: Copy + PartialEq, S: Copy + PartialEq> Arc for ArcTpl<W, L, S> {
    type Weight = W;
    type Label = L;
    type StateId = S;

    #[inline(always)]
    fn new(ilabel: L, olabel: L, weight: W, nextstate: S) -> Self {
        Self {
            ilabel,
            olabel,
            weight,
            nextstate,
        }
    }
    #[inline(always)]
    fn ilabel(&self) -> Self::Label {
        self.ilabel
    }
    #[inline(always)]
    fn olabel(&self) -> Self::Label {
        self.olabel
    }
    #[inline(always)]
    fn weight(&self) -> &Self::Weight {
        &self.weight
    }
    #[inline(always)]
    fn nextstate(&self) -> Self::StateId {
        self.nextstate
    }
    #[inline(always)]
    fn type_name() -> String {
        let w_type = W::type_name();
        if w_type == "tropical" {
            "standard".to_string()
        } else {
            w_type.to_string()
        }
    }
}

pub type StdArc = ArcTpl<TropicalWeight>;
pub type Std64Arc = ArcTpl<TropicalWeight64>;
pub type LogArc = ArcTpl<LogWeight>;
pub type Log64Arc = ArcTpl<Log64Weight>;
pub type RealArc = ArcTpl<RealWeight>;
pub type Real64Arc = ArcTpl<Real64Weight>;
pub type MinMaxArc = ArcTpl<MinMaxWeight>;
pub type MinMax64Arc = ArcTpl<MinMaxWeight64>;
pub type SignedLogArc = ArcTpl<SignedLogWeight>;
pub type SignedLog64Arc = ArcTpl<SignedLog64Weight>;
pub type LexicographicArc = ArcTpl<LexicographicWeight>;
pub type Lexicographic64Arc = ArcTpl<Lexicographic64Weight>;
pub type LexicographicMinMaxTropicalArc = ArcTpl<LexicographicMinMaxTropicalWeight>;
pub type LexicographicMinMaxTropical64Arc = ArcTpl<LexicographicMinMaxTropical64Weight>;
pub type LexicographicMinMaxArc = ArcTpl<LexicographicMinMaxWeight>;
pub type LexicographicMinMax64Arc = ArcTpl<LexicographicMinMax64Weight>;
pub type ExpectationLogArc = ArcTpl<ExpectationLogWeight>;
pub type ExpectationLog64Arc = ArcTpl<ExpectationLog64Weight>;
pub type ExpectationRealArc = ArcTpl<ExpectationRealWeight>;
pub type ExpectationReal64Arc = ArcTpl<ExpectationReal64Weight>;
pub type ExpectationTropicalArc = ArcTpl<ExpectationTropicalWeight>;
pub type ExpectationTropical64Arc = ArcTpl<ExpectationTropical64Weight>;
pub type ExpectationMinMaxArc = ArcTpl<ExpectationMinMaxWeight>;
pub type ExpectationMinMax64Arc = ArcTpl<ExpectationMinMax64Weight>;
pub type ExpectationSignedLogArc = ArcTpl<ExpectationSignedLogWeight>;
pub type ExpectationSignedLog64Arc = ArcTpl<ExpectationSignedLog64Weight>;
pub type Power3TropicalArc = ArcTpl<Power3TropicalWeight>;
pub type Power3Tropical64Arc = ArcTpl<Power3Tropical64Weight>;
pub type Power3LogArc = ArcTpl<Power3LogWeight>;
pub type Power3Log64Arc = ArcTpl<Power3Log64Weight>;
pub type Power3RealArc = ArcTpl<Power3RealWeight>;
pub type Power3Real64Arc = ArcTpl<Power3Real64Weight>;
pub type Power3MinMaxArc = ArcTpl<Power3MinMaxWeight>;
pub type Power3MinMax64Arc = ArcTpl<Power3MinMax64Weight>;
pub type SparsePowerTropicalArc = ArcTpl<SparsePowerTropicalWeight>;
pub type SparsePowerTropical64Arc = ArcTpl<SparsePowerTropical64Weight>;
pub type SparsePowerLogArc = ArcTpl<SparsePowerLogWeight>;
pub type SparsePowerLog64Arc = ArcTpl<SparsePowerLog64Weight>;
pub type SparsePowerRealArc = ArcTpl<SparsePowerRealWeight>;
pub type SparsePowerReal64Arc = ArcTpl<SparsePowerReal64Weight>;
pub type SparsePowerMinMaxArc = ArcTpl<SparsePowerMinMaxWeight>;
pub type SparsePowerMinMax64Arc = ArcTpl<SparsePowerMinMax64Weight>;

pub(crate) type LexicographicStdArc = ArcTpl<LexicographicWeight>;
