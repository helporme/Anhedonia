use std::collections::HashSet;

use crate::dependency::Dependency;
use crate::node::{Node, NodePacked};
use crate::tasks::{GetTaskBarrierRef, TaskBarrier};

pub struct NodeStackWithBarrier<'a, K> {
    nodes: Vec<NodePacked<'a, K>>
}

impl<'a, Kit: GetTaskBarrierRef + 'a> Node<'a, Kit> for NodeStackWithBarrier<'a, Kit> {
    fn configure(&'a self, kit: &mut Kit) {
        for node in self.nodes.iter() {
            node.inner_ref().configure(kit);
        }
    }

    fn build(&'a mut self, kit: &'a Kit) {
        for node in self.nodes.iter_mut() {
            node.inner_mut().build(kit);
        }
    }

    fn execute(&'a self, kit: &'a Kit) {
        for node in self.nodes.iter() {
            node.inner_ref().execute(kit);
        }
        kit.task_barrier().wait();
    }
}

impl<'a, K: 'a> NodeStackWithBarrier<'a, K> {
    pub const fn new(nodes: Vec<NodePacked<'a, K>>) -> Self {
        NodeStackWithBarrier {
            nodes
        }
    }
}

impl<'a, K: 'a> Default for NodeStackWithBarrier<'a, K> {
    fn default() -> Self {
        NodeStackWithBarrier::new(Vec::default())
    }
}

impl<'a, K: GetTaskBarrierRef + 'a> From<NodeStackWithBarrier<'a, K>> for NodePacked<'a, K> {
    fn from(node_stack: NodeStackWithBarrier<'a, K>) -> Self {
        let mut dependencies: HashSet<Dependency> = HashSet::default();

        for node in node_stack.nodes.iter() {
            dependencies.extend(node.dependencies().iter().cloned());
        }

        NodePacked::new(node_stack, dependencies)
    }
}
