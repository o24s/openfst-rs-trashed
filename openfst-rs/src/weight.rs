pub const DELTA: f32 = 1.0 / 1024.0;
// Semiring properties
pub const LEFT_SEMIRING: u64 = 0x0000000000000001;
pub const RIGHT_SEMIRING: u64 = 0x0000000000000002;
pub const SEMIRING: u64 = LEFT_SEMIRING | RIGHT_SEMIRING;
pub const COMMUTATIVE: u64 = 0x0000000000000004;
pub const IDEMPOTENT: u64 = 0x0000000000000008;
pub const PATH: u64 = 0x0000000000000010;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DivideType {
    Left = 0,
    Right = 1,
    Any = 2,
}

pub trait Weight: Clone + PartialEq + std::fmt::Display + std::str::FromStr {
    type ReverseWeight: Weight;

    fn zero() -> Self;
    fn one() -> Self;
    fn no_weight() -> Self;

    fn type_name() -> &'static str;
    fn properties() -> u64;

    // Semiring operations
    fn plus(w1: &Self, w2: &Self) -> Self;
    fn times(w1: &Self, w2: &Self) -> Self;
    fn divide(w1: &Self, w2: &Self, typ: DivideType) -> Self;

    fn reverse(w: &Self) -> Self::ReverseWeight;
    fn is_member(&self) -> bool;

    fn approx_equal(w1: &Self, w2: &Self, delta: f32) -> bool;
    fn quantize(w: &Self, delta: f32) -> Self;
}

pub trait Semiring: Clone + PartialEq {
    fn zero() -> Self;
    fn one() -> Self;
    fn plus(&self, other: &Self) -> Self;
    fn times(&self, other: &Self) -> Self;
    fn type_name() -> &'static str;
}

/// Weight forms a left semiring (required for determinize and pushing to initial).
pub trait LeftSemiring: Weight {}

/// Weight forms a right semiring (required for shortest path and pushing to final).
pub trait RightSemiring: Weight {}

/// Weight has the path property, meaning addition is exactly `min` (required for shortest path).
pub trait PathWeight: Weight {}

/// Weight is idempotent, meaning `plus(x, x) == x`.
pub trait IdempotentWeight: Weight {}

/// Weight is commutative, meaning `times(x, y) == times(y, x)`.
pub trait CommutativeWeight: Weight {}

/// Power is the iterated product for arbitrary semirings.
pub fn power<W: Weight>(weight: &W, n: usize) -> W {
    let mut result = W::one();
    for _ in 0..n {
        result = W::times(&result, weight);
    }
    result
}

/// A simple adder class. For floating point weights like LogWeight or RealWeight,
/// Kahan-compensated adders are provided in the float_weight module to avoid
/// precision loss in long sums.
pub struct Adder<W: Weight> {
    sum: W,
}

impl<W: Weight> Adder<W> {
    pub fn new() -> Self {
        Self { sum: W::zero() }
    }
    pub fn add(&mut self, w: &W) -> W {
        self.sum = W::plus(&self.sum, w);
        self.sum.clone()
    }
    pub fn sum(&self) -> W {
        self.sum.clone()
    }
    pub fn reset(&mut self, w: W) {
        self.sum = w;
    }
}

impl<W: Weight> Default for Adder<W> {
    fn default() -> Self {
        Self::new()
    }
}
