use crate::{nodes::NodePacked, GraphBuilderNodes};

pub trait GraphBuilderWithNode<'n, Kit> {
    fn with_node(self, node: impl Into<NodePacked<'n, Kit>>) -> Self;
    fn with_nodes<N, I>(self, nodes: I) -> Self
        where N: Into<NodePacked<'n, Kit>>,
              I: IntoIterator<Item=N>;
}

impl<'n, Kit, G: GraphBuilderNodes<'n, Kit>> GraphBuilderWithNode<'n, Kit> for G {
    fn with_node(mut self, node: impl Into<NodePacked<'n, Kit>>) -> Self {
        self.nodes().push(node.into());
        self
    }

    fn with_nodes<N, I>(mut self, nodes: I) -> Self
        where N: Into<NodePacked<'n, Kit>>,
              I: IntoIterator<Item=N> {
        self.nodes().extend(nodes.into_iter().map(|n| n.into()));
        self
    }
}