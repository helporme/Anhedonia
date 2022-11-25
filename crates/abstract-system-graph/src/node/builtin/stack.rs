// use std::collections::HashSet;
//
// use crate::dependency::Dependency;
// use crate::node::{Node, NodePacked};
//
// pub struct NodeStack<'a, K> {
//     nodes: Vec<NodePacked<'a, K>>
// }
//
// impl<'a, Kit: 'a> Node<'a, Kit> for NodeStack<'a, Kit> {
//     fn configure(&'a self, kit: &mut Kit) {
//         for node in self.nodes.iter() {
//             node.inner_ref().configure(kit);
//         }
//     }
//
//     fn build(&'a mut self, kit: &'a Kit) {
//         for node in self.nodes.iter_mut() {
//             node.inner_mut().build(kit);
//         }
//     }
//
//     fn execute(&'a self, kit: &'a Kit) {
//         for node in self.nodes.iter() {
//             node.inner_ref().execute(kit);
//         }
//     }
// }
//
// impl<'a, K: 'a> NodeStack<'a, K> {
//     pub const fn new(nodes: Vec<NodePacked<'a, K>>) -> Self {
//         NodeStack {
//             nodes
//         }
//     }
// }
//
// impl<'a, K: 'a> Default for NodeStack<'a, K> {
//     fn default() -> Self {
//         NodeStack::new(Vec::default())
//     }
// }
//
// impl<'a, K: 'a> From<NodeStack<'a, K>> for NodePacked<'a, K> {
//     fn from(node_stack: NodeStack<'a, K>) -> Self {
//         let mut dependencies: HashSet<Dependency> = HashSet::default();
//
//         for node in node_stack.nodes.iter() {
//             dependencies.extend(node.dependencies().iter().cloned());
//         }
//
//         NodePacked::new(node_stack, dependencies)
//     }
// }
