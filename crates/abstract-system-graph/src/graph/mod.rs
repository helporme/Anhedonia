pub mod builder;

pub use builder::*;

pub trait Graph {
    type Kit;

    fn execute(&self);
}

