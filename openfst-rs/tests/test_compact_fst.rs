use openfst_rs::fst::ExpandedFst;
use openfst_rs::{
    arc::{Arc, StdArc},
    compact_fst::StdCompactStringFst,
    float_weight::TropicalWeight,
    fst::{DynamicFst, Fst, MutableFst},
    vector_fst::StdVectorFst,
    weight::Weight,
};
use tempfile::NamedTempFile;

#[test]
fn test_compact_string_fst_lifecycle() {
    let tmp_file = NamedTempFile::new().unwrap();
    let tmp_file_path = tmp_file.path();

    let mut vec_fst = StdVectorFst::new();
    let s0 = vec_fst.add_state();
    let s1 = vec_fst.add_state();
    let s2 = vec_fst.add_state();
    vec_fst.set_start(s0);
    vec_fst.set_final(s2, TropicalWeight::one());

    // Linear string: 'a'(1) -> 'b'(2)
    vec_fst.add_arc(s0, StdArc::new(1, 1, TropicalWeight::one(), s1));
    vec_fst.add_arc(s1, StdArc::new(2, 2, TropicalWeight::one(), s2));

    let compact_fst = StdCompactStringFst::new_from_fst(&vec_fst);

    assert!(compact_fst.fst_type().starts_with("compact"));
    assert_eq!(compact_fst.num_states(), 3);

    let arc0 = compact_fst.arcs(s0).next().unwrap();
    assert_eq!(arc0.ilabel(), 1);

    assert!(
        compact_fst.write(tmp_file_path),
        "Failed to write CompactStringFst to file"
    );

    let dyn_fst = DynamicFst::<StdArc>::read(tmp_file_path).expect("Failed to read via DynamicFst");
    assert!(dyn_fst.fst_type().starts_with("compact"));
    assert_eq!(dyn_fst.num_states_if_known(), Some(3));
}
