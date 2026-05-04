use crate::arc::Arc;
use crate::cache::CacheOptions;
use crate::error::OpenFstError;
use crate::ffi::WeightFfi;
use crate::ffi::arc_map::ArcMapConvertFfi;
use crate::ffi::arc_map::ArcMapInplaceFfi;
use crate::ffi::fst::FstFfi;
use crate::fst::{AsFstCxx, Fst, GenericFstArcIter, GenericFstStateIter};
use crate::ops::arc_map::{MapMapper, extract_map_args};
use crate::symbol_table::SymbolTable;
use cxx::UniquePtr;

pub type StdArcMapFst = ArcMapFst<crate::arc::StdArc>;
pub type Log64ArcMapFst = ArcMapFst<crate::arc::Log64Arc>;

/// A lazy FST that dynamically maps arcs and weights of an input FST.
pub struct ArcMapFst<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + ArcMapInplaceFfi,
    A::Weight: WeightFfi,
{
    inner: UniquePtr<A::FstCxx>,
}

impl<A> ArcMapFst<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + ArcMapInplaceFfi,
    A::Weight: WeightFfi,
{
    pub fn new<F>(ifst: &F, mapper: &MapMapper<A::Weight>) -> Result<Self, OpenFstError>
    where
        F: Fst<A> + AsFstCxx<A>,
    {
        Self::with_cache_options(ifst, mapper, &CacheOptions::default())
    }

    pub fn with_cache_options<F>(
        ifst: &F,
        mapper: &MapMapper<A::Weight>,
        cache: &CacheOptions,
    ) -> Result<Self, OpenFstError>
    where
        F: Fst<A> + AsFstCxx<A>,
    {
        let (mtype, w, p, d, sf_label) = extract_map_args::<A>(mapper);
        let inner = A::create_arc_map_fst(
            unsafe { ifst.as_fst_cxx() },
            mtype,
            w,
            p,
            d,
            sf_label,
            cache.gc,
            cache.gc_limit,
        )?;
        Ok(Self { inner })
    }
}

/// A lazy FST that dynamically converts the weights and arc types of an input FST.
pub struct ArcMapConvertFst<ToArc: Arc + FstFfi>
where
    ToArc::Weight: WeightFfi,
{
    inner: UniquePtr<ToArc::FstCxx>,
}

impl<ToArc> ArcMapConvertFst<ToArc>
where
    ToArc: Arc<StateId = i32, Label = i32> + FstFfi,
    ToArc::Weight: WeightFfi,
{
    pub fn new<FromArc, F>(ifst: &F) -> Self
    where
        FromArc: Arc<StateId = i32, Label = i32> + FstFfi + ArcMapConvertFfi<ToArc>,
        FromArc::Weight: WeightFfi,
        F: Fst<FromArc> + AsFstCxx<FromArc>,
    {
        Self::with_cache_options::<FromArc, F>(ifst, &CacheOptions::default())
    }

    pub fn with_cache_options<FromArc, F>(ifst: &F, cache: &CacheOptions) -> Self
    where
        FromArc: Arc<StateId = i32, Label = i32> + FstFfi + ArcMapConvertFfi<ToArc>,
        FromArc::Weight: WeightFfi,
        F: Fst<FromArc> + AsFstCxx<FromArc>,
    {
        let inner = FromArc::create_arc_map_convert_fst(
            unsafe { ifst.as_fst_cxx() },
            cache.gc,
            cache.gc_limit,
        );
        Self { inner }
    }
}

unsafe impl<A> AsFstCxx<A> for ArcMapFst<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + ArcMapInplaceFfi,
    A::Weight: WeightFfi,
{
    #[inline]
    unsafe fn as_fst_cxx(&self) -> &A::FstCxx {
        &self.inner
    }
}

impl<A> Fst<A> for ArcMapFst<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + ArcMapInplaceFfi,
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

unsafe impl<ToArc> AsFstCxx<ToArc> for ArcMapConvertFst<ToArc>
where
    ToArc: Arc<StateId = i32, Label = i32> + FstFfi,
    ToArc::Weight: WeightFfi,
{
    #[inline]
    unsafe fn as_fst_cxx(&self) -> &ToArc::FstCxx {
        &self.inner
    }
}

impl<ToArc> Fst<ToArc> for ArcMapConvertFst<ToArc>
where
    ToArc: Arc<StateId = i32, Label = i32> + FstFfi,
    ToArc::Weight: WeightFfi,
{
    type StateIter<'a>
        = GenericFstStateIter<'a, ToArc>
    where
        Self: 'a;
    type ArcIter<'a>
        = GenericFstArcIter<'a, ToArc>
    where
        Self: 'a;

    #[inline]
    fn fst_type(&self) -> String {
        ToArc::fst_type(unsafe { self.as_fst_cxx() })
    }
    #[inline]
    fn start(&self) -> ToArc::StateId {
        ToArc::fst_start(unsafe { self.as_fst_cxx() })
    }
    #[inline]
    fn final_weight(&self, state: ToArc::StateId) -> ToArc::Weight {
        ToArc::Weight::from_ffi(ToArc::fst_final_weight(unsafe { self.as_fst_cxx() }, state))
    }
    #[inline]
    fn num_arcs(&self, state: ToArc::StateId) -> usize {
        ToArc::fst_num_arcs(unsafe { self.as_fst_cxx() }, state)
    }
    #[inline]
    fn num_input_epsilons(&self, state: ToArc::StateId) -> usize {
        ToArc::fst_num_input_epsilons(unsafe { self.as_fst_cxx() }, state)
    }
    #[inline]
    fn num_output_epsilons(&self, state: ToArc::StateId) -> usize {
        ToArc::fst_num_output_epsilons(unsafe { self.as_fst_cxx() }, state)
    }
    #[inline]
    fn num_states_if_known(&self) -> Option<usize> {
        let n = ToArc::fst_num_states(unsafe { self.as_fst_cxx() });
        if n < 0 { None } else { Some(n as usize) }
    }
    #[inline]
    fn properties(&self, mask: u64, test: bool) -> u64 {
        ToArc::fst_properties(unsafe { self.as_fst_cxx() }, mask, test)
    }
    fn input_symbols(&self) -> Option<SymbolTable> {
        let ptr = ToArc::fst_input_symbols(unsafe { self.as_fst_cxx() });
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { SymbolTable::from_raw_ptr(ptr) })
        }
    }
    fn output_symbols(&self) -> Option<SymbolTable> {
        let ptr = ToArc::fst_output_symbols(unsafe { self.as_fst_cxx() });
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { SymbolTable::from_raw_ptr(ptr) })
        }
    }
    fn states<'a>(&'a self) -> Self::StateIter<'a> {
        GenericFstStateIter::new(unsafe { self.as_fst_cxx() })
    }
    fn arcs<'a>(&'a self, state: ToArc::StateId) -> Self::ArcIter<'a> {
        GenericFstArcIter::new(unsafe { self.as_fst_cxx() }, state)
    }
}
