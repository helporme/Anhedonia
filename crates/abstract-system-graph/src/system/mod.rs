pub mod function;

pub use function::{FunctionSystem, SystemFn};

use crate::resource::Link;

/// A system that is called with specified resources with read/write access.
///
/// With the help of resource access descriptions, systems can be invoked in optimal
/// order and multi-threaded. System is called by [`Node`], which is part of the [`Graph`].
///
/// Any function whose arguments implement [`Link`] can be converted to a system using
/// [`FunctionSystem`]. The trait [`SystemFn`] is always automatically implemented for the function.
///
/// # Examples
///```
/// use abstract_system_graph::prelude::*;
///
/// pub struct MyResource {
///     pub my_favourite_number: i32
/// }
///
/// // Link is implemented for a reference to the Resource
/// fn my_system(resource: &MyResource) {
///     println!("Favorite number is '{}'!", resource.my_favourite_number);
/// }
///
/// fn main<G: Graph>(&mut graph: G) {
///    // Currently looks verbose
///    let system = FunctionSystem::from(my_system);
///    let node = StraightNode::from(system);
///
///    graph.add_node(node.into())
/// }
/// ```
///
/// The system is an addition to the node that will execute it with specified resources and
/// multithreading. However, the system is not designed to manage the graph and its elements,
/// if you need such functionality you should implement a [`Node`]
/// instead of a system.
///
///[`Node`]: crate::node::Node
///[`Graph`]: crate::graph::Graph
///[`FunctionSystem`]: crate::system::function::FunctionSystem
///[`SystemFn`]: crate::system::function::SystemFn
///
pub trait System {
    /// [`Link`] is a way to access a resource (a reference for example).
    /// A tuple of links also implement a link, often a tuple of links is the `Input`.
    type Input: Link;
    type Output;

    fn run(&self, input: Self::Input) -> Self::Output;
}