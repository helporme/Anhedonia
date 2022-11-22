use std::collections::HashMap;

use crate::dependency::{DependencyId, DependencyRelation};
use crate::node::{NodePacked, NodeStack, NodeStackWithBarrier};
use crate::tasks::GetTaskBarrierRef;

pub type SortResult<T> = Result<T, SortError>;

pub enum SortError {
    CycleDependencies
}

pub fn sort_as_node_stack_with_barriers<'a, K: GetTaskBarrierRef + 'a>(
    nodes: Vec<NodePacked<'a, K>>) -> SortResult<NodeStackWithBarrier<'a, K>> {

    Ok(NodeStackWithBarrier::new(sort_into_parallel_groups(nodes)?
        .drain(..)
        .map(|nested_nodes| NodeStackWithBarrier::new(nested_nodes).into())
        .collect::<Vec<NodePacked<'a, K>>>()))
}

pub fn sort_as_node_stacks<'a, K: 'a>(
    nodes: Vec<NodePacked<'a, K>>) -> SortResult<NodeStack<'a, K>> {

    Ok(NodeStack::new(sort_into_parallel_groups(nodes)?
        .drain(..)
        .map(|nested_nodes| NodeStack::new(nested_nodes).into())
        .collect::<Vec<NodePacked<'a, K>>>()))
}

pub fn sort_into_parallel_groups<'a, K: 'a>(
    mut nodes: Vec<NodePacked<'a, K>>) -> SortResult<Vec<Vec<NodePacked<'a, K>>>> {

    let rely_map = into_rely_map(&nodes[..]);
    let mut nodes = nodes.drain(..).map(Some).collect::<Vec<_>>();
    let mut nodes_left = nodes.len();

    let mut node_stacks: Vec<Vec<NodePacked<'a, K>>> = Vec::default();

    while nodes_left > 0 {
        let mut node_stack: Vec<NodePacked<'a, K>> = Vec::default();

        for i in 0..nodes.len() {
            if rely_map[i]
                .as_ref()
                .map_or(false, |node_indices| node_indices
                    .iter()
                    .all(|i| nodes[*i].is_none())) {

                nodes_left -= 1;
                node_stack.push(nodes[i].take().unwrap());
            }
        }

        if node_stack.is_empty() && nodes_left > 0 {
            return Err(SortError::CycleDependencies);
        }

        node_stacks.push(node_stack);
    }

    Ok(node_stacks)
}

fn into_rely_map<'a, 'b, K: 'a>(nodes: &[NodePacked<'a, K>]) -> Vec<Option<Vec<usize>>> {
    let mut writes_to_dep: HashMap<DependencyId, Vec<usize>> = HashMap::default();
    let mut reads_from_dep: HashMap<DependencyId, Vec<usize>> = HashMap::default();

    for (node_idx, node) in nodes.iter().enumerate() {
        for dep in node.dependencies().iter() {
            let node_indices = match dep.relation() {
                DependencyRelation::Read => reads_from_dep.entry(*dep.id()),
                DependencyRelation::Write => writes_to_dep.entry(*dep.id())
            }.or_insert_with(Vec::default);

            node_indices.push(node_idx)
        }
    }

    let mut rely_map: Vec<Option<Vec<usize>>> = Vec::with_capacity(nodes.len());

    for (dep_id, node_indices) in reads_from_dep.iter() {
       for node_idx in node_indices.iter() {
           if let Some(writers) = writes_to_dep.get(dep_id) {
               let relies = rely_map[*node_idx].get_or_insert_with(Vec::default);
               relies.extend(writers);
           }
       }
    }

    rely_map
}
