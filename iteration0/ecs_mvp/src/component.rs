use std::{
    any::TypeId,
    hash::{Hash, Hasher},
    mem::{align_of, size_of}
};


#[derive(Debug, Clone, Copy)]
pub struct ComponentType {
    id: TypeId,
    size: usize,
    align: usize
}

impl Hash for ComponentType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl ComponentType {
    pub fn of<T: Component>() -> Self {
        Self {
            id: TypeId::of::<T>(),
            size: size_of::<T>(),
            align: align_of::<T>(),
        }
    }
    
    pub fn id(&self) -> &TypeId {
        &self.id
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn align(&self) -> usize {
        self.align
    }
}

impl PartialEq for ComponentType {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(other.id())
    }
}

impl Eq for ComponentType {}

impl PartialOrd for ComponentType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(other.id())
    }
}

impl Ord for ComponentType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(other.id())
    }
}

pub trait Component: 'static + Sized + Send + Sync {}

impl<T: 'static + Sized + Send + Sync> Component for T {}
