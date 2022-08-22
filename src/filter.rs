use crate::{BorrowIterator, Variant};
use crate::reverse::ReverseBorrowIterator;

pub struct Filter<I, F> {
    iter: I,
    fun: F,
}

impl<I, F> Filter<I, F> {
    pub fn new(iter: I, fun: F) -> Self { Filter { iter, fun } }
}

impl<I: BorrowIterator, F: for<'a> FnMut(Variant<'a, I::Item>) -> bool> BorrowIterator
    for Filter<I, F>
{
    type Item = I::Item;

    fn advance<'b>(&'b mut self) {
        while let Some(mut x) = self.iter.next() {
            if (self.fun)(x) {
                break;
            }
        }
    }

    fn get<'b>(&'b mut self) -> Option<Variant<'b, Self::Item>> { self.iter.get() }

    fn size_hint(&self) -> (usize, Option<usize>) { (0, self.iter.size_hint().1) }
}

impl<I: ReverseBorrowIterator, F: for<'a> FnMut(Variant<'a, I::Item>) -> bool> ReverseBorrowIterator
    for Filter<I, F>
{
    fn advance_back(&mut self) {
        while let Some(mut x) = self.iter.next_back() {
            if (self.fun)(x) {
                break;
            }
        }
    }
}
