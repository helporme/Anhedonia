// I need to split this up into several files I guess

use core::slice;
// I need to figure out how to force lsp to unfold it
use std::{alloc::Layout, collections::hash_map::{HashMap, DefaultHasher}, fmt::Display, hash::{Hash, Hasher}, mem::size_of};

use crate::component::Component;

use super::{
    component::ComponentType,
    entity::Entity
};

// Dynamic chunk size based on user tips?
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
    // Not the worst idea, but I need to think a little more how to index components 
    component_map: HashMap<ComponentType, usize>,
    alloc_layout: Layout,
    chunk_capacity: usize
}

impl ArchetypeLayout {
    pub fn new(mut components: Vec<ComponentType>) -> Self {
        components.sort_by_key(|c| c.id().clone());
        
        let id = ArchetypeLayout::calc_archetype_id(&components);
        let chunk_capacity = ArchetypeLayout::calc_chunk_capacity(&components);
        let alloc_layout = ArchetypeLayout::create_alloc_layout(&components);

        let mut component_map = HashMap::new();
        for (i, component) in components.iter().enumerate() {
            component_map.insert(component.clone(), i);
        }

        Self {
            id,
            components,
            component_map,
            alloc_layout,
            chunk_capacity 
        }
    }
    
    fn create_alloc_layout(components: &Vec<ComponentType>) -> Layout {
        let align: usize = components.iter().map(|c| c.align()).max().unwrap();
        Layout::from_size_align(ARCHETYPE_CHUNK_SIZE, align).unwrap()
    }

    fn calc_chunk_capacity(components: &Vec<ComponentType>) -> usize {
        let components_size: usize = components.iter().map(|c| c.size()).sum();
        // Non float div
        ARCHETYPE_CHUNK_SIZE / components_size
    }

    fn calc_archetype_id(components: &[ComponentType]) -> u64 {
        let mut hasher = DefaultHasher::default();
        
        for component in components {
            component.hash(&mut hasher);
        }

        hasher.finish()
    }

    pub fn component_index(&self, component: &ComponentType) -> Option<&usize> {
        self.component_map.get(component)
    }

    pub fn components(&self) -> &Vec<ComponentType> {
        &self.components
    }

    pub fn alloc_layout(&self) -> &Layout {
        &self.alloc_layout
    }

    pub fn chunk_capacity(&self) -> usize {
        self.chunk_capacity
    }
}

#[derive(Debug)]
pub struct ArchetypeChunk<'a> {
    layout: &'a ArchetypeLayout,
    entities: Vec<Entity>,
    component_slices: Vec<ComponentSlice>, 
    ptr: *mut u8
}

impl<'a> ArchetypeChunk<'a> {
    pub fn new(layout: &'a ArchetypeLayout, ptr: *mut u8) -> Self {
        let entities: Vec<Entity> = Vec::with_capacity(layout.chunk_capacity());
        let mut component_slices: Vec<ComponentSlice> = Vec::with_capacity(layout.components().len());
        
        // I need to move it to a new function
        let mut offset = 0;
        for component in layout.components().iter() {
            component_slices.push(unsafe { ComponentSlice::new(ptr.add(offset)) });
            offset += component.size() * layout.chunk_capacity();
        }

        Self {
            layout,
            component_slices,
            entities,
            ptr
        }
    }
    
    // I need to set it as pub(crate) or even pub(super)

    pub fn push_entity(&mut self, entity: Entity) -> usize {
        debug_assert!(self.entities.len() + 1 < self.layout.chunk_capacity(), "Chunk is full, unable to push more entities");

        self.entities.push(entity);
        // *fill components uninitialized memory with the 0x00*

        // I think here can be an usize result
        self.entities.len() - 1
    }

    pub fn write_component<T: Component>(&mut self, entity_idx: usize, component: T) {
        debug_assert!(entity_idx < self.entities.len(), "Invalid entity index (not id): {}/{}",
            entity_idx, self.entities.len());

        // I need to figure out what to do with the component_index
        // parameter and the returned Option of reference of usize
        let component_idx = self.layout.component_index(&ComponentType::of::<T>()).unwrap();
        let component_slice = &mut self.component_slices[*component_idx];
        unsafe { component_slice.write(component, entity_idx); }
    }
    
    pub fn read_component<T: Component>(&self, entity_idx: usize) -> &T {
        debug_assert!(entity_idx < self.entities.len(), "Invalid entity index (not id): {}/{}",
            entity_idx, self.entities.len());

        let component_idx = self.layout.component_index(&ComponentType::of::<T>()).unwrap();
        let component_slice = &self.component_slices[*component_idx];
        unsafe { component_slice.read::<T>(entity_idx) }
    }

    pub fn read_component_mut<T: Component>(&mut self, entity_idx: usize) -> &mut T {
        debug_assert!(entity_idx < self.entities.len(), "Invalid entity index (not id): {}/{}",
            entity_idx, self.entities.len());

        let component_idx = self.layout.component_index(&ComponentType::of::<T>()).unwrap();
        let component_slice = &mut self.component_slices[*component_idx];
        unsafe { component_slice.read_mut::<T>(entity_idx) }
    }

    pub fn components<T: Component>(&self) -> &[T] {
        let component = ComponentType::of::<T>();
        let component_idx = self.layout.component_index(&component).unwrap();
        let component_slice = &self.component_slices[*component_idx];
        
        unsafe {
            component_slice.as_slice::<T>(self.layout.chunk_capacity() * component.size())
        }
    }

    pub fn components_mut<T: Component>(&mut self) -> &mut [T] {
        let component = ComponentType::of::<T>();
        let component_idx = self.layout.component_index(&component).unwrap();
        let component_slice = &mut self.component_slices[*component_idx];
        
        unsafe {
            component_slice.read_slice_mut::<T>(0, component.size() * self.layout.chunk_capacity())
        }
    }
        
    pub fn print_component_slice<T: Component>(&self) {
        let component = ComponentType::of::<T>();
        let component_idx = self.layout.component_index(&component).unwrap();
        let component_slice = &self.component_slices[*component_idx];

        println!("{:?}", unsafe {
            slice::from_raw_parts(component_slice.ptr, component.size() * self.layout.chunk_capacity()) 
        });
    }
}

// Maybe ComponentSlice<T: Component>(*mut u8) and store it somehow?
#[derive(Debug)]
pub struct ComponentSlice {
    pub ptr: *mut u8
}

impl ComponentSlice {
    pub fn new(ptr: *mut u8) -> Self {
        Self { ptr }
    }

    pub unsafe fn write<T: Component>(&mut self, component: T, idx: usize) {
        let offset = idx * size_of::<T>();
        let component_ptr = self.ptr.add(offset) as *mut T;
        *component_ptr = component;
    }

    pub unsafe fn write_slice<T: Component>(&mut self, components: &[T], idx: usize) {
        let offset = idx * size_of::<T>();
        let component_ptr = self.ptr.add(offset) as *mut T;
        component_ptr.copy_from(components.as_ptr(), components.len());
    }

    pub unsafe fn read<T: Component>(&self, idx: usize) -> &T {
        let offset = idx * size_of::<T>();
        let component_ptr = self.ptr.add(offset) as *const T;
        component_ptr.as_ref().unwrap()
    }

    pub unsafe fn read_mut<T: Component>(&mut self, idx: usize) -> &mut T {
        let offset = idx * size_of::<T>();
        let component_ptr = self.ptr.add(offset) as *mut T;
        component_ptr.as_mut().unwrap()
    }

    pub unsafe fn read_slice<T: Component>(&self, idx: usize, len: usize) -> &[T] {
        let offset = idx * size_of::<T>();
        let component_ptr = self.ptr.add(offset) as *const T;
        std::slice::from_raw_parts(component_ptr, len)
    }

    pub unsafe fn read_slice_mut<T: Component>(&mut self, idx: usize, len: usize) -> &mut [T] {
        let offset = idx * size_of::<T>();
        let component_ptr = self.ptr.add(offset) as *mut T;
        std::slice::from_raw_parts_mut(component_ptr, len)
    }

    pub unsafe fn as_slice<T: Component>(&self, len: usize) -> &[T] {
        let component_ptr = self.ptr as *const T;
        std::slice::from_raw_parts(component_ptr, len * size_of::<T>())
    }

    pub unsafe fn as_slice_mut<T: Component>(&mut self, len: usize) -> &mut [T] {
        let component_ptr = self.ptr as *mut T;
        std::slice::from_raw_parts_mut(component_ptr, len * size_of::<T>())
    }
}
