use std::marker::PhantomData;

use crate::iter::BorrowIterator;
use crate::type_fun::{TypeFun, Variant};

pub struct Map<I, O, F> {
    iter: I,
    phantom: PhantomData<O>,
    f: F,
}

impl<I, O, F> Map<I, O, F> {
    pub fn new(iter: I, f: F) -> Self {
        Map {
            iter,
            phantom: PhantomData,
            f,
        }
    }
}

impl<
        I: BorrowIterator,
        O: TypeFun,
        F: for<'a> FnMut(&'a (), Variant<'a, I::Item>) -> Variant<'a, O>,
    > BorrowIterator for Map<I, O, F>
where
    for<'a> Variant<'a, O>: Sized,
{
    type Item = O;

    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.iter.count()
    }

    fn advance<'b>(&'b mut self) { self.iter.advance(); }

    fn get<'b>(&'b mut self) -> Option<Variant<'b, Self::Item>> {
        Some((self.f)(&(), self.iter.get()?))
    }
}
