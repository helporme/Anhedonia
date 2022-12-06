pub mod links;

pub use links::{Lock, Ref, Mut};

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::RwLock;

use crate::resource::Linker;

type AnyRwLock = RwLock<Box<dyn Any>>;
type AnyMap = HashMap<TypeId, AnyRwLock>;

#[derive(Default)]
pub struct AnyStorage {
    map: AnyMap
}

impl AnyStorage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn contains<R: 'static>(&self) -> bool {
        self.map.contains_key(&TypeId::of::<R>())
    }

    pub(crate) fn get_lock<R: 'static>(&self) -> Option<&AnyRwLock> {
        self.map.get(&TypeId::of::<R>())
    }

    pub fn insert<R: 'static>(&mut self, res: R) -> Option<R> {
        let prev_res = self.map.insert(TypeId::of::<R>(), RwLock::new(Box::new(res)));
        prev_res.map(|lock| *lock.into_inner().unwrap().downcast().unwrap())
    }

    pub fn remove<R: 'static>(&mut self) -> Option<R> {
        self.map.remove(&TypeId::of::<R>())
            .map(|lock| *lock.into_inner().unwrap().downcast().unwrap())
    }
}

impl<'_fn, R: 'static, Storage: AsRef<AnyStorage>> Linker<'_fn, Lock<'_fn, R>> for Storage {
    fn link(&'_fn self) -> Option<Lock<'_fn, R>> {
        self.as_ref().get_lock::<R>().map(|lock| Lock::new(lock))
    }

    fn can_be_linked(&self) -> bool {
        self.as_ref().contains::<R>()
    }
}

impl<'_fn, R: 'static, Storage: AsRef<AnyStorage>> Linker<'_fn, Ref<'_fn, R>> for Storage {
    fn link(&'_fn self) -> Option<Ref<'_fn, R>> {
        match self.as_ref().get_lock::<R>() {
            Some(lock) => match lock.read() {
                Ok(guard) => Some(Ref::<R>::new(guard)),
                _ => None
            },
            _ => None
        }
    }

    fn can_be_linked(&self) -> bool {
        self.as_ref().contains::<R>()
    }
}

impl<'_fn, R: 'static, Storage: AsRef<AnyStorage>> Linker<'_fn, Mut<'_fn, R>> for Storage {
    fn link(&'_fn self) -> Option<Mut<'_fn, R>> {
        match self.as_ref().get_lock::<R>() {
            Some(lock) => match lock.write() {
                Ok(guard) => Some(Mut::<R>::new(guard)),
                _ => None
            },
            _ => None
        }
    }

    fn can_be_linked(&self) -> bool {
        self.as_ref().contains::<R>()
    }
}
