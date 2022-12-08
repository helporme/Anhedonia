use crate::node::SystemNode;
use crate::resource::{AsLinker, Link};
use crate::system::{FunctionSystem, SystemFn};

use super::GraphBuilder;

pub trait SystemSupport<'n, In: Link> {
    fn add_system<Out: 'n>(self, system_fn: impl SystemFn<In, Out> + 'n) -> Self;
    fn add_systems<Out: 'n, S, I>(self, system_fns: I) -> Self
        where S: SystemFn<In, Out> + 'n,
              I: IntoIterator<Item=S>;
}

impl<'n, In, Kit> SystemSupport<'n, In> for GraphBuilder<'n, Kit>
    where In: Link + 'n, Kit: AsLinker<In> {

    fn add_system<Out: 'n>(self, system_fn: impl SystemFn<In, Out> + 'n) -> Self {
        self.add_node(SystemNode::from(FunctionSystem::from(system_fn)))
    }

    fn add_systems<Out: 'n, S, I>(self, system_fns: I) -> Self
        where S: SystemFn<In, Out> + 'n,
              I: IntoIterator<Item=S> {

        self.add_nodes(system_fns
            .into_iter()
            .map(|sys_fn| SystemNode::from(FunctionSystem::from(sys_fn))))
    }
}
