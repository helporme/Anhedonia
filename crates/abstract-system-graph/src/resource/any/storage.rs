use std::any::TypeId;
use std::collections::HashMap;

use crate::any::AnyResource;
use crate::any::links::AnyRwLock;

pub type AnyMap = HashMap<TypeId, AnyRwLock>;

#[derive(Default)]
pub struct AnyStorage {
    map: AnyMap
}

impl AnyStorage {
    pub fn ensure_lock<R: AnyResource>(&mut self) {
        self.map
            .entry(TypeId::of::<R>())
            .or_insert_with(|| AnyRwLock::new(Box::new(R::default())));
    }

    pub fn request_lock<R: AnyResource>(&self) -> Option<&AnyRwLock> {
        self.map.get(&TypeId::of::<R>())
    }
}