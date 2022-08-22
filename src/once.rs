use crate::{BorrowIterator, UnsizedMutF};

pub fn once<T>(value: T) -> Once<T> { Once { value, state: 0 } }

pub struct Once<T> {
    state: usize,
    value: T,
}

impl<T: 'static> BorrowIterator for Once<T> {
    type Item = UnsizedMutF<T>;

    fn advance<'b>(&'b mut self) { self.state += 1; }

    fn get<'b>(&'b mut self) -> Option<&'b mut T> {
        match self.state {
            0 => None,
            1 => Some(&mut self.value),
            2 => None,
            _ => panic!(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) { (1, Some(1)) }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        1
    }
}
