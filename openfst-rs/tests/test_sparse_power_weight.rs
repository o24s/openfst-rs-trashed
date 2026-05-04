use openfst_rs::arc::{Arc, SparsePowerTropicalArc};
use openfst_rs::float_weight::TropicalWeight;
use openfst_rs::fst::{Fst, MutableFst};
use openfst_rs::sparse_power_weight::SparsePowerTropicalWeight;
use openfst_rs::vector_fst::VectorFst;
use openfst_rs::weight::Weight;

#[test]
fn test_sparse_power_weight_ops() {
    // default = 0.0 (Tropical::One), elems = [(1, 10.0), (3, 20.0)]
    let w1 = SparsePowerTropicalWeight::new(
        TropicalWeight::one(),
        vec![(1, TropicalWeight(10.0)), (3, TropicalWeight(20.0))],
    );

    // default = 0.0, elems = [(1, 5.0), (2, 15.0)]
    let w2 = SparsePowerTropicalWeight::new(
        TropicalWeight::one(),
        vec![(1, TropicalWeight(5.0)), (2, TropicalWeight(15.0))],
    );

    let p = SparsePowerTropicalWeight::plus(&w1, &w2);

    assert_eq!(p.default_weight.value(), 0.0);

    assert_eq!(p.elements.len(), 1);
    assert_eq!(p.elements[0].0, 1);
    assert_eq!(p.elements[0].1.value(), 5.0);

    let t = SparsePowerTropicalWeight::times(&w1, &w2);

    assert_eq!(t.default_weight.value(), 0.0);

    // key 1: 10.0 + 5.0 = 15.0
    // key 2: 0.0 + 15.0 = 15.0
    // key 3: 20.0 + 0.0 = 20.0
    assert_eq!(t.elements.len(), 3);
    assert_eq!(t.elements[0], (1, TropicalWeight(15.0)));
    assert_eq!(t.elements[1], (2, TropicalWeight(15.0)));
    assert_eq!(t.elements[2], (3, TropicalWeight(20.0)));
}

#[test]
fn test_sparse_power_weight_ffi() {
    let mut fst = VectorFst::<SparsePowerTropicalArc>::new();
    let s0 = fst.add_state();
    let s1 = fst.add_state();
    fst.set_start(s0);
    fst.set_final(s1, SparsePowerTropicalWeight::one()); // default: 0.0, elems: empty

    // w = default: 5.0, elems: { 1: 2.0, 5: 3.0 }
    let w = SparsePowerTropicalWeight::new(
        TropicalWeight(5.0),
        vec![(1, TropicalWeight(2.0)), (5, TropicalWeight(3.0))],
    );

    fst.add_arc(s0, SparsePowerTropicalArc::new(1, 2, w.clone(), s1));

    let arc = fst.arcs(s0).next().unwrap();
    let retrieved_w = arc.weight();

    assert_eq!(retrieved_w.default_weight.value(), 5.0);
    assert_eq!(retrieved_w.elements.len(), 2);
    assert_eq!(retrieved_w.elements[0], (1, TropicalWeight(2.0)));
    assert_eq!(retrieved_w.elements[1], (5, TropicalWeight(3.0)));

    let s = w.to_string();
    assert_eq!(s, "5,1:2,5:3");

    let parsed: SparsePowerTropicalWeight = s.parse().unwrap();
    assert_eq!(parsed, w);
}
