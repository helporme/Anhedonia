
// Add resources to the graph
macro_rules! add_resources {
    ($graph:ident, $($val:ident),*) => {
        $($graph.kit_mut().any_storage.insert($val);)*
    };
}

// Implement a unit struct for each identifier
macro_rules! impl_resources {
    ($($val:ident),*) => {
        $(struct $val;)*
    };
}

// Build the graph with systems
macro_rules! build_graph {
    ($($system:ident),*) => {
        GarphBuilder::new()
            $(.with_system($system))*
            .with_kit(macros::composed!(any_storage: AnyStorage => AnyStorage::new()))
            .with_sorter(graph_frame::sorting::stacks_sorter())
            .build()
    };
}
