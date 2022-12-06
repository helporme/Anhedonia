pub mod any_storage;
pub mod auto_impls;

pub use any_storage::links::{Lock, Mut, Ref};

use crate::dependency::DependencyWriter;

pub trait Link {
    fn write_deps(writer: &mut DependencyWriter);
}

pub trait Linker<'_fn, L: Link> {
    fn link(&'_fn self) -> Option<L>;
    fn can_be_linked(&self) -> bool;
}

pub trait FiniteLinker<L: Link> {
    fn link(&self) -> Option<L>;
    fn can_be_linked(&self) -> bool;
}

pub trait AsLinker<L: Link> {
    type Output: FiniteLinker<L>;

    fn as_ref(&self) -> &Self::Output;
}

impl<L: Link, Li: FiniteLinker<L>> AsLinker<L> for Li {
    type Output = Li;

    fn as_ref(&self) -> &Self::Output {
        self
    }
}