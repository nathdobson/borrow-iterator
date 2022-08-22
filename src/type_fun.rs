use std::marker::PhantomData;

pub trait TypeFun {
    type Variant<'a>: 'a;
}

pub type Variant<'a, F> = <F as TypeFun>::Variant<'a>;

pub struct ConstF<T: ?Sized>(PhantomData<T>);

pub struct RefF<T>(PhantomData<T>);
pub struct MutF<T>(PhantomData<T>);

pub struct UnsizedRefF<T: ?Sized + 'static>(PhantomData<T>);
pub struct UnsizedMutF<T: ?Sized + 'static>(PhantomData<T>);

pub struct PairF<T1: ?Sized, T2: ?Sized>(PhantomData<T1>, PhantomData<T2>);

#[cfg(feature = "itertools")]
pub struct EitherOrBothF<T1: ?Sized, T2: ?Sized>(PhantomData<T1>, PhantomData<T2>);

impl<T: 'static> TypeFun for ConstF<T> {
    type Variant<'a> = T;
}

impl<T: TypeFun> TypeFun for RefF<T> {
    type Variant<'a> = &'a T::Variant<'a>;
}
impl<T: TypeFun> TypeFun for MutF<T> {
    type Variant<'a> = &'a mut T::Variant<'a>;
}

impl<T1: ?Sized + TypeFun, T2: ?Sized + TypeFun> TypeFun for PairF<T1, T2> {
    type Variant<'a> = (T1::Variant<'a>, T2::Variant<'a>);
}

#[cfg(feature = "itertools")]
impl<T1: TypeFun + ?Sized, T2: TypeFun + ?Sized> TypeFun for EitherOrBothF<T1, T2> {
    type Variant<'a> = itertools::EitherOrBoth<T1::Variant<'a>, T2::Variant<'a>>;
}

impl<T: 'static + ?Sized> TypeFun for UnsizedRefF<T> {
    type Variant<'a> = &'a T;
}

impl<T: 'static + ?Sized> TypeFun for UnsizedMutF<T> {
    type Variant<'a> = &'a mut T;
}

pub trait EqF: PartialEqF {}

impl<T1: EqF + TypeFun, T2: EqF> EqF for PairF<T1, T2>
where
    for<'a> Variant<'a, T1>: Sized,
    for<'a> Variant<'a, T2>: Sized,
{
}

pub trait PartialEqF<F2: TypeFun + ?Sized = Self>: TypeFun {
    fn eqf<'a>(x: &Self::Variant<'a>, y: &F2::Variant<'a>) -> bool;
}

impl<T1: PartialEqF, T2: PartialEqF> PartialEqF for PairF<T1, T2>
where
    for<'a> Variant<'a, T1>: Sized,
    for<'a> Variant<'a, T2>: Sized,
{
    fn eqf<'a>(x: &Self::Variant<'a>, y: &Self::Variant<'a>) -> bool {
        T1::eqf(&x.0, &y.0) && T2::eqf(&x.1, &y.1)
    }
}

pub trait ToOwnedF: TypeFun {
    type Owned: 'static;
    fn to_owned<'a>(x: Variant<'a, Self>) -> Self::Owned;
    fn to_owned_ref<'a>(x: &Variant<'a, Self>) -> Self::Owned;
}

impl<T: 'static + Clone> ToOwnedF for ConstF<T> {
    type Owned = T;
    fn to_owned<'a>(x: Variant<'a, Self>) -> Self::Owned { x }
    fn to_owned_ref<'a>(x: &Variant<'a, Self>) -> Self::Owned { x.clone() }
}

impl<T: 'static + ToOwned + ?Sized> ToOwnedF for UnsizedRefF<T> {
    type Owned = T::Owned;
    fn to_owned<'a>(x: Variant<'a, Self>) -> Self::Owned { x.to_owned() }
    fn to_owned_ref<'a>(x: &Variant<'a, Self>) -> Self::Owned { (*x).to_owned() }
}

impl<T: ToOwnedF> ToOwnedF for RefF<T> {
    type Owned = T::Owned;
    fn to_owned<'a>(x: Variant<'a, Self>) -> Self::Owned { T::to_owned_ref(x) }
    fn to_owned_ref<'a>(x: &Variant<'a, Self>) -> Self::Owned { T::to_owned_ref(x) }
}
