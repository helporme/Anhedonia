pub mod sorting;
pub mod builtin;

pub use builtin::*;

use std::collections::HashSet;
use std::fmt::{Debug, Formatter};

use crate::dependency::Dependency;

pub trait Node<Kit> {
    fn execute(&self, kit: &mut Kit);
}

pub struct NodePacked<'n, Kit> {
    node: Box<dyn Node<Kit> + 'n>,
    dependencies: HashSet<Dependency>,
}

impl<'n, Kit> NodePacked<'n, Kit> {
    pub fn new<N: Node<Kit> + 'n>(node: N, dependencies: HashSet<Dependency>) -> Self {
        Self { node: Box::new(node), dependencies }
    }

    pub fn inner_ref(&self) -> &dyn Node<Kit> {
        self.node.as_ref()
    }

    pub fn inner_mut(&mut self) -> &mut dyn Node<Kit> {
        self.node.as_mut()
    }

    pub const fn dependencies(&self) -> &HashSet<Dependency> {
        &self.dependencies
    }
}

impl<'n, Kit: 'n> Debug for NodePacked<'n, Kit> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PackedNode")
            .field("dependencies", &self.dependencies)
            .finish()
    }
}