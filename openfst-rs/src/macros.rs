/// Creates a linear FST from a string or an iterable of labels.
///
/// # Examples
/// ```
/// use openfst_rs::{fst_linear, vector_fst::StdVectorFst};
///
/// let fst = fst_linear!(StdVectorFst, "hello");
/// let fst_from_array = fst_linear!(StdVectorFst, [1, 2, 3]);
/// ```
#[macro_export]
macro_rules! fst_linear {
    ($fst_type:ty, [ $($label:expr),* $(,)? ]) => {{
        let mut fst = <$fst_type>::new();
        let labels = vec![$($label),*];
        $crate::string::StringCompiler::compile_iter(&mut fst, labels);
        fst
    }};

    ($fst_type:ty, $text:expr) => {{
        let mut fst = <$fst_type>::new();
        $crate::string::StringCompiler::compile_bytes(&mut fst, $text);
        fst
    }};
}
