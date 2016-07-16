use std::any::{Any, TypeId};
use std::hash::Hash;
use std::collections::HashMap;

use Entity;
use Component;


pub struct Map<E: Eq + Hash = Entity> {
    map: HashMap<TypeId, HashMap<E, Box<Any>>>,
}

impl<E: Eq + Hash> Map<E> {
    pub fn new() -> Map<E> {
        Map { map: HashMap::new() }
    }
    
    pub fn borrow<C: Component>(&self, entity: E) -> Option<&C> {
        self.map
            .get(&TypeId::of::<C>())
            .and_then(|m| m.get(&entity))
            .map(|b| b.downcast_ref::<C>().expect("downcast to C"))
    }
    
    pub fn get<C: Component + Clone>(&self, entity: E) -> Option<C> {
        self.borrow(entity).map(Clone::clone)
    }
    
    pub fn insert<C: Component>(&mut self, entity: E, value: C) -> Option<C> {
        self.map
            .entry(TypeId::of::<C>())
            .or_insert_with(HashMap::new)
            .insert(entity, Box::new(value))
            .map(|b| *b.downcast::<C>().expect("downcast to C"))
    }
    
    pub fn remove<C: Component>(&mut self, entity: E) -> Option<C> {
        self.map
            .get_mut(&TypeId::of::<C>())
            .and_then(|m| m.remove(&entity))
            .map(|b| *b.downcast::<C>().expect("downcast to C"))
    }
    
    pub fn set<C: Component>(&mut self, entity: E, value: Option<C>) -> Option<C> {
        match value {
            Some(value) => self.insert(entity, value),
            None        => self.remove(entity),
        }
    }
}
