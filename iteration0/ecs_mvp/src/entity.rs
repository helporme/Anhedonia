use std::sync::atomic::{AtomicU64, Ordering};


static LAST_ENTITY_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
pub struct Entity {
    pub id: u64
}

impl Entity {
    pub fn new() -> Self {
        // Auto id must be removed
        Self {
            id: LAST_ENTITY_ID.fetch_add(1, Ordering::Relaxed)
        }
    }
}


