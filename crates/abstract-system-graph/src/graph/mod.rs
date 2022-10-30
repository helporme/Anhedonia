mod runner;

use crate::node::NodePacked;

// todo: doc
pub trait Graph<'a> {
    type ExecutionKit;

    fn add_node(&mut self, node: NodePacked<'a, Self::ExecutionKit>);

    fn execute(&mut self);
}
