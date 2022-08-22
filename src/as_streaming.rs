use std::marker::PhantomData;

use crate::iter::BorrowIterator;
use crate::type_fun::{TypeFun, Variant};

pub struct AsStreaming<I, T, O, F> {
    iter: I,
    current: Option<T>,
    phantom: PhantomData<O>,
    fun: F,
}

impl<I, T, O, F> AsStreaming<I, T, O, F>
where
    I: Iterator,
    O: TypeFun,
    F: for<'a> Fn(&'a I::Item) -> Variant<'a, O>,
{
    pub fn new(iter: I, fun: F) -> Self {
        AsStreaming {
            iter,
            current: None,
            phantom: PhantomData,
            fun,
        }
    }
}

impl<'it, I, T, O, F> BorrowIterator for AsStreaming<I, T, O, F>
where
    I: Iterator<Item = T>,
    O: TypeFun,
    F: for<'a> Fn(&'a T) -> Variant<'a, O>,
{
    type Item = O;

    fn advance(&mut self) {
        self.current = self.iter.next();
    }

    fn get<'b>(&'b mut self) -> Option<Variant<'b, Self::Item>> {
        Some((self.fun)(self.current.as_ref()?))
    }

    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.iter.count()
    }
}
