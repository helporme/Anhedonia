use crate::graph::Graph;
use crate::node::NodePacked;

pub mod system;

pub fn builder<'a, K>() -> GraphBuilder<'a, K> {
    GraphBuilder::new()
}

#[derive(Default)]
pub struct GraphBuilder<'a, K> {
    nodes: Vec<NodePacked<'a, K>>
}

impl<'a, K> GraphBuilder<'a, K> {
    pub fn new() -> Self {
        Self { nodes: Vec::default() }
    }

    pub fn add_node(mut self, node: impl Into<NodePacked<'a, K>>) -> Self {
        self.nodes.push(node.into());
        self
    }

    pub fn add_nodes<N, I>(mut self, nodes: I) -> Self
        where N: Into<NodePacked<'a, K>>,
              I: IntoIterator<Item=N> {

        self.nodes.extend(nodes.into_iter().map(|n| n.into()));
        self
    }

    pub fn build<G: Graph<Kit = K> + From<Vec<NodePacked<'a, K>>>>(self) -> G {
        G::from(self.nodes)
    }
}

