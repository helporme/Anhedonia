pub mod system;
pub mod node;
pub mod graph;
pub mod resource;
pub mod dependency;
pub mod tasks;

pub use prelude::*;

pub mod prelude {
    pub use crate::system::*;
    pub use crate::resource::*;
    // pub use crate::node::{Node, StraightNode};
    pub use crate::graph::Graph;
}
