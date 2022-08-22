use itertools::EitherOrBoth;

use crate::iter::BorrowIterator;
use crate::type_fun::{EitherOrBothF, Variant};

pub struct ZipLongest<A, B> {
    a: A,
    b: B,
}

impl<A, B> ZipLongest<A, B> {
    pub fn new(a: A, b: B) -> Self { ZipLongest { a, b } }
}

impl<A: BorrowIterator, B: BorrowIterator> BorrowIterator for ZipLongest<A, B> {
    type Item = EitherOrBothF<A::Item, B::Item>;

    fn advance<'b>(&'b mut self) {
        self.a.advance();
        self.b.advance();
    }

    fn get<'b>(&'b mut self) -> Option<Variant<'b, Self::Item>> {
        match (self.a.get(), self.b.get()) {
            (None, None) => None,
            (Some(a), None) => Some(EitherOrBoth::Left(a)),
            (None, Some(b)) => Some(EitherOrBoth::Right(b)),
            (Some(a), Some(b)) => Some(EitherOrBoth::Both(a, b)),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (min1, max1) = self.a.size_hint();
        let (min2, max2) = self.b.size_hint();
        (min1.max(min2), max1.max(max2))
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.a.count().max(self.b.count())
    }
}
