use std::any::Any;
use std::hash::Hash;

pub mod storage;


pub trait Entity: Any + Eq + Hash + Default { }
impl<T: Any + Eq + Hash + Default> Entity for T { }

pub trait Component: Any { }
impl<T: Any> Component for T { }
