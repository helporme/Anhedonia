use std::any::TypeId;
use std::collections::HashSet;

// todo: doc
#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Dependency {
    id: DependencyId,
    relation: DependencyRelation
}

impl Dependency {
    pub const fn new(id: DependencyId, relation: DependencyRelation) -> Self {
        Self { id, relation }
    }

    pub fn read_of<T: 'static>() -> Self {
        Self::new(DependencyId::of::<T>(), DependencyRelation::Read)
    }

    pub fn write_of<T: 'static>() -> Self {
        Self::new(DependencyId::of::<T>(), DependencyRelation::Write)
    }

    pub const fn id(&self) -> &DependencyId {
        &self.id
    }

    pub const fn relation(&self) -> &DependencyRelation {
        &self.relation
    }
}

/// Global and unique dependency identifier based on type
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct DependencyId(TypeId);

impl DependencyId {
    #[inline]
    pub fn of<T: 'static>() -> Self {
        Self(TypeId::of::<T>())
    }
}

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum DependencyRelation {
    Read,
    Write,
}

#[derive(Default)]
pub struct DependencyWriter {
    dependencies: HashSet<Dependency>
}

impl DependencyWriter {
    pub const fn new(dependencies: HashSet<Dependency>) -> Self {
        Self { dependencies }
    }

    pub fn write(&mut self, dependency: Dependency) {
        self.dependencies.insert(dependency);
    }
}

impl From<DependencyWriter> for HashSet<Dependency> {
    fn from(writer: DependencyWriter) -> Self {
        writer.dependencies
    }
}