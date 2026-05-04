use openfst_rs::arc::{Arc, Log64Arc, LogArc, MinMaxArc, Real64Arc, RealArc, StdArc};
use openfst_rs::float_weight::{LogWeight, MinMaxWeight, Real64Weight, RealWeight, TropicalWeight};
use openfst_rs::fst::{ExpandedFst, Fst, MutableFst};
use openfst_rs::ops::arc_map::MapTypeConversion;
use openfst_rs::ops::shortest_distance::ShortestDistance;
use openfst_rs::vector_fst::VectorFst;
use openfst_rs::weight::Weight;

#[test]
fn test_all_arc_types_creation() {
    // LogArc (f32)
    let mut fst_log = VectorFst::<LogArc>::new();
    let s0 = fst_log.add_state();
    let s1 = fst_log.add_state();
    fst_log.set_start(s0);
    fst_log.set_final(s1, LogWeight::one()); // One = 0.0
    fst_log.add_arc(s0, LogArc::new(1, 1, LogWeight(2.5), s1));
    assert_eq!(fst_log.num_states(), 2);
    assert_eq!(fst_log.fst_type(), "vector");

    // Real64Arc (f64)
    let mut fst_real64 = VectorFst::<Real64Arc>::new();
    let s0 = fst_real64.add_state();
    let s1 = fst_real64.add_state();
    fst_real64.set_start(s0);
    fst_real64.set_final(s1, Real64Weight::one()); // One = 1.0
    fst_real64.add_arc(s0, Real64Arc::new(1, 1, Real64Weight(0.5), s1));
    assert_eq!(fst_real64.num_states(), 2);

    // MinMaxArc (f32)
    let mut fst_minmax = VectorFst::<MinMaxArc>::new();
    let s0 = fst_minmax.add_state();
    let s1 = fst_minmax.add_state();
    fst_minmax.set_start(s0);
    fst_minmax.set_final(s1, MinMaxWeight::one()); // One = -Infinity
    fst_minmax.add_arc(s0, MinMaxArc::new(1, 1, MinMaxWeight(10.0), s1));
    assert_eq!(fst_minmax.num_states(), 2);
}

#[test]
fn test_semiring_properties() {
    // Tropical (StdArc)
    // Plus(x, y) = min(x, y)
    let mut fst_std = VectorFst::<StdArc>::new();
    let s0 = fst_std.add_state();
    let s1 = fst_std.add_state();
    fst_std.set_start(s0);
    fst_std.set_final(s1, TropicalWeight::one());
    fst_std.add_arc(s0, StdArc::new(1, 1, TropicalWeight(1.0), s1));
    fst_std.add_arc(s0, StdArc::new(2, 2, TropicalWeight(2.0), s1));

    let dist_std = fst_std.shortest_distance(1e-5).unwrap();
    assert_eq!(
        dist_std.value(),
        1.0,
        "Tropical Plus should be min(1.0, 2.0)"
    );

    // Log (LogArc)
    // Plus(x, y) = -ln(exp(-x) + exp(-y))
    let mut fst_log = VectorFst::<LogArc>::new();
    let l0 = fst_log.add_state();
    let l1 = fst_log.add_state();
    fst_log.set_start(l0);
    fst_log.set_final(l1, LogWeight::one());
    fst_log.add_arc(l0, LogArc::new(1, 1, LogWeight(1.0), l1));
    fst_log.add_arc(l0, LogArc::new(2, 2, LogWeight(2.0), l1));

    let dist_log = fst_log.shortest_distance(1e-5).unwrap();
    let expected_log = -((-1.0f32).exp() + (-2.0f32).exp()).ln();
    assert!(
        (dist_log.value() - expected_log).abs() < 1e-4,
        "Log Plus should be -ln(e^-1 + e^-2)"
    );

    // Real (RealArc)
    // Plus(x, y) = x + y
    let mut fst_real = VectorFst::<RealArc>::new();
    let r0 = fst_real.add_state();
    let r1 = fst_real.add_state();
    fst_real.set_start(r0);
    fst_real.set_final(r1, RealWeight::one());
    fst_real.add_arc(r0, RealArc::new(1, 1, RealWeight(1.0), r1));
    fst_real.add_arc(r0, RealArc::new(2, 2, RealWeight(2.0), r1));

    let dist_real = fst_real.shortest_distance(1e-5).unwrap();
    assert_eq!(dist_real.value(), 3.0, "Real Plus should be 1.0 + 2.0");

    // MinMax (MinMaxArc)
    // Plus(x, y) = min(x, y)
    let mut fst_minmax = VectorFst::<MinMaxArc>::new();
    let m0 = fst_minmax.add_state();
    let m1 = fst_minmax.add_state();
    fst_minmax.set_start(m0);
    fst_minmax.set_final(m1, MinMaxWeight::one()); // One = -Infinity
    fst_minmax.add_arc(m0, MinMaxArc::new(1, 1, MinMaxWeight(1.0), m1));
    fst_minmax.add_arc(m0, MinMaxArc::new(2, 2, MinMaxWeight(2.0), m1));

    let dist_minmax = fst_minmax.shortest_distance(1e-5).unwrap();
    assert_eq!(
        dist_minmax.value(),
        1.0,
        "MinMax Plus should be min(1.0, 2.0)"
    );
}
#[test]
fn test_crossproduct_type_conversion() {
    let mut fst_std = VectorFst::<StdArc>::new();
    let s0 = fst_std.add_state();
    let s1 = fst_std.add_state();
    fst_std.set_start(s0);
    fst_std.set_final(s1, TropicalWeight::one()); // 0.0
    // Tropical Cost = 2.0
    fst_std.add_arc(s0, StdArc::new(1, 2, TropicalWeight(2.0), s1));

    // StdArc (Tropical, Cost) -> Log64Arc (Log, Cost)
    let mut fst_log64 = VectorFst::<Log64Arc>::new();
    fst_log64.map_type_of(&fst_std); // これはOK

    // Log64Arc (Log, Cost) -> Real64Arc (Real, Probability)
    let mut fst_real64 = VectorFst::<Real64Arc>::new();
    fst_real64.map_type_of(&fst_log64);

    let arc_real64 = fst_real64.arcs(s0).next().unwrap();
    let expected_prob = (-2.0f64).exp();
    assert!(
        (arc_real64.weight().value() - expected_prob).abs() < 1e-6,
        "Cost 2.0 should be converted to Prob exp(-2.0) via Log domain"
    );
    // exp(-0.0) = 1.0 (Real64Weight::one())
    assert_eq!(fst_real64.final_weight(s1).value(), 1.0);

    // Real64Arc (f64, Probability) -> LogArc (f32, Cost)
    // Cost = -ln(Probability)
    let mut fst_log = VectorFst::<LogArc>::new();
    fst_log.map_type_of(&fst_real64);

    let arc_log = fst_log.arcs(s0).next().unwrap();
    assert!(
        (arc_log.weight().value() - 2.0).abs() < 1e-5,
        "Prob exp(-2.0) should be converted back to Cost 2.0 in Log domain"
    );

    // -ln(1.0) = 0.0 (LogWeight::one())
    assert_eq!(fst_log.final_weight(s1).value(), 0.0);
}

#[test]
fn test_string_compiler_with_other_arcs() {
    use openfst_rs::string::StringPrinter;

    let fst_log64 = openfst_rs::fst_linear!(VectorFst<Log64Arc>, "openfst");

    assert_eq!(fst_log64.num_states_if_known(), Some(8));

    let (s, w) = fst_log64.print_bytes().unwrap();
    assert_eq!(s, "openfst");
    assert_eq!(w.value(), 0.0);
}
