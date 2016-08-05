//! # entity_system
//! 
//! An Entity System library aimed for use in a 3D voxel building game. One
//! major focus is extensibility and generalization: Different storage methods
//! for component data may be chosen and combined, or reimplemented, without
//! breaking the basic API. Utility methods are available for accessing and
//! modifying components on entities.
//! 
//! ## Read More
//! 
//! - [Entity Systems Wiki](http://entity-systems-wiki.t-machine.org/)
//! - [Wikipedia: Entity component system](https://en.wikipedia.org/wiki/Entity_component_system)

use std::any::Any;
use std::fmt::Debug;
use std::hash::Hash;

pub mod storage;

/// Trait for entities identifiers, used as keys in storage data structures.
/// 
/// Entities, while at the heart of entity system design, are little more than
/// identifiers. It is left up to the user to pick a type fitting their need.
/// 
/// A default implementation is available for all types that can be converted
/// to `u32`.
pub trait Entity: Any + Eq + Hash + Debug + Default {
    /// Returns an integer used by certain storage implementations.
    /// The value 0 is returned for invalid or out of range entities.
    /// 
    /// Implementations should keep values packed together close to 0 and not
    /// return the same value for tow distinct entities.
    fn index(self) -> u32;
}
impl<T: Any + Eq + Hash + Debug + Default + Into<u32>> Entity for T
    { fn index(self) -> u32 { self.into() } }

/// Marker trait for types that contain component data.
/// 
/// Components represent data or simply flags associated with entities through
/// various available data structures.
/// 
/// Automatically implemented for most types.
pub trait Component: Any + Debug { }
impl<T: Any + Debug> Component for T { }

/// Error type for entity system operations.
pub enum Error {
    /// The requested entity doesn't exist.
    EntityMissing,
    /// The requested entity isn't supported in this data structure.
    EntityUnsupported,
    /// The requested component type wasn't found on the entity.
    ComponentMissing,
    /// The requested component type isn't supported in this data structure.
    ComponentUnsupported,
    /// The requested component type can only be read from this entity, not set.
    ComponentReadonly,
    /// The requested action isn't supported through this data structure.
    ActionUnsupported,
}

/// Trait which acts as a reference to an entity, providing utility functions
/// for accessing and modifying components associated with it.
pub trait EntityRef {
    
    /// Borrows the entity's component value of type C.
    fn borrow<C: Component>(&self) -> Result<&C, Error>;
    
    /// Borrows the entity's component value of type C as mutable.
    fn borrow_mut<C: Component>(&self) -> Result<&mut C, Error>;
    
    /// Returns a clone of the entity's component value of type C.
    fn get<C: Component + Clone>(&self) -> Result<C, Error>
        { self.borrow().map(Clone::clone) }
    
    /// Returns if the entity has a component of type C.
    /// 
    /// Errors other than Error::ComponentMissing will be passed through.
    fn has<C: Component>(&self) -> Result<bool, Error> {
        match self.borrow::<C>() {
            Ok(_) => Result::Ok(true),
            Err(Error::ComponentMissing) => Result::Ok(false),
            Err(err) => Result::Err(err),
        }
    }
    
    /// Associates a component value with the entity,
    /// returning the previous value (if any).
    fn add<C: Component>(&self, value: C) -> Result<Option<C>, Error>;
    
    /// Removes a component association from the entity,
    /// returning the previous value (if any).
    fn remove<C: Component>(&self) -> Result<Option<C>, Error>;
    
    /// Sets a component value associated with the entity,
    /// returning the previous value (if any).
    fn set<C: Component>(&self, value: Option<C>) -> Result<Option<C>, Error>;
    
    /// Returns an iterator over the entity's components.
    fn iter(&self) -> Iterator<Item=Box<&Any>>;
    
}
