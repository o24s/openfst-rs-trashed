use crate::arc::Arc;
use crate::ffi::WeightFfi;
use crate::ffi::fst::FstFfi;
use crate::ffi::vector_fst::VectorFstFfi;
use crate::fst::{AsFstCxx, AsMutFstCxx, ExpandedFst, Fst, GenericFstArcIter, MutableFst};
use crate::symbol_table::SymbolTable;
use cxx::UniquePtr;
use std::path::Path;

pub struct VectorFst<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + VectorFstFfi,
    A::Weight: WeightFfi,
{
    inner: UniquePtr<A::VectorFstCxx>,
}

impl<A> VectorFst<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + VectorFstFfi,
    A::Weight: WeightFfi,
{
    pub fn new() -> Self {
        Self {
            inner: A::create_vector_fst(),
        }
    }

    pub fn read<P: AsRef<Path>>(path: P) -> Option<Self> {
        let ptr = A::read_vector_fst(path.as_ref().to_str()?);
        if ptr.is_null() {
            None
        } else {
            Some(Self { inner: ptr })
        }
    }
    pub fn write<P: AsRef<Path>>(&self, path: P) -> bool {
        if let Some(p) = path.as_ref().to_str() {
            A::write_vector_fst(&self.inner, p)
        } else {
            false
        }
    }
}

pub type StdVectorFst = VectorFst<crate::arc::StdArc>;
pub type Log64VectorFst = VectorFst<crate::arc::Log64Arc>;

impl<A> Default for VectorFst<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + VectorFstFfi,
    A::Weight: WeightFfi,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<A> Clone for VectorFst<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + VectorFstFfi,
    A::Weight: WeightFfi,
{
    fn clone(&self) -> Self {
        Self {
            inner: A::copy_vector_fst(&self.inner),
        }
    }
}

unsafe impl<A> AsFstCxx<A> for VectorFst<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + VectorFstFfi,
    A::Weight: WeightFfi,
{
    #[inline]
    unsafe fn as_fst_cxx(&self) -> &A::FstCxx {
        A::vector_fst_as_fst(&*self.inner)
    }
}

unsafe impl<A> AsMutFstCxx<A> for VectorFst<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + VectorFstFfi,
    A::Weight: WeightFfi,
{
    #[inline]
    unsafe fn as_mut_fst_cxx(&mut self) -> std::pin::Pin<&mut A::MutableFstCxx> {
        A::vector_fst_as_mutable_fst(self.inner.pin_mut())
    }
}

pub struct VectorFstStateIter<'a> {
    num_states: i32,
    pos: i32,
    _marker: std::marker::PhantomData<&'a ()>,
}
impl<'a> Iterator for VectorFstStateIter<'a> {
    type Item = i32;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.num_states {
            None
        } else {
            let state = self.pos;
            self.pos += 1;
            Some(state)
        }
    }
}

impl<A> Fst<A> for VectorFst<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + VectorFstFfi,
    A::Weight: WeightFfi,
{
    type StateIter<'a>
        = VectorFstStateIter<'a>
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

    #[inline]
    fn input_symbols(&self) -> Option<SymbolTable> {
        let ptr = A::fst_input_symbols(unsafe { self.as_fst_cxx() });
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { SymbolTable::from_raw_ptr(ptr) })
        }
    }

    #[inline]
    fn output_symbols(&self) -> Option<SymbolTable> {
        let ptr = A::fst_output_symbols(unsafe { self.as_fst_cxx() });
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { SymbolTable::from_raw_ptr(ptr) })
        }
    }

    #[inline]
    fn states<'a>(&'a self) -> Self::StateIter<'a> {
        VectorFstStateIter {
            num_states: self.num_states(),
            pos: 0,
            _marker: std::marker::PhantomData,
        }
    }

    fn arcs<'a>(&'a self, state: A::StateId) -> Self::ArcIter<'a> {
        GenericFstArcIter::new(unsafe { self.as_fst_cxx() }, state)
    }
}

impl<A> ExpandedFst<A> for VectorFst<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + VectorFstFfi,
    A::Weight: WeightFfi,
{
    #[inline]
    fn num_states(&self) -> A::StateId {
        A::fst_num_states(unsafe { self.as_fst_cxx() })
    }
}

impl<A> MutableFst<A> for VectorFst<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + VectorFstFfi,
    A::Weight: WeightFfi,
{
    #[inline]
    fn set_start(&mut self, state: A::StateId) {
        A::fst_set_start(unsafe { self.as_mut_fst_cxx() }, state);
    }

    #[inline]
    fn set_final(&mut self, state: A::StateId, weight: A::Weight) {
        A::fst_set_final(unsafe { self.as_mut_fst_cxx() }, state, weight.as_ffi());
    }

    #[inline]
    fn set_properties(&mut self, props: u64, mask: u64) {
        A::fst_set_properties(unsafe { self.as_mut_fst_cxx() }, props, mask);
    }

    #[inline]
    fn add_state(&mut self) -> A::StateId {
        A::fst_add_state(unsafe { self.as_mut_fst_cxx() })
    }

    #[inline]
    fn add_states(&mut self, n: usize) {
        A::fst_add_states(unsafe { self.as_mut_fst_cxx() }, n);
    }

    #[inline]
    fn add_arc(&mut self, state: A::StateId, arc: A) {
        A::fst_add_arc(
            unsafe { self.as_mut_fst_cxx() },
            state,
            arc.ilabel(),
            arc.olabel(),
            arc.weight().as_ffi(),
            arc.nextstate(),
        );
    }

    #[inline]
    fn delete_arcs(&mut self, state: A::StateId) {
        A::fst_delete_arcs(unsafe { self.as_mut_fst_cxx() }, state);
    }

    #[inline]
    fn delete_arcs_n(&mut self, state: A::StateId, n: usize) {
        A::fst_delete_arcs_n(unsafe { self.as_mut_fst_cxx() }, state, n);
    }

    #[inline]
    fn delete_all_states(&mut self) {
        A::fst_delete_all_states(unsafe { self.as_mut_fst_cxx() });
    }

    #[inline]
    fn delete_states(&mut self, states: &[A::StateId]) {
        A::fst_delete_states(unsafe { self.as_mut_fst_cxx() }, states);
    }

    #[inline]
    fn reserve_states(&mut self, n: usize) {
        A::fst_reserve_states(unsafe { self.as_mut_fst_cxx() }, n);
    }

    #[inline]
    fn reserve_arcs(&mut self, state: A::StateId, n: usize) {
        A::fst_reserve_arcs(unsafe { self.as_mut_fst_cxx() }, state, n);
    }

    #[inline]
    fn set_input_symbols(&mut self, syms: Option<&SymbolTable>) {
        let ptr = syms.map_or(std::ptr::null(), |s| s.as_raw_ptr() as *const _);
        unsafe { A::fst_set_input_symbols(self.as_mut_fst_cxx(), ptr) }
    }

    #[inline]
    fn set_output_symbols(&mut self, syms: Option<&SymbolTable>) {
        let ptr = syms.map_or(std::ptr::null(), |s| s.as_raw_ptr() as *const _);
        unsafe { A::fst_set_output_symbols(self.as_mut_fst_cxx(), ptr) }
    }
}
