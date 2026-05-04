use crate::arc::{self, Arc};
use crate::error::OpenFstError;
use crate::ffi;
use crate::fst::{Fst, MutableFst};
use crate::weight::Weight;

pub trait StringCompiler<A: Arc> {
    /// Compiles an iterator of labels into a linear FST.
    fn compile_iter<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = A::Label>;

    /// Compiles a UTF-8 string into a linear FST at the byte level.
    fn compile_bytes(&mut self, text: &str);
}

impl<A, M> StringCompiler<A> for M
where
    A: Arc + ffi::fst::FstFfi,
    M: MutableFst<A>,
    <A as arc::Arc>::Weight: ffi::WeightFfi,
{
    fn compile_iter<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = A::Label>,
    {
        self.delete_all_states();
        let mut curr = self.add_state();
        self.set_start(curr);

        for label in iter {
            let next = self.add_state();
            self.add_arc(curr, A::new(label, label, A::Weight::one(), next));
            curr = next;
        }
        self.set_final(curr, A::Weight::one());
    }

    fn compile_bytes(&mut self, text: &str) {
        let iter = text.bytes().map(|b| {
            let label_val: i32 = b as i32;
            unsafe { std::mem::transmute_copy(&label_val) }
        });
        self.compile_iter(iter);
    }
}

pub trait StringPrinter<A: Arc> {
    /// Extracts the output labels of a linear FST.
    /// Returns the labels and the total path weight.
    fn string_to_output_labels(&self) -> Result<(Vec<A::Label>, A::Weight), OpenFstError>;

    /// Decodes a byte-level linear FST back to a Rust String.
    fn print_bytes(&self) -> Result<(String, A::Weight), OpenFstError>;
}

impl<A, F> StringPrinter<A> for F
where
    A: Arc + ffi::fst::FstFfi,
    A::Label: TryInto<u8> + PartialEq + Copy,
    F: Fst<A>,
    <A as arc::Arc>::Weight: ffi::WeightFfi,
{
    fn string_to_output_labels(&self) -> Result<(Vec<A::Label>, A::Weight), OpenFstError> {
        let mut labels = Vec::new();
        let mut total_weight = A::Weight::one();

        let mut curr = self.start();
        if curr < 0 {
            return Err(OpenFstError::new("Invalid start state"));
        }

        let mut final_weight = self.final_weight(curr);
        while final_weight == A::Weight::zero() {
            let mut arc_iter = self.arcs(curr);
            let arc = match arc_iter.next() {
                Some(a) => a,
                None => return Err(OpenFstError::new("Does not reach final state")),
            };

            if arc_iter.next().is_some() {
                return Err(OpenFstError::new(format!(
                    "State {} has multiple outgoing arcs",
                    curr
                )));
            }

            labels.push(arc.olabel());
            total_weight = A::Weight::times(&total_weight, arc.weight());

            curr = arc.nextstate();
            final_weight = self.final_weight(curr);
        }

        if self.num_arcs(curr) != 0 {
            return Err(OpenFstError::new(format!(
                "Final state {} has outgoing arc(s)",
                curr
            )));
        }

        total_weight = A::Weight::times(&total_weight, &final_weight);
        Ok((labels, total_weight))
    }

    fn print_bytes(&self) -> Result<(String, A::Weight), OpenFstError> {
        let (labels, weight) = self.string_to_output_labels()?;

        let bytes: Vec<u8> = labels
            .into_iter()
            .map(|l| l.try_into().unwrap_or(0))
            .collect();

        let s = String::from_utf8(bytes)
            .map_err(|_| OpenFstError::new("Failed to parse labels as UTF-8 string"))?;

        Ok((s, weight))
    }
}
