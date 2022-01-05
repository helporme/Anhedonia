use crate::{archetype::{ArchetypeChunk, ArchetypeLayout}, component::ComponentType, entity::Entity};

mod world;
mod entity;
mod component;
mod archetype;

fn main() { unsafe {
    let components = vec![ComponentType::of::<u32>(), ComponentType::of::<[u8; 10]>()];
    let layout = ArchetypeLayout::new(components);
    
    let chunk_ptr: *mut u8 = std::alloc::alloc(layout.alloc_layout().clone());
    let mut chunk = ArchetypeChunk::new(&layout, chunk_ptr);

    let entity_idx = chunk.push_entity(Entity::new());
    
    let arr: [u8; 10] = [255, 254, 253, 252, 251, 250, 249, 248, 247, 246];
    chunk.write_component(entity_idx, u32::MAX);
    chunk.write_component(entity_idx, arr);

    chunk.print_component_slice::<u32>();
    chunk.print_component_slice::<[u8; 10]>();
}}
