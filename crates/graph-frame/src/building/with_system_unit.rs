use crate::GraphBuilderNodes;
use crate::nodes::SystemUnitNode;
use crate::system::{FunctionSystem, SystemFn};

pub trait GraphBuilderWithSystemUnit<'n, Kit> {
    fn with_system_unit<Out: 'n>(self, system_fn: impl SystemFn<(), Out> + 'n) -> Self;
}

impl<'n, Kit, G: GraphBuilderNodes<'n, Kit>> GraphBuilderWithSystemUnit<'n, Kit> for G {
    fn with_system_unit<Out: 'n>(mut self, system_fn: impl SystemFn<(), Out> + 'n) -> Self {
        self.nodes().push(SystemUnitNode::from(FunctionSystem::from(system_fn)).into());
        self
    }
}
