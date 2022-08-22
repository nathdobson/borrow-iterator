use crate::iter::BorrowIterator;
use crate::type_fun::ConstF;

pub struct AsStd<I> {
    iter: I,
}

impl<I> AsStd<I> {
    pub fn new(iter: I) -> Self { AsStd { iter } }
}

impl<I: BorrowIterator<Item = ConstF<T>>, T: 'static> Iterator for AsStd<I> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> { self.iter.next() }
}
