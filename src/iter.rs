use crate::as_std::AsStd;
use crate::filter::Filter;
use crate::map::Map;
use crate::type_fun::{PartialEqF, TypeFun, Variant};
use crate::{ConstF, ToOwnedF};

pub type Cloned<I: BorrowIterator>
where
    I::Item: ToOwnedF,
= impl BorrowIterator<Item = ConstF<<I::Item as ToOwnedF>::Owned>>;

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub trait BorrowIterator {
    type Item: TypeFun;
    fn advance<'b>(&'b mut self);
    fn get<'b>(&'b mut self) -> Option<Variant<'b, Self::Item>>;
    fn next<'b>(&'b mut self) -> Option<Variant<'b, Self::Item>> {
        self.advance();
        self.get()
    }
    fn size_hint(&self) -> (usize, Option<usize>);
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.fold(0, |x, _| x + 1)
    }
    fn fold<B, F>(mut self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: for<'a> FnMut(B, Variant<'a, Self::Item>) -> B,
    {
        let mut accum = init;
        while let Some(x) = self.next() {
            accum = f(accum, x);
        }
        accum
    }
    #[cfg(feature = "itertools")]
    fn zip_longest<I>(self, other: I) -> crate::zip_longest::ZipLongest<Self, I>
    where
        Self: Sized,
        I: BorrowIterator,
    {
        crate::zip_longest::ZipLongest::new(self, other)
    }
    fn all<P: for<'b> FnMut(Variant<'b, Self::Item>) -> bool>(mut self, mut p: P) -> bool
    where
        Self: Sized,
    {
        while let Some(next) = self.next() {
            if !p(next) {
                return false;
            }
        }
        true
    }
    fn eq<I2>(mut self, mut other: I2) -> bool
    where
        Self: Sized,
        I2: BorrowIterator,
        Self::Item: PartialEqF<I2::Item>,
    {
        loop {
            match (self.next(), other.next()) {
                (None, None) => return true,
                (Some(_), None) => return false,
                (None, Some(_)) => return false,
                (Some(x), Some(y)) => {
                    if !Self::Item::eqf(&x, &y) {
                        return false;
                    }
                }
            }
        }
    }
    fn map<O, F>(self, f: F) -> Map<Self, O, F>
    where
        Self: Sized,
        O: TypeFun,
        F: for<'a> FnMut(&'a (), Variant<'a, Self::Item>) -> Variant<'a, O>,
    {
        Map::new(self, f)
    }
    fn as_std(self) -> AsStd<Self>
    where
        Self: Sized,
    {
        AsStd::new(self)
    }
    fn for_each<F>(mut self, mut f: F)
    where
        Self: Sized,
        F: for<'a> FnMut(Variant<'a, Self::Item>),
    {
        while let Some(x) = self.next() {
            f(x);
        }
    }
    fn filter<F>(self, f: F) -> Filter<Self, F>
    where
        Self: Sized,
        F: for<'a> FnMut(Variant<'a, Self::Item>) -> bool,
    {
        Filter::new(self, f)
    }
    fn collect<T: 'static, B: FromIterator<T>>(self) -> B
    where
        Self: Sized + BorrowIterator<Item = ConstF<T>>,
    {
        B::from_iter(self.as_std())
    }
    fn cloned(self) -> Cloned<Self>
    where
        Self: Sized,
        Self::Item: ToOwnedF,
    {
        self.map::<ConstF<<Self::Item as ToOwnedF>::Owned>, _>(|&(), x| Self::Item::to_owned(x))
    }
}
