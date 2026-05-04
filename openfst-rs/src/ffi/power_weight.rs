#[cxx::bridge(namespace = "fst_rust::ffi")]
pub mod bridge {
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    struct PowerWeightValueF32_3 {
        w: [f32; 3],
    }

    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    struct PowerWeightValueF64_3 {
        w: [f64; 3],
    }

    extern "Rust" {
        fn _force_cxx_vec_f32_power_3(v: Vec<PowerWeightValueF32_3>);
        fn _force_cxx_vec_f64_power_3(v: Vec<PowerWeightValueF64_3>);
    }
}
pub use bridge::*;

fn _force_cxx_vec_f32_power_3(_v: Vec<PowerWeightValueF32_3>) {}
fn _force_cxx_vec_f64_power_3(_v: Vec<PowerWeightValueF64_3>) {}
