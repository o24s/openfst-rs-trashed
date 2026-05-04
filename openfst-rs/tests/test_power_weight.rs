use openfst_rs::arc::{Arc, StdArc};
use openfst_rs::float_weight::TropicalWeight;
use openfst_rs::fst::{Fst, MutableFst};
use openfst_rs::ops::power::{MapPower, ProjectPower};
use openfst_rs::power_weight::{Power3TropicalWeight, PowerWeight};
use openfst_rs::vector_fst::VectorFst;
use openfst_rs::weight::Weight;

#[test]
fn test_power_weight() {
    let mut fst = VectorFst::<StdArc>::new();
    let s0 = fst.add_state();
    let s1 = fst.add_state();
    fst.set_start(s0);
    fst.set_final(s1, TropicalWeight(0.5));
    fst.add_arc(s0, StdArc::new(1, 2, TropicalWeight(2.0), s1));

    // Map StdArc -> Power3TropicalArc
    let mut p_fst = fst.map_to_power::<openfst_rs::arc::Power3TropicalArc>(1);

    let arc = p_fst.arcs(s0).next().unwrap();

    assert_eq!(arc.weight().0[1].value(), 2.0);
    assert_eq!(arc.weight().0[0].value(), f32::INFINITY);

    p_fst.project_power(1, 0);

    let arc2 = p_fst.arcs(s0).next().unwrap();
    assert_eq!(arc2.weight().0[0].value(), 2.0);
    assert_eq!(arc2.weight().0[1].value(), f32::INFINITY);

    let fst_back = p_fst.map_from_power::<StdArc>(0);
    let arc_back = fst_back.arcs(s0).next().unwrap();
    assert_eq!(arc_back.weight().value(), 2.0);
}

#[test]
fn test_power_weight_ops() {
    let w1 = PowerWeight([
        TropicalWeight(1.0),
        TropicalWeight(2.0),
        TropicalWeight(3.0),
    ]);
    let w2 = PowerWeight([
        TropicalWeight(3.0),
        TropicalWeight(1.0),
        TropicalWeight(2.0),
    ]);

    let p = Power3TropicalWeight::plus(&w1, &w2);
    assert_eq!(p.0[0].value(), 1.0);
    assert_eq!(p.0[1].value(), 1.0);
    assert_eq!(p.0[2].value(), 2.0);

    let t = Power3TropicalWeight::times(&w1, &w2);
    assert_eq!(t.0[0].value(), 4.0);
    assert_eq!(t.0[1].value(), 3.0);
    assert_eq!(t.0[2].value(), 5.0);
}
