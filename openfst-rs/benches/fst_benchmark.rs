use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

use openfst_rs::arc::{Arc, StdArc};
use openfst_rs::float_weight::TropicalWeight;
use openfst_rs::fst::MutableFst;
use openfst_rs::ops::arcsort::ArcSort;
use openfst_rs::ops::compose::{Compose, ComposeOptions};
use openfst_rs::ops::shortest_path::{ShortestPath, ShortestPathOptions};
use openfst_rs::vector_fst::StdVectorFst as CppFst;
use openfst_rs::weight::Weight;

use rustfst::algorithms::compose::compose as rust_compose;
use rustfst::algorithms::shortest_path as rust_shortest_path;
use rustfst::algorithms::tr_sort;
use rustfst::fst_properties::FstProperties;
use rustfst::prelude::{MutableFst as _, *};

const WEIGHT_DEFAULT: f32 = 0.1;
const LABEL_EPS: u32 = 0;

// Setup functions for openfst-rs (OpenFst (C++))
fn setup_dense_cpp(frames: usize, classes: usize) -> CppFst {
    let mut fst = CppFst::new();
    let mut prev = fst.add_state();
    fst.set_start(prev);
    for _ in 0..frames {
        let next = fst.add_state();
        for c in 0..classes {
            fst.add_arc(
                prev,
                StdArc::new(c as i32, c as i32, TropicalWeight(WEIGHT_DEFAULT), next),
            );
        }
        prev = next;
    }
    fst.set_final(prev, TropicalWeight::one());
    fst
}

fn setup_ref_graph_cpp(tokens: usize) -> CppFst {
    let mut fst = CppFst::new();
    let mut curr = fst.add_state();
    fst.set_start(curr);
    for i in 0..tokens {
        let next = fst.add_state();
        fst.add_arc(
            curr,
            StdArc::new((i % 10) as i32, (i % 10) as i32, TropicalWeight(0.5), next),
        );
        fst.add_arc(
            curr,
            StdArc::new(
                LABEL_EPS as i32,
                LABEL_EPS as i32,
                TropicalWeight(1.0),
                next,
            ),
        );
        curr = next;
    }
    fst.set_final(curr, TropicalWeight::one());
    fst
}

// Setup functions for rustfst
fn setup_dense_rust(frames: usize, classes: usize) -> VectorFst<rustfst::prelude::TropicalWeight> {
    let mut fst = VectorFst::new();
    let mut prev = fst.add_state();
    fst.set_start(prev).unwrap();
    for _ in 0..frames {
        let next = fst.add_state();
        for c in 0..classes {
            fst.add_tr(prev, Tr::new(c as u32, c as u32, WEIGHT_DEFAULT, next))
                .unwrap();
        }
        prev = next;
    }
    fst.set_final(prev, 0.0).unwrap();
    fst
}

fn setup_ref_graph_rust(tokens: usize) -> VectorFst<rustfst::prelude::TropicalWeight> {
    let mut fst = VectorFst::new();
    let mut curr = fst.add_state();
    fst.set_start(curr).unwrap();
    for i in 0..tokens {
        let next = fst.add_state();
        fst.add_tr(curr, Tr::new((i % 10) as u32, (i % 10) as u32, 0.5, next))
            .unwrap();
        fst.add_tr(curr, Tr::new(LABEL_EPS, LABEL_EPS, 1.0, next))
            .unwrap();
        curr = next;
    }
    fst.set_final(curr, 0.0).unwrap();
    fst
}

fn bench_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("FST_Construction");

    for size in [1_000, 10_000, 100_000].iter() {
        group.bench_with_input(
            BenchmarkId::new("openfst-rs (OpenFst)", size),
            size,
            |b, &s| {
                b.iter(|| {
                    let mut fst = CppFst::new();
                    let mut prev = fst.add_state();
                    fst.set_start(prev);
                    for i in 1..=s {
                        let next = fst.add_state();
                        fst.add_arc(
                            prev,
                            StdArc::new(i, i, TropicalWeight(WEIGHT_DEFAULT), next),
                        );
                        prev = next;
                    }
                    fst.set_final(prev, TropicalWeight::one());
                    black_box(fst);
                });
            },
        );

        group.bench_with_input(BenchmarkId::new("Rustfst", size), size, |b, &s| {
            b.iter(|| {
                let mut fst = VectorFst::<rustfst::prelude::TropicalWeight>::new();
                let mut prev = fst.add_state();
                fst.set_start(prev).unwrap();
                for i in 1..=s {
                    let next = fst.add_state();
                    fst.add_tr(prev, Tr::new(i as u32, i as u32, WEIGHT_DEFAULT, next))
                        .unwrap();
                    prev = next;
                }
                fst.set_final(prev, 0.0).unwrap();
                black_box(fst);
            });
        });
    }
    group.finish();
}

fn bench_search_operation(c: &mut Criterion) {
    let mut group = c.benchmark_group("FST_Compose_and_ShortestPath");

    let settings = [(100, 10, 50), (500, 50, 200), (1000, 100, 500)];

    for (f, c_size, t) in settings.iter() {
        let label = format!("Frames{}_Vocab{}_Graph{}", f, c_size, t);

        // OpenFST Setup
        let cpp_lhs = setup_dense_cpp(*f, *c_size);
        let mut cpp_rhs = setup_ref_graph_cpp(*t);
        cpp_rhs.arcsort_ilabel();

        group.bench_function(BenchmarkId::new("openfst-rs (OpenFst)", &label), |b| {
            b.iter(|| {
                // Compose
                let mut lat = CppFst::new();
                lat.compose(&cpp_lhs, &cpp_rhs, &ComposeOptions::default());
                // ShortestPath
                let mut path = CppFst::new();
                path.shortest_path(&lat, &ShortestPathOptions::default());

                black_box(path);
            });
        });

        let mut rust_lhs = setup_dense_rust(*f, *c_size);
        let mut rust_rhs = setup_ref_graph_rust(*t);
        tr_sort(&mut rust_lhs, ILabelCompare {});
        tr_sort(&mut rust_rhs, ILabelCompare {});
        rust_lhs.set_properties(FstProperties::I_LABEL_SORTED);
        rust_rhs.set_properties(FstProperties::I_LABEL_SORTED);

        group.bench_function(BenchmarkId::new("Rustfst", &label), |b| {
            b.iter(|| {
                let lat: VectorFst<rustfst::prelude::TropicalWeight> =
                    rust_compose::<
                        rustfst::prelude::TropicalWeight,
                        VectorFst<rustfst::prelude::TropicalWeight>,
                        VectorFst<rustfst::prelude::TropicalWeight>,
                        _,
                        _,
                        _,
                    >(&rust_lhs, &rust_rhs)
                    .unwrap();

                let path: VectorFst<rustfst::prelude::TropicalWeight> =
                    rust_shortest_path(&lat).unwrap();

                black_box(path);
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_construction, bench_search_operation);
criterion_main!(benches);
