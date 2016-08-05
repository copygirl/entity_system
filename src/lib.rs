use std::any::Any;
use std::hash::Hash;

pub mod storage;


pub trait Entity: Any + Eq + Hash + Debug + Default {
    fn index(self) -> u32;
}
impl<T: Any + Eq + Hash + Debug + Default + Into<u32>> Entity for T
    { fn index(self) -> u32 { self.into() } }

pub trait Component: Any, Debug { }
impl<T: Any + Debug> Component for T { }

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
        { self.borrow().map(Clone::clone) }
    
    fn has<C: Component>(&self) -> Result<bool, Error> {
        match self.borrow::<C>() {
            Ok(_) => Result::Ok(true),
            Err(Error::ComponentMissing) => Result::Ok(false),
            Err(err) => Result::Err(err),
        }
    }
    
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
    
}
