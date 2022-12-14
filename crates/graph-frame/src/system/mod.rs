pub mod function;

pub use function::{FunctionSystem, SystemFn};

use crate::linking::Link;

pub trait System {
    /// [`Link`] is a way to access a resource (a reference for example).
    /// A tuple of links also implement a link, often a tuple of links is the `Input`.
    type Input: Link;
    type Output;

    fn run(&self, input: Self::Input) -> Self::Output;
}