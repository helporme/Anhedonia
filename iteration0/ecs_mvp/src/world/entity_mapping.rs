use std::collections::HashMap;

use crate::{
    entity::Entity,
    archetype::ArchetypeChunk
};


const ENTITY_BUCKET_SIZE: usize = 64;
const ENTITY_BUCKET_SIZE_U64: u64 = ENTITY_BUCKET_SIZE as u64;

pub type EntityBucket<'a> = [Option<EntityLocation<'a>>; ENTITY_BUCKET_SIZE];


#[derive(Default)]
pub struct EntityMapping<'a> {
    buckets: HashMap<u64, EntityBucket<'a>>,
    length: usize
}

impl<'a> EntityMapping<'a> {
    pub fn put(&mut self, entity: Entity, location: EntityLocation<'a>) {
        let bucket_idx = entity.id / ENTITY_BUCKET_SIZE_U64;
        let in_bucket_idx = (entity.id % ENTITY_BUCKET_SIZE_U64) as usize;
        
        let bucket = self.buckets
            .entry(bucket_idx)
            .or_insert_with(|| [None; ENTITY_BUCKET_SIZE]);

        bucket[in_bucket_idx] = Some(location);
        self.length += 1;
    }

    pub fn fill(&mut self, entities: &[Entity], location: EntityLocation<'a>) {
        let last_bucket_idx = u64::MAX;
        let mut last_bucket: Option<&mut EntityBucket> = None;

        for entity in entities {
            let bucket_idx = entity.id / ENTITY_BUCKET_SIZE_U64;
            let in_bucket_idx = (entity.id % ENTITY_BUCKET_SIZE_U64) as usize;

            if bucket_idx != last_bucket_idx {
                last_bucket = Some(
                    self.buckets
                    .entry(bucket_idx)
                    .or_insert_with(|| [None; ENTITY_BUCKET_SIZE])
                );
            }
            
            if let Some(ref mut bucket) = last_bucket {
                let location = location.offset_entity_idx(1);
                bucket[in_bucket_idx] = Some(location);
            }
        }

        self.length += entities.len();
    }

    pub fn get(&self, entity: Entity) -> Option<EntityLocation<'a>> {
        let bucket_idx = entity.id / ENTITY_BUCKET_SIZE_U64;

        if let Some(bucket) = self.buckets.get(&bucket_idx) {
            let in_bucket_idx = (entity.id % ENTITY_BUCKET_SIZE_U64) as usize;
            return bucket[in_bucket_idx]
        }

        None
    }

    pub fn remove(&mut self, entity: Entity) {
        let bucket_idx = entity.id / ENTITY_BUCKET_SIZE_U64;

        if let Some(bucket) = self.buckets.get_mut(&bucket_idx) {
            let in_bucket_idx = (entity.id % ENTITY_BUCKET_SIZE_U64) as usize;
            bucket[in_bucket_idx] = None;
            self.length -= 1;
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

#[derive(Clone, Copy)]
pub struct EntityLocation<'a> {
    chunk: &'a ArchetypeChunk<'a>,
    entity_idx: u32,
}

impl<'a> EntityLocation<'a> {
    pub fn new(chunk: &'a ArchetypeChunk, entity_idx: u32) -> Self {
        Self {
            chunk,
            entity_idx
        }
    }

    pub fn offset_entity_idx(&self, idx_offset: u32) -> Self {
        Self {
            chunk: self.chunk,
            entity_idx: self.entity_idx + idx_offset
        }
    }

    pub fn chunk(&self) -> &'a ArchetypeChunk<'a> {
        &self.chunk
    }

    pub fn entity_idx(&self) -> u32 {
        self.entity_idx
    }
}

