mod common;

use abstract_system_graph::prelude::*;
use abstract_system_graph::graph::{Graph, graph_builder};
use abstract_system_graph::resource::{Mut, Ref};

use crate::common::TestGraph;

#[test]
fn correct_sorted_node_exec_order() {
    fn system_1(val: Ref<u32>, val1: Ref<u64>) {
        assert_eq!(*val as u64, *val1);
    }

    fn system_21(mut val: Mut<u32>) {
        *val += 1;
    }

    fn system_22(mut val: Mut<u64>) {
        *val += 1;
    }

    let mut graph: TestGraph = graph_builder()
        .add_system(system_1)
        .add_system(system_21)
        .add_system(system_22)
        .build();

    graph.kit.any_storage.insert(0u32);
    graph.kit.any_storage.insert(0u64);

    for _ in 0..100 {
        graph.execute();
    }

    let val = graph.kit.any_storage.remove::<u32>().unwrap();
    let val1 = graph.kit.any_storage.remove::<u64>().unwrap();

    assert_eq!(val, 100);
    assert_eq!(val1, 100);
}
