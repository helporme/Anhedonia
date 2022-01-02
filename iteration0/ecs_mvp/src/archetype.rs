use std::{
    alloc::Layout,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher}
};

use super::{
    component::ComponentType,
    entity::Entity
};

const ARCHETYPE_CHUNK_SIZE: usize = 16000;
const ARCHETYPE_CHUNK_SIZE_U32: u32 = ARCHETYPE_CHUNK_SIZE as u32;

#[derive(Debug)]
pub struct Archetype<'a> {
    layout: ArchetypeLayout,
    chunks: Vec<ArchetypeChunk<'a>>
}

impl From<ArchetypeLayout> for Archetype<'_> {
    fn from(layout: ArchetypeLayout) -> Self {
        Self {
            layout,
            chunks: Vec::default(),
        }
    }
}

#[derive(Debug)]
pub struct ArchetypeLayout {
    id: u64,
    components: Vec<ComponentType>,
    alloc_layout: Layout,
    chunk_capacity: u32
}

impl ArchetypeLayout {
    pub fn from(mut components: Vec<ComponentType>) -> Self {
        components.sort_by_key(|c| c.id().clone());

        let id = ArchetypeLayout::calc_archetype_id(&components);
        let chunk_capacity = ArchetypeLayout::calc_chunk_capacity(&components);
        let alloc_layout = ArchetypeLayout::create_alloc_layout(&components);

        Self {
            id,
            components,
            alloc_layout,
            chunk_capacity 
        }
    }
    
    fn create_alloc_layout(components: &Vec<ComponentType>) -> Layout {
        let align: usize = components.iter().map(|c| c.align()).max().unwrap();
        Layout::from_size_align(ARCHETYPE_CHUNK_SIZE, align).unwrap()
    }

    fn calc_chunk_capacity(components: &Vec<ComponentType>) -> u32 {
        let components_size: u32 = components.iter().map(|c| c.size() as u32).sum();
        // Non float div
        ARCHETYPE_CHUNK_SIZE_U32 / components_size
    }

    fn calc_archetype_id(components: &[ComponentType]) -> u64 {
        let mut hasher = DefaultHasher::default();
        
        for component in components {
            component.hash(&mut hasher);
        }

        hasher.finish()
    }

    pub fn components(&self) -> &Vec<ComponentType> {
        &self.components
    }

    pub fn alloc_layout(&self) -> &Layout {
        &self.alloc_layout
    }

    pub fn chunk_capacity(&self) -> u32 {
        self.chunk_capacity
    }
}

pub struct ArchetypeChunk<'a> {
    layout: &'a ArchetypeLayout,
    entities: Vec<Entity>,
    ptr: *mut u8,
    component_slices: Vec<ComponentSlice<'a>>
}

// impl<'a> ArchetypeChunk<'a> {
//     fn new(layout: &'a ArchetypeLayout, ptr: *mut u8) -> Self {
//         let entities = Vec::default();
//         Self {
//             layout,
//             entities,
//             ptr
//         }
//     }
// }
// 
pub struct ComponentSlice<'a> {
    bytes: &'a mut [u8]
}

