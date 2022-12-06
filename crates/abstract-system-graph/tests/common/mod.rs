use abstract_system_graph::graph::Graph;
use abstract_system_graph::node::Node;
use abstract_system_graph::node::NodeStack;
use abstract_system_graph::resource::any_storage::AnyStorage;

pub mod sorted_graph {
    use abstract_system_graph::node::NodePacked;
    use abstract_system_graph::node::sorting::sort_by_dependencies_flat;

    use crate::common::{TestGraph, TestKit};

    impl<'n> From<Vec<NodePacked<'n, TestKit>>> for TestGraph<'n> {
        fn from(nodes: Vec<NodePacked<'n, TestKit>>) -> Self {
            Self {
                kit: TestKit::default(),
                nodes: sort_by_dependencies_flat(nodes).unwrap()
            }
        }
    }
}

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

#[derive(Default)]
pub struct TestKit {
    pub any_storage: AnyStorage
}

impl<'a> AsRef<AnyStorage> for TestKit {
    fn as_ref(&self) -> &AnyStorage {
        &self.any_storage
    }
}
