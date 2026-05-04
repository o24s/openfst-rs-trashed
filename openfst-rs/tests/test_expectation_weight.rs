use openfst_rs::arc::{Arc, ExpectationRealArc};
use openfst_rs::expectation_weight::{ExpectationRealWeight, ExpectationTropicalWeight};
use openfst_rs::fst::MutableFst;
use openfst_rs::ops::shortest_distance::{ShortestDistance, ShortestDistanceOptions};
use openfst_rs::vector_fst::VectorFst;
use openfst_rs::weight::Weight;

#[test]
fn test_expectation_real_ops() {
    let w1 = ExpectationRealWeight::new(0.5, 10.0);
    let w2 = ExpectationRealWeight::new(0.5, 20.0);

    // Plus:
    // P = 0.5 + 0.5 = 1.0
    // V = 10.0 + 20.0 = 30.0
    let p = ExpectationRealWeight::plus(&w1, &w2);
    assert_eq!(p.value().w1, 1.0);
    assert_eq!(p.value().w2, 30.0);

    // Times:
    // P = 0.5 * 0.5 = 0.25
    // V = (p1 * v2) + (p2 * v1) = (0.5 * 20.0) + (0.5 * 10.0) = 15.0
    let t = ExpectationRealWeight::times(&w1, &w2);
    assert_eq!(t.value().w1, 0.25);
    assert_eq!(t.value().w2, 15.0);

    let r = ExpectationRealWeight::reverse(&w1);
    assert_eq!(r.value().w1, 0.5);
    assert_eq!(r.value().w2, 10.0);
}

#[test]
fn test_expectation_tropical_ops() {
    let w1 = ExpectationTropicalWeight::new(1.0, 5.0);
    let w2 = ExpectationTropicalWeight::new(2.0, 3.0);

    // Plus:
    // P = min(1.0, 2.0) = 1.0
    // V = min(5.0, 3.0) = 3.0
    let p = ExpectationTropicalWeight::plus(&w1, &w2);
    assert_eq!(p.value().w1, 1.0);
    assert_eq!(p.value().w2, 3.0);

    // Times:
    // P = 1.0 + 2.0 = 3.0
    // V = min(1.0 + 3.0, 2.0 + 5.0) = min(4.0, 7.0) = 4.0
    let t = ExpectationTropicalWeight::times(&w1, &w2);
    assert_eq!(t.value().w1, 3.0);
    assert_eq!(t.value().w2, 4.0);
}

#[test]
fn test_expectation_fst_shortest_distance() {
    let mut fst = VectorFst::<ExpectationRealArc>::new();
    let s0 = fst.add_state();
    let s1 = fst.add_state();
    fst.set_start(s0);

    // Prob = 1.0, Value = 0.0
    fst.set_final(s1, ExpectationRealWeight::one());

    fst.add_arc(
        s0,
        ExpectationRealArc::new(1, 1, ExpectationRealWeight::new(0.6, 60.0), s1),
    );

    fst.add_arc(
        s0,
        ExpectationRealArc::new(2, 2, ExpectationRealWeight::new(0.4, 20.0), s1),
    );

    let opts = ShortestDistanceOptions {
        reverse: true,
        ..Default::default()
    };
    let distances = fst.shortest_distance_vec(&opts).unwrap();

    let expected = distances[s0 as usize];

    // Prob = 0.6 + 0.4 = 1.0
    assert!((expected.value().w1 - 1.0).abs() < 1e-5);

    // Value = 60.0 + 20.0 = 80.0
    assert!(
        (expected.value().w2 - 80.0).abs() < 1e-5,
        "Expected value should be 80.0"
    );
}
