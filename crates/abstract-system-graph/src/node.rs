use crate::graph::SystemGraph;
use crate::system::SystemExecutor;
use crate::dependency::Dependency;

/// The node with which the graph works.
pub struct SystemGraphNode<SysGraph: SystemGraph> {
    executor: Box<dyn SystemExecutor<SysGraph>>,
    dependencies: [Dependency]
}