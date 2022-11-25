use std::any::TypeId;

use crate::resource::any::AnyRwLock;
use crate::resource::any::links::{Lock, Ref, Mut};
use crate::resource::Linker;

use super::AnyMap;

pub struct AnyStorage {
    map: AnyMap
}

impl AnyStorage {
    pub fn contains<R: 'static>(&self) -> bool {
        self.map.contains_key(&TypeId::of::<R>())
    }

    pub fn get<R: 'static>(&self) -> Option<&AnyRwLock> {
        self.map.get(&TypeId::of::<R>())
    }
}

impl<'a, R: 'static, S: AsRef<AnyStorage>> Linker<Lock<'a, R>> for S {
    fn link(&self) -> Option<Lock<'a, R>> {
        // Currently, it is impossible to design an architecture that can explain to the
        // compiler, through generics, that self and linc's lifetimes are the same (even with GAT
        // and HKT). And it will give you the option of not adding Node's lifetime.
        //
        // There are three solutions to the problem of lifetimes:
        // 1. Don't add lifetimes to the link implementation and write the implementation
        //    through pointers. (unsafe, more complexity to implement Ref and Mut);
        // 2. Use mem::transmute to downcast a lifetime of a link. (unsafe)
        // 3. Add a lifetime to the node and make the graph, and the nodes immutable after build
        //    (not tested, significant reduction in functionality).

        // self.as_ref().get::<R>().map(|lock| Lock::new(lock)) // lifetime '1 must outlive 'a;
        todo!()
    }

    fn can_be_linked(&self) -> bool {
        self.as_ref().contains::<R>()
    }
}