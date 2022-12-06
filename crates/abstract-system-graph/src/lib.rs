pub mod system;
pub mod node;
pub mod graph;
pub mod resource;
pub mod dependency;
pub mod tasks;

pub mod prelude {
    pub use crate::node::builtin::*;
    pub use crate::graph::{graph_builder, builder::*};
}