use crate::GraphBuilderNodes;
use crate::nodes::SystemNode;
use crate::linking::{AsLinker, Link};
use crate::system::{FunctionSystem, SystemFn};

pub trait GraphBuilderWithSystem<'n, Kit, In: Link> {
    fn with_system<Out: 'n>(self, system_fn: impl SystemFn<In, Out> + 'n) -> Self;
    fn with_systems<Out: 'n, S, I>(self, system_fns: I) -> Self
        where S: SystemFn<In, Out> + 'n,
              I: IntoIterator<Item=S>;
}

impl<'n, In, Kit, G: GraphBuilderNodes<'n, Kit>> GraphBuilderWithSystem<'n, Kit, In> for G
    where In: Link + 'n, Kit: AsLinker<In> {

    fn with_system<Out: 'n>(mut self, system_fn: impl SystemFn<In, Out> + 'n) -> Self {
        self.nodes().push(SystemNode::from(FunctionSystem::from(system_fn)).into());
        self
    }

    fn with_systems<Out: 'n, S, I>(mut self, system_fns: I) -> Self
        where S: SystemFn<In, Out> + 'n,
              I: IntoIterator<Item=S> {

        self.nodes().extend(system_fns
            .into_iter()
            .map(|sys_fn| SystemNode::from(FunctionSystem::from(sys_fn)).into()));
        self
    }
}