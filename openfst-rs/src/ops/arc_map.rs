use crate::arc::Arc;
use crate::error::OpenFstError;
use crate::ffi::WeightFfi;
use crate::ffi::arc_map::{ArcMapConvertFfi, ArcMapInplaceFfi, GallicMapFfi, MapType};
use crate::ffi::fst::FstFfi;
use crate::fst::{AsFstCxx, AsMutFstCxx, Fst, MutableFst};
use crate::weight::Weight;
use cxx::UniquePtr;

#[derive(Debug, Clone)]
pub enum MapMapper<W> {
    Identity,
    InputEpsilon,
    OutputEpsilon,
    SuperFinal { label: i32 },
    Plus(W),
    Times(W),
    Power(f64),
    InvertWeight,
    RmWeight,
    Quantize { delta: f32 },
    ReverseWeight,
}

pub trait Map<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + ArcMapInplaceFfi,
    A::Weight: WeightFfi,
{
    fn map(&mut self, mapper: &MapMapper<A::Weight>) -> Result<(), OpenFstError>;
    fn map_of<F>(&mut self, ifst: &F, mapper: &MapMapper<A::Weight>) -> Result<(), OpenFstError>
    where
        F: Fst<A> + AsFstCxx<A>;
}

impl<M, A> Map<A> for M
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + ArcMapInplaceFfi,
    A::Weight: WeightFfi,
    M: MutableFst<A> + AsMutFstCxx<A>,
{
    fn map(&mut self, mapper: &MapMapper<A::Weight>) -> Result<(), OpenFstError> {
        let (mtype, w, p, d, sf_label) = extract_map_args::<A>(mapper);
        A::fst_arc_map_inplace(unsafe { self.as_mut_fst_cxx() }, mtype, w, p, d, sf_label)?;
        Ok(())
    }

    fn map_of<F>(&mut self, ifst: &F, mapper: &MapMapper<A::Weight>) -> Result<(), OpenFstError>
    where
        F: Fst<A> + AsFstCxx<A>,
    {
        let (mtype, w, p, d, sf_label) = extract_map_args::<A>(mapper);
        A::fst_arc_map_into(
            unsafe { ifst.as_fst_cxx() },
            unsafe { self.as_mut_fst_cxx() },
            mtype,
            w,
            p,
            d,
            sf_label,
        )?;
        Ok(())
    }
}

pub(crate) fn extract_map_args<A: Arc + FstFfi + ArcMapInplaceFfi>(
    mapper: &MapMapper<A::Weight>,
) -> (MapType, <A::Weight as WeightFfi>::ValueType, f64, f32, i32)
where
    A::Weight: WeightFfi,
{
    let z = A::Weight::zero().as_ffi();
    match mapper {
        MapMapper::Identity => (MapType::Identity, z, 0.0, 0.0, 0),
        MapMapper::InputEpsilon => (MapType::InputEpsilon, z, 0.0, 0.0, 0),
        MapMapper::OutputEpsilon => (MapType::OutputEpsilon, z, 0.0, 0.0, 0),
        MapMapper::SuperFinal { label } => (MapType::SuperFinal, z, 0.0, 0.0, *label),
        MapMapper::Plus(w) => (MapType::Plus, w.as_ffi(), 0.0, 0.0, 0),
        MapMapper::Times(w) => (MapType::Times, w.as_ffi(), 0.0, 0.0, 0),
        MapMapper::Power(p) => (MapType::Power, z, *p, 0.0, 0),
        MapMapper::InvertWeight => (MapType::InvertWeight, z, 0.0, 0.0, 0),
        MapMapper::RmWeight => (MapType::RmWeight, z, 0.0, 0.0, 0),
        MapMapper::Quantize { delta } => (MapType::Quantize, z, 0.0, *delta, 0),
        MapMapper::ReverseWeight => (MapType::ReverseWeight, z, 0.0, 0.0, 0),
    }
}

pub trait MapTypeConversion<FromA, ToA>
where
    FromA: Arc<StateId = i32, Label = i32> + FstFfi + ArcMapConvertFfi<ToA>,
    FromA::Weight: WeightFfi,
    ToA: Arc<StateId = i32, Label = i32> + FstFfi,
    ToA::Weight: WeightFfi,
{
    fn map_type_of<F>(&mut self, ifst: &F)
    where
        F: Fst<FromA> + AsFstCxx<FromA>;
}

impl<M, FromA, ToA> MapTypeConversion<FromA, ToA> for M
where
    FromA: Arc<StateId = i32, Label = i32> + FstFfi + ArcMapConvertFfi<ToA>,
    FromA::Weight: WeightFfi,
    ToA: Arc<StateId = i32, Label = i32> + FstFfi,
    ToA::Weight: WeightFfi,
    M: MutableFst<ToA> + AsMutFstCxx<ToA>,
{
    fn map_type_of<F>(&mut self, ifst: &F)
    where
        F: Fst<FromA> + AsFstCxx<FromA>,
    {
        FromA::fst_arc_map_convert(unsafe { ifst.as_fst_cxx() }, unsafe {
            self.as_mut_fst_cxx()
        });
    }
}

pub struct GallicFst<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + GallicMapFfi,
    A::Weight: WeightFfi,
{
    pub(crate) inner: UniquePtr<A::GallicFstCxx>,
}

pub trait MapGallic<A>
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + GallicMapFfi,
    A::Weight: WeightFfi,
{
    fn map_to_gallic<F>(ifst: &F) -> GallicFst<A>
    where
        F: Fst<A> + AsFstCxx<A>;

    fn map_from_gallic(&mut self, gallic_fst: &GallicFst<A>, superfinal_label: i32);
    fn gallic_to_new_symbols(&mut self, gallic_fst: &GallicFst<A>);
}

impl<M, A> MapGallic<A> for M
where
    A: Arc<StateId = i32, Label = i32> + FstFfi + GallicMapFfi,
    A::Weight: WeightFfi,
    M: MutableFst<A> + AsMutFstCxx<A>,
{
    fn map_to_gallic<F>(ifst: &F) -> GallicFst<A>
    where
        F: Fst<A> + AsFstCxx<A>,
    {
        GallicFst {
            inner: A::fst_map_to_gallic(unsafe { ifst.as_fst_cxx() }),
        }
    }

    fn map_from_gallic(&mut self, gallic_fst: &GallicFst<A>, superfinal_label: i32) {
        A::fst_map_from_gallic(
            &gallic_fst.inner,
            unsafe { self.as_mut_fst_cxx() },
            superfinal_label,
        );
    }

    fn gallic_to_new_symbols(&mut self, gallic_fst: &GallicFst<A>) {
        A::fst_gallic_to_new_symbols(&gallic_fst.inner, unsafe { self.as_mut_fst_cxx() });
    }
}
