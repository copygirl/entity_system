use std::any::Any;
use std::hash::Hash;

pub mod storage;


pub trait Entity: Any + Eq + Hash + Default { }
impl<T: Any + Eq + Hash + Default> Entity for T { }

pub trait Component: Any { }
impl<T: Any> Component for T { }

pub enum Error {
    EntityMissing,
    EntityUnsupported,
    ComponentMissing,
    ComponentUnsupported,
}

pub trait EntityRef {
    
    fn borrow<C: Component>(&self) -> Result<&C, Error>;
    
    fn borrow_mut<C: Component>(&self) -> Result<&mut C, Error>;
    
    fn get<C: Component + Clone>(&self) -> Result<C, Error>
        { self.borrow().cloned() }
    
    fn insert<C: Component>(&self, value: C) -> Result<C, Error>
        { self.set(Option::Some(value)) }
    
    fn remove<C: Component>(&self) -> Result<C, Error>
        { self.set(Option::None) }
    
    fn set<C: Component>(&self, value: Option<C>) -> Result<C, Error> {
        match value {
            Some(value) => self.insert(value),
            None        => self.remove(),
        }
    }
    
    fn iter(&self) -> Iterator<Item=Box<&Any>>;
    
    fn iter_mut(&self) -> Iterator<Item=Box<&mut Any>>;
    
}
