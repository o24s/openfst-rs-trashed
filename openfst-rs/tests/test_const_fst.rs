use openfst_rs::arc::{Arc, StdArc};
use openfst_rs::const_fst::StdConstFst;
use openfst_rs::float_weight::TropicalWeight;
use openfst_rs::fst::{DynamicFst, Fst, MutableFst};
use openfst_rs::ops::shortest_distance::ShortestDistance;
use openfst_rs::vector_fst::StdVectorFst;
use tempfile::NamedTempFile;

#[test]
fn test_const_fst_lifecycle() {
    let tmp_file = NamedTempFile::new().unwrap();
    let tmp_file_path = tmp_file.path();

    let mut vec_fst = StdVectorFst::new();
    let s0 = vec_fst.add_state();
    let s1 = vec_fst.add_state();
    vec_fst.set_start(s0);
    vec_fst.set_final(s1, TropicalWeight(2.0));
    vec_fst.add_arc(s0, StdArc::new(1, 2, TropicalWeight(1.5), s1));

    let const_fst = StdConstFst::new_from_fst(&vec_fst);

    assert!(const_fst.fst_type().starts_with("const"));
    assert_eq!(const_fst.num_states_if_known(), Some(2));

    assert!(
        const_fst.write(tmp_file_path),
        "Failed to write ConstFst to file"
    );

    let dyn_fst = DynamicFst::<StdArc>::read(tmp_file_path).expect("Failed to read via DynamicFst");
    assert!(dyn_fst.fst_type().starts_with("const"));

    let loaded_const = StdConstFst::read(tmp_file_path).expect("Failed to read ConstFst directly");

    let dist = loaded_const.shortest_distance(1e-5).unwrap();
    assert_eq!(dist.value(), 1.5 + 2.0);
}
