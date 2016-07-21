use std::any::Any;
use std::hash::Hash;

pub mod storage;


pub trait Entity: Any + Eq + Hash { }
impl<T: Any + Eq + Hash> Entity for T { }

pub trait Component: Any { }
impl<T: Any> Component for T { }
