pub mod with_node;
pub mod with_system;
pub mod with_system_unit;

pub use with_node::GraphBuilderWithNode;
pub use with_system::GraphBuilderWithSystem;
pub use with_system_unit::GraphBuilderWithSystemUnit;

use crate::{nodes::NodePacked, sorting::{default_sort, NodeSorter}};

pub trait GraphBuilderNodes<'n, Kit> {
    fn nodes(&mut self) -> &mut Vec<NodePacked<'n, Kit>>;
}

pub struct GarphBuilder<'n, Kit, Sorter> {
    nodes: Vec<NodePacked<'n, Kit>>,
    sorter: Option<Sorter>,
}

impl<'n, Kit, Sorter> GarphBuilder<'n, Kit, Sorter> {
    pub fn new() -> Self {
        Self { nodes: Vec::default(), sorter: None}
    }

    pub fn with_sorter(mut self, sorter: Sorter) -> Self {
        self.sorter = Some(sorter);
        self
    }

    pub fn with_kit(self, kit: Kit) -> GraphBuilderWithKit<'n, Kit, Sorter> {
        GraphBuilderWithKit { nodes: self.nodes, sorter: self.sorter, kit }
    }
}

impl<'n, Kit, Sorter> GraphBuilderNodes<'n, Kit> for GarphBuilder<'n, Kit, Sorter> {
    fn nodes(&mut self) -> &mut Vec<NodePacked<'n, Kit>> {
        &mut self.nodes
    }
}

pub struct GraphBuilderWithKit<'n, Kit, Sorter> {
    kit: Kit,
    sorter: Option<Sorter>,
    nodes: Vec<NodePacked<'n, Kit>>,
}

impl<'n, Kit: 'n, Sorter: NodeSorter<'n, Kit>> GraphBuilderWithKit<'n, Kit, Sorter> {
    pub fn with_sorter(mut self, sorter: Sorter) -> Self {
        self.sorter = Some(sorter);
        self
    }

    pub fn build(self) -> Graph<'n, Kit> {
        let base = if let Some(mut sorter) = self.sorter {
            sorter.sort(self.nodes).unwrap()
        } else {
            default_sort(self.nodes).unwrap()
        };

        Graph { base, kit: self.kit }
    }
}

impl<'n, Kit, Sorter> GraphBuilderNodes<'n, Kit> for GraphBuilderWithKit<'n, Kit, Sorter> {
    fn nodes(&mut self) -> &mut Vec<NodePacked<'n, Kit>> {
        &mut self.nodes
    }
}

pub struct Graph<'n, Kit> {
    base: NodePacked<'n, Kit>,
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
        self.base.inner_ref().execute(&mut self.kit);
    }

    pub fn execute_forever(&mut self) {
        loop {
            self.execute();
        }
    }
}