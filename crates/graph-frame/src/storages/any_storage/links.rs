use std::any::Any;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::linking::Link;
use crate::dependency::{Dependency, DependencyWriter};

pub type AnyRwLock = RwLock<Box<dyn Any>>;
pub type AnyRwReadGuard<'a> = RwLockReadGuard<'a, Box<dyn Any>>;
pub type AnyRwWriteGuard<'a> = RwLockWriteGuard<'a, Box<dyn Any>>;

struct AnyStorageDependency<R: 'static> {
    _marker: PhantomData<R>
}

pub struct Lock<'a, R> {
    source: &'a AnyRwLock,
    _marker: PhantomData<R>
}

impl<'a, R: 'static> Lock<'a, R> {
    pub const fn new(source: &'a AnyRwLock) -> Self {
        Self { source, _marker: PhantomData }
    }

    pub fn read(&'a self) -> Ref<'a, R> {
        Ref::new(self.source.read().unwrap())
    }

    pub fn write(&'a self) -> Mut<'a, R> {
        Mut::new(self.source.write().unwrap())
    }
}

impl<'a, R: 'static> Link for Lock<'a, R> {
    fn write_deps(writer: &mut DependencyWriter) {
        writer.write(Dependency::write_of::<AnyStorageDependency<R>>());
    }
}

impl<'a, R: 'static> Clone for Lock<'a, R> {
    fn clone(&self) -> Self {
        Self { source: self.source, _marker: PhantomData }
    }
}

pub struct Ref<'a, R> {
    source: AnyRwReadGuard<'a>,
    _marker: PhantomData<R>
}

impl<'a, R: 'static> Ref<'a, R> {
    pub const fn new(source: AnyRwReadGuard<'a>) -> Self {
        Self { source, _marker: PhantomData }
    }
}

impl<'a, R: 'static> Link for Ref<'a, R> {
    fn write_deps(writer: &mut DependencyWriter) {
        writer.write(Dependency::read_of::<AnyStorageDependency<R>>())
    }
}

impl<'a, R: 'static> Deref for Ref<'a, R> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        self.source.downcast_ref::<R>().unwrap()
    }
}

pub struct Mut<'a, R> {
    source: AnyRwWriteGuard<'a>,
    _marker: PhantomData<R>
}

impl<'a, R: 'static> Mut<'a, R> {
    pub const fn new(source: AnyRwWriteGuard<'a>) -> Self {
        Self { source, _marker: PhantomData }
    }
}

impl<'a, R: 'static> Link for Mut<'a, R> {
    fn write_deps(writer: &mut DependencyWriter) {
        writer.write(Dependency::write_of::<AnyStorageDependency<R>>())
    }
}

impl<'a, R: 'static> Deref for Mut<'a, R> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        self.source.downcast_ref::<R>().unwrap()
    }
}

impl<'a, R: 'static> DerefMut for Mut<'a, R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.source.downcast_mut::<R>().unwrap()
    }
}
