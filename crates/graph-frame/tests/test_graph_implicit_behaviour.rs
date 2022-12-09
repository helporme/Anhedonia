#[macro_use]
mod utils;

use std::sync::RwLock;
use graph_frame::building::*;
use graph_frame::resource::any_storage::AnyStorage;
use graph_frame::resource::{Mut, Ref};

impl_resources!(A, B, C, D, E, F, I, O);

type Order = RwLock<Vec<&'static str>>;

#[test]
fn node_exec_only_when_res_available() {
    let mut graph = build_graph!(system_0, system_1);

    let order = Order::default();
    add_resources!(graph, order, I, B);

    graph.execute();

    let order = graph.kit_mut().any_storage.remove::<RwLock<Vec<&'static str>>>()
        .unwrap()
        .into_inner()
        .unwrap();
    
    assert_eq!(order, vec!["system_1"]);
}

#[test]
fn correct_sorted_node_exec_order() {
    let mut graph = build_graph!(system_2, system_3, system_5, system_4, system_0, system_1);

    let order = Order::default();
    add_resources!(graph, order, A, B, C, D, E, F, I, O);

    graph.execute();

    let order = graph.kit_mut().any_storage.remove::<RwLock<Vec<&'static str>>>()
        .unwrap()
        .into_inner()
        .unwrap();
    
    assert_eq!(order, (0..6).map(|i| format!("system_{}", i)).collect::<Vec<_>>());
}

#[allow(unused_variables)]
fn system_0(i: Ref<I>, a: Mut<A>, b: Mut<B>, 
    order: Ref<RwLock<Vec<&'static str>>>) {
    order.write().unwrap().push("system_0");
}

#[allow(unused_variables)]
fn system_1(i: Ref<I>, b: Mut<B>,
    order: Ref<RwLock<Vec<&'static str>>>) {
    order.write().unwrap().push("system_1");
}

#[allow(unused_variables)]
fn system_2(a: Ref<A>, f: Mut<F>, 
    order: Ref<RwLock<Vec<&'static str>>>) {
    order.write().unwrap().push("system_2");
}

#[allow(unused_variables)]
fn system_3(a: Ref<A>, b: Ref<B>, e: Mut<E>, 
    order: Ref<RwLock<Vec<&'static str>>>) {
    order.write().unwrap().push("system_3");
}

#[allow(unused_variables)]
fn system_4(b: Ref<B>, d: Mut<D>, 
    order: Ref<RwLock<Vec<&'static str>>>) {
    order.write().unwrap().push("system_4");
}

#[allow(unused_variables)]
fn system_5(f: Ref<F>, e: Ref<E>, d: Ref<D>, r: Mut<O>, 
    order: Ref<RwLock<Vec<&'static str>>>) {
    order.write().unwrap().push("system_5");
}
