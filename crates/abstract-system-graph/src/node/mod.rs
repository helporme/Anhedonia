pub mod sorting;
pub mod builtin;

pub use builtin::*;

use std::collections::HashSet;

use crate::dependency::Dependency;

pub trait Node<Kit> {
    fn execute(&self, kit: &mut Kit);
}

pub struct NodePacked<'a, Kit> {
    node: Box<dyn Node<Kit> + 'a>,
    dependencies: HashSet<Dependency>,
}

impl<'a, Kit> NodePacked<'a, Kit> {
    pub fn new<N: Node<Kit> + 'a>(node: N, dependencies: HashSet<Dependency>) -> Self {
        Self { node: Box::new(node), dependencies }
    }

    pub fn inner_ref(&'a self) -> &dyn Node<Kit> {
        self.node.as_ref()
    }

    pub fn inner_mut(&'a mut self) -> &mut dyn Node<Kit> {
        self.node.as_mut()
    }

    pub const fn dependencies(&self) -> &HashSet<Dependency> {
        &self.dependencies
    }
}
