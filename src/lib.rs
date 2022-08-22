#![feature(generic_associated_types)]
#![feature(unboxed_closures)]
#![feature(trait_alias)]
#![feature(type_alias_impl_trait)]

pub use as_std::*;
pub use as_streaming::*;
pub use iter::*;
pub use iterator_ext::*;
pub use map::*;
pub use once::*;
pub use type_fun::*;
#[cfg(feature = "itertools")]
pub use zip_longest::*;

mod as_std;
mod as_streaming;
mod filter;
mod iter;
mod iterator_ext;
mod map;
mod once;
#[cfg(test)]
mod tests;
mod type_fun;
#[cfg(feature = "itertools")]
mod zip_longest;
mod reverse;

