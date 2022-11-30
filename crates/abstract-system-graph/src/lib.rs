pub mod system;
pub mod node;
pub mod graph;
pub mod resource;
pub mod dependency;
pub mod tasks;


mod tests {
    use crate::graph;
    use crate::graph::Graph;
    use crate::node::{NodePacked, SystemNode};
    use crate::node::system_unit::SystemUnitNode;
    use crate::resource::any_storage::{AnyStorage, Lock, Ref};
    use crate::system::FunctionSystem;

    struct Giraph;

    impl Graph for Giraph {
        type Kit = GiraphKit;

        fn execute(&self) {
            todo!()
        }
    }

    impl<'a, Kit> From<Vec<NodePacked<'a, Kit>>> for Giraph {
        fn from(value: Vec<NodePacked<'a, Kit>>) -> Self {
            todo!()
        }
    }

    struct GiraphKit;

    impl<'a> AsRef<AnyStorage> for GiraphKit {
        fn as_ref(&self) -> &AnyStorage {
            todo!()
        }
    }

    pub fn main() {
        let _graph = graph::builder()
            .add_node(SystemUnitNode::from(FunctionSystem::from(system)))
            .add_node(SystemNode::from(FunctionSystem::from(system2)))
            .build::<Giraph>();
    }

    pub fn system() {

    }

    pub fn system2(l: Lock<u8>, i: Ref<u8>) {
    }
}