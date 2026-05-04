use openfst_rs::arc::{Arc, Log64Arc, StdArc};
use openfst_rs::arc_map_fst::{ArcMapConvertFst, StdArcMapFst};
use openfst_rs::float_weight::{Log64Weight, TropicalWeight};
use openfst_rs::fst::ExpandedFst;
use openfst_rs::fst::{Fst, MutableFst};
use openfst_rs::ops::arc_map::{Map, MapGallic, MapMapper, MapTypeConversion};
use openfst_rs::ops::arcsort::ArcSort;
use openfst_rs::ops::closure::{Closure, ClosureType};
use openfst_rs::ops::compose::{Compose, ComposeOptions};
use openfst_rs::ops::concat::Concat;
use openfst_rs::ops::connect::Connect;
use openfst_rs::ops::determinize::{Determinize, DeterminizeOptions};
use openfst_rs::ops::difference::Difference;
use openfst_rs::ops::disambiguate::{Disambiguate, DisambiguateOptions};
use openfst_rs::ops::encode::{ENCODE_LABELS, ENCODE_WEIGHTS, Encode, EncodeMapper};
use openfst_rs::ops::epsnormalize::{EpsNormalize, EpsNormalizeType};
use openfst_rs::ops::equivalent::{
    EqualType, Equivalent, EquivalentVerified, RandEquivalentOptions,
};
use openfst_rs::ops::intersect::Intersect;
use openfst_rs::ops::invert::Invert;
use openfst_rs::ops::minimize::{Minimize, MinimizeOptions};
use openfst_rs::ops::project::{Project, ProjectType};
use openfst_rs::ops::prune::{Prune, PruneOptions};
use openfst_rs::ops::push::{Push, ReweightType};
use openfst_rs::ops::randgen::{ArcSelectorType, RandGen, RandGenOptions};
use openfst_rs::ops::relabel::Relabel;
use openfst_rs::ops::replace::{Replace, ReplaceLabelType, ReplaceOptions};
use openfst_rs::ops::reverse::Reverse;
use openfst_rs::ops::rmepsilon::{RmEpsilon, RmEpsilonOptions};
use openfst_rs::ops::rmfinalepsilon::RmFinalEpsilon;
use openfst_rs::ops::shortest_distance::{ShortestDistance, ShortestDistanceOptions};
use openfst_rs::ops::shortest_path::{ShortestPath, ShortestPathOptions};
use openfst_rs::ops::statesort::StateSort;
use openfst_rs::ops::topsort::TopSort;
use openfst_rs::ops::union::Union;
use openfst_rs::ops::verify::Verify;
use openfst_rs::properties::{
    Acceptor, DetEpsFreeAcceptor, FstPropertiesExt, UnweightedDetEpsFreeAcceptor,
};
use openfst_rs::symbol_table::SymbolTable;
use openfst_rs::vector_fst::{Log64VectorFst, StdVectorFst};
use openfst_rs::weight::Weight;
use tempfile::NamedTempFile;

const EPSILON: i32 = 0;

#[test]
fn test_arcsort() {
    let mut fst = StdVectorFst::new();
    let s0 = fst.add_state();
    fst.set_start(s0);
    fst.set_final(s0, TropicalWeight::one());

    fst.add_arc(s0, StdArc::new(3, 1, TropicalWeight::one(), s0));
    fst.add_arc(s0, StdArc::new(1, 3, TropicalWeight::one(), s0));
    fst.add_arc(s0, StdArc::new(2, 2, TropicalWeight::one(), s0));

    fst.arcsort_ilabel();
    let arcs_in: Vec<_> = fst.arcs(s0).collect();
    assert_eq!(arcs_in.len(), 3);
    assert_eq!(arcs_in[0].ilabel(), 1);
    assert_eq!(arcs_in[1].ilabel(), 2);
    assert_eq!(arcs_in[2].ilabel(), 3);

    fst.arcsort_olabel();
    let arcs_out: Vec<_> = fst.arcs(s0).collect();
    assert_eq!(arcs_out[0].olabel(), 1);
    assert_eq!(arcs_out[1].olabel(), 2);
    assert_eq!(arcs_out[2].olabel(), 3);
}

#[test]
fn test_invert() {
    let mut fst = StdVectorFst::new();
    let s0 = fst.add_state();
    let s1 = fst.add_state();
    fst.set_start(s0);
    fst.set_final(s1, TropicalWeight::one());

    // a(1) : x(2) : 0.5
    fst.add_arc(s0, StdArc::new(1, 2, TropicalWeight(0.5), s1));

    let mut inv_fst = StdVectorFst::new();
    inv_fst.invert_of(&fst);

    let arc1 = inv_fst.arcs(inv_fst.start()).next().unwrap();
    assert_eq!(arc1.ilabel(), 2, "Inverted ilabel should be 2");
    assert_eq!(arc1.olabel(), 1, "Inverted olabel should be 1");
    assert_eq!(arc1.weight().value(), 0.5, "Weight should be unchanged");

    fst.invert();
    let arc2 = fst.arcs(fst.start()).next().unwrap();
    assert_eq!(arc2.ilabel(), 2, "In-place inverted ilabel should be 2");
    assert_eq!(arc2.olabel(), 1, "In-place inverted olabel should be 1");
}

#[test]
fn test_rmepsilon() {
    let mut fst = StdVectorFst::new();
    let s0 = fst.add_state();
    let s1 = fst.add_state();
    let s2 = fst.add_state();
    fst.set_start(s0);
    fst.set_final(s2, TropicalWeight::one());

    // s0 --(eps:eps, 1.0)--> s1
    fst.add_arc(s0, StdArc::new(EPSILON, EPSILON, TropicalWeight(1.0), s1));
    // s1 --(a(1):b(2), 2.0)--> s2
    fst.add_arc(s1, StdArc::new(1, 2, TropicalWeight(2.0), s2));

    fst.rmepsilon(&RmEpsilonOptions::default());

    let arcs: Vec<_> = fst.arcs(fst.start()).collect();
    assert_eq!(arcs.len(), 1);
    assert_eq!(arcs[0].ilabel(), 1);
    assert_eq!(arcs[0].olabel(), 2);
    assert_eq!(arcs[0].weight().value(), 3.0);
}

#[test]
fn test_compose_and_shortest_path() {
    // FST1: 1->2 (cost 0.5), 2->3 (cost 1.0) => 1:2:3
    let mut fst1 = StdVectorFst::new();
    let s0 = fst1.add_state();
    let s1 = fst1.add_state();
    let s2 = fst1.add_state();
    fst1.set_start(s0);
    fst1.set_final(s2, TropicalWeight::one());
    fst1.add_arc(s0, StdArc::new(1, 2, TropicalWeight(0.5), s1));
    fst1.add_arc(s0, StdArc::new(1, 2, TropicalWeight(5.0), s1)); // Sub-optimal path
    fst1.add_arc(s1, StdArc::new(2, 3, TropicalWeight(1.0), s2));

    fst1.arcsort_olabel();

    // FST2: 2->4 (cost 1.5), 3->5 (cost 2.0) => 2:4, 3:5
    let mut fst2 = StdVectorFst::new();
    let t0 = fst2.add_state();
    let t1 = fst2.add_state();
    let t2 = fst2.add_state();
    fst2.set_start(t0);
    fst2.set_final(t2, TropicalWeight::one());
    fst2.add_arc(t0, StdArc::new(2, 4, TropicalWeight(1.5), t1));
    fst2.add_arc(t1, StdArc::new(3, 5, TropicalWeight(2.0), t2));

    fst2.arcsort_ilabel();

    // Compose: fst1 ∘ fst2
    let mut composed = StdVectorFst::new();
    composed.compose(&fst1, &fst2, &ComposeOptions::default());

    // ShortestPath
    let mut shortest = StdVectorFst::new();
    shortest.shortest_path(&composed, &ShortestPathOptions::default());

    // expected:
    // State0 --(1:4, 0.5 + 1.5 = 2.0)--> State1 --(2:5, 1.0 + 2.0 = 3.0)--> Final
    // Total cost: 5.0

    let mut curr = shortest.start();
    let mut total_weight = 0.0;

    let arc1 = shortest.arcs(curr).next().unwrap();
    assert_eq!(arc1.ilabel(), 1);
    assert_eq!(arc1.olabel(), 4);
    assert_eq!(arc1.weight().value(), 2.0);
    total_weight += arc1.weight().value();
    curr = arc1.nextstate();

    let arc2 = shortest.arcs(curr).next().unwrap();
    assert_eq!(arc2.ilabel(), 2);
    assert_eq!(arc2.olabel(), 5);
    assert_eq!(arc2.weight().value(), 3.0);
    total_weight += arc2.weight().value();

    assert_eq!(
        total_weight, 5.0,
        "Total shortest path weight should be 5.0"
    );
}

#[test]
fn test_project() {
    let mut fst = StdVectorFst::new();
    let s0 = fst.add_state();
    let s1 = fst.add_state();
    fst.set_start(s0);
    fst.set_final(s1, TropicalWeight::one());

    // arc: a(1) : x(2) : 0.5
    fst.add_arc(s0, StdArc::new(1, 2, TropicalWeight(0.5), s1));

    let mut proj_in_fst = StdVectorFst::new();
    proj_in_fst.project_of(&fst, ProjectType::Input);

    let arc1 = proj_in_fst.arcs(proj_in_fst.start()).next().unwrap();
    assert_eq!(arc1.ilabel(), 1, "Projected input ilabel should be 1");
    assert_eq!(arc1.olabel(), 1, "Projected input olabel should be 1");
    assert_eq!(arc1.weight().value(), 0.5);

    fst.project(ProjectType::Output);
    let arc2 = fst.arcs(fst.start()).next().unwrap();

    assert_eq!(arc2.ilabel(), 2, "Projected output ilabel should be 2");
    assert_eq!(arc2.olabel(), 2, "Projected output olabel should be 2");
    assert_eq!(arc2.weight().value(), 0.5);
}

#[test]
fn test_determinize() {
    // Path 1: 0 --(a, a, 1.0)--> 1 --(b, b, 2.0)--> 3  (Total cost = 3.0)
    // Path 2: 0 --(a, a, 3.0)--> 2 --(b, b, 4.0)--> 3  (Total cost = 7.0)
    let mut fst = StdVectorFst::new();
    let s0 = fst.add_state();
    let s1 = fst.add_state();
    let s2 = fst.add_state();
    let s3 = fst.add_state();
    fst.set_start(s0);
    fst.set_final(s3, TropicalWeight::one());

    fst.add_arc(s0, StdArc::new(1, 1, TropicalWeight(1.0), s1));
    fst.add_arc(s1, StdArc::new(2, 2, TropicalWeight(2.0), s3));

    fst.add_arc(s0, StdArc::new(1, 1, TropicalWeight(3.0), s2));
    fst.add_arc(s2, StdArc::new(2, 2, TropicalWeight(4.0), s3));

    let mut det_fst = StdVectorFst::new();
    det_fst.determinize(&fst, &DeterminizeOptions::default());

    let arcs_from_start: Vec<_> = det_fst.arcs(det_fst.start()).collect();

    assert_eq!(
        arcs_from_start.len(),
        1,
        "Determinized FST must have a single outgoing arc for label 'a'"
    );
    assert_eq!(arcs_from_start[0].ilabel(), 1);
    assert_eq!(
        arcs_from_start[0].weight().value(),
        1.0,
        "Weight pushed to front should be min(1.0, 3.0) = 1.0"
    );
}

#[test]
fn test_minimize() {
    let mut fst = StdVectorFst::new();
    let s0 = fst.add_state(); // 0
    let s1 = fst.add_state(); // 1
    let s2 = fst.add_state(); // 2

    fst.set_start(s0);

    fst.set_final(s1, TropicalWeight::one());
    fst.set_final(s2, TropicalWeight::one());

    fst.add_arc(s0, StdArc::new(1, 1, TropicalWeight(2.0), s1));
    fst.add_arc(s0, StdArc::new(2, 2, TropicalWeight(3.0), s2));

    assert_eq!(
        fst.num_states(),
        3,
        "Before minimize, there should be 3 states"
    );

    fst.minimize(&MinimizeOptions::default()).unwrap();

    assert_eq!(
        fst.num_states(),
        2,
        "After minimize, equivalent final states should be merged into 2 states"
    );
}

#[test]
fn test_union_and_concat() {
    let mut fst_a = StdVectorFst::new();
    let s0 = fst_a.add_state();
    let s1 = fst_a.add_state();
    fst_a.set_start(s0);
    fst_a.set_final(s1, TropicalWeight::one());
    fst_a.add_arc(s0, StdArc::new(1, 1, TropicalWeight(0.5), s1));

    let mut fst_b = StdVectorFst::new();
    let t0 = fst_b.add_state();
    let t1 = fst_b.add_state();
    fst_b.set_start(t0);
    fst_b.set_final(t1, TropicalWeight::one());
    fst_b.add_arc(t0, StdArc::new(2, 2, TropicalWeight(1.0), t1));

    // Union: "a" | "b"
    let mut union_fst = fst_a.clone();
    union_fst.union(&fst_b);

    union_fst.rmepsilon(&RmEpsilonOptions::default());
    let mut det_union = StdVectorFst::new();
    det_union.determinize(&union_fst, &DeterminizeOptions::default());
    det_union.minimize(&MinimizeOptions::default()).unwrap();

    let start_u = det_union.start();
    let arcs_u: Vec<_> = det_union.arcs(start_u).collect();
    assert_eq!(arcs_u.len(), 2, "Union should have 2 paths");

    // Concat: "a" followed by "b"
    let mut concat_fst = fst_a.clone();
    concat_fst.concat(&fst_b);

    concat_fst.rmepsilon(&RmEpsilonOptions::default());
    let mut det_concat = StdVectorFst::new();
    det_concat.determinize(&concat_fst, &DeterminizeOptions::default());
    det_concat.minimize(&MinimizeOptions::default()).unwrap();

    let start_c = det_concat.start();
    let arcs_c1: Vec<_> = det_concat.arcs(start_c).collect();
    assert_eq!(arcs_c1.len(), 1, "Concat should have exactly 1 initial arc");
    assert_eq!(arcs_c1[0].ilabel(), 1, "First label must be 'a'(1)");

    let mid_state = arcs_c1[0].nextstate();
    let arcs_c2: Vec<_> = det_concat.arcs(mid_state).collect();
    assert_eq!(
        arcs_c2.len(),
        1,
        "Concat should have exactly 1 following arc"
    );
    assert_eq!(arcs_c2[0].ilabel(), 2, "Second label must be 'b'(2)");

    // 0.5 + 1.0 = 1.5
    assert_eq!(
        arcs_c1[0].weight().value() + arcs_c2[0].weight().value(),
        1.5
    );
}

#[test]
fn test_closure() {
    let mut fst = StdVectorFst::new();
    let s0 = fst.add_state();
    let s1 = fst.add_state();
    fst.set_start(s0);
    fst.set_final(s1, TropicalWeight::one());
    fst.add_arc(s0, StdArc::new(1, 1, TropicalWeight(1.0), s1));

    fst.closure(ClosureType::Star);

    fst.rmepsilon(&RmEpsilonOptions::default());
    let mut det_fst = StdVectorFst::new();
    det_fst.determinize(&fst, &DeterminizeOptions::default());
    det_fst.minimize(&MinimizeOptions::default()).unwrap();

    let start = det_fst.start();

    let start_final_weight = det_fst.final_weight(start);
    assert!(
        start_final_weight.is_member() && start_final_weight.value() == 0.0,
        "Start state should be final with weight 0.0 for A*"
    );

    let arcs: Vec<_> = det_fst.arcs(start).collect();
    assert_eq!(arcs.len(), 1, "There should be 1 arc allowing repetitions");
    assert_eq!(arcs[0].ilabel(), 1, "The loop label should be 'a'(1)");
    assert_eq!(arcs[0].weight().value(), 1.0, "Cost per step should be 1.0");

    let next = arcs[0].nextstate();
    let next_final_weight = det_fst.final_weight(next);
    assert!(
        next_final_weight.is_member() && next_final_weight.value() == 0.0,
        "Next state should also be final to accept multiple 'a's"
    );
}

#[test]
fn test_equivalent_family() {
    let mut fst1 = StdVectorFst::new();
    let s1_0 = fst1.add_state();
    let s1_1 = fst1.add_state();
    fst1.set_start(s1_0);
    fst1.set_final(s1_1, TropicalWeight::one());

    fst1.add_arc(s1_0, StdArc::new(1, 1, TropicalWeight(0.5), s1_1));

    let fst2 = fst1.clone();

    let delta = 1e-4;

    assert!(
        fst1.equal(&fst2, delta, EqualType::EQUAL_FSTS),
        "Clones should be exactly equal"
    );

    assert!(fst1.isomorphic(&fst2, delta), "Clones should be isomorphic");

    // Verified properties required for equivalent()
    let det_fst1 = fst1.verify::<DetEpsFreeAcceptor>().unwrap();
    let det_fst2 = fst2.verify::<DetEpsFreeAcceptor>().unwrap();

    assert!(
        det_fst1.equivalent(det_fst2, delta).unwrap(),
        "Clones should be analytically equivalent"
    );

    let opts = RandEquivalentOptions {
        npath: 10,
        ..Default::default()
    };
    assert!(
        fst1.randequivalent(&fst2, &opts).unwrap(),
        "Clones should be stochastically equivalent"
    );

    let mut fst3 = StdVectorFst::new();
    let s3_1 = fst3.add_state();
    let s3_0 = fst3.add_state();
    fst3.set_start(s3_0);
    fst3.set_final(s3_1, TropicalWeight::one());

    fst3.add_arc(s3_0, StdArc::new(1, 1, TropicalWeight(0.5), s3_1));

    assert!(
        !fst1.equal(&fst3, delta, EqualType::EQUAL_FSTS),
        "State ordering differs, so not equal"
    );

    assert!(
        fst1.isomorphic(&fst3, delta),
        "Graph structure is the same, should be isomorphic"
    );

    let det_fst3 = fst3.verify::<DetEpsFreeAcceptor>().unwrap();
    assert!(
        det_fst1.equivalent(det_fst3, delta).unwrap(),
        "Accepted languages are the same, should be equivalent"
    );
}

#[test]
fn test_connect() {
    let mut fst = StdVectorFst::new();
    let s0 = fst.add_state();
    let s1 = fst.add_state();
    let s2 = fst.add_state();
    let s3 = fst.add_state();

    fst.set_start(s0);
    fst.set_final(s2, TropicalWeight::one());

    fst.add_arc(s0, StdArc::new(1, 1, TropicalWeight(0.5), s1));
    fst.add_arc(s1, StdArc::new(2, 2, TropicalWeight(0.5), s2));

    fst.add_arc(s1, StdArc::new(3, 3, TropicalWeight(1.0), s3));

    assert_eq!(fst.num_states(), 4);

    fst.connect();

    assert_eq!(fst.num_states(), 3, "Dead states should be removed");
    assert_eq!(fst.num_arcs(s1), 1, "Dead-end arc should be removed");
}

#[test]
fn test_topsort() {
    let mut fst = StdVectorFst::new();

    let s0 = fst.add_state(); // final
    let s1 = fst.add_state(); // mid
    let s2 = fst.add_state(); // start

    fst.set_start(s2);
    fst.set_final(s0, TropicalWeight::one());

    // (s2 -> s1, s1 -> s0)
    fst.add_arc(s2, StdArc::new(1, 1, TropicalWeight(1.0), s1));
    fst.add_arc(s1, StdArc::new(2, 2, TropicalWeight(1.0), s0));

    let is_acyclic = fst.topsort();
    assert!(is_acyclic, "DAG should be successfully top-sorted");

    let new_start = fst.start();
    assert_eq!(
        new_start, 0,
        "Start state ID should be reset to 0 after TopSort"
    );

    let arcs: Vec<_> = fst.arcs(new_start).collect();
    assert_eq!(arcs.len(), 1);
    assert_eq!(
        arcs[0].nextstate(),
        1,
        "Next state should be 1 after TopSort"
    );
}

#[test]
fn test_topsort_cyclic() {
    let mut fst = StdVectorFst::new();
    let s0 = fst.add_state();
    let s1 = fst.add_state();

    fst.set_start(s0);
    fst.set_final(s1, TropicalWeight::one());

    fst.add_arc(s0, StdArc::new(1, 1, TropicalWeight(1.0), s1));
    // (s1 -> s0)
    fst.add_arc(s1, StdArc::new(2, 2, TropicalWeight(1.0), s0));

    let is_acyclic = fst.topsort();
    assert!(
        !is_acyclic,
        "Cyclic FST cannot be top-sorted and should return false"
    );
}

#[test]
fn test_shortest_distance() {
    let mut fst = StdVectorFst::new();
    let s0 = fst.add_state();
    let s1 = fst.add_state();
    let s2 = fst.add_state();

    fst.set_start(s0);
    fst.set_final(s2, TropicalWeight::one());

    fst.add_arc(s0, StdArc::new(1, 1, TropicalWeight(1.0), s1));
    fst.add_arc(s1, StdArc::new(2, 2, TropicalWeight(2.0), s2));

    fst.add_arc(s0, StdArc::new(3, 3, TropicalWeight(4.0), s2));

    let total_dist = fst.shortest_distance(1e-6).unwrap();
    assert_eq!(total_dist.value(), 3.0);

    // s0=0.0, s1=1.0, s2=3.0
    let dist_vec = fst
        .shortest_distance_vec(&ShortestDistanceOptions::default())
        .unwrap();
    assert_eq!(dist_vec[0].value(), 0.0);
    assert_eq!(dist_vec[1].value(), 1.0);
    assert_eq!(dist_vec[2].value(), 3.0);

    let mut log_fst = Log64VectorFst::new();
    let ls0 = log_fst.add_state();
    let ls1 = log_fst.add_state();
    log_fst.set_start(ls0);
    log_fst.set_final(ls1, Log64Weight::one());

    let w = -0.5f64.ln();
    log_fst.add_arc(ls0, Log64Arc::new(1, 1, Log64Weight(w), ls1));
    log_fst.add_arc(ls0, Log64Arc::new(2, 2, Log64Weight(w), ls1));

    let log_total = log_fst.shortest_distance(1e-6).unwrap();
    assert!(
        log_total.value().abs() < 1e-5,
        "Sum of two 0.5 probabilities should be 1.0 (LogWeight 0.0)"
    );
}

#[test]
fn test_push_and_prune() {
    let mut fst = StdVectorFst::new();
    let s0 = fst.add_state();
    let s1 = fst.add_state();
    let s2 = fst.add_state();
    fst.set_start(s0);
    fst.set_final(s2, TropicalWeight::one());

    fst.add_arc(s0, StdArc::new(1, 1, TropicalWeight(1.0), s1));
    fst.add_arc(s1, StdArc::new(2, 2, TropicalWeight(1.0), s2));
    fst.add_arc(s0, StdArc::new(3, 3, TropicalWeight(5.0), s2));

    let mut pushed_fst = fst.clone();
    pushed_fst.push(ReweightType::ToInitial, 1e-6, true);

    let arcs_from_s0: Vec<_> = pushed_fst.arcs(s0).collect();
    let arc1 = arcs_from_s0.iter().find(|a| a.ilabel() == 1).unwrap();
    assert!(arc1.weight().value().abs() < 1e-5);

    let arc2 = arcs_from_s0.iter().find(|a| a.ilabel() == 3).unwrap();
    assert_eq!(arc2.weight().value(), 3.0);

    let opts = PruneOptions {
        weight_threshold: TropicalWeight(2.5),
        ..Default::default()
    };
    fst.prune(&opts);

    let pruned_arcs: Vec<_> = fst.arcs(s0).collect();
    assert_eq!(pruned_arcs.len(), 1, "The heavy path should be pruned");
    assert_eq!(
        pruned_arcs[0].ilabel(),
        1,
        "Only the optimal path should remain"
    );
}

#[test]
fn test_intersect_and_difference() {
    let mut fst_a = StdVectorFst::new();
    let a0 = fst_a.add_state();
    let a1 = fst_a.add_state();
    fst_a.set_start(a0);
    fst_a.set_final(a1, TropicalWeight::one());
    fst_a.add_arc(a0, StdArc::new(1, 1, TropicalWeight(0.5), a1)); // Accept "1", cost 0.5
    fst_a.add_arc(a0, StdArc::new(2, 2, TropicalWeight(0.5), a1)); // Accept "2", cost 0.5
    fst_a.arcsort_ilabel();

    let mut fst_b = StdVectorFst::new();
    let b0 = fst_b.add_state();
    let b1 = fst_b.add_state();
    fst_b.set_start(b0);
    fst_b.set_final(b1, TropicalWeight::one());
    fst_b.add_arc(b0, StdArc::new(2, 2, TropicalWeight(1.0), b1)); // Accept "2", cost 1.0
    fst_b.add_arc(b0, StdArc::new(3, 3, TropicalWeight(1.0), b1)); // Accept "3", cost 1.0
    fst_b.arcsort_ilabel();

    let mut fst_c = StdVectorFst::new();
    let c0 = fst_c.add_state();
    let c1 = fst_c.add_state();
    fst_c.set_start(c0);
    fst_c.set_final(c1, TropicalWeight::one());
    fst_c.add_arc(c0, StdArc::new(2, 2, TropicalWeight::one(), c1)); // Accept "2", unweighted
    fst_c.add_arc(c0, StdArc::new(3, 3, TropicalWeight::one(), c1)); // Accept "3", unweighted
    fst_c.arcsort_ilabel();

    // Verify properties statically required by Ops
    let acc_a = fst_a.verify::<Acceptor>().unwrap();
    let acc_b = fst_b.verify::<Acceptor>().unwrap();
    let det_c = fst_c.verify::<UnweightedDetEpsFreeAcceptor>().unwrap();

    // Intersect (A ∩ B)
    let mut intersect_fst = StdVectorFst::new();
    intersect_fst.intersect_verified(acc_a, acc_b, &ComposeOptions::default());

    let arcs: Vec<_> = intersect_fst.arcs(intersect_fst.start()).collect();
    assert_eq!(arcs.len(), 1);
    assert_eq!(arcs[0].ilabel(), 2);
    // 0.5 + 1.0
    assert_eq!(arcs[0].weight().value(), 1.5);

    // Difference (A - C)
    let mut diff_fst = StdVectorFst::new();
    diff_fst.difference_verified(acc_a, det_c, &ComposeOptions::default());

    let diff_arcs: Vec<_> = diff_fst.arcs(diff_fst.start()).collect();
    assert_eq!(diff_arcs.len(), 1);
    assert_eq!(diff_arcs[0].ilabel(), 1);
    assert_eq!(diff_arcs[0].weight().value(), 0.5);
}

#[test]
fn test_reverse() {
    let mut fst = StdVectorFst::new();
    let s0 = fst.add_state();
    let s1 = fst.add_state();
    fst.set_start(s0);
    fst.set_final(s1, TropicalWeight::one());
    // 1 -> 2 -> (end)
    fst.add_arc(s0, StdArc::new(1, 2, TropicalWeight(1.0), s1));

    let mut rev_fst = StdVectorFst::new();
    rev_fst.reverse_of(&fst, false);

    let new_start = rev_fst.start();
    let arcs: Vec<_> = rev_fst.arcs(new_start).collect();
    assert_eq!(arcs.len(), 1);
    assert_eq!(arcs[0].ilabel(), 1);
    assert_eq!(arcs[0].olabel(), 2);
}

#[test]
fn test_relabel() {
    let mut fst = StdVectorFst::new();
    let s0 = fst.add_state();
    let s1 = fst.add_state();
    fst.set_start(s0);
    fst.set_final(s1, TropicalWeight::one());

    // a(1) : x(2)
    fst.add_arc(s0, StdArc::new(1, 2, TropicalWeight(0.0), s1));

    let ipairs = vec![(1, 10)];
    let opairs = vec![(2, 20)];

    fst.relabel(&ipairs, &opairs).unwrap();

    let arc = fst.arcs(s0).next().unwrap();
    assert_eq!(arc.ilabel(), 10, "Input label should be relabeled to 10");
    assert_eq!(arc.olabel(), 20, "Output label should be relabeled to 20");
}

#[test]
fn test_replace() {
    let mut syms = SymbolTable::new("vocab");
    let the = syms.add_symbol("the", 1);
    let cat = syms.add_symbol("cat", 2);
    let dog = syms.add_symbol("dog", 3);

    let noun_nt = syms.add_symbol("<NOUN>", 10);

    // start -> "the" -> <NOUN> -> final
    let mut root_fst = StdVectorFst::new();
    let r0 = root_fst.add_state();
    let r1 = root_fst.add_state();
    let r2 = root_fst.add_state();
    root_fst.set_start(r0);
    root_fst.set_final(r2, TropicalWeight::one());

    // r0 --(the)--> r1
    root_fst.add_arc(
        r0,
        StdArc::new(the as i32, the as i32, TropicalWeight(0.5), r1),
    );
    // r1 --(<NOUN>)--> r2
    root_fst.add_arc(
        r1,
        StdArc::new(noun_nt as i32, noun_nt as i32, TropicalWeight::one(), r2),
    );

    let mut noun_fst = StdVectorFst::new();
    let n0 = noun_fst.add_state();
    let n1 = noun_fst.add_state();
    noun_fst.set_start(n0);
    noun_fst.set_final(n1, TropicalWeight::one());

    noun_fst.add_arc(
        n0,
        StdArc::new(cat as i32, cat as i32, TropicalWeight(1.0), n1),
    );
    noun_fst.add_arc(
        n0,
        StdArc::new(dog as i32, dog as i32, TropicalWeight(2.0), n1),
    );

    let root_label = 0;
    let pairs = vec![(root_label, &root_fst), (noun_nt as i32, &noun_fst)];

    let mut replaced_fst = StdVectorFst::new();

    let opts = ReplaceOptions {
        root: root_label as i64,
        call_label_type: ReplaceLabelType::Neither,
        return_label_type: ReplaceLabelType::Neither,
        ..Default::default()
    };

    replaced_fst
        .replace_of(&pairs, &opts)
        .expect("Replace failed");

    replaced_fst.rmepsilon(&RmEpsilonOptions::default());
    let mut det_fst = StdVectorFst::new();
    det_fst.determinize(&replaced_fst, &DeterminizeOptions::default());

    let start = det_fst.start();
    let arcs_from_start: Vec<_> = det_fst.arcs(start).collect();

    assert_eq!(arcs_from_start.len(), 1);
    assert_eq!(arcs_from_start[0].ilabel(), the as i32);

    let mid_state = arcs_from_start[0].nextstate();
    let mut next_arcs: Vec<_> = det_fst.arcs(mid_state).collect();
    next_arcs.sort_by_key(|a| a.ilabel());

    assert_eq!(next_arcs.len(), 2, "Should branch into 'cat' and 'dog'");
    assert_eq!(next_arcs[0].ilabel(), cat as i32);
    assert_eq!(next_arcs[1].ilabel(), dog as i32);

    let cost_the = 0.5;
    assert_eq!(
        arcs_from_start[0].weight().value() + next_arcs[0].weight().value(),
        cost_the + 1.0
    );
    assert_eq!(
        arcs_from_start[0].weight().value() + next_arcs[1].weight().value(),
        cost_the + 2.0
    );
}

#[test]
fn test_encode_decode() {
    let mut fst = StdVectorFst::new();
    let s0 = fst.add_state();
    let s1 = fst.add_state();
    fst.set_start(s0);
    fst.set_final(s1, TropicalWeight::one());

    // transducer arc: 1:2, weight: 0.5
    fst.add_arc(s0, StdArc::new(1, 2, TropicalWeight(0.5), s1));

    // Encode
    let mut mapper = EncodeMapper::new(ENCODE_LABELS | ENCODE_WEIGHTS);
    {
        let verified_acceptor = fst.encode(&mut mapper);
        let arc = verified_acceptor.arcs(s0).next().unwrap();
        assert_eq!(arc.ilabel(), arc.olabel());
        assert_eq!(arc.weight().value(), 0.0);
    }

    // Decode
    fst.decode(&mapper);

    let arc = fst.arcs(s0).next().unwrap();
    assert_eq!(arc.ilabel(), 1);
    assert_eq!(arc.olabel(), 2);
    assert_eq!(arc.weight().value(), 0.5);
}

#[test]
fn test_encode_decode_io() {
    let mut fst = StdVectorFst::new();
    let s0 = fst.add_state();
    let s1 = fst.add_state();
    fst.set_start(s0);
    fst.set_final(s1, TropicalWeight::one());
    fst.add_arc(s0, StdArc::new(1, 2, TropicalWeight(0.5), s1));

    let mut mapper = EncodeMapper::new(ENCODE_LABELS | ENCODE_WEIGHTS);

    let _ = fst.encode(&mut mapper);

    let tmp_file = NamedTempFile::new().unwrap();
    let tmp_path = tmp_file.path();
    assert!(mapper.write(tmp_path), "Failed to write mapper");

    let loaded_mapper = EncodeMapper::<StdArc>::read(tmp_path).expect("Failed to read mapper");

    assert_eq!(loaded_mapper.flags(), ENCODE_LABELS | ENCODE_WEIGHTS);

    fst.decode(&loaded_mapper);

    let arc = fst.arcs(s0).next().unwrap();
    assert_eq!(arc.ilabel(), 1);
    assert_eq!(arc.olabel(), 2);
    assert_eq!(arc.weight().value(), 0.5);
}

#[test]
fn test_arc_map_all_mappers() {
    let mut fst = StdVectorFst::new();
    let s0 = fst.add_state();
    let s1 = fst.add_state();
    fst.set_start(s0);
    fst.set_final(s1, TropicalWeight(2.0));
    fst.add_arc(s0, StdArc::new(1, 2, TropicalWeight(3.0), s1));

    // Identity
    let mut out1 = StdVectorFst::new();
    out1.map_of(&fst, &MapMapper::Identity).unwrap();
    assert_eq!(out1.arcs(s0).next().unwrap().ilabel(), 1);

    // InputEpsilon
    let mut out2 = StdVectorFst::new();
    out2.map_of(&fst, &MapMapper::InputEpsilon).unwrap();
    assert_eq!(out2.arcs(s0).next().unwrap().ilabel(), 0);
    assert_eq!(out2.arcs(s0).next().unwrap().olabel(), 2);

    // OutputEpsilon
    let mut out3 = StdVectorFst::new();
    out3.map_of(&fst, &MapMapper::OutputEpsilon).unwrap();
    assert_eq!(out3.arcs(s0).next().unwrap().ilabel(), 1);
    assert_eq!(out3.arcs(s0).next().unwrap().olabel(), 0);

    // ReverseWeight
    fst.map(&MapMapper::ReverseWeight).unwrap();

    // InvertWeight (1/3.0 = -3.0 in Tropical)
    fst.map(&MapMapper::InvertWeight).unwrap();
    assert_eq!(fst.arcs(s0).next().unwrap().weight().value(), -3.0);
}

#[test]
fn test_arc_map_type_conversion_crossproduct() {
    let mut fst_std = StdVectorFst::new();
    let s0 = fst_std.add_state();
    let s1 = fst_std.add_state();
    fst_std.set_start(s0);
    fst_std.set_final(s1, TropicalWeight::one());
    fst_std.add_arc(s0, StdArc::new(1, 2, TropicalWeight(2.0), s1));

    // Std -> Log64
    let mut fst_log64 = Log64VectorFst::new();
    fst_log64.map_type_of(&fst_std);

    assert_eq!(fst_log64.num_states(), 2);
    assert_eq!(fst_log64.arcs(s0).next().unwrap().weight().value(), 2.0);

    // Log64 -> Std
    let mut fst_std_back = StdVectorFst::new();
    fst_std_back.map_type_of(&fst_log64);

    assert_eq!(fst_std_back.num_states(), 2);
    assert_eq!(fst_std_back.arcs(s0).next().unwrap().weight().value(), 2.0);
}

#[test]
fn test_arc_map_gallic() {
    let mut fst = StdVectorFst::new();
    let s0 = fst.add_state();
    let s1 = fst.add_state();
    fst.set_start(s0);
    fst.set_final(s1, TropicalWeight::one());
    fst.add_arc(s0, StdArc::new(1, 2, TropicalWeight(2.0), s1));

    // ToGallic (A -> GallicArc<A>)
    let gallic = StdVectorFst::map_to_gallic(&fst);

    // FromGallic (GallicArc<A> -> A)
    let mut fst_out = StdVectorFst::new();
    fst_out.map_from_gallic(&gallic, 0);

    // The FST is restored
    assert_eq!(fst_out.num_states(), 2);
    let arc = fst_out.arcs(fst_out.start()).next().unwrap();
    assert_eq!(arc.ilabel(), 1);
    assert_eq!(arc.olabel(), 2);
    assert_eq!(arc.weight().value(), 2.0);

    // GallicToNewSymbols
    let mut new_sym_fst = StdVectorFst::new();
    new_sym_fst.gallic_to_new_symbols(&gallic);
    assert!(new_sym_fst.num_states() >= 2);
}

#[test]
fn test_lazy_arc_map() {
    let mut fst = StdVectorFst::new();
    let s0 = fst.add_state();
    let s1 = fst.add_state();
    fst.set_start(s0);
    fst.set_final(s1, TropicalWeight(2.0));
    fst.add_arc(s0, StdArc::new(1, 2, TropicalWeight(3.0), s1));

    // Lazy Identity Map
    let lazy_id = StdArcMapFst::new(&fst, &MapMapper::Identity).unwrap();
    assert_eq!(lazy_id.arcs(s0).next().unwrap().ilabel(), 1);

    // Lazy InputEpsilon Map
    let lazy_eps = StdArcMapFst::new(&fst, &MapMapper::InputEpsilon).unwrap();
    assert_eq!(lazy_eps.arcs(s0).next().unwrap().ilabel(), 0);
    assert_eq!(lazy_eps.arcs(s0).next().unwrap().olabel(), 2);
}

#[test]
fn test_lazy_arc_map_convert() {
    let mut fst_std = StdVectorFst::new();
    let s0 = fst_std.add_state();
    let s1 = fst_std.add_state();
    fst_std.set_start(s0);
    fst_std.set_final(s1, TropicalWeight::one());
    fst_std.add_arc(s0, StdArc::new(1, 2, TropicalWeight(2.0), s1));

    // Lazy conversion from StdArc -> Log64Arc
    let lazy_log64 =
        ArcMapConvertFst::<openfst_rs::arc::Log64Arc>::new::<openfst_rs::arc::StdArc, _>(&fst_std);
    let arc = lazy_log64.arcs(s0).next().unwrap();
    assert_eq!(arc.weight().value(), 2.0);
}

#[test]
fn test_reweight() {
    use openfst_rs::ops::reweight::Reweight;

    let mut fst = StdVectorFst::new();
    let s0 = fst.add_state(); // state 0 (Start)
    let s1 = fst.add_state(); // state 1 (Final)
    fst.set_start(s0);
    fst.set_final(s1, TropicalWeight(2.0));

    // s0 -> s1, cost 1.0
    fst.add_arc(s0, StdArc::new(1, 1, TropicalWeight(1.0), s1));

    // Provide potentials indexed by state ID.
    // p[0] = 3.0, p[1] = 5.0 (cost/weight values)
    let potentials = [TropicalWeight(3.0), TropicalWeight(5.0)];

    // Reweight towards initial
    // Math for Tropical semiring (cost):
    // - arc_weight(0->1) = w + p[1] - p[0] = 1.0 + 5.0 - 3.0 = 3.0
    // - final_weight(1)  = f - p[1]        = 2.0 - 5.0 = -3.0
    // - Start state compensation to preserve total path weight:
    //    Because s0 is acyclic, p[0] is added to all outgoing arcs from s0.
    //    final arc_weight(0->1) = 3.0 + p[0] = 3.0 + 3.0 = 6.0.
    //
    // Total path weight check:
    // Original: 1.0 + 2.0 = 3.0
    // Reweighted: 6.0 + (-3.0) = 3.0
    fst.reweight(&potentials, ReweightType::ToInitial);

    let arc = fst.arcs(s0).next().unwrap();
    assert_eq!(
        arc.weight().value(),
        6.0,
        "Arc weight should be updated to 6.0"
    );
    assert_eq!(
        fst.final_weight(s1).value(),
        -3.0,
        "Final weight should be updated to -3.0"
    );
}

#[test]
fn test_synchronize() {
    use openfst_rs::ops::synchronize::{Synchronize, SynchronizeFst};

    let mut fst = StdVectorFst::new();
    let s0 = fst.add_state();
    let s1 = fst.add_state();
    let s2 = fst.add_state();

    fst.set_start(s0);
    fst.set_final(s2, TropicalWeight(0.0));

    // s0 --(1:eps)--> s1
    fst.add_arc(s0, StdArc::new(1, EPSILON, TropicalWeight(0.5), s1));
    // s1 --(eps:2)--> s2
    fst.add_arc(s1, StdArc::new(EPSILON, 2, TropicalWeight(1.0), s2));

    // Test Eager Synchronize
    let mut sync_fst = StdVectorFst::new();
    sync_fst.synchronize_of(&fst);

    // In the synchronized version, the algorithm delays outputting labels
    // until it has both an input and an output to emit simultaneously.
    //
    // Reads "1", but output is "eps". It emits (eps:eps, 0.5)
    // and buffers "1" in its internal state string.
    let start = sync_fst.start();
    let arc1 = sync_fst.arcs(start).next().unwrap();
    assert_eq!(arc1.ilabel(), EPSILON);
    assert_eq!(arc1.olabel(), EPSILON);
    assert_eq!(arc1.weight().value(), 0.5);

    // From the buffered state, it reads "eps" and outputs "2".
    // It pairs the buffered "1" with the output "2", emitting (1:2, 1.0).
    let mid_state = arc1.nextstate();
    let arc2 = sync_fst.arcs(mid_state).next().unwrap();
    assert_eq!(arc2.ilabel(), 1);
    assert_eq!(arc2.olabel(), 2);
    assert_eq!(arc2.weight().value(), 1.0);

    // Test Lazy Synchronize
    let lazy_sync = SynchronizeFst::new(&fst);
    let lazy_start = lazy_sync.start();
    let lazy_arc1 = lazy_sync.arcs(lazy_start).next().unwrap();
    assert_eq!(lazy_arc1.ilabel(), EPSILON);

    let lazy_mid_state = lazy_arc1.nextstate();
    let lazy_arc2 = lazy_sync.arcs(lazy_mid_state).next().unwrap();
    assert_eq!(lazy_arc2.ilabel(), 1);
    assert_eq!(lazy_arc2.olabel(), 2);
}

#[test]
fn test_epsnormalize() {
    let mut fst = StdVectorFst::new();
    let s0 = fst.add_state();
    let s1 = fst.add_state();
    let s2 = fst.add_state();
    fst.set_start(s0);
    fst.set_final(s2, TropicalWeight::one());

    // s0 --(eps:1)--> s1
    fst.add_arc(s0, StdArc::new(EPSILON, 1, TropicalWeight(0.5), s1));
    // s1 --(2:eps)--> s2
    fst.add_arc(s1, StdArc::new(2, EPSILON, TropicalWeight(1.0), s2));

    // Test EpsNormalizeType::Input
    // Input-normalized means epsilon inputs are pushed towards the end of paths.
    let mut out_in = StdVectorFst::new();
    out_in.epsnormalize_of(&fst, EpsNormalizeType::Input);

    let arc_in = out_in.arcs(out_in.start()).next().unwrap();
    assert_eq!(arc_in.ilabel(), 2);
    assert_eq!(arc_in.olabel(), 1);

    // Test EpsNormalizeType::Output
    // Output-normalized means epsilon outputs are pushed towards the end of paths.
    let mut out_out = StdVectorFst::new();
    out_out.epsnormalize_of(&fst, EpsNormalizeType::Output);

    let arc_out = out_out.arcs(out_out.start()).next().unwrap();
    assert_eq!(arc_out.olabel(), 1);
    assert_eq!(arc_out.ilabel(), EPSILON);
}

#[test]
fn test_randgen() {
    let mut fst = StdVectorFst::new();
    let s0 = fst.add_state();
    let s1 = fst.add_state();
    let s2 = fst.add_state();

    fst.set_start(s0);
    fst.set_final(s2, TropicalWeight::one());

    // State 0 branches to s1 or s2
    fst.add_arc(s0, StdArc::new(1, 1, TropicalWeight(0.5), s1));
    fst.add_arc(s0, StdArc::new(2, 2, TropicalWeight(0.5), s2));

    // State 1 branches to s2
    fst.add_arc(s1, StdArc::new(3, 3, TropicalWeight(1.0), s2));

    let mut out_fst = StdVectorFst::new();

    // Generate 5 random paths
    let opts = RandGenOptions {
        selector_type: ArcSelectorType::Uniform,
        npath: 5,
        weighted: false,
        ..Default::default()
    };

    out_fst.randgen_of(&fst, &opts).unwrap();

    // With weighted = false, the output is an unweighted DAG tree of the generated paths.
    // It should contain states and arcs.
    assert!(out_fst.num_states() >= 2);
}

#[test]
fn test_rmfinalepsilon() {
    let mut fst = StdVectorFst::new();
    let s0 = fst.add_state(); // start
    let s1 = fst.add_state(); // final
    let s2 = fst.add_state(); // final, but only reachable via eps from s1

    fst.set_start(s0);
    fst.set_final(s1, TropicalWeight(1.0));
    fst.set_final(s2, TropicalWeight(2.0));

    // s0 --(a(1):b(2))--> s1
    fst.add_arc(s0, StdArc::new(1, 2, TropicalWeight(0.5), s1));
    // s1 --(eps:eps)--> s2
    fst.add_arc(s1, StdArc::new(EPSILON, EPSILON, TropicalWeight(1.5), s2));

    // State 2 is a final state that is only reached via an epsilon arc.
    // RmFinalEpsilon should remove state 2, and transfer its properties (final weight and path weight) back to state 1.
    fst.rmfinalepsilon();

    let final_weight = fst.final_weight(s1);

    // original s1 final weight (1.0) PLUS (in Tropical, PLUS = min)
    // the epsilon path weight to s2 (1.5) TIMES (in Tropical, TIMES = add) final weight of s2 (2.0)
    // => min(1.0, 1.5 + 2.0) = min(1.0, 3.5) = 1.0
    assert_eq!(final_weight.value(), 1.0);

    // Depending on the internals of RmFinalEpsilon and Connect, s2 may be deleted or isolated.
    // We verify that s1 no longer has the epsilon arc.
    assert_eq!(fst.num_arcs(s1), 0);
}

#[test]
fn test_statesort() {
    let mut fst = StdVectorFst::new();
    let s0 = fst.add_state(); // ID = 0
    let s1 = fst.add_state(); // ID = 1

    fst.set_start(s0);
    fst.set_final(s1, TropicalWeight::one());

    // s0 -> s1
    fst.add_arc(s0, StdArc::new(1, 2, TropicalWeight(0.5), s1));

    // Create a permutation array: swap 0 and 1.
    // "order[i] is the NEW state ID for the OLD state ID i"
    // order[0] = 1 (old start state becomes ID 1)
    // order[1] = 0 (old final state becomes ID 0)
    let order = vec![1, 0];

    fst.statesort(&order);

    assert_eq!(fst.start(), 1);

    // Final(s1) -> Final(0)
    assert_eq!(fst.final_weight(0).value(), 0.0);

    // The arc should now go from state 1 to state 0.
    let arc = fst.arcs(1).next().unwrap();
    assert_eq!(arc.nextstate(), 0);
    assert_eq!(arc.weight().value(), 0.5);
}

#[test]
fn test_verify_detects_error() {
    let mut fst = StdVectorFst::new();
    let s0 = fst.add_state();
    fst.set_start(s0);

    // Deliberately introduce corruption by adding a negative label.
    // Normally labels < 0 are invalid (except epsilon = 0).
    fst.add_arc(s0, StdArc::new(-10, 1, TropicalWeight(1.0), s0));

    // verification without allowing negative labels should fail
    let res = fst.verify_fst(false);
    assert!(res.is_err());

    let err_msg = res.unwrap_err().to_string();
    assert!(
        err_msg.contains("is negative"),
        "Should detect negative input label"
    );
}

#[test]
fn test_disambiguate() {
    let mut fst = StdVectorFst::new();
    let s0 = fst.add_state();
    let s1 = fst.add_state();
    let s2 = fst.add_state();
    let s3 = fst.add_state();
    fst.set_start(s0);
    fst.set_final(s3, TropicalWeight::one());

    // Path 1 (Cost = 3.0)
    fst.add_arc(s0, StdArc::new(1, 1, TropicalWeight(1.0), s1));
    fst.add_arc(s1, StdArc::new(2, 2, TropicalWeight(2.0), s3));

    // Path 2 (Cost = 7.0)
    fst.add_arc(s0, StdArc::new(1, 1, TropicalWeight(3.0), s2));
    fst.add_arc(s2, StdArc::new(2, 2, TropicalWeight(4.0), s3));

    let mut dis_fst = StdVectorFst::new();
    dis_fst.disambiguate(&fst, &DisambiguateOptions::default());

    // Disambiguate makes it so there are not two distinct paths with the same input sequence.
    // The FST should successfully disambiguate (the optimal path remains).
    assert!(dis_fst.num_states() > 0);

    let arcs_from_start: Vec<_> = dis_fst.arcs(dis_fst.start()).collect();
    assert_eq!(
        arcs_from_start.len(),
        1,
        "Should have a single ambiguous-free path for label 1"
    );
}
