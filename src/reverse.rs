use crate::{BorrowIterator, Variant};

pub trait ReverseBorrowIterator: BorrowIterator {
    fn advance_back(&mut self);
    fn next_back<'a>(&'a mut self) -> Option<Variant<'a, Self::Item>> {
        self.advance_back();
        self.get()
    }
}
