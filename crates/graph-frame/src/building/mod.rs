pub mod with_node;
pub mod with_system;
pub mod with_system_unit;

pub use with_node::GraphBuilderWithNode;
pub use with_system::GraphBuilderWithSystem;
pub use with_system_unit::GraphBuilderWithSystemUnit;

use crate::nodes::{Node, NodePacked, NodeStack};

pub trait GraphBuilderNodes<'n, Kit> {
    fn nodes(&mut self) -> &mut Vec<NodePacked<'n, Kit>>;
}

pub trait GraphBuilderKit<'n, Kit> {
    fn kit(&mut self) -> &mut Kit;
}

pub struct GarphBuilder<'n, Kit> {
    nodes: Vec<NodePacked<'n, Kit>>,
}

impl<'n, Kit> GarphBuilder<'n, Kit> {
    pub fn new() -> Self {
        Self { nodes: Vec::default() }
    }

    pub fn with_kit(self, kit: Kit) -> GraphBuilderKitted<'n, Kit> {
        GraphBuilderKitted { nodes: self.nodes, kit }
    }
}

impl<'n, Kit> GraphBuilderNodes<'n, Kit> for GarphBuilder<'n, Kit> {
    fn nodes(&mut self) -> &mut Vec<NodePacked<'n, Kit>> {
        &mut self.nodes
    }
}

pub struct GraphBuilderKitted<'n, Kit> {
    kit: Kit,
    nodes: Vec<NodePacked<'n, Kit>>,
}

impl<'n, Kit: 'n> GraphBuilderKitted<'n, Kit> {
    pub fn build(self) -> Graph<'n, Kit> {
        Graph { base: NodeStack::from(self.nodes), kit: self.kit }
    }
}

impl<'n, Kit> GraphBuilderNodes<'n, Kit> for GraphBuilderKitted<'n, Kit> {
    fn nodes(&mut self) -> &mut Vec<NodePacked<'n, Kit>> {
        &mut self.nodes
    }
}

impl<'n, Kit> GraphBuilderKit<'n, Kit> for GraphBuilderKitted<'n, Kit> {
    fn kit(&mut self) -> &mut Kit {
        &mut self.kit
    }
}

pub struct Graph<'n, Kit> {
    base: NodeStack<'n, Kit>,
    kit: Kit,
}

impl<'n, Kit> Graph<'n, Kit> {
    pub fn kit(&self) -> &Kit {
        &self.kit
    }

    pub fn kit_mut(&mut self) -> &mut Kit {
        &mut self.kit
    }

    pub fn execute(&mut self) {
        self.base.execute(&mut self.kit);
    }

    pub fn execute_forever(&mut self) {
        loop {
            self.execute();
        }
    }
}