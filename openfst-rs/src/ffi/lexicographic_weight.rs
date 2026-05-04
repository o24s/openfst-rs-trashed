#[cxx::bridge(namespace = "fst_rust::ffi")]
pub mod bridge {
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    struct LexicographicWeightValueF32 {
        w1: f32,
        w2: f32,
    }

    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    struct LexicographicWeightValueF64 {
        w1: f64,
        w2: f64,
    }

    extern "Rust" {
        fn _force_cxx_vec_f32_lexicographic(v: Vec<LexicographicWeightValueF32>);
        fn _force_cxx_vec_f64_lexicographic(v: Vec<LexicographicWeightValueF64>);
    }
}
pub use bridge::{LexicographicWeightValueF32, LexicographicWeightValueF64};

fn _force_cxx_vec_f32_lexicographic(_v: Vec<LexicographicWeightValueF32>) {}
fn _force_cxx_vec_f64_lexicographic(_v: Vec<LexicographicWeightValueF64>) {}
