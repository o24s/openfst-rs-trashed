use openfst_rs::arc::{Arc, LexicographicArc};
use openfst_rs::fst::{Fst, MutableFst};
use openfst_rs::lexicographic_weight::{
    LexicographicMinMaxTropicalWeight, LexicographicMinMaxWeight, LexicographicWeight,
};
use openfst_rs::ops::shortest_path::{ShortestPath, ShortestPathOptions};
use openfst_rs::vector_fst::VectorFst;
use openfst_rs::weight::Weight;

#[test]
fn test_lexicographic_weight_ops() {
    let w1 = LexicographicWeight::new(1.0, 5.0);
    let w2 = LexicographicWeight::new(2.0, 1.0);
    let w3 = LexicographicWeight::new(1.0, 3.0);

    // min((1.0, 5.0), (2.0, 1.0)) = (1.0, 5.0)
    let p1 = LexicographicWeight::plus(&w1, &w2);
    assert_eq!(p1.value().w1, 1.0);
    assert_eq!(p1.value().w2, 5.0);

    // min((1.0, 5.0), (1.0, 3.0)) = (1.0, 3.0)
    let p2 = LexicographicWeight::plus(&w1, &w3);
    assert_eq!(p2.value().w1, 1.0);
    assert_eq!(p2.value().w2, 3.0);

    let t1 = LexicographicWeight::times(&w1, &w2);
    assert_eq!(t1.value().w1, 3.0);
    assert_eq!(t1.value().w2, 6.0);
}

#[test]
fn test_lexicographic_shortest_path() {
    let mut fst = VectorFst::<LexicographicArc>::new();
    let s0 = fst.add_state();
    let s1 = fst.add_state();
    let s2 = fst.add_state();

    fst.set_start(s0);
    fst.set_final(s2, LexicographicWeight::one()); // One = (0.0, 0.0)

    fst.add_arc(
        s0,
        LexicographicArc::new(1, 1, LexicographicWeight::new(1.0, 5.0), s1),
    );
    fst.add_arc(
        s1,
        LexicographicArc::new(2, 2, LexicographicWeight::one(), s2),
    );

    fst.add_arc(
        s0,
        LexicographicArc::new(3, 3, LexicographicWeight::new(1.0, 3.0), s2),
    );

    fst.add_arc(
        s0,
        LexicographicArc::new(4, 4, LexicographicWeight::new(2.0, 1.0), s2),
    );

    let mut out_fst = VectorFst::<LexicographicArc>::new();
    out_fst.shortest_path(&fst, &ShortestPathOptions::default());

    let arcs: Vec<_> = out_fst.arcs(out_fst.start()).collect();
    assert_eq!(arcs.len(), 1);

    let best_weight = arcs[0].weight();
    assert_eq!(best_weight.value().w1, 1.0, "1st weight should be 1.0");
    assert_eq!(
        best_weight.value().w2,
        3.0,
        "2nd weight should break the tie and be 3.0"
    );
}

#[test]
fn test_lexicographic_minmax_tropical_ops() {
    let w1 = LexicographicMinMaxTropicalWeight::new(1.0, 5.0);
    let w2 = LexicographicMinMaxTropicalWeight::new(2.0, 1.0);

    let p = LexicographicMinMaxTropicalWeight::plus(&w1, &w2);
    assert_eq!(p.value().w1, 1.0);
    assert_eq!(p.value().w2, 5.0);

    let t = LexicographicMinMaxTropicalWeight::times(&w1, &w2);
    assert_eq!(t.value().w1, 2.0); // max(1.0, 2.0)
    assert_eq!(t.value().w2, 6.0); // 5.0 + 1.0
}

#[test]
fn test_lexicographic_minmax_ops() {
    let w1 = LexicographicMinMaxWeight::new(1.0, 5.0);
    let w2 = LexicographicMinMaxWeight::new(2.0, 1.0);

    let p = LexicographicMinMaxWeight::plus(&w1, &w2);
    assert_eq!(p.value().w1, 1.0);
    assert_eq!(p.value().w2, 5.0);

    let t = LexicographicMinMaxWeight::times(&w1, &w2);
    assert_eq!(t.value().w1, 2.0); // max(1.0, 2.0)
    assert_eq!(t.value().w2, 5.0); // max(5.0, 1.0)
}
