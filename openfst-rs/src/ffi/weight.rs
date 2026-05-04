#[cxx::bridge(namespace = "fst_rust::ffi")]
pub mod bridge {
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    struct PairWeightValueF32 {
        w1: f32,
        w2: f32,
    }

    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    struct PairWeightValueF64 {
        w1: f32,
        w2: f64,
    }

    extern "Rust" {
        fn _force_cxx_vec_f32(v: Vec<PairWeightValueF32>);
        fn _force_cxx_vec_f64(v: Vec<PairWeightValueF64>);
    }
}
pub use bridge::{PairWeightValueF32, PairWeightValueF64};

fn _force_cxx_vec_f32(_v: Vec<PairWeightValueF32>) {}
fn _force_cxx_vec_f64(_v: Vec<PairWeightValueF64>) {}
