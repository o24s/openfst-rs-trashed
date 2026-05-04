#[cxx::bridge(namespace = "fst_rust::ffi")]
pub mod bridge {
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    struct ExpectationWeightValueF32 {
        w1: f32,
        w2: f32,
    }

    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    struct ExpectationWeightValueF64 {
        w1: f64,
        w2: f64,
    }

    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    struct ExpectationWeightValuePairF32 {
        w1_1: f32,
        w1_2: f32,
        w2_1: f32,
        w2_2: f32,
    }

    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    struct ExpectationWeightValuePairF64 {
        w1_1: f64,
        w1_2: f64,
        w2_1: f64,
        w2_2: f64,
    }

    extern "Rust" {
        fn _force_cxx_vec_f32_expectation(v: Vec<ExpectationWeightValueF32>);
        fn _force_cxx_vec_f32_expectation_pair(v: Vec<ExpectationWeightValuePairF32>);
        fn _force_cxx_vec_f64_expectation(v: Vec<ExpectationWeightValueF64>);
        fn _force_cxx_vec_f64_expectation_pair(v: Vec<ExpectationWeightValuePairF64>);
    }
}
pub use bridge::*;

fn _force_cxx_vec_f32_expectation(_v: Vec<ExpectationWeightValueF32>) {}
fn _force_cxx_vec_f32_expectation_pair(_v: Vec<ExpectationWeightValuePairF32>) {}
fn _force_cxx_vec_f64_expectation(_v: Vec<ExpectationWeightValueF64>) {}
fn _force_cxx_vec_f64_expectation_pair(_v: Vec<ExpectationWeightValuePairF64>) {}
