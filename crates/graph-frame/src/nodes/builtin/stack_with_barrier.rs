use std::collections::HashSet;

use crate::dependency::Dependency;
use crate::nodes::{Node, NodePacked};
use crate::tasks::{AsTaskBarrier, TaskBarrier};

pub struct NodeStackWithBarrier<'n, K> {
    nodes: Vec<NodePacked<'n, K>>
}

impl<'n, Kit: AsTaskBarrier + 'n> Node<Kit> for NodeStackWithBarrier<'n, Kit> {
    fn execute(&self, kit: &mut Kit) {
        for node in self.nodes.iter() {
            node.inner_ref().execute(kit);
        }

        let task_barrier = kit.as_ref();
        task_barrier.wait()
    }
}

impl<'n, Kit: 'n> From<Vec<NodePacked<'n, Kit>>> for NodeStackWithBarrier<'n, Kit> {
    fn from(nodes: Vec<NodePacked<'n, Kit>>) -> Self {
        Self {
            nodes
        }
    }
}


impl<'n, Kit: 'n> Default for NodeStackWithBarrier<'n, Kit> {
    fn default() -> Self {
        NodeStackWithBarrier::from(Vec::default())
    }
}

impl<'n, Kit: AsTaskBarrier + 'n> From<NodeStackWithBarrier<'n, Kit>> for NodePacked<'n, Kit> {
    fn from(node_stack: NodeStackWithBarrier<'n, Kit>) -> Self {
        let mut dependencies: HashSet<Dependency> = HashSet::default();

        for node in node_stack.nodes.iter() {
            dependencies.extend(node.dependencies().iter().cloned());
        }

        NodePacked::new(node_stack, dependencies)
    }
}
