use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::collections::hash_map::{Iter, IterMut};
use std::marker::PhantomData;

use Entity;
use Component;


pub struct ComponentMap<C: Component, E: Entity> {
    map: HashMap<E, C>,
}

impl<C: Component, E: Entity> ComponentMap<C, E> {
    pub fn new() -> ComponentMap<C, E> {
        ComponentMap { map: HashMap::new() }
    }
    
    pub fn borrow(&self, entity: &E) -> Option<&C> {
        self.map.get(entity)
    }
    
    pub fn borrow_mut(&mut self, entity: &E) -> Option<&mut C> {
        self.map.get_mut(entity)
    }
    
    pub fn get(&self, entity: &E) -> Option<C> where C: Clone {
        self.borrow(entity).cloned()
    }
    
    pub fn insert(&mut self, entity: E, value: C) -> Option<C> {
        self.map.insert(entity, value)
    }
    
    pub fn remove(&mut self, entity: &E) -> Option<C> {
        self.map.remove(entity)
    }
    
    pub fn set(&mut self, entity: E, value: Option<C>) -> Option<C> {
        match value {
            Some(value) => self.insert(entity, value),
            None        => self.remove(&entity),
        }
    }
    
    pub fn iter(&self) -> Iter<E, C> {
        self.map.iter()
    }
    
    pub fn iter_mut(&mut self) -> IterMut<E, C> {
        self.map.iter_mut()
    }
}


pub struct Map<E: Entity> {
    map: HashMap<TypeId, Box<Any>>,
    phan: PhantomData<E>,
}

impl<E: Entity> Map<E> {
    pub fn new() -> Map<E> {
        Map {
            map: HashMap::new(),
            phan: PhantomData,
        }
    }
    
    pub fn component_map_get<C: Component>(&self) -> Option<&ComponentMap<C, E>> {
        self.map
            .get(&TypeId::of::<C>())
            .and_then(|b| b.downcast_ref::<ComponentMap<C, E>>())
    }
    
    pub fn component_map_get_mut<C: Component>(&mut self) -> Option<&mut ComponentMap<C, E>> {
        self.map
            .get_mut(&TypeId::of::<C>())
            .and_then(|m| m.downcast_mut::<ComponentMap<C, E>>())
    }
    
    pub fn component_map_get_or_create<C: Component>(&mut self) -> &mut ComponentMap<C, E> {
        self.map
            .entry(TypeId::of::<C>())
            .or_insert_with(|| Box::new(ComponentMap::<C, E>::new()))
            .downcast_mut::<ComponentMap<C, E>>()
            .expect("downcast to component map")
    }
    
    pub fn borrow<C: Component>(&self, entity: E) -> Option<&C> {
        self.component_map_get()
            .and_then(|m| m.borrow(&entity))
    }
    
    pub fn borrow_mut<C: Component>(&mut self, entity: E) -> Option<&mut C> {
        self.component_map_get_mut()
            .and_then(|m| m.borrow_mut(&entity))
    }
    
    pub fn get<C: Component + Clone>(&self, entity: E) -> Option<C> {
        self.component_map_get()
            .and_then(|m| m.get(&entity))
    }
    
    pub fn insert<C: Component>(&mut self, entity: E, value: C) -> Option<C> {
        self.component_map_get_or_create()
            .insert(entity, value)
    }
    
    pub fn remove<C: Component>(&mut self, entity: E) -> Option<C> {
        self.component_map_get_mut()
            .and_then(|m| m.remove(&entity))
    }
    
    pub fn set<C: Component>(&mut self, entity: E, value: Option<C>) -> Option<C> {
        self.component_map_get_or_create()
            .set(entity, value)
    }
}
