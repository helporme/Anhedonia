pub mod tuples;
pub mod references;
pub mod copied;

pub use copied::*;
pub use references::*;
pub use tuples::*;

use std::any::TypeId;
use std::hash::Hash;

use crate::dependency::DependencyWriter;

/// Trait used to describe a resource type.
pub trait Resource: 'static {}

impl<T: 'static> Resource for T {}

/// Global and unique identifier of resource.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct ResourceId(TypeId);

impl ResourceId {
    /// Get [`ResourceId`] from any [`Resource`] type.
    ///
    /// # Examples:
    /// ```rust
    /// use abstract_system_graph::resource::{Resource, ResourceId};
    ///
    /// struct MyResource();
    /// impl Resource for MyResource { }
    ///
    /// let id = ResourceId::of::<MyResource>();
    /// ```
    #[inline]
    pub fn of<R: Resource>() -> Self {
        Self(TypeId::of::<R>())
    }
}

/// Trait used to describe the ability to access a resource, such as reference.
pub trait Link: Sized {
    type Resource: Resource;

    fn write_dependencies(writer: &mut DependencyWriter);
}

pub trait ResourceLinker<'a> {
    fn acquire_ref<R: Resource>(&self) -> Option<&'a R>;
    fn acquire_mut<R: Resource>(&self) -> Option<&'a mut R>; // todo: & -> &mut
    fn has<R: Resource>(&self) -> bool;
}

pub trait LinkSupport<'a, L: Link>: ResourceLinker<'a> {
    fn acquire_link(&self) -> Option<L>;
    fn has_link(&self) -> bool;
}
