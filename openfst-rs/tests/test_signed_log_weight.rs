use std::f32;

use openfst_rs::arc::{Arc, SignedLogArc};
use openfst_rs::float_weight::SignedLogWeight;
use openfst_rs::fst::MutableFst;
use openfst_rs::ops::shortest_distance::ShortestDistance;
use openfst_rs::vector_fst::VectorFst;
use openfst_rs::weight::Weight;

#[test]
fn test_signed_log_weight_ops() {
    // +0.5 -> sign = 1.0, neg_log_prob = -ln(0.5) ≈ 0.693147
    let p_05 = SignedLogWeight::new(1.0, f32::consts::LN_2);

    // -0.5 -> sign = -1.0, neg_log_prob = -ln(0.5) ≈ 0.693147
    let m_05 = SignedLogWeight::new(-1.0, f32::consts::LN_2);

    let sum = SignedLogWeight::plus(&p_05, &m_05);
    assert_eq!(sum.value().w2, f32::INFINITY);

    let prod = SignedLogWeight::times(&p_05, &m_05);
    assert_eq!(prod.value().w1, -1.0);
    assert!((prod.value().w2 - 1.386294).abs() < 1e-4);
}

#[test]
fn test_signed_log_cancellation_fst() {
    let mut fst = VectorFst::<SignedLogArc>::new();
    let s0 = fst.add_state();
    let s1 = fst.add_state();
    fst.set_start(s0);
    fst.set_final(s1, SignedLogWeight::one());

    fst.add_arc(
        s0,
        SignedLogArc::new(1, 1, SignedLogWeight::new(1.0, f32::consts::LN_2), s1),
    );
    fst.add_arc(
        s0,
        SignedLogArc::new(2, 2, SignedLogWeight::new(-1.0, f32::consts::LN_2), s1),
    );

    // (+0.5) + (-0.5) = 0.0
    let total_sum = fst.shortest_distance(1e-5).unwrap();

    assert_eq!(
        total_sum.value().w2,
        f32::INFINITY,
        "Positive and negative probabilities should cancel out to zero"
    );
}
