use crate::resource::{Channel, ChannelEstablisher};
use crate::any::{AnyResource, Lock, Mut, Ref};
use crate::any::links::AnyRwLock;
use crate::any::storage::AnyStorage;

pub struct LockChannel<'a, R: AnyResource> {
    lock: Lock<'a, R>
}

impl<'a, R: AnyResource> LockChannel<'a, R> {
    pub const fn new(any_lock: &'a AnyRwLock) -> Self {
        Self { lock: Lock::new(any_lock) }
    }
}

impl<'a, R: AnyResource> Channel<'a, Lock<'a, R>> for LockChannel<'a, R> {
    fn obtain(&'a self) -> Option<Lock<'a, R>> {
        Some(self.lock.clone())
    }

    fn is_alive(&self) -> bool {
        true
    }
}

impl<'a, R: AnyResource> Channel<'a, Ref<'a, R>> for LockChannel<'a, R> {
    fn obtain(&'a self) -> Option<Ref<'a, R>> {
        Some(self.lock.read())
    }

    fn is_alive(&self) -> bool {
        true
    }
}

impl<'a, R: AnyResource> Channel<'a, Mut<'a, R>> for LockChannel<'a, R> {
    fn obtain(&'a self) -> Option<Mut<'a, R>> {
        Some(self.lock.write())
    }

    fn is_alive(&self) -> bool {
        true
    }
}

impl<'a, R: AnyResource> ChannelEstablisher<'a, LockChannel<'a, R>> for AnyStorage {
    fn configure(&mut self) {
        self.ensure_lock::<R>()
    }

    fn establish(&'a self) -> Option<LockChannel<'a, R>> {
        self.request_lock::<R>().map(|lock| LockChannel::new(lock))
    }
}
