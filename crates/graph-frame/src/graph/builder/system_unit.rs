use crate::graph::GraphBuilder;
use crate::node::SystemUnitNode;
use crate::system::{FunctionSystem, SystemFn};

pub trait SystemUnitSupport<'n> {
    fn add_system_unit<Out: 'n>(self, system_fn: impl SystemFn<(), Out> + 'n) -> Self;
}

impl<'n, Kit> SystemUnitSupport<'n> for GraphBuilder<'n, Kit> {
    fn add_system_unit<Out: 'n>(self, system_fn: impl SystemFn<(), Out> + 'n) -> Self {
        self.add_node(SystemUnitNode::from(FunctionSystem::from(system_fn)))
    }
}
