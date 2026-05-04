use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::encode::EncodeFfi;
use crate::ffi::fst::FstFfi;
use crate::fst::{AsMutFstCxx, MutableFst};
use crate::properties::{Acceptor, Verified};
use cxx::UniquePtr;
use std::path::Path;

pub const ENCODE_LABELS: u8 = 0x01;
pub const ENCODE_WEIGHTS: u8 = 0x02;
pub const ENCODE_FLAGS: u8 = ENCODE_LABELS | ENCODE_WEIGHTS;

/// Represents an active encoding mapper.
/// This object must be kept alive and passed to `Decode` to restore the FST.
pub struct EncodeMapper<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + EncodeFfi,
    A::Weight: WeightFfi,
{
    pub(crate) inner: UniquePtr<A::EncodeMapperCxx>,
}

impl<A> EncodeMapper<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + EncodeFfi,
    A::Weight: WeightFfi,
{
    /// Creates a new EncodeMapper.
    /// You can combine `ENCODE_LABELS` and `ENCODE_WEIGHTS` via bitwise OR.
    pub fn new(flags: u8) -> Self {
        Self {
            inner: A::create_encode_mapper(flags),
        }
    }

    /// Reads an EncodeMapper from a file.
    pub fn read<P: AsRef<Path>>(path: P) -> Option<Self> {
        let ptr = A::read_encode_mapper(path.as_ref().to_str()?);
        if ptr.is_null() {
            None
        } else {
            Some(Self { inner: ptr })
        }
    }

    /// Writes the EncodeMapper to a file.
    pub fn write<P: AsRef<Path>>(&self, path: P) -> bool {
        if let Some(p) = path.as_ref().to_str() {
            A::write_encode_mapper(&self.inner, p)
        } else {
            false
        }
    }

    /// Returns the bitwise flags that were used to create this mapper.
    pub fn flags(&self) -> u8 {
        A::encode_mapper_flags(&self.inner)
    }
}

pub trait Encode<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + EncodeFfi,
    A::Weight: WeightFfi,
{
    /// Encodes the FST's labels and/or weights into a single label.
    /// This structurally converts a Transducer into an Acceptor.
    fn encode<'a>(&'a mut self, mapper: &mut EncodeMapper<A>) -> Verified<&'a mut Self, Acceptor>;

    /// Decodes the FST back to its original Transducer/Weighted form using the same mapper.
    fn decode(&mut self, mapper: &EncodeMapper<A>);
}

impl<M, A> Encode<A> for M
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + EncodeFfi,
    A::Weight: WeightFfi,
    M: MutableFst<A> + AsMutFstCxx<A>,
{
    fn encode<'a>(&'a mut self, mapper: &mut EncodeMapper<A>) -> Verified<&'a mut Self, Acceptor> {
        A::fst_encode(unsafe { self.as_mut_fst_cxx() }, mapper.inner.pin_mut());

        unsafe { Verified::assume(self) }
    }

    fn decode(&mut self, mapper: &EncodeMapper<A>) {
        A::fst_decode(unsafe { self.as_mut_fst_cxx() }, &*mapper.inner);
    }
}
