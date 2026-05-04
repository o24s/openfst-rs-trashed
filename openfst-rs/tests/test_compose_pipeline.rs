use openfst_rs::arc::{Arc, StdArc};
use openfst_rs::compose_fst::StdComposeFst;
use openfst_rs::float_weight::TropicalWeight;
use openfst_rs::fst::{Fst, MutableFst};
use openfst_rs::ops::arcsort::ArcSort;
use openfst_rs::ops::compose::{Compose, ComposeOptions};
use openfst_rs::ops::shortest_path::{ShortestPath, ShortestPathOptions};
use openfst_rs::symbol_table::SymbolTable;
use openfst_rs::vector_fst::StdVectorFst;
use openfst_rs::weight::Weight;

fn build_test_fsts() -> (StdVectorFst, StdVectorFst) {
    let mut syms = SymbolTable::new("vocab");
    syms.add_symbol("eps", 0);
    syms.add_symbol("a", 1);
    syms.add_symbol("x", 2);
    syms.add_symbol("y", 3);

    let mut fst1 = StdVectorFst::new();
    fst1.set_input_symbols(Some(&syms));
    fst1.set_output_symbols(Some(&syms));
    let s0 = fst1.add_state();
    let s1 = fst1.add_state();
    fst1.set_start(s0);
    fst1.set_final(s1, TropicalWeight::one());
    fst1.add_arc(s0, StdArc::new(1, 2, TropicalWeight(0.5), s1));
    fst1.arcsort_olabel();

    let mut fst2 = StdVectorFst::new();
    fst2.set_input_symbols(Some(&syms));
    fst2.set_output_symbols(Some(&syms));
    let s0 = fst2.add_state();
    let s1 = fst2.add_state();
    fst2.set_start(s0);
    fst2.set_final(s1, TropicalWeight::one());
    fst2.add_arc(s0, StdArc::new(2, 3, TropicalWeight(1.5), s1));
    fst2.arcsort_ilabel();

    (fst1, fst2)
}

#[test]
fn test_eager_compose_pipeline() {
    let (fst1, fst2) = build_test_fsts();

    // Eager composition
    let mut composed = StdVectorFst::new();
    composed.compose(&fst1, &fst2, &ComposeOptions::default());

    let mut shortest = StdVectorFst::new();
    shortest.shortest_path(&composed, &ShortestPathOptions::default());

    let mut arc_iter = shortest.arcs(shortest.start());
    let arc = arc_iter.next().unwrap();

    assert_eq!(arc.ilabel(), 1, "Input label should be 'a'(1)");
    assert_eq!(arc.olabel(), 3, "Output label should be 'y'(3)");
    assert_eq!(
        arc.weight().value(),
        2.0,
        "Weight should be 0.5 + 1.5 = 2.0"
    );

    let next_state = arc.nextstate();
    assert_eq!(
        shortest.final_weight(next_state).value(),
        0.0,
        "Final weight should be 0.0"
    );
}

#[test]
fn test_lazy_compose_pipeline() {
    let (fst1, fst2) = build_test_fsts();

    let lazy_composed = StdComposeFst::new(&fst1, &fst2);

    let mut shortest = StdVectorFst::new();
    shortest.shortest_path(&lazy_composed, &ShortestPathOptions::default());

    let mut arc_iter = shortest.arcs(shortest.start());
    let arc = arc_iter.next().unwrap();

    assert_eq!(arc.ilabel(), 1, "Lazy Input label should be 'a'(1)");
    assert_eq!(arc.olabel(), 3, "Lazy Output label should be 'y'(3)");
    assert_eq!(
        arc.weight().value(),
        2.0,
        "Lazy Weight should be 0.5 + 1.5 = 2.0"
    );
}
