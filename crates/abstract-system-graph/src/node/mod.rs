pub mod straight;

pub use straight::StraightNode;

use std::collections::HashSet;
use crate::dependency::Dependency;

// todo: doc
pub trait Node<'a, ExecutionKit>: 'a {
    fn configure(&'a self, kit: &mut ExecutionKit);
    fn initialize(&'a mut self, kit: &'a ExecutionKit);
    fn execute(&'a self);
}

// todo: doc
pub struct NodePacked<'a, Context> {
    node: Box<dyn Node<'a, Context>>,
    dependencies: HashSet<Dependency>,
}

impl<'a, Context> NodePacked<'a, Context> {
    pub fn new<N: Node<'a, Context>>(node: N, dependencies: HashSet<Dependency>) -> Self {
        Self { node: Box::new(node), dependencies }
    }

    pub fn inner(&'a mut self) -> &dyn Node<Context> {
        self.node.as_ref()
    }
}