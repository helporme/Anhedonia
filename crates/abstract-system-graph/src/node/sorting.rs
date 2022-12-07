use std::collections::HashMap;

use crate::dependency::{DependencyId, DependencyRelation};
use crate::node::NodePacked;

pub type SortResult<T> = Result<T, SortError>;

#[derive(Debug)]
pub enum SortError {
    CycleDependencies
}

pub fn sort_by_dependencies_flat_with<'a, K, F>(
    nodes: Vec<NodePacked<'a, K>>, f: F) -> SortResult<NodePacked<'a, K>>
    where K: 'a, F: Fn(Vec<NodePacked<'a, K>>) -> NodePacked<'a, K> {

    Ok(f(sort_by_dependencies(nodes)?
        .drain(..)
        .map(|nested_nodes| f(nested_nodes).into())
        .collect::<Vec<NodePacked<'a, K>>>()))
}

pub fn sort_by_dependencies_flat<'a, Kit, Flat>(nodes: Vec<NodePacked<'a, Kit>>) -> SortResult<Flat>
    where Kit: 'a,
          Flat: From<Vec<NodePacked<'a, Kit>>> + Into<NodePacked<'a, Kit>>{

    Ok(Flat::from(sort_by_dependencies(nodes)?
        .drain(..)
        .map(|nested_nodes| Flat::from(nested_nodes).into())
        .collect::<Vec<NodePacked<'a, Kit>>>()))
}

pub fn sort_by_dependencies<'a, K: 'a>(
    mut nodes: Vec<NodePacked<'a, K>>) -> SortResult<Vec<Vec<NodePacked<'a, K>>>> {

    let rely_map = into_rely_map(&nodes[..]);
    let mut nodes = nodes.drain(..).map(Some).collect::<Vec<_>>();
    let mut nodes_left = nodes.len();

    let mut node_stacks: Vec<Vec<NodePacked<'a, K>>> = Vec::default();

    while nodes_left > 0 {
        let free_node_indices: Vec<usize> = (0..nodes.len())
            .filter(|&i| {
                nodes[i].is_some() && rely_map[i]
                    .as_ref() .map_or(true, |node_indices| node_indices
                    .iter()
                    .all(|i| nodes[*i].is_none())) })
            .collect();

        let node_stack: Vec<NodePacked<'a, K>> = free_node_indices.into_iter()
            .map(|i| nodes[i].take().unwrap())
            .collect();

        nodes_left -= node_stack.len();

        if node_stack.is_empty() && nodes_left > 0 {
            return Err(SortError::CycleDependencies);
        }

        node_stacks.push(node_stack);
    }

    Ok(node_stacks)
}

fn into_rely_map<'a, K: 'a>(nodes: &[NodePacked<'a, K>]) -> Vec<Option<Vec<usize>>> {
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

    let mut rely_map: Vec<Option<Vec<usize>>> = vec![None; nodes.len()];

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

#[cfg(test)]
mod tests {
    use crate::{node::Node, dependency::Dependency};

    use super::*;

    impl Node<u32> for u32 {
        fn execute(&self, out: &mut u32) {
            *out = *self;
        }
    }

    #[test]
    fn proper_sort_by_dependencies() {
        struct A; struct B;

        let nodes: Vec<NodePacked<u32>> = vec![
            NodePacked::new(0, [Dependency::read_of::<A>()].into_iter().collect()),
            NodePacked::new(1, [Dependency::read_of::<B>()].into_iter().collect()),
            NodePacked::new(2, [Dependency::write_of::<A>(), Dependency::write_of::<B>()].into_iter().collect())
            ];
        
        let sorted = sort_by_dependencies(nodes).unwrap();
        let mut check_val = 3;

        assert_eq!(sorted.len(), 2);
        assert_eq!(sorted[0].len(), 1);
        assert_eq!(sorted[1].len(), 2);

        sorted[0][0].inner_ref().execute(&mut check_val);
        assert_eq!(check_val, 2);
        sorted[1][1].inner_ref().execute(&mut check_val);
        assert!(check_val == 1 || check_val == 0);
        sorted[1][0].inner_ref().execute(&mut check_val);
        assert!(check_val == 0 || check_val == 1);
    }
}