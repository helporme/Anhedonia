pub mod group_sorter;

pub use group_sorter::{sorter_by_deps_folded ,sorter_by_deps_folded_with};

use crate::nodes::NodePacked;

pub type SortResult<T> = Result<T, SortError>;

#[derive(Debug)]
pub enum SortError {
    CycleDependencies
}

pub trait NodeSorter<'n, Kit> {
    fn sort(&mut self, nodes: Vec<NodePacked<'n, Kit>>) -> SortResult<NodePacked<'n, Kit>>;
}

impl<'n, Kit, SortFn> NodeSorter<'n, Kit> for SortFn 
    where SortFn: Fn(Vec<NodePacked<'n, Kit>>) -> SortResult<NodePacked<'n, Kit>> {

    fn sort(&mut self, nodes: Vec<NodePacked<'n, Kit>>) -> SortResult<NodePacked<'n, Kit>> {
        self(nodes)
    }
}