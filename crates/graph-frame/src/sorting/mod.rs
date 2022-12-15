pub mod stack_sorter;

pub use stack_sorter::{stacks_sorter, fn_stacks_sorter, sort_by_dependencies};

use crate::nodes::{NodePacked, NodeStack};

pub type SortResult<T> = Result<T, SortError>;

#[derive(Debug)]
pub enum SortError {
    CycleDependencies
}

pub trait NodeSorter<'n, Kit> {
    fn sort(&mut self, nodes: Vec<NodePacked<'n, Kit>>) -> SortResult<NodePacked<'n, Kit>>;
}

impl<'n, Kit: 'n, SortFn> NodeSorter<'n, Kit> for SortFn 
    where SortFn: Fn(Vec<NodePacked<'n, Kit>>) -> SortResult<NodePacked<'n, Kit>> {

    fn sort(&mut self, nodes: Vec<NodePacked<'n, Kit>>) -> SortResult<NodePacked<'n, Kit>> {
        self(nodes)
    }
}

pub fn default_sort<'n, Kit: 'n>(nodes: Vec<NodePacked<'n, Kit>>) -> SortResult<NodePacked<'n, Kit>> {
    Ok(NodeStack::from(nodes).into())
}