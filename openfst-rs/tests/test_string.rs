use openfst_rs::fst::Fst;
use openfst_rs::fst_linear;
use openfst_rs::string::StringPrinter;
use openfst_rs::vector_fst::StdVectorFst;

#[test]
fn test_pure_rust_string_compiler() {
    let fst = fst_linear!(StdVectorFst, "hello");

    assert_eq!(fst.num_states_if_known(), Some(6));

    let (s, w) = fst.print_bytes().unwrap();
    assert_eq!(s, "hello");
    assert_eq!(w.value(), 0.0); // TropicalWeight::one() = 0.0
}

#[test]
fn test_pure_rust_array_compiler() {
    let fst = fst_linear!(StdVectorFst, [10, 20, 30]);

    let (labels, _) = fst.string_to_output_labels().unwrap();
    assert_eq!(labels, vec![10, 20, 30]);
}
