// use crate::node::SystemNode;
// use crate::resource::{Channel, GetChannelEstablisherMut};
// use crate::system::{FunctionSystem, System, SystemFn};
//
// use super::GraphBuilder;
//
// pub trait GraphBuilderSystemSupport<'a, Sys: System, Chn> {
//     fn add_system(self, system_fn: impl SystemFn<Sys::Input, Sys::Output> + 'a) -> Self;
//     fn add_systems<S, I>(self, system_fns: I) -> Self
//         where S: SystemFn<Sys::Input, Sys::Output> + 'a,
//               I: IntoIterator<Item=S>;
// }
//
// impl<'a, Sys, Chn, Kit> GraphBuilderSystemSupport<'a, Sys, Chn> for GraphBuilder<'a, Kit>
//     where Sys: System + 'a,
//           Chn: Channel<'a, Sys::Input> + 'a,
//           Kit: GetChannelEstablisherMut<'a, Chn> {
//
//     fn add_system(self, system_fn: impl SystemFn<Sys::Input, Sys::Output> + 'a) -> Self {
//         self.add_node(SystemNode::from(FunctionSystem::from(system_fn)))
//     }
//
//     fn add_systems<S, I>(self, system_fns: I) -> Self
//         where S: SystemFn<Sys::Input, Sys::Output> + 'a,
//               I: IntoIterator<Item=S> {
//
//         self.add_nodes(system_fns
//             .into_iter()
//             .map(|sys_fn| SystemNode::from(FunctionSystem::from(sys_fn))))
//     }
// }
