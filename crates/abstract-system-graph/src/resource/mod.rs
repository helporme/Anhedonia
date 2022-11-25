pub mod any;
pub mod auto_impls;

use crate::dependency::DependencyWriter;

pub trait Link {
    fn write_deps(writer: &mut DependencyWriter);
}

pub trait Linker<L: Link> {
    fn link(&self) -> Option<L>;
    fn can_be_linked(&self) -> bool;
}

pub trait LinkerCompound<L: Link> {
    fn link(&self) -> Option<L>;
    fn can_be_linked(&self) -> bool;
}
