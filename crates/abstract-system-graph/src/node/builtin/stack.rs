use std::collections::HashSet;

use crate::dependency::Dependency;
use crate::node::{Node, NodePacked};

pub struct NodeStack<'n, K> {
    nodes: Vec<NodePacked<'n, K>>
}

impl<'n, Kit: 'n> Node<Kit> for NodeStack<'n, Kit> {
    fn execute(&self, kit: &mut Kit) {
        for node in self.nodes.iter() {
            node.inner_ref().execute(kit);
        }
    }
}

impl<'n, Kit: 'n> NodeStack<'n, Kit> {
    pub const fn new(nodes: Vec<NodePacked<'n, Kit>>) -> Self {
        NodeStack {
            nodes
        }
    }
}

impl<'a, Kit: 'a> Default for NodeStack<'a, Kit> {
    fn default() -> Self {
        NodeStack::new(Vec::default())
    }
}

impl<'a, Kit: 'a> From<NodeStack<'a, Kit>> for NodePacked<'a, Kit> {
    fn from(node_stack: NodeStack<'a, Kit>) -> Self {
        let mut dependencies: HashSet<Dependency> = HashSet::default();

        for node in node_stack.nodes.iter() {
            dependencies.extend(node.dependencies().iter().cloned());
        }

        NodePacked::new(node_stack, dependencies)
    }
}
