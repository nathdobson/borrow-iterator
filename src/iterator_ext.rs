use crate::as_streaming::AsStreaming;
use crate::type_fun::{TypeFun, Variant};

pub trait IteratorExt: Iterator {
    fn as_streaming<O, F>(self, fun: F) -> AsStreaming<Self, Self::Item, O, F>
        where
            Self: Sized,
            O: TypeFun,
            F: for<'a> Fn(&'a Self::Item) -> Variant<'a, O>,
    {
        AsStreaming::new(self, fun)
    }
}

impl<T: ?Sized> IteratorExt for T where T: Iterator {}
