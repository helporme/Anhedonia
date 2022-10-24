use crate::node::NodePacked;

// todo: doc
pub trait Graph: Sized {
    type Context;

    fn add_node(&mut self, node: NodePacked<Self::Context>);
}