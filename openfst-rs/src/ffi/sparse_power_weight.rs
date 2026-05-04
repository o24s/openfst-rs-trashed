#[cxx::bridge(namespace = "fst_rust::ffi")]
pub mod bridge {
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    struct SparseTupleElementF32 {
        key: i32,
        weight: f32,
    }

    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    struct SparseTupleElementF64 {
        key: i32,
        weight: f64,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct SparsePowerWeightValueF32 {
        default_weight: f32,
        elements_ptr: *const SparseTupleElementF32,
        elements_len: usize,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct SparsePowerWeightValueF64 {
        default_weight: f64,
        elements_ptr: *const SparseTupleElementF64,
        elements_len: usize,
    }

    extern "Rust" {
        fn _force_cxx_vec_sparse_element_f32(v: Vec<SparseTupleElementF32>);
        fn _force_cxx_vec_sparse_element_f64(v: Vec<SparseTupleElementF64>);
        fn _force_cxx_vec_sparse_power_f32(v: Vec<SparsePowerWeightValueF32>);
        fn _force_cxx_vec_sparse_power_f64(v: Vec<SparsePowerWeightValueF64>);
    }
}
pub use bridge::*;

fn _force_cxx_vec_sparse_element_f32(_v: Vec<SparseTupleElementF32>) {}
fn _force_cxx_vec_sparse_element_f64(_v: Vec<SparseTupleElementF64>) {}
fn _force_cxx_vec_sparse_power_f32(_v: Vec<SparsePowerWeightValueF32>) {}
fn _force_cxx_vec_sparse_power_f64(_v: Vec<SparsePowerWeightValueF64>) {}
