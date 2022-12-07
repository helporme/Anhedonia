mod common;

use std::sync::RwLock;

use abstract_system_graph::prelude::*;
use abstract_system_graph::graph::{Graph, graph_builder};
use abstract_system_graph::resource::{Mut, Ref, Lock};

use crate::common::TestGraph;

#[test]
fn node_exec_only_when_res_available() {
    fn system0(_: Ref<u8>) {
        unreachable!()
    }

    fn system1(lock: Lock<u16>) {
        *lock.write() += 1;
    }

    let mut graph: TestGraph = graph_builder()
        .add_system(system0)
        .add_system(system1)
        .build();

    graph.kit.any_storage.insert(0u16);

    for _ in 0..3 {
        graph.execute();
    }
}

#[test]
fn correct_sorted_node_exec_order() {
    //        
    //  /-> s0 -> A -> s2 -> F -\
    // I       \                 > s5 -> O
    //  \-> s1 -> B -> s3 -> E -/
    //            \--> s4 -> D /

    struct A; struct B; struct C;
    struct D; struct E; struct F;
    struct I; struct O;

    fn system_0(i: Ref<I>, a: Mut<A>, b: Mut<B>, order: Ref<RwLock<Vec<&'static str>>>) {
        order.write().unwrap().push("system_0");
    }

    fn system_1(i: Ref<I>, b: Mut<B>, order: Ref<RwLock<Vec<&'static str>>>) {
        order.write().unwrap().push("system_1");
    }

    fn system_2(a: Ref<A>, f: Mut<F>, order: Ref<RwLock<Vec<&'static str>>>) {
        order.write().unwrap().push("system_2");
    }

    fn system_3(a: Ref<A>, b: Ref<B>, e: Mut<E>, order: Ref<RwLock<Vec<&'static str>>>) {
        order.write().unwrap().push("system_3");
    }

    fn system_4(b: Ref<B>, d: Mut<D>, order: Ref<RwLock<Vec<&'static str>>>) {
        order.write().unwrap().push("system_4");
    }

    fn system_5(f: Ref<F>, e: Ref<E>, d: Ref<D>, r: Mut<O>, order: Ref<RwLock<Vec<&'static str>>>) {
        order.write().unwrap().push("system_5");
    }

    let mut graph: TestGraph = graph_builder()
        .add_system(system_2)
        .add_system(system_3)
        .add_system(system_5)
        .add_system(system_4)
        .add_system(system_0)
        .add_system(system_1)
        .build();

    graph.kit.any_storage.insert(RwLock::new(Vec::<&'static str>::new()));
    graph.kit.any_storage.insert(A);
    graph.kit.any_storage.insert(B);
    graph.kit.any_storage.insert(C);
    graph.kit.any_storage.insert(D);
    graph.kit.any_storage.insert(E);
    graph.kit.any_storage.insert(F);
    graph.kit.any_storage.insert(I);
    graph.kit.any_storage.insert(O);

    graph.execute();

    let order = graph.kit.any_storage.remove::<RwLock<Vec<&'static str>>>()
        .unwrap()
        .into_inner()
        .unwrap();
    
    assert_eq!(order, vec!["system_0", "system_1", "system_2", "system_3", "system_4", "system_5"]);
}