pub mod straight;

pub use straight::StraightNode;

use std::collections::HashSet;
use crate::dependency::Dependency;

// todo: doc
pub trait Node<Context> {
    fn execute(&mut self, context: &mut Context);
    fn is_executable(&self, context: &Context) -> bool;
}

// todo: doc
pub struct NodePacked<'n, Context> {
    node: Box<dyn Node<Context> + 'n>,
    dependencies: HashSet<Dependency>
}

impl<'n, Context> NodePacked<'n, Context> {
    pub fn new<N: Node<Context> + 'n>(
        node: N, dependencies: HashSet<Dependency>) -> Self {

        Self { node: Box::new(node), dependencies }
    }
}

