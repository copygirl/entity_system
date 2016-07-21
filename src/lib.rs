use std::any::Any;
use std::hash::Hash;

pub mod storage;


pub trait Entity: Any + Eq + Hash + Clone { }
impl<T: Any + Eq + Hash + Clone> Entity for T { }

pub trait Component: Any { }
impl<T: Any> Component for T { }
