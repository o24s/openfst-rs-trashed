use std::sync::OnceLock;

use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use crate::symbol_table::SymbolTable;
use crate::weight::Weight;

fn ensure_registries_initialized() {
    static INIT: OnceLock<()> = OnceLock::new();
    INIT.get_or_init(|| {
        crate::ffi::const_fst::ffi_binding::init_const_fst_registry();
        crate::ffi::compact_fst::init_compact_fst_registry();
    });
}

pub const FST_MAGIC_NUMBER: i32 = 2125659606;
pub const NO_LABEL: i32 = -1;
pub const NO_STATE_ID: i32 = -1;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchType {
    Input = 1,
    Output = 2,
    Both = 3,
    None = 4,
    Unknown = 5,
}

pub const ARC_ILABEL_VALUE: u8 = 0x01;
pub const ARC_OLABEL_VALUE: u8 = 0x02;
pub const ARC_WEIGHT_VALUE: u8 = 0x04;
pub const ARC_NEXT_STATE_VALUE: u8 = 0x08;
pub const ARC_NO_CACHE: u8 = 0x10;
pub const ARC_VALUE_FLAGS: u8 =
    ARC_ILABEL_VALUE | ARC_OLABEL_VALUE | ARC_WEIGHT_VALUE | ARC_NEXT_STATE_VALUE;
pub const ARC_FLAGS: u8 = ARC_VALUE_FLAGS | ARC_NO_CACHE;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileReadMode {
    Read,
    Map,
}

#[derive(Debug, Clone)]
pub struct FstReadOptions {
    pub source: String,
    pub mode: FileReadMode,
    pub read_isymbols: bool,
    pub read_osymbols: bool,
}
impl Default for FstReadOptions {
    fn default() -> Self {
        Self {
            source: "<unspecified>".to_string(),
            mode: FileReadMode::Read,
            read_isymbols: true,
            read_osymbols: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FstWriteOptions {
    pub source: String,
    pub write_header: bool,
    pub write_isymbols: bool,
    pub write_osymbols: bool,
    pub align: bool,
    pub stream_write: bool,
}
impl Default for FstWriteOptions {
    fn default() -> Self {
        Self {
            source: "<unspecified>".to_string(),
            write_header: true,
            write_isymbols: true,
            write_osymbols: true,
            align: false,
            stream_write: false,
        }
    }
}

/// A trait for extracting a reference to the underlying C++ FST object.
///
/// # Safety
///
/// Implementing this trait is unsafe. Implementers must guarantee the following:
/// - The underlying C++ object exists, is properly aligned, and correctly initialized.
/// - The FST wrapper struct must correctly manage the lifetime of the C++ object so that
///   it outlives the returned reference.
pub unsafe trait AsFstCxx<A: Arc<StateId = i32, Label = i32> + FstFfi>
where
    A::Weight: WeightFfi,
{
    /// Extracts a reference to the underlying C++ FST object.
    ///
    /// # Safety
    ///
    /// Calling this method is unsafe because the caller must guarantee that the
    /// underlying C++ object is not mutated concurrently from C++ side, or otherwise
    /// invalidated through raw pointers, for the entire lifetime of the returned `&A::FstCxx`.
    /// Rust's shared aliasing rules (multiple readers, no writers) must be respected
    /// globally across both the Rust and C++ boundaries.
    unsafe fn as_fst_cxx(&self) -> &A::FstCxx;
}

/// A trait for extracting a pinned mutable reference to the underlying C++ mutable FST object.
///
/// # Safety
///
/// Implementing this trait is unsafe. Implementers must guarantee the following:
/// - The underlying C++ object is structurally pinned in memory and will not be moved,
///   respecting the guarantees of `std::pin::Pin`.
/// - The FST wrapper struct exclusively owns or safely borrows the underlying C++ object.
pub unsafe trait AsMutFstCxx<A: Arc<StateId = i32, Label = i32> + FstFfi>
where
    A::Weight: WeightFfi,
{
    /// Extracts a pinned mutable reference to the underlying C++ mutable FST object.
    ///
    /// # Safety
    ///
    /// Calling this method is unsafe because the caller must ensure that creating this
    /// mutable reference does not violate Rust's exclusive mutability rules.
    /// Specifically, there must be absolutely no other active aliases (either in Rust or
    /// held in C++) to the same underlying C++ object while the returned
    /// `Pin<&mut A::MutableFstCxx>` is alive.
    unsafe fn as_mut_fst_cxx(&mut self) -> std::pin::Pin<&mut A::MutableFstCxx>;
}

pub trait Fst<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi,
    A::Weight: WeightFfi,
{
    type StateIter<'a>: Iterator<Item = A::StateId>
    where
        Self: 'a;
    type ArcIter<'a>: FstArcIterator<'a, A>
    where
        Self: 'a;

    fn fst_type(&self) -> String;
    fn start(&self) -> A::StateId;
    fn final_weight(&self, state: A::StateId) -> A::Weight;
    fn num_arcs(&self, state: A::StateId) -> usize;
    fn num_input_epsilons(&self, state: A::StateId) -> usize;
    fn num_output_epsilons(&self, state: A::StateId) -> usize;
    fn num_states_if_known(&self) -> Option<usize>;
    fn properties(&self, mask: u64, test: bool) -> u64;

    fn input_symbols(&self) -> Option<SymbolTable>;
    fn output_symbols(&self) -> Option<SymbolTable>;

    fn states<'a>(&'a self) -> Self::StateIter<'a>;
    fn arcs<'a>(&'a self, state: A::StateId) -> Self::ArcIter<'a>;
}

pub trait FstArcIterator<'a, A: Arc>: Iterator<Item = A> {
    fn position(&self) -> usize;
    fn reset(&mut self);
    fn seek(&mut self, position: usize);
    fn flags(&self) -> u8;
    fn set_flags(&mut self, flags: u8, mask: u8);
}

pub trait ExpandedFst<A: Arc>: Fst<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi,
    A::Weight: WeightFfi,
{
    fn num_states(&self) -> A::StateId;
}

pub trait MutableFst<A>: ExpandedFst<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi,
    A::Weight: WeightFfi,
{
    fn set_start(&mut self, state: A::StateId);
    fn set_final(&mut self, state: A::StateId, weight: A::Weight);
    fn set_properties(&mut self, props: u64, mask: u64);
    fn add_state(&mut self) -> A::StateId;
    fn add_states(&mut self, n: usize);
    fn add_arc(&mut self, state: A::StateId, arc: A);
    fn delete_arcs_n(&mut self, state: A::StateId, n: usize);
    fn delete_arcs(&mut self, state: A::StateId);
    fn delete_all_states(&mut self);
    fn delete_states(&mut self, states: &[A::StateId]);
    fn reserve_states(&mut self, n: usize);
    fn reserve_arcs(&mut self, state: A::StateId, n: usize);

    fn set_input_symbols(&mut self, syms: Option<&SymbolTable>);
    fn set_output_symbols(&mut self, syms: Option<&SymbolTable>);
}

pub struct GenericFstStateIter<'a, A: Arc<StateId = i32, Label = i32> + FstFfi>
where
    A::Weight: WeightFfi,
{
    inner: cxx::UniquePtr<A::StateIteratorCxx>,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a, A: Arc<StateId = i32, Label = i32> + FstFfi> GenericFstStateIter<'a, A>
where
    A::Weight: WeightFfi,
{
    pub(crate) fn new(fst: &'a A::FstCxx) -> Self {
        Self {
            inner: A::create_state_iter(fst),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'a, A: Arc<StateId = i32, Label = i32> + FstFfi> Iterator for GenericFstStateIter<'a, A>
where
    A::Weight: WeightFfi,
{
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if A::state_iter_done(&*self.inner) {
            None
        } else {
            let s = A::state_iter_value(&*self.inner);
            A::state_iter_next(self.inner.pin_mut());
            Some(s)
        }
    }
}

pub struct GenericFstArcIter<'a, A: Arc<StateId = i32, Label = i32> + FstFfi>
where
    A::Weight: WeightFfi,
{
    inner: cxx::UniquePtr<A::ArcIteratorCxx>,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a, A: Arc<StateId = i32, Label = i32> + FstFfi> GenericFstArcIter<'a, A>
where
    A::Weight: WeightFfi,
{
    pub(crate) fn new(fst: &'a A::FstCxx, state: i32) -> Self {
        Self {
            inner: A::create_arc_iter(fst, state),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'a, A: Arc<StateId = i32, Label = i32> + FstFfi> Iterator for GenericFstArcIter<'a, A>
where
    A::Weight: WeightFfi,
{
    type Item = A;
    fn next(&mut self) -> Option<Self::Item> {
        let mut il = 0;
        let mut ol = 0;
        let mut ns = 0;
        let mut w = A::Weight::zero().as_ffi();
        if unsafe { A::arc_iter_advance(self.inner.pin_mut(), &mut il, &mut ol, &mut w, &mut ns) } {
            Some(A::new(il, ol, A::Weight::from_ffi(w), ns))
        } else {
            None
        }
    }
}

impl<'a, A: Arc<StateId = i32, Label = i32> + FstFfi> FstArcIterator<'a, A>
    for GenericFstArcIter<'a, A>
where
    A::Weight: WeightFfi,
{
    fn position(&self) -> usize {
        A::arc_iter_position(&*self.inner)
    }
    fn reset(&mut self) {
        A::arc_iter_reset(self.inner.pin_mut());
    }
    fn seek(&mut self, position: usize) {
        A::arc_iter_seek(self.inner.pin_mut(), position);
    }
    fn flags(&self) -> u8 {
        A::arc_iter_flags(&*self.inner)
    }
    fn set_flags(&mut self, flags: u8, mask: u8) {
        A::arc_iter_set_flags(self.inner.pin_mut(), flags, mask);
    }
}

/// A wrapper for an dynamically-loaded FST whose exact implementation type
/// (e.g. VectorFst, ConstFst) is determined at runtime based on the file header.
pub struct DynamicFst<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi,
    A::Weight: WeightFfi,
{
    inner: cxx::UniquePtr<A::FstCxx>,
}

impl<A> DynamicFst<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi,
    A::Weight: WeightFfi,
{
    /// Reads an FST from a file, dynamically determining its type from the file header.
    /// This is the idiomatic way to load arbitrary FST files.
    pub fn read<P: AsRef<std::path::Path>>(path: P) -> Option<Self> {
        ensure_registries_initialized();

        let ptr = A::fst_read(path.as_ref().to_str()?);
        if ptr.is_null() {
            None
        } else {
            Some(Self { inner: ptr })
        }
    }
}

pub type StdDynamicFst = DynamicFst<crate::arc::StdArc>;
pub type Log64DynamicFst = DynamicFst<crate::arc::Log64Arc>;

unsafe impl<A> AsFstCxx<A> for DynamicFst<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi,
    A::Weight: WeightFfi,
{
    #[inline]
    unsafe fn as_fst_cxx(&self) -> &A::FstCxx {
        &self.inner
    }
}

impl<A> Fst<A> for DynamicFst<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi,
    A::Weight: WeightFfi,
{
    type StateIter<'a>
        = GenericFstStateIter<'a, A>
    where
        Self: 'a;
    type ArcIter<'a>
        = GenericFstArcIter<'a, A>
    where
        Self: 'a;

    #[inline]
    fn fst_type(&self) -> String {
        A::fst_type(unsafe { self.as_fst_cxx() })
    }
    #[inline]
    fn start(&self) -> A::StateId {
        A::fst_start(unsafe { self.as_fst_cxx() })
    }
    #[inline]
    fn final_weight(&self, state: A::StateId) -> A::Weight {
        A::Weight::from_ffi(A::fst_final_weight(unsafe { self.as_fst_cxx() }, state))
    }
    #[inline]
    fn num_arcs(&self, state: A::StateId) -> usize {
        A::fst_num_arcs(unsafe { self.as_fst_cxx() }, state)
    }
    #[inline]
    fn num_input_epsilons(&self, state: A::StateId) -> usize {
        A::fst_num_input_epsilons(unsafe { self.as_fst_cxx() }, state)
    }
    #[inline]
    fn num_output_epsilons(&self, state: A::StateId) -> usize {
        A::fst_num_output_epsilons(unsafe { self.as_fst_cxx() }, state)
    }
    #[inline]
    fn num_states_if_known(&self) -> Option<usize> {
        let n = A::fst_num_states(unsafe { self.as_fst_cxx() });
        if n < 0 { None } else { Some(n as usize) }
    }
    #[inline]
    fn properties(&self, mask: u64, test: bool) -> u64 {
        A::fst_properties(unsafe { self.as_fst_cxx() }, mask, test)
    }

    fn input_symbols(&self) -> Option<SymbolTable> {
        let ptr = A::fst_input_symbols(unsafe { self.as_fst_cxx() });
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { SymbolTable::from_raw_ptr(ptr) })
        }
    }
    fn output_symbols(&self) -> Option<SymbolTable> {
        let ptr = A::fst_output_symbols(unsafe { self.as_fst_cxx() });
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { SymbolTable::from_raw_ptr(ptr) })
        }
    }

    fn states<'a>(&'a self) -> Self::StateIter<'a> {
        GenericFstStateIter::new(unsafe { self.as_fst_cxx() })
    }
    fn arcs<'a>(&'a self, state: A::StateId) -> Self::ArcIter<'a> {
        GenericFstArcIter::new(unsafe { self.as_fst_cxx() }, state)
    }
}
