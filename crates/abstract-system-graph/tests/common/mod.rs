use abstract_system_graph::graph::Graph;
use abstract_system_graph::node::{Node, NodeStack, NodePacked};
use abstract_system_graph::resource::any_storage::AnyStorage;
use abstract_system_graph::node::sorting::sort_by_dependencies_flat;

pub struct TestGraph<'n> {
    pub kit: TestKit,
    pub nodes: NodeStack<'n, TestKit>
}

impl<'n> Graph for TestGraph<'n> {
    type Kit = TestKit;

    fn execute(&mut self) {
        self.nodes.execute(&mut self.kit)
    }
}

impl<'n> From<Vec<NodePacked<'n, TestKit>>> for TestGraph<'n> {
    fn from(nodes: Vec<NodePacked<'n, TestKit>>) -> Self {
        Self {
            kit: TestKit::default(),
            nodes: sort_by_dependencies_flat(nodes).unwrap()
        }
    }
}

#[derive(Default)]
pub struct TestKit {
    pub any_storage: AnyStorage
}

impl<'a> AsRef<AnyStorage> for TestKit {
    fn as_ref(&self) -> &AnyStorage {
        &self.any_storage
    }
}
